use ollama_models_info_fetcher::{fetch_model_info, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let info = fetch_model_info("deepseek-r1").await?;
    println!("{:?}", info);
    Ok(())
}
