use poise::serenity_prelude::{Colour, CreateEmbed};

pub fn user_not_in_db() -> CreateEmbed {
    CreateEmbed::new()
        .title("User not found")
        .description("This user may not have used Avion in this server before.")
        .field(
            "Is this you?",
            "If so, just run `/register` to get started.",
            false,
        )
        .colour(Colour::RED)
}

pub fn bots_not_allowed() -> CreateEmbed {
    CreateEmbed::new()
        .title("Bots not allowed with this command")
        .description("You can't use this command with bots.")
        .colour(Colour::RED)
}

pub fn cannot_use_yourself() -> CreateEmbed {
    CreateEmbed::new()
        .title("Wait a second...")
        .description("You can't use this command with yourself!")
        .colour(Colour::RED)
}
