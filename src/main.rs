use std::io::stdout;
use std::io::Write;

use futures::StreamExt;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::chat::MessageRole;
use ollama_rs::Ollama;
use tokio::io::AsyncWriteExt;
use yt_chat::consts::DEFAULT_SYSTEM_MOCK;
use yt_chat::consts::MODEL;
use yt_chat::Result;
// region:      --- Modules

// endregion:   --- Modules
#[tokio::main]
async fn main() -> Result<()> {
    let mut stdout = tokio::io::stdout();
    let ollama = Ollama::default();

    let system_msg = ChatMessage::new(MessageRole::System, DEFAULT_SYSTEM_MOCK.to_string());

    let mut thread_msgs: Vec<ChatMessage> = vec![system_msg];

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
