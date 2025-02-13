//! fetching the tags or varients available of a given model and their acutal sizes.

use crate::{OResult, Varient};
use scraper::{Html, Selector};

pub(super) fn get_varient(page: &Html) -> OResult<Vec<Varient>> {
    let nav = Selector::parse("nav#tags-nav")?;
    let div = Selector::parse("div")?;
    let a = Selector::parse("a")?;
    let span = Selector::parse("span")?;

    let tags: Vec<Varient> = page
        .select(&nav)
        .next()
        .ok_or("Failed to get the nav hidden element!")?
        .select(&div)
        .next()
        .ok_or("Failed to get div of the tags")?
        .select(&a)
        .filter_map(|e| {
            let link = e.attr("href")?;
            // dbg!(link);
            if !link.ends_with("tags") {
                let token_size = e.select(&span).next()?.attr("title")?;
                // dbg!(token_size);
                let size = e
                    .select(&span)
                    .nth(2)?
                    .children()
                    .find(|t| t.value().as_text().is_some_and(|v| v.contains("GB")))?
                    .value()
                    .as_text()?;
                // dbg!(size.to_string());
                return Some(Varient::new(token_size, size));
            }
            None
        })
        .collect();
    Ok(tags)
}

#[cfg(test)]
mod varient_test {
    use crate::Varient;

    use super::super::get_model_page;
    use super::get_varient;

    #[tokio::test]
    async fn get_tag_size() {
        let page = get_model_page("deepseek-r1").await.unwrap();
        let tags = get_varient(&page);
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
}
