use color_eyre::Result;
use log::{debug, info, warn};

use poise::serenity_prelude as serenity;
use serenity::model::Colour;
mod timestamp;
use timestamp::{Format as TimestampFormat, TimestampExt};

//use libc::malloc_trim;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use std::str::FromStr;

/// Displays your or another user's username, avatar, and account creation date
#[poise::command(slash_command, rename = "userinfo")]
async fn user_info(
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

/// Information about Avion
#[poise::command(slash_command)]
async fn about(ctx: Context<'_>) -> Result<(), Error> {
    // TODO: finish the command
    ctx.say("Avion 0.0.1").await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();
    dotenvy::dotenv()?;

    let token = std::env::var("DISCORD_TOKEN")?;
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![user_info(), about()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                debug!("Registering slash commands...");
                if cfg!(debug_assertions) {
                    let guild_id = std::env::var("DISCORD_SERVER_ID")?;
                    warn!("In debug - will register commands in the test guild ({guild_id})");
                    let id = serenity::GuildId::from_str(guild_id.as_str())?;
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, id)
                        .await?;
                } else {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }

                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client?.start().await?;

    Ok(())
}

async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            info!("Logged in as {}", data_about_bot.user.name);
        }
        _ => {}
    };
    Ok(())
}
