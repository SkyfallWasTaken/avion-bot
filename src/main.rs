use color_eyre::Result;
use log::{debug, info, warn};

use poise::serenity_prelude as serenity;
use serenity::gateway::ActivityData;

mod commands;
use commands::*;
mod timestamp;

//use libc::malloc_trim;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();
    #[cfg(debug_assertions)]
    {
        warn!("Debug mode enabled, loading from .env file");
        dotenvy::dotenv()?;
    }

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
                    let guild_id = std::env::var("DISCORD_TESTING_GUILD_ID")?;
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
        .activity(ActivityData::watching("over your servers :)"))
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
