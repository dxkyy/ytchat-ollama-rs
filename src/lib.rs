use futures::StreamExt;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationFinalResponseData;
use ollama_rs::Ollama;
use tokio::io::AsyncWriteExt;
pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>;

pub mod consts {
    pub const MODEL: &str = "llama3";
}

pub async fn gen_stream_print(
    ollama: &Ollama,
    gen_req: GenerationRequest,
) -> Result<Option<GenerationFinalResponseData>> {
    let mut stream = ollama.generate_stream(gen_req).await?;

    let mut stdout = tokio::io::stdout();
    let mut char_count = 0;

    while let Some(res) = stream.next().await {
        let res = res.map_err(|_| "stream_next error")?;
        let bytes = res.last().unwrap().response.as_bytes();

        // TODO: better wrapping :(
        char_count += bytes.len();
        if char_count > 80 {
            stdout.write_all(b"\n").await?;
            char_count = 0;
        }

        stdout.write_all(bytes).await?;
        stdout.flush().await;

        if let Some(final_data) = &res.last().unwrap().final_data {
            stdout.write_all(b"\n").await?;
            stdout.flush().await;
            return Ok(Some(final_data.clone()));
        }
    }

    stdout.write_all(b"\n").await?;
    stdout.flush().await;

    Ok(None)
}
