//! this functionality is for getting the category or class where the model falls in !!

use crate::{Category, OResult};
use scraper::{Html, Selector};

/// extracts the model Category : {Tool,Vision ,Embedding,Other}
pub(super) fn get_category(page: &Html) -> OResult<Category> {
    let div_id = Selector::parse("div#summary")?;

    let div_parent = page
        .select(&div_id)
        .next()
        .ok_or("element div summary to find category has problem!")?
        .parent()
        .ok_or("failed to get the parent of div#summary element")?;

    let second_div = div_parent
        .children()
        .filter(|n| n.value().as_element().is_some())
        .nth(1) // the 2nd one is Text("\n\n\t\t")
        .ok_or("failed to get the second div element that contains the category!!")?;
    // dbg!(&second_div.value());
    let first_div_of_the_second = second_div
        .children()
        .find(|n| n.value().as_element().is_some())
        .ok_or("the div element inside category function has problem!")?;
    //dbg!(&first_div_of_the_second.value());

    let first_span = first_div_of_the_second
        .children()
        .find(|n| n.value().as_element().is_some())
        .ok_or("failed to get the first category")?;

    //dbg!(&first_span.value());

    let text = first_span
        .first_child()
        .ok_or("failed to get the text of span")?
        .value()
        .as_text();
    // dbg!(&text);
    let cat = text.map_or(Category::Other, |t| match t.to_string().as_str() {
        "tools" => Category::Tool,
        "vision" => Category::Vision,
        "embedding" => Category::Embedding,
        _ => Category::Other,
    });

    Ok(cat)
}

#[cfg(test)]
mod quick_test {
    use super::{super::get_model_page, get_category, Category};
    async fn test_category(model_name: &str, expected_cat: Category) {
        let page = get_model_page(model_name).await.unwrap();
        let cat = get_category(&page);
        //dbg!(&cat);
        assert!(cat.is_ok());
        assert_eq!(cat.unwrap(), expected_cat);
    }

    #[tokio::test]
    async fn test_cat_other() {
        test_category("deepseek-r1", Category::Other).await;
    }
    #[tokio::test]
    async fn test_cat_tool() {
        test_category("llama3.3", Category::Tool).await;
    }
    #[tokio::test]
    async fn test_cat_vision() {
        test_category("llava", Category::Vision).await;
    }
    #[tokio::test]
    async fn test_cat_embedding() {
        test_category("mxbai-embed-large", Category::Embedding).await;
    }
}
