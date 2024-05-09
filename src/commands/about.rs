use poise::serenity_prelude as serenity;
use serenity::{Colour, CreateEmbed, CreateEmbedFooter, Timestamp};

use crate::util::image_urls;
use crate::util::timestamp::{Format as TimestampFormat, TimestampExt};
use crate::{Context, Error};

/// Information about Avion
#[poise::command(slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    // TODO: finish the command
    let build_timestamp = Timestamp::parse(env!("VERGEN_BUILD_TIMESTAMP"))?
        .to_discord_timestamp(TimestampFormat::LongDateShortTime);

    let embed = CreateEmbed::default()
        .title("About Avion")
        .field("Version", env!("CARGO_PKG_VERSION"), true)
        .field("Build timestamp", build_timestamp, true)
        .field("", "", false)
        .field("Rust version", env!("VERGEN_RUSTC_SEMVER"), true)
        .field("Git commit", format!("`{}`", env!("VERGEN_GIT_SHA")), true)
        .thumbnail(image_urls::AVION_AVATAR)
        .colour(Colour::BLUE)
        .footer(CreateEmbedFooter::new(
            "Third-party licenses: TODO (ask @justhypex)",
        ));
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
