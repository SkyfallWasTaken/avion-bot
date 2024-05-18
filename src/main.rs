#![warn(rust_2018_idioms)]
use std::str::FromStr;

use color_eyre::Result;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};
use tracing_subscriber::prelude::*;

use poise::serenity_prelude as serenity;
use poise::FrameworkContext;
use serenity::{ActivityData, FullEvent, GatewayIntents};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

mod commands;
use commands::{about, avatar, balance, give, register, user_info};
mod embeds;
mod util;
//use libc::malloc_trim; malloc_trim(0) trick for performance

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
struct Data {
    pub db: PgPool,
}

#[derive(Serialize, Deserialize)]
struct Config {
    discord_token: String,
    #[serde(rename = "database_url")]
    db_url: String,
    #[serde(rename = "discord_testing_guild_id")]
    testing_guild_id: Option<String>,
    sentry_dsn: Option<String>,
}

async fn bot_main(config: Config) -> Result<()> {
    let intents = GatewayIntents::GUILD_INTEGRATIONS | GatewayIntents::GUILDS;

    let commands = vec![
        user_info(),
        about(),
        avatar(),
        balance(),
        give(),
        register(),
    ];
    for command in &commands {
        assert!(
            !(command.description.is_none() && command.subcommands.is_empty()),
            "Command `{}` has no description",
            command.name
        );
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            event_handler: |framework, event| Box::pin(event_handler(framework, event)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                debug!("Registering slash commands...");
                if let Some(testing_guild_id) = &config.testing_guild_id {
                    warn!(testing_guild_id, "Registering commands in the test guild");
                    let guild_id = serenity::GuildId::from_str(testing_guild_id.as_str())?;
                    poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        guild_id,
                    )
                    .await?;
                } else {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }

                debug!("Creating PgPool...");
                let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&config.db_url)
                    .await?;

                Ok(Data { db: pool })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(&config.discord_token, intents)
        .activity(ActivityData::watching("over your server"))
        .framework(framework)
        .await;
    client?.start().await?;

    Ok(())
}

#[allow(clippy::unused_async)]
async fn event_handler(
    _framework: FrameworkContext<'_, Data, Error>,
    event: &FullEvent,
) -> Result<(), Error> {
    if let FullEvent::Ready { data_about_bot, .. } = event {
        info!(
            "Ready! Logged in as {}#{}",
            data_about_bot.user.name,
            // Should never be None, as bots still use the "Name#0000" format instead of usernames
            data_about_bot.user.discriminator.unwrap()
        );
    }

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let _ = dotenvy::dotenv();
    let config = envy::from_env::<Config>()?;
    let _guard;

    match &config.sentry_dsn {
        Some(dsn) => {
            debug!("Initializing Sentry...");
            let options = sentry::ClientOptions::default();
            let options = sentry::apply_defaults(options);

            _guard = sentry::init((dsn.clone(), options));
        }
        _ => {
            warn!("No Sentry DSN provided, not initializing Sentry");
        }
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(sentry_tracing::layer())
        .init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(bot_main(config))?;

    Ok(())
}
