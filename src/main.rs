use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use yt_chat::Result;
use yt_chat::consts::MODEL;
use yt_chat::consts::DEFAULT_SYSTEM_MOCK;
// region:      --- Modules

// endregion:   --- Modules
#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let model = MODEL.to_string();
    let prompt = "What is the best programming language?".to_string();

    let gen_req = GenerationRequest::new(model, prompt).system(DEFAULT_SYSTEM_MOCK.to_string());

    //  -- Single Response Generation
    let res = ollama.generate(gen_req).await?;
    println!("->> res {}", res.response);

    Ok(())
}
