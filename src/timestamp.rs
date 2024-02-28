use serenity::model::timestamp::Timestamp;

/// The format in which you want the timestamp to be generated.
#[derive(strum_macros::Display)]
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

pub trait TimestampExt {
    fn to_discord_timestamp(&self, format: Format) -> String;
}

impl TimestampExt for Timestamp {
    fn to_discord_timestamp(&self, format: Format) -> String {
        let epoch = self.unix_timestamp();
        let format_string = format.to_string();
        format!("<t:{epoch}:{format_string}>")
    }
}
