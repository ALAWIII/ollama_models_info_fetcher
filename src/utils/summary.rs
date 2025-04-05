//! fetching the summary content from the top of a given model page.

use crate::{anyhow, create_selector, Result};
use scraper::Html;

/// gets the summary content of a given model page !!!.
pub(super) fn get_summary_content(page: &Html) -> Result<String> {
    let div_id = create_selector("div#summary")?;
    let h2_id = create_selector("h2#summary-display")?;
    let span_id = create_selector("span#summary-content")?;
    let content = page
        .select(&div_id)
        .next()
        .ok_or("element div summary has problem!")
        .map_err(|e| anyhow!(format!("{e}")))?
        .select(&h2_id)
        .next()
        .ok_or("element h2 summary has problem!")
        .map_err(|e| anyhow!(format!("{e}")))?
        .select(&span_id)
        .next()
        .ok_or("element span summary has problem!")
        .map_err(|e| anyhow!(format!("{e}")))?
        .first_child()
        .ok_or("no text summary found !")
        .map_err(|e| anyhow!(format!("{e}")))?
        .value()
        .as_text()
        .ok_or("the first child of span is not text")
        .map_err(|e| anyhow!(format!("{e}")))?
        .trim();

    Ok(content.into())
}

#[cfg(test)]
mod quick_test {
    use super::{super::get_model_page, get_summary_content};
    #[tokio::test]
    async fn summary_cont() {
        let page = get_model_page("deepseek-r1").await.unwrap();
        let summary = get_summary_content(&page).unwrap();
        let expected_summary = "DeepSeek's first-generation of reasoning models with comparable performance to OpenAI-o1, including six dense models distilled from DeepSeek-R1 based on Llama and Qwen.";
        assert_eq!(summary, expected_summary);
    }
}
