use ollama_models_info_fetcher::{fetch_all_available_models, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // getting all models that are available on the Ollama.com !!!
    let all_models = fetch_all_available_models().await?;

    // iterating over all models names and printing them!
    for name in all_models {
        println!("{}", name);
    }
    Ok(())
}
