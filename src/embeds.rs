use poise::serenity_prelude as serenity;
use serenity::{Colour, CreateEmbed};

pub fn user_not_in_db() -> CreateEmbed {
    CreateEmbed::new()
        .title("User not found")
        .description("This user may not have used Avion before.")
        .colour(Colour::RED)
}
