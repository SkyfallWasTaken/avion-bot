use crate::embeds;
use crate::{Context, Data, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbedAuthor};
use serenity::Colour;

#[derive(Debug, poise::Modal)]
#[allow(dead_code)] // fields only used for Debug print
struct MyModal {
    first_input: String,
    second_input: Option<String>,
}
#[poise::command(slash_command)]
pub async fn modal(ctx: poise::ApplicationContext<'_, Data, Error>) -> Result<(), Error> {
    use poise::Modal as _;

    let data = MyModal::execute(ctx).await?;
    println!("Got data: {:?}", data);

    Ok(())
}

/// Gets a user's balance in the server.
#[poise::command(slash_command, guild_only)]
pub async fn give(
    ctx: Context<'_>,
    #[description = "Selected user"] user: serenity::User,
) -> Result<(), Error> {
    if user.bot {
        ctx.send(poise::CreateReply::default().embed(embeds::bots_not_allowed()))
            .await?;
        return Ok(());
    }
    if user.id == ctx.author().id {
        ctx.send(poise::CreateReply::default().embed(embeds::cannot_use_yourself()))
            .await?;
        return Ok(());
    }

    // the command is server only
    let guild = ctx
        .guild_id()
        .unwrap()
        .to_partial_guild(&ctx.http())
        .await?;
    let guild_icon_url = guild.icon_url().unwrap_or_default();
    let db = &ctx.data().db;

    let balances_record = sqlx::query!(
        "
SELECT wallet_balance
FROM users
WHERE user_id = $1 AND guild_id = $2
        ",
        user.id.to_string(),
        guild.id.to_string()
    )
    .fetch_one(db)
    .await;

    let balances = match balances_record {
        Ok(record) => record,
        Err(sqlx::Error::RowNotFound) => {
            ctx.send(poise::CreateReply::default().embed(embeds::user_not_in_db()))
                .await?;
            return Ok(());
        }
        Err(err) => return Err(Box::new(err)),
    };

    let embed = serenity::CreateEmbed::new()
        .title(format!("Give to @{username}?", username = user.name))
        .field("Wallet Balance", balances.wallet_balance.to_string(), true)
        .author(CreateEmbedAuthor::new(guild.name).icon_url(guild_icon_url))
        .colour(Colour::GOLD);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
