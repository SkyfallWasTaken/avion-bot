use color_eyre::Result;
use poise::serenity_prelude as serenity;
use sqlx::PgPool;

pub fn get_user_balances(user_id: serenity::UserId, guild_id: serenity::GuildId) -> Result<Record> {
    Ok(sqlx::query!(
        "
SELECT wallet_balance
FROM users
WHERE user_id = $1 AND guild_id = $2
    ",
        user_id.to_string(),
        guild_id.to_string()
    )
    .fetch_one(db)
    .await?)
}
