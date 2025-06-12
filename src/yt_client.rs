use reqwest::Client;

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct YouTubeSnippet {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct YouTubeVideoItem {
    pub id: String,
    pub snippet: YouTubeSnippet,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct YouTubeApiResponse {
    pub items: Vec<YouTubeVideoItem>,
}

#[derive(Clone)]
pub struct YouTubeClient {
    client: Client,
    api_key: String,
}

impl YouTubeClient {
    pub fn new(api_key: String) -> Self {
        YouTubeClient {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn is_valid_video_id(&self, video_id: &str) -> bool {
        let url = format!(
            "https://www.youtube.com/oembed?url=https://www.youtube.com/watch?v={}&format=json",
            video_id
        );

        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    pub async fn get_video_data(
        &self,
        video_id: &str,
    ) -> Result<YouTubeApiResponse, Box<dyn Error>> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}",
            video_id, self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<YouTubeApiResponse>()
            .await?;

        if response.items.is_empty() {
            return Err("Video not found.".into());
        }

        Ok(response)
    }
}
