use serenity::model::timestamp::Timestamp;
use std::fmt;

/// The format in which you want the timestamp to be generated.
pub enum Format {
    ShortTime,
    LongTime,

    ShortDate,
    LongDate,
    LongDateShortTime,
    LongDateDayAndShortTime,

    Relative,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r#type = match self {
            Format::ShortTime => "t",
            Format::LongTime => "T",

            Format::ShortDate => "d",
            Format::LongDate => "D",
            Format::LongDateShortTime => "f",
            Format::LongDateDayAndShortTime => "f",

            Format::Relative => "R",
        };
        write!(f, "{type}")
    }
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
