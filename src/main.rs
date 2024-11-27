use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationContext;
use ollama_rs::Ollama;
use yt_chat::consts::DEFAULT_SYSTEM_MOCK;
use yt_chat::consts::MODEL;
use yt_chat::gen_stream_print;
use yt_chat::Result;
// region:      --- Modules

// endregion:   --- Modules
#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let model = MODEL.to_string();

    let prompts = &["Why is the sky red?", "What was my first question?"];

    let mut last_ctx: Option<GenerationContext> = None;

    for prompt in prompts {
        println!("->> prompt: {prompt}");
        let mut gen_req = GenerationRequest::new(model.to_owned(), prompt.to_string())
            .system(DEFAULT_SYSTEM_MOCK.to_string());

        if let Some(last_ctx) = last_ctx.take() {
            gen_req = gen_req.context(last_ctx);
        }

        let final_data = gen_stream_print(&ollama, gen_req).await?;

        if let Some(final_data) = final_data {
            last_ctx = Some(final_data.context);
        }
    }

    Ok(())
}
