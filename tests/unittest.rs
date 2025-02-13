use std::{fs::File, io::Write};

use ollama_models_info_fetcher::{
    fetch_all_available_models, fetch_model_info, get_model_page, OResult,
};
use scraper::Selector;
use serde_json::{to_string, to_string_pretty};

#[tokio::test]
async fn model_exist() {
    let hot = get_model_page("deepseek-r1").await;
    assert!(hot.is_ok());
}

#[tokio::test]
async fn model_not_found() {
    let hot = get_model_page("botato").await;
    assert!(hot.is_err());
    assert!(hot.is_err_and(|m| m.to_string() == "model : botato not found"));
}
//------------------------------------
pub async fn get_all_available_models_exp() -> Vec<String> {
    let document = get_model_page("").await.unwrap();
    let div_repo = Selector::parse("div#repo").unwrap();
    let ul_selector = Selector::parse(r#"ul[role="list"]"#).unwrap();
    let li_selector = Selector::parse("li").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let lines: Vec<_> = document
        .select(&div_repo)
        .next()
        .unwrap()
        .select(&ul_selector)
        .next()
        .unwrap()
        .select(&li_selector)
        .map(|li| {
            li.select(&a_selector)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap()
                .split("/")
                .last()
                .unwrap()
                .to_string()
        })
        .collect();

    lines
}

#[tokio::test]
async fn all_models() {
    let models = fetch_all_available_models().await;
    let expected = get_all_available_models_exp().await;
    assert!(&models.is_ok());
    let models = models.unwrap();
    assert_eq!(expected, models);
    assert_eq!(expected.len(), models.len());
}

//-------------------------------------------

#[tokio::test]
async fn fetching_model() {
    let info = fetch_model_info("sailor2").await;
    assert!(info.is_ok());
    //dbg!(&info.unwrap());
}
#[tokio::test]
async fn openhermes_edge() {
    let info = fetch_model_info("openhermes").await;
    // dbg!(info);
    assert!(info.is_ok())
}
//--------------------testing all models !! ------------------------

#[tokio::test]
async fn fetching_info_all_models() {
    let models = fetch_all_available_models().await.unwrap();
    for model_name in models {
        //println!("{model_name}");
        assert!(fetch_model_info(&model_name).await.is_ok());
    }
}
//------------------------testing json conversions-------------

#[tokio::test]
async fn json_model() -> OResult<()> {
    let model = fetch_model_info("goliath").await?;
    let jsona = to_string(&model)?;
    let expected = r#"{"name":"goliath","varients":[{"token_size":"latest","size":"66GB"}],"category":"Other","summary_content":"A language model created by combining two fine-tuned Llama 2 70B models into one.","readme_content":"A large model used by merging the layers of two models: [Xwin](https://ollama.ai/library/xwinlm) and Euryale.\n\n## References\n\n[HuggingFace](https://huggingface.co/alpindale/goliath-120b)"}"#;
    // println!("{}", jsona);
    assert_eq!(jsona, expected);
    Ok(())
}
//--------------------
#[tokio::test]
async fn write_to_file() -> OResult<()> {
    let mut f = File::create("./tosn.json")?;
    let model = fetch_model_info("goliath").await?;
    let pretty_json = to_string_pretty(&vec![&model])?;
    f.write_all(pretty_json.as_bytes())?;
    Ok(())
}
