use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::Colour;

#[poise::command(slash_command, subcommands("user"))]
pub async fn avatar(_: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// Gets a user's global avatar.
#[poise::command(slash_command)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Selected user - defaults to you"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let embed = serenity::CreateEmbed::new()
        .title(format!("@{username}'s avatar", username = u.name))
        .image(u.face())
        .colour(Colour::BLUE);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
