use crate::embeds;
use crate::{Context, Data, Error};
use poise::serenity_prelude as serenity;
use serenity::{
    ButtonStyle, Colour, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedAuthor,
    CreateInteractionResponseFollowup,
};

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
        .unwrap_or_else(|| String::from("Default Icon URL"));
    let db = &ctx.data().db;

    let giver_balances = sqlx::query!(
        "
SELECT wallet_balance
FROM users
WHERE user_id = $1 AND guild_id = $2
        ",
        ctx.author().id.to_string(),
        guild.id.to_string()
    )
    .fetch_one(db)
    .await?;
    let receiver_balance_record = sqlx::query!(
        "
SELECT wallet_balance
FROM users
WHERE user_id = $1 AND guild_id = $2
    ",
        receiver.id.to_string(),
        guild.id.to_string()
    )
    .fetch_one(db)
    .await;

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
    let msg = ctx.send(reply).await?;

    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| {
            mci.data.custom_id == "confirm_give" || mci.data.custom_id == "cancel_give"
        })
        .await
    {
        match mci.data.custom_id.as_str() {
            "confirm_give" => {
                let mut transaction = db.begin().await?;

                // Increase receiver's wallet balance
                sqlx::query(
                    "UPDATE users SET wallet_balance = wallet_balance + $1 WHERE user_id = $2 AND guild_id = $3",
                )
                .bind(amount)
                .bind(receiver.id.to_string())
                .bind(guild.id.to_string())
                .execute(&mut *transaction)
                .await?;

                // Decrease giver's wallet balance
                sqlx::query(
                    "UPDATE users SET wallet_balance = wallet_balance - $1 WHERE user_id = $2 AND guild_id = $3",
                )
                .bind(amount)
                .bind(giver.id.to_string())
                .bind(guild.id.to_string())
                .execute(&mut *transaction)
                .await?;

                transaction.commit().await?;

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

                msg.edit(
                    ctx,
                    poise::CreateReply::default()
                        .embed(embed)
                        .components(vec![]),
                )
                .await?;
            }
            "cancel_give" => {}
            _ => unreachable!(),
        };
    }

    Ok(())
}
