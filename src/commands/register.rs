use crate::{Context, Error};
use poise::serenity_prelude::{Colour, CreateEmbed};

/// Register your user account in the server economy.
#[poise::command(slash_command, guild_only)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    let db = &ctx.data().db;

    let user_exists = sqlx::query!(
        "
        SELECT EXISTS(SELECT 1 FROM users WHERE user_id = $1 AND guild_id = $2)
        ",
        ctx.author().id.to_string(),
        ctx.guild_id().unwrap().to_string()
    )
    .fetch_one(db)
    .await?
    .exists
    .unwrap_or(false); // TODO: could probably change sql query to return a boolean

    if !user_exists {
        sqlx::query!(
            "
            INSERT into users (user_id, guild_id)
            VALUES ($1, $2)
            ",
            ctx.author().id.to_string(),
            ctx.guild_id().unwrap().to_string()
        )
        .execute(db)
        .await?;

        let embed = CreateEmbed::new()
            .title("Success!")
            .colour(Colour::DARK_TEAL) // FIXME: use a better color
            .description("Successfully registered your account in the server economy!");
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        let embed = CreateEmbed::new()
            .title("Error")
            .colour(Colour::RED)
            .description("You are already registered in the server economy!");
        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}
