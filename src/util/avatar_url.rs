// Based off @Wilzzu's answer on https://stackoverflow.com/questions/54556637/discord-js-gives-null-avatarurl-for-server-members-that-have-the-default-avatar

use poise::serenity_prelude as serenity;

/// Gets a user's avatar URL. If they don't have one, returns their
/// default one.
pub fn get_avatar_url(user: &serenity::User) -> String {
    match user.avatar_url() {
        Some(avatar_url) => avatar_url,
        _ => {
            let index = match user.discriminator {
                Some(discriminator) => {
                    // User is using old User#0000 system
                    (discriminator.get() % 5) as u64
                }
                None => {
                    // User is using new username system
                    (user.id.get() >> 22) % 6
                }
            };
            format!("https://cdn.discordapp.com/embed/avatars/{index}.png")
        }
    }
}
