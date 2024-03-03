use poise::serenity_prelude as serenity;
use serenity::model::timestamp::Timestamp;
use serenity::{Colour, CreateEmbed, CreateEmbedFooter};

use crate::timestamp::{Format as TimestampFormat, TimestampExt};
use crate::{Context, Error};

/// Information about Avion
#[poise::command(slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    // TODO: finish the command
    let build_timestamp = Timestamp::parse(env!("VERGEN_BUILD_TIMESTAMP"))?
        .to_discord_timestamp(TimestampFormat::LongDateShortTime);
    panic!("Everything is on fire!");
    let embed = CreateEmbed::default()
        .title("About Avion")
        .field("Version", env!("CARGO_PKG_VERSION"), true)
        .field("Build timestamp", build_timestamp, true)
        .field("", "", false)
        .field("Rust version", env!("VERGEN_RUSTC_SEMVER"), true)
        .field("Git commit", format!("`{}`", env!("VERGEN_GIT_SHA")), true)
        // TODO: add images
        // .thumbnail("https://cdn.discordapp.com/avatars/8980/8980.png")
        .colour(Colour::BLUE)
        .footer(CreateEmbedFooter::new(
            "Third-party licenses: TODO (ask @justhypex)",
        ));
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
