use poise::serenity_prelude as serenity;
use serenity::Timestamp;

/// The format in which you want the timestamp to be generated.
#[derive(strum_macros::Display)]
#[allow(unused)]
pub enum Format {
    #[strum(to_string = "t")]
    ShortTime,
    #[strum(to_string = "T")]
    LongTime,

    #[strum(to_string = "d")]
    ShortDate,
    #[strum(to_string = "D")]
    LongDate,
    #[strum(to_string = "f")]
    LongDateShortTime,
    #[strum(to_string = "F")]
    LongDateDayAndShortTime,

    #[strum(to_string = "R")]
    Relative,
}

#[allow(clippy::module_name_repetitions)]
pub trait TimestampExt {
    /// Converts a Serenity `Timestamp` into a Discord timestamp.
    ///
    /// Example:
    /// ```rs
    /// use serenity::model::timestamp::Timestamp;
    /// use crate::timestamp::Format;
    ///
    /// let serenity_timestamp = Timestamp::now();
    /// let discord_timestamp = serenity_timestamp.to_discord_timestamp(Format::LongDate);
    /// println!("{discord_timestamp}");
    /// ```
    fn to_discord_timestamp(&self, format: Format) -> String;
}

impl TimestampExt for Timestamp {
    fn to_discord_timestamp(&self, format: Format) -> String {
        let epoch = self.unix_timestamp();
        let format_string = format.to_string();
        format!("<t:{epoch}:{format_string}>")
    }
}
