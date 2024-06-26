use poise::serenity_prelude::{GuildId, UserId};
use sqlx::PgPool;

pub struct UserBalances {
    pub bank_balance: i32,
    pub wallet_balance: i32,
}

impl UserBalances {
    pub async fn from_user_and_guild_ids(
        user_id: UserId,
        guild_id: GuildId,
        db: &PgPool,
    ) -> sqlx::Result<Self> {
        let record = sqlx::query!(
            "
    SELECT bank_balance, wallet_balance
    FROM users
    WHERE user_id = $1 AND guild_id = $2
        ",
            user_id.to_string(),
            guild_id.to_string()
        )
        .fetch_one(db)
        .await?;

        Ok(Self {
            bank_balance: record.bank_balance,
            wallet_balance: record.wallet_balance,
        })
    }
}
