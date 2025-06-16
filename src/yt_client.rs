use function_timer::time;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub height: u32,
    pub url: String,
    pub width: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnails {
    pub default: Thumbnail,
    pub high: Thumbnail,
    pub maxres: Option<Thumbnail>,
    pub medium: Thumbnail,
    pub standard: Option<Thumbnail>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Localized {
    pub description: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YouTubeSnippet {
    pub category_id: String,
    pub channel_id: String,
    pub channel_title: String,
    pub description: String,
    pub live_broadcast_content: String,
    pub localized: Localized,
    pub published_at: String,
    pub thumbnails: Thumbnails,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YouTubeVideoItem {
    pub etag: String,
    pub id: String,
    pub kind: String,
    pub snippet: YouTubeSnippet,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub results_per_page: u32,
    pub total_results: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YouTubeApiResponse {
    pub etag: String,
    pub kind: String,
    pub items: Vec<YouTubeVideoItem>,
    pub page_info: PageInfo,
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

    #[time]
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

    #[time]
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
