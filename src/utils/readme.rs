//! fetches the readme content of a model on the ollama website!

use crate::{anyhow, create_selector, Result};
use htmd::convert;
use scraper::Html;

pub(super) fn get_readme(page: &Html) -> Result<String> {
    let div = create_selector("div#readme")?;
    let div_body = create_selector("div")?;
    let html_portion = page
        .select(&div)
        .next()
        .ok_or("Failed to get the readme div element!!")
        .map_err(|e| anyhow!(format!("{e}")))?
        .select(&div_body)
        .nth(1)
        .ok_or("Failed to get the inner div element of readme")
        .map_err(|e| anyhow!(format!("{e}")))?;
    let inner = html_portion.inner_html();
    //dbg!(&inner);
    Ok(convert(&inner)?)
}

#[cfg(test)]
mod quick_test {
    use std::{fs::File, io::Write};

    use crate::{get_model_page, utils::readme::get_readme};

    #[tokio::test]
    async fn readme_model() {
        let page = get_model_page("notux").await.unwrap();
        let readme = get_readme(&page).unwrap();
        let expected_text = "This model is a fine-tuned version";
        // dbg!(&readme);
        assert!(readme.contains(expected_text));
    }
    #[allow(unused)]
    async fn readme_file() {
        let page = get_model_page("deepseek-r1").await.unwrap();
        let readme = get_readme(&page).unwrap();
        let mut f = File::create("./test.md").unwrap();
        f.write_all(readme.as_bytes()).unwrap();
    }
}
