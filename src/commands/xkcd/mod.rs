use crate::{Context, Error};

use poise::serenity_prelude::{Colour, CreateEmbed};

mod api;
use api::Xkcd;

#[poise::command(slash_command, subcommands("today", "comic", "random"))]
#[allow(clippy::unused_async)]
pub async fn xkcd(_: Context<'_>) -> Result<(), Error> {
    unreachable!()
}

/// Get today's comic from XKCD.
#[poise::command(slash_command)]
pub async fn today(ctx: Context<'_>) -> Result<(), Error> {
    let xkcd = Xkcd::from_num(None, &ctx.data().client).await?;
    ctx.send(poise::CreateReply::default().embed(xkcd.into_embed()))
        .await?;

    Ok(())
}

/// Get a specific comic from XKCD.
#[poise::command(slash_command)]
pub async fn comic(
    ctx: Context<'_>,
    #[description = "Comic number"] num: usize,
) -> Result<(), Error> {
    if let Ok(xkcd) = Xkcd::from_num(Some(num), &ctx.data().client).await {
        ctx.send(poise::CreateReply::default().embed(xkcd.into_embed()))
            .await?;
    } else {
        let embed = CreateEmbed::default()
            .title("Comic not found")
            .description("It looks like the comic you requested does not exist. Sorry!")
            .colour(Colour::RED);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    };

    Ok(())
}

/// Get a random comic from XKCD.
#[poise::command(slash_command)]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let latest = Xkcd::from_num(None, &data.client).await?;
    let num = fastrand::usize(1..=latest.num);
    let xkcd = Xkcd::from_num(Some(num), &data.client).await?;

    ctx.send(poise::CreateReply::default().embed(xkcd.into_embed()))
        .await?;

    Ok(())
}
