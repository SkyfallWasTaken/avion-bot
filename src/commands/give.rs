use std::time::Duration;

use poise::serenity_prelude as serenity;
use serenity::{
    ButtonStyle, Colour, ComponentInteractionDataKind, CreateActionRow, CreateButton, CreateEmbed,
    CreateEmbedAuthor, GuildId, UserId,
};

use crate::{embeds, util::db::UserBalances};
use crate::{Context, Error};

enum UserSelection {
    Confirm,
    Cancel,
}

/// Give coins to another user.
#[poise::command(slash_command, guild_only)]
pub async fn give(
    ctx: Context<'_>,
    #[description = "Selected user"] receiver: serenity::User,
    #[description = "The amount to give"]
    #[min = 1]
    amount: i32,
) -> Result<(), Error> {
    let giver = ctx.author();
    if receiver.bot {
        ctx.send(poise::CreateReply::default().embed(embeds::bots_not_allowed()))
            .await?;
        return Ok(());
    }
    if receiver.id == giver.id {
        ctx.send(poise::CreateReply::default().embed(embeds::cannot_use_yourself()))
            .await?;
        return Ok(());
    }

    let guild = ctx
        .guild_id()
        .ok_or("Guild ID not found")?
        .to_partial_guild(&ctx.http())
        .await?;
    let guild_icon_url = guild
        .icon_url()
        .unwrap_or_else(|| String::from("Default Icon URL")); // TODO: fix this
    let db = &ctx.data().db;

    let giver_balances = UserBalances::from_user_and_guild_ids(giver.id, guild.id, db).await?;
    let receiver_balance_record =
        UserBalances::from_user_and_guild_ids(receiver.id, guild.id, db).await;

    let receiver_balances = match receiver_balance_record {
        Ok(record) => record,
        Err(sqlx::Error::RowNotFound) => {
            ctx.send(poise::CreateReply::default().embed(embeds::user_not_in_db()))
                .await?;
            return Ok(());
        }
        Err(err) => return Err(Box::new(err)),
    };

    if giver_balances.wallet_balance < amount {
        let embed = CreateEmbed::new()
            .title("Not enough money")
            .description(format!(
                "You need **{remaining_coins}** more coins to give **{amount}.**",
                remaining_coins = amount - giver_balances.wallet_balance,
                amount = amount
            ))
            .author(CreateEmbedAuthor::new(guild.name).icon_url(guild_icon_url.clone()))
            .colour(Colour::RED);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    let guild_author = CreateEmbedAuthor::new(guild.name).icon_url(guild_icon_url);
    let reply = {
        let components = vec![CreateActionRow::Buttons(vec![
            CreateButton::new("confirm_give")
                .label("Give")
                .style(ButtonStyle::Success),
            CreateButton::new("cancel_give")
                .label("Cancel")
                .style(ButtonStyle::Secondary),
        ])];

        let embed = CreateEmbed::new()
            .title(format!("Give to @{username}?", username = receiver.name))
            .description(format!(
                "Are you sure you want to give **{amount}** coins to **@{username}**?\n\nYour future balances are below.",
                amount = amount,
                username = receiver.name
            ))
            .field("Your wallet balance", (giver_balances.wallet_balance - amount).to_string(), true)
            .field(format!("@{username}'s wallet balance", username = receiver.name), (receiver_balances.wallet_balance + amount).to_string(), true)
            .author(guild_author.clone())
            .colour(Colour::GOLD);

        poise::CreateReply::default()
            .embed(embed)
            .components(components)
    };

    let reply_handle = ctx.send(reply).await?;
    let m = reply_handle.message().await?;

    let Some(interaction) = m
        .await_component_interaction(&ctx.serenity_context().shard)
        .timeout(Duration::from_secs(60 * 3))
        .author_id(ctx.author().id)
        .await
    else {
        m.reply(&ctx, "Timed out").await.unwrap();
        m.delete(&ctx).await?;
        return Ok(());
    };

    let user_selection = match &interaction.data.kind {
        ComponentInteractionDataKind::Button => match interaction.data.custom_id.as_str() {
            "confirm_give" => UserSelection::Confirm,
            "cancel_give" => UserSelection::Cancel,
            _ => panic!("unexpected custom id"),
        },
        _ => panic!("unexpected interaction data kind"),
    };

    match user_selection {
        UserSelection::Confirm => {
            let transaction = PerformGive {
                giver_id: giver.id,
                receiver_id: receiver.id,
                guild_id: guild.id,
                amount,
                pool: db.clone(),
            };
            ctx.defer().await?;
            transaction.execute().await?;

            let embed = CreateEmbed::new()
                .title("Success!")
                .description("The new balances are below.")
                .field(
                    "Your wallet balance",
                    (giver_balances.wallet_balance - amount).to_string(),
                    true,
                )
                .field(
                    format!("@{username}'s wallet balance", username = receiver.name),
                    (receiver_balances.wallet_balance + amount).to_string(),
                    true,
                )
                .author(guild_author.clone())
                .colour(Colour::DARK_TEAL); // FIXME: use a better color

            let mut msg = interaction.message.clone();
            msg.edit(
                ctx,
                serenity::EditMessage::new().embed(embed).components(vec![]),
            )
            .await?;

            interaction
                .create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
                .await?;
        }
        UserSelection::Cancel => {
            let mut msg = interaction.message.clone();
            msg.edit(
                ctx,
                serenity::EditMessage::new()
                    .content("Cancelled command.")
                    .suppress_embeds(true)
                    .components(vec![]),
            )
            .await?;

            interaction
                .create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
                .await?;
        }
    }

    Ok(())
}

struct PerformGive {
    giver_id: UserId,
    receiver_id: UserId,
    guild_id: GuildId,
    amount: i32,
    pool: sqlx::PgPool,
}
impl PerformGive {
    pub async fn execute(&self) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;
        let giver_id = self.giver_id.to_string();
        let receiver_id = self.receiver_id.to_string();
        let guild_id = &self.guild_id.to_string();

        // Increase receiver's wallet balance
        sqlx::query(
            "UPDATE users SET wallet_balance = wallet_balance + $1 WHERE user_id = $2 AND guild_id = $3",
        )
        .bind(self.amount)
        .bind(receiver_id)
        .bind(guild_id)
        .execute(&mut *transaction)
        .await?;

        // Decrease giver's wallet balance
        sqlx::query(
            "UPDATE users SET wallet_balance = wallet_balance - $1 WHERE user_id = $2 AND guild_id = $3",
        )
        .bind(self.amount)
        .bind(giver_id)
        .bind(guild_id)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }
}
