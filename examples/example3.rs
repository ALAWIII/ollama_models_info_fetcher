use ollama_models_info_fetcher::{convert_to_json, fetch_model_info, OResult};

#[tokio::main]
async fn main() -> OResult<()> {
    let model = fetch_model_info("deepseek-r1").await?;
    let json = convert_to_json(&model)?;
    print!("{}", json);
    Ok(())
}
