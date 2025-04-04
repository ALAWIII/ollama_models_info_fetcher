//! major utils interfaces that will help in getting the right info.

use crate::{
    utils::{get_category, get_readme, get_summary_content, get_varient},
    Category, Model, ModelBuilder, ModelNotFound, OResult,
};
use once_cell::sync::Lazy;
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::to_string_pretty;
use std::sync::Arc;
static CLIENT: Lazy<Arc<Client>> = Lazy::new(|| Arc::new(Client::new()));

/// returns all available ollama models that are located on :
/// [Ollama](https://ollama.com/library/) website.
pub async fn fetch_all_available_models() -> OResult<Vec<String>> {
    let document = get_model_page("").await?;

    let div_repo = Selector::parse("div#repo")?;
    let ul_selector = Selector::parse(r#"ul[role="list"]"#)?;
    let li_selector = Selector::parse("li")?;
    let a_selector = Selector::parse("a")?;

    let div = document
        .select(&div_repo)
        .next()
        .ok_or("element div[id=repo] not found")?;
    let ul = div
        .select(&ul_selector)
        .next()
        .ok_or("ul list element not found")?;

    let lines: Vec<_> = ul
        .select(&li_selector)
        .filter_map(|li| {
            let href = li
                .select(&a_selector)
                .next()
                .and_then(|a| a.value().attr("href"))
                .ok_or("a url link or href attribute not found")
                .ok()?;

            Some(
                href.split("/")
                    .last()
                    .ok_or("Failed to extract last part of href")
                    .ok()?
                    .to_string(),
            )
        })
        .collect();

    Ok(lines)
}

/// if the model exist returns the corresponding page, otherwise returns ***error not found*** !!
pub async fn get_model_page(model_name: &str) -> OResult<Html> {
    let url = Url::parse("https://ollama.com/library/")?.join(model_name)?;
    let response = CLIENT.get(url).send().await?;
    if !response.status().is_success() {
        return Err(Box::new(ModelNotFound(format!(
            "model : {model_name} not found"
        ))));
    }
    Ok(Html::parse_document(&response.text().await?))
}

/// give it an valid existed model name and it will feed you back with all its required corresponded information!!
///
/// # Example
///
/// ```ignore
/// let model_info = fetch_model_info("deepseek-r1");
/// ```
pub async fn fetch_model_info(model_name: &str) -> OResult<Model> {
    let model_page = get_model_page(model_name).await?;
    let summary = get_summary_content(&model_page).unwrap_or("".to_owned());
    let category = get_category(&model_page).unwrap_or(Category::Other);
    let varients =
        get_varient(get_model_page(&format!("{}/tags", model_name)).await?, 10).unwrap_or_default();
    let readme = get_readme(&model_page).unwrap_or("".to_owned());

    Ok(ModelBuilder::new()
        .name(model_name)
        .summary_content(&summary)
        .category(category)
        .varients(varients)
        .readme_content(&readme)
        .build())
}
/// quick method that returns the serialized of the model to JSON format!!
pub fn convert_to_json<T: Serialize>(model: &T) -> OResult<String> {
    Ok(to_string_pretty(&model)?)
}
