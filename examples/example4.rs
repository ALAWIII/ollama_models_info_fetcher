use ollama_models_info_fetcher::{
    convert_to_json, fetch_all_available_models, fetch_model_info, Result,
};

use std::{fs::File, io::Write};

#[tokio::main]
async fn main() -> Result<()> {
    //creating json file to write into!
    let mut f = File::create("./models.json")?;

    let all_models = fetch_all_available_models().await?;

    let mut models_info = vec![];

    for model_name in all_models {
        models_info.push(fetch_model_info(&model_name).await?);
    }
    let to_json = convert_to_json(&models_info)?;

    f.write_all(to_json.as_bytes())?;
    Ok(())
}
