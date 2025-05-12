//! fetching the tags or varients available of a given model and their acutal sizes.

use crate::{anyhow, create_selector, Result, Varient};
use scraper::Html;

pub(super) fn get_varient(page: Html, n_tags: usize) -> Result<Vec<Varient>> {
    let main_s = create_selector("main")?;
    let section = create_selector("section")?;
    let ul = create_selector("ul")?;
    // get all li in iterator
    let div_t_s = create_selector(r#"div.flex.flex-col.space-y-\[6px\].col-span-12"#)?;
    let a = create_selector("a")?;
    let div_s = create_selector("div.grid.grid-cols-12.text-neutral-500.text-sm.items-center")?;
    let li = create_selector("li")?;
    let general_div = create_selector("div")?;
    let tags = page
        .select(&main_s)
        .next()
        .ok_or("Failed to get main element!")
        .map_err(|e| anyhow!(format!("{e}")))? //getting the main element
        .select(&section)
        .next()
        .ok_or("Failed to get section element!")
        .map_err(|e| anyhow!(format!("{e}")))? // successfully getting the section element
        .select(&ul)
        .next()
        .ok_or("failed to get the ul element")
        .map_err(|e| anyhow!(format!("{e}")))?
        .select(&li)
        .take(n_tags);
    let token_sizes: Vec<_> = tags
        .clone()
        .filter_map(|l| l.select(&a).next())
        .map(|a| {
            a.attr("href")
                .unwrap()
                .split(":")
                .last()
                .unwrap()
                .to_owned()
        })
        .collect();
    let size: Vec<_> = tags
        .filter_map(|l| l.select(&div_t_s).next()) // fetches the outer div
        .filter_map(|d| d.select(&div_s).next()) // fetches the second inner one
        .filter_map(|d| d.select(&general_div).next()) // fetches the first one
        .filter_map(|d| d.select(&general_div).nth(1)) // get the div that contains the text!
        .map(|t| t.first_child().unwrap().value().as_text().unwrap().trim())
        .collect();
    let varients = token_sizes
        .into_iter()
        .zip(size)
        .map(|(t, s)| Varient::new(&t, s))
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
        let page = get_model_page("gemma3/tags").await?;

        let tags = get_varient(page, 10)?;
        dbg!(tags);

        Ok(())
    }
}
