//! fetching the tags or varients available of a given model and their acutal sizes.
use regex::Regex;

use crate::{anyhow, create_selector, Result, Varient};
use scraper::Html;

pub(super) fn get_varient(page: Html, n_tags: usize) -> Result<Vec<Varient>> {
    let size_regex = Regex::new(r"(\d+(\.\d+)?(GB|MB|KB))").unwrap();
    let main_s = create_selector("main")?;
    let section = create_selector("section")?;

    let list_div = create_selector(r#"div.min-w-full.divide-y.divide-gray-200"#)?;
    //let div = Selector::parse("div")?;

    let tag_div =
        create_selector(r#"div.break-all.font-medium.text-gray-900.group-hover\:underline"#)?;
    let size_div =
        create_selector(r#"div.flex.items-baseline.space-x-1.text-\[13px\].text-neutral-500"#)?;
    let span_size_div = create_selector("span.font-mono")?;

    let tags = page
        .select(&main_s)
        .next()
        .ok_or("Failed to get main element!")
        .map_err(|e| anyhow!(format!("{e}")))? //getting the main element
        .select(&section)
        .next()
        .ok_or("Failed to get section element!")
        .map_err(|e| anyhow!(format!("{e}")))? // successfully getting the section element
        .select(&list_div)
        .next()
        .ok_or("Failed to get the list of divs!")
        .map_err(|e| anyhow!(format!("{e}")))?;
    let token_sizes = tags
        .select(&tag_div)
        .take(n_tags)
        .map(|t| -> Result<String> {
            Ok(t.first_child()
                .ok_or("Failed to get the tag child!")
                .map_err(|e| anyhow!(format!("{e}")))?
                .value()
                .as_text()
                .ok_or("Failed to get the text of tag!")
                .map_err(|e| anyhow!(format!("{e}")))?
                .to_string()
                .trim()
                .to_string())
        })
        .filter_map(Result::ok)
        .collect::<Vec<String>>();
    let size = tags
        .select(&size_div)
        .take(n_tags)
        .map(|t| -> Result<String> {
            let t_size = t
                .select(&span_size_div)
                .next()
                .ok_or("Failed to get the span!")
                .map_err(|e| anyhow!(format!("{e}")))?
                .next_sibling()
                .ok_or("Failed to get the size in GB!")
                .map_err(|e| anyhow!(format!("{e}")))?
                .value()
                .as_text()
                .ok_or("Failed to get the text of size!")
                .map_err(|e| anyhow!(format!("{e}")))?
                .to_string();

            Ok(size_regex
                .captures(&t_size)
                .ok_or("Failed to extract the size!")
                .map_err(|e| anyhow!(format!("{e}")))?
                .get(1)
                .ok_or("Failed to get the value of size!")
                .map_err(|e| anyhow!(format!("{e}")))?
                .as_str()
                .to_string())
        })
        .filter_map(Result::ok)
        .collect::<Vec<String>>();
    let varients = token_sizes
        .into_iter()
        .zip(size)
        .map(|(t, s)| Varient::new(&t, &s))
        .collect::<Vec<Varient>>();

    Ok(varients)
}

#[cfg(test)]
mod varient_test {
    use crate::{Result, Varient};

    use super::super::get_model_page;
    use super::get_varient;

    #[should_panic]
    #[tokio::test]
    async fn get_tag_size() {
        let page = get_model_page("deepseek-r1/tags").await.unwrap();
        let tags = get_varient(page, 10);
        let expected_varients = [
            ("1.5b", "1.1GB"),
            ("7b", "4.7GB"),
            ("8b", "4.9GB"),
            ("14b", "9.0GB"),
            ("32b", "20GB"),
            ("70b", "43GB"),
            ("671b", "404GB"),
        ]
        .map(|(tag, size)| Varient::new(tag, size));
        assert!(tags.is_ok());
        assert_eq!(tags.unwrap(), &expected_varients);
    }

    #[tokio::test]
    async fn test_get_var2() -> Result<()> {
        let page = get_model_page("qwen2.5/tags").await?;
        let tags = get_varient(page, 10)?;
        dbg!(tags);

        Ok(())
    }
}
