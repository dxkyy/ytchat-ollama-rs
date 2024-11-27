mod video_fetcher;
// region:      --- Modules
use dotenv::dotenv;
use futures::StreamExt;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::MessageRole;
use ollama_rs::Ollama;
use std::env;
use tokio::io::AsyncWriteExt;
use video_fetcher::{fetch_video_data, VideoInfo};
use yt_chat::consts::MODEL;
use yt_chat::Result;
// endregion:   --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    let mut stdout = tokio::io::stdout();
    let ollama = Ollama::default();
    dotenv().ok();
    let api_key = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set");

    let mut video_url = String::new();
    println!("Enter the YouTubevideo URL:");
    std::io::stdin()
        .read_line(&mut video_url)
        .expect("Failed to read line");
    let input = video_url
        .split("v=")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .trim();

    match fetch_video_data(&api_key, &input.to_string()).await? {
        Some(video_info) => {
            println!("title: {}", video_info.title);
            let system_msg = ChatMessage::new(
                MessageRole::System,
                "This is basically a chat with a YouTube video. The users first message will give you all neccesarry information about the video. You will answer questions about the video and with information from the video.".to_string(),
            );
            let mut thread_msgs: Vec<ChatMessage> = vec![system_msg];

            let first_msg = ChatMessage::new(
                MessageRole::User,
                format!("The following chat is about a video with the title \"{}\" by {} published on {}. The video description is: {}. The video transcript is: {}\n\n If you have understood everything, please reply with \"This conversation is about the video\"[video title]\". I am now ready to chat\n.",
                    video_info.title,
                    video_info.channel_title,
                    video_info.published_at,
                    video_info.description,
                    video_info.transcript
                ),
            );
            thread_msgs.push(first_msg);
            let chat_req = ChatMessageRequest::new(MODEL.to_string(), thread_msgs.clone());

            let msg_content = run_chat_req(&ollama, chat_req).await?;

            if let Some(content) = msg_content {
                let assistant_message = ChatMessage::new(MessageRole::Assistant, content);
                thread_msgs.push(assistant_message);
            }

            loop {
                // write a > before the user input
                stdout.write_all(b"> ").await?;
                stdout.flush().await?;
                let mut input = String::new();
                std::io::stdin().read_line(&mut input);
                let input = input.trim_end();
                if input.eq_ignore_ascii_case("exit") {
                    break;
                }

                let prompt_msg = ChatMessage::new(MessageRole::User, input.to_string());

                thread_msgs.push(prompt_msg);

                let chat_req = ChatMessageRequest::new(MODEL.to_string(), thread_msgs.clone());

                let msg_content = run_chat_req(&ollama, chat_req).await?;

                if let Some(content) = msg_content {
                    let assistant_message = ChatMessage::new(MessageRole::Assistant, content);
                    thread_msgs.push(assistant_message);
                }
            }
        }
        None => println!("No video information was found."),
    }
    Ok(())
}

pub async fn run_chat_req(ollama: &Ollama, chat_req: ChatMessageRequest) -> Result<Option<String>> {
    let mut stream = ollama.send_chat_messages_stream(chat_req).await?;

    let mut stdout = tokio::io::stdout();
    let mut char_count = 0;
    let mut current_asst_msg_elems: Vec<String> = Vec::new();

    while let Some(res) = stream.next().await {
        let res = res.map_err(|_| "stream.next error")?;

        if let Some(msg) = res.message {
            let msg_content = msg.content;

            // TODO: better wrapping :(
            char_count += msg_content.len();
            if char_count > 80 {
                stdout.write_all(b"\n").await?;
                char_count = 0;
            }

            stdout.write_all(msg_content.as_bytes()).await?;
            stdout.flush().await?;

            current_asst_msg_elems.push(msg_content);
        }

        if let Some(_final_res) = res.final_data {
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;

            let asst_content = current_asst_msg_elems.join("");
            return Ok(Some(asst_content));
        }
    }

    stdout.write_all(b"\n").await?;
    stdout.flush().await?;

    Ok(None)
}
