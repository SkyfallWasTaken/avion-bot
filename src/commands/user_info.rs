// TODO: make this a user command too
use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::model::Colour;

use crate::timestamp::{Format as TimestampFormat, TimestampExt};

/// Displays your or another user's username, avatar, and account creation date
#[poise::command(slash_command, rename = "userinfo")]
pub async fn user_info(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let account_creation_date = u
        .created_at()
        .to_discord_timestamp(TimestampFormat::LongDate);
    let formatted_username = format!("@{username}", username = u.name);
    let display_name = u.global_name.clone().unwrap_or(u.name.clone());

    let mut embed = serenity::CreateEmbed::new()
        .title(formatted_username)
        .colour(Colour::BLUE)
        .fields(vec![
            ("Display name", display_name, true),
            ("Account creation date", account_creation_date, true),
        ])
        .field("", "", false)
        .fields(vec![
            ("User ID", format!("`{}`", u.id), true),
            ("Is bot", if u.bot { "Yes" } else { "No" }.into(), true),
        ]);
    if let Some(avatar_url) = u.avatar_url() {
        embed = embed.thumbnail(avatar_url);
    }
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
