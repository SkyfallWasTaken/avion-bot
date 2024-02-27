use color_eyre::Result;
use log::{debug, info};
use poise::serenity_prelude as serenity;

mod timestamp;
use timestamp::{Format as TimestampFormat, TimestampExt};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
async fn account_age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!(
        "{}'s account was created on {} at {}",
        u.name,
        u.created_at()
            .to_discord_timestamp(TimestampFormat::LongDate),
        u.created_at()
            .to_discord_timestamp(TimestampFormat::ShortTime)
    );
    ctx.say(response).await?;
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
            commands: vec![account_age()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                debug!("Registering slash commands...");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("Started client!");
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
