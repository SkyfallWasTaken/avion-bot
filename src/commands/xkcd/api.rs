use crate::Error;
use poise::serenity_prelude::{Colour, CreateEmbed, CreateEmbedFooter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Xkcd {
    pub safe_title: String,
    pub alt: String,
    #[serde(rename = "img")]
    pub image_url: String,
    pub num: usize,
}

impl Xkcd {
    pub async fn from_num(num: Option<usize>, client: &reqwest::Client) -> Result<Self, Error> {
        let url = match num {
            Some(id) => format!("https://xkcd.com/{}/info.0.json", id),
            None => "https://xkcd.com/info.0.json".to_string(),
        };
        let response = client.get(&url).send().await?;
        let json: Xkcd = response.json().await?;

        Ok(json)
    }

    pub fn into_embed(self) -> CreateEmbed {
        CreateEmbed::new()
            .title(self.safe_title)
            .footer(CreateEmbedFooter::new(self.alt))
            .image(self.image_url)
            .colour(Colour::BLUE)
    }
}
