use crate::embeds;
use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbedAuthor};
use serenity::Colour;

/// Gets a user's balance in the server.
#[poise::command(slash_command, guild_only)]
pub async fn balance(
    ctx: Context<'_>,
    #[description = "Selected user - defaults to you"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
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
SELECT wallet_balance, bank_balance
FROM users
WHERE user_id = $1 AND guild_id = $2
        ",
        u.id.to_string(),
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
        .title(format!("@{username}'s balances", username = u.name))
        .field("Wallet Balance", balances.wallet_balance.to_string(), true)
        .field("Bank Balance", balances.bank_balance.to_string(), true)
        .author(CreateEmbedAuthor::new(guild.name).icon_url(guild_icon_url))
        .colour(Colour::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
