use reqwest::Client;
use serde_json::Value;
use ytranscript::TranscriptConfig;
use ytranscript::YoutubeTranscript;

#[derive(Debug)]
pub struct VideoInfo {
    pub title: String,
    pub description: String,
    pub channel_title: String,
    pub published_at: String,
    pub transcript: String,
    pub transcript_length: usize,
}

pub async fn fetch_video_data(
    api_key: &str,
    video_id: &str,
) -> Result<Option<VideoInfo>, Box<dyn std::error::Error>> {
    let client = Client::new();

    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}",
        video_id, api_key
    );

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        println!("API request failed with status: {}", response.status());
        println!("Response body: {}", response.text().await?);
        return Err("API request failed".into());
    }

    let json: Value = response.json().await?;

    if let Some(error) = json.get("error") {
        print!("API returned an error: {:?}", error);
        return Err("API returned an error".into());
    }

    if let Some(items) = json.get("items").and_then(|i| i.as_array()) {
        if let Some(first_item) = items.first() {
            let snippet = first_item.get("snippet").ok_or("No snippet found")?;

            let transcript = fetch_transcript(video_id, Some("en")).await?;

            let video_info = VideoInfo {
                title: snippet
                    .get("title")
                    .and_then(|t| t.as_str())
                    .unwrap_or_default()
                    .to_string(),
                description: snippet
                    .get("description")
                    .and_then(|d| d.as_str())
                    .unwrap_or_default()
                    .to_string(),
                channel_title: snippet
                    .get("channelTitle")
                    .and_then(|c| c.as_str())
                    .unwrap_or_default()
                    .to_string(),
                published_at: snippet
                    .get("publishedAt")
                    .and_then(|p| p.as_str())
                    .unwrap_or_default()
                    .to_string(),

                transcript: transcript.to_string(),
                transcript_length: transcript.len(),
            };

            return Ok(Some(video_info));
        }
    }

    Ok(None)
}

async fn fetch_transcript(
    video_id: &str,
    language: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    // Create a TranscriptConfig with the optional language
    let config = TranscriptConfig {
        lang: language.map(|s| s.to_string()),
    };

    let transcript = YoutubeTranscript::fetch_transcript(video_id, Some(config)).await?;

    // Join all transcript entries with a space between them
    let full_transcript = transcript
        .into_iter()
        .map(|entry| entry.text)
        .collect::<Vec<String>>()
        .join(" ");

    Ok(full_transcript)
}
