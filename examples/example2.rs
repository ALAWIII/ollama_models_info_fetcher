use ollama_models_info_fetcher::{fetch_model_info, OResult};

#[tokio::main]
async fn main() -> OResult<()> {
    let info = fetch_model_info("deepseek-r1").await?;
    println!("{:?}", info);
    Ok(())
}
