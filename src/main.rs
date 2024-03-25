use std::str::FromStr;

use color_eyre::Result;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use poise::serenity_prelude as serenity;
use poise::FrameworkContext;
use serenity::{ActivityData, FullEvent, GatewayIntents, Interaction};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

mod commands;
use commands::*;
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
    #[serde(rename = "DISCORD_TOKEN")]
    discord_token: String,
    #[serde(rename = "DATABASE_URL")]
    db_url: String,
    #[serde(rename = "DISCORD_TESTING_GUILD_ID")]
    testing_guild_id: Option<String>,
    #[serde(rename = "SENTRY_DSN")]
    sentry_dsn: Option<String>,
}

async fn bot_main(config: Config) -> Result<()> {
    let intents = GatewayIntents::GUILD_INTEGRATIONS | GatewayIntents::GUILDS;

    let commands = vec![user_info(), about(), avatar(), balance(), give()];
    for command in &commands {
        if command.description.is_none() && command.subcommands.is_empty() {
            panic!("Command `{}` has no description", command.name)
        }
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

async fn event_handler(
    framework: FrameworkContext<'_, Data, Error>,
    event: &FullEvent,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            info!(
                "Ready! Logged in as {}#{}",
                data_about_bot.user.name,
                // Should never be None, as bots still use the "Name#0000" format instead of usernames
                data_about_bot.user.discriminator.unwrap()
            );
        }
        FullEvent::InteractionCreate { interaction } => {
            // TODO: maybe use generics or if let?
            let user = match interaction {
                Interaction::Command(cmd) => Some(&cmd.user),
                Interaction::Component(cmd) => Some(&cmd.user),
                _ => None,
            };
            let guild_id = match interaction {
                Interaction::Command(cmd) => cmd.guild_id,
                Interaction::Component(cmd) => cmd.guild_id,
                _ => None,
            };

            if let Some(user) = user {
                if let Some(guild_id) = guild_id {
                    let db = &framework.user_data.db;
                    sqlx::query!(
                        "
                        INSERT into users (user_id, guild_id)
                        VALUES ($1, $2)
                        ON CONFLICT (user_id, guild_id) DO NOTHING
                        ",
                        user.id.to_string(),
                        guild_id.to_string()
                    )
                    .execute(db)
                    .await?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    let _ = dotenvy::dotenv();
    let config = envy::from_env::<Config>()?;

    let _guard;
    match &config.sentry_dsn {
        Some(dsn) => {
            debug!("Initializing Sentry...");
            _guard = Some(sentry::init((
                dsn.clone(),
                sentry::ClientOptions {
                    release: sentry::release_name!(),
                    ..Default::default()
                },
            )));
        }
        _ => {
            warn!("No Sentry DSN provided, not initializing Sentry");
            _guard = None;
        }
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(bot_main(config))?;

    Ok(())
}
