use std::env;
use std::str::FromStr;

use color_eyre::eyre::WrapErr;
use color_eyre::Result;
use tracing::{debug, info, warn};

use poise::serenity_prelude as serenity;
use poise::FrameworkContext;
use serenity::{ActivityData, FullEvent, Interaction};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

mod commands;
use commands::*;
mod embeds;
mod util;
//use libc::malloc_trim; malloc_trim(0) trick for performance

// User data, which is stored and accessible in all command invocations
struct Data {
    pub db: PgPool,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn bot_main() -> Result<()> {
    let token = env::var("DISCORD_TOKEN").context("env variable is `DISCORD_TOKEN`")?;
    let db_url = env::var("DATABASE_URL").context("env variable is `DATABASE_URL`")?;
    let intents = serenity::GatewayIntents::GUILD_INTEGRATIONS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![user_info(), about(), avatar(), balance()],
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

                debug!("Creating PgPool...");
                let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&db_url)
                    .await?;

                Ok(Data { db: pool })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
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

            if let Some(user) = user {
                let db = &framework.user_data.db;
                sqlx::query!(
                    "
                    INSERT INTO users (user_id)
                    VALUES ($1)
                    ON CONFLICT (user_id) DO NOTHING
                    ",
                    user.id.to_string()
                )
                .execute(db)
                .await?;
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
        warn!("SENTRY_URL not set, not initializing Sentry");
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(bot_main())?;

    Ok(())
}
