use color_eyre::Result;
use log::{debug, info, warn};

use poise::serenity_prelude as serenity;
use serenity::gateway::ActivityData;

mod commands;
use commands::*;
mod util;

use std::env;
use std::str::FromStr;

//use libc::malloc_trim; malloc_trim(0) trick for performance

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn bot_main() -> Result<()> {
    let token = env::var("DISCORD_TOKEN")?;
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![user_info(), about(), avatar()],
            event_handler: |framework, event| Box::pin(event_handler(framework, event)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                debug!("Registering slash commands...");
                if cfg!(debug_assertions) {
                    let guild_id = env::var("DISCORD_TESTING_GUILD_ID")?;
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
    _framework: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
        info!(
            "Logged in as {}#{}",
            data_about_bot.user.name,
            // Should never be None, as bots still use the "Name#0000" format instead of usernames
            data_about_bot.user.discriminator.unwrap()
        );
    }
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();
    let _ = dotenvy::dotenv();
    let _guard;

    if let Ok(sentry_url) = env::var("SENTRY_URL") {
        debug!("Initializing Sentry...");
        _guard = sentry::init((
            sentry_url,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        ));
    } else {
        warn!("SENTRY_URL not set, not initializing Sentry")
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(bot_main())?;

    Ok(())
}
