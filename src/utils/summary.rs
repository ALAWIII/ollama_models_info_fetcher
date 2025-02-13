//! fetching the summary content from the top of a given model page.

use crate::OResult;
use scraper::{Html, Selector};

/// gets the summary content of a given model page !!!.
pub(super) fn get_summary_content(page: &Html) -> OResult<String> {
    let div_id = Selector::parse("div#summary")?;
    let h2_id = Selector::parse("h2#summary-display")?;
    let span_id = Selector::parse("span#summary-content")?;
    let content = page
        .select(&div_id)
        .next()
        .ok_or("element div summary has problem!")?
        .select(&h2_id)
        .next()
        .ok_or("element h2 summary has problem!")?
        .select(&span_id)
        .next()
        .ok_or("element span summary has problem!")?
        .first_child()
        .ok_or("no text summary found !")?
        .value()
        .as_text()
        .ok_or("the first child of span is not text")?
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
