# ollama_models_info_fetcher [![License: MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE) [![ollama_models_info_fetcher on crates.io](https://img.shields.io/crates/v/ollama_models_info_fetcher)](https://crates.io/crates/ollama_models_info_fetcher) [![ollama_models_info_fetcher on docs.rs](https://docs.rs/ollama_models_info_fetcher/badge.svg)](https://docs.rs/ollama_models_info_fetcher)

ollama model information fetcher.

for getting every single info about ollama models,  those models are hosted on ***[Ollama][__link0]***

## Examples

these examples are the most you will need to use !!

### Example 1

You can get all models names available that are hosted on the ***[Ollama][__link1]*** website.

```rust
use ollama_models_info_fetcher::{fetch_all_available_models, OResult};

#[tokio::main]
async fn main() -> OResult<()> {
    // getting all models that are available on the Ollama.com !!!
    let all_models = fetch_all_available_models().await?;

    // iterating over all models names and printing them!
    for name in all_models {
        println!("{}", name);
    }
    Ok(())
}

```

### Example 2

you can fetch and get a model info by feeding it a name !

```rust
use ollama_models_info_fetcher::{fetch_model_info, OResult};

#[tokio::main]
async fn main() -> OResult<()> {
   let info = fetch_model_info("deepseek-r1").await?;
   println!("{:?}", info);
   Ok(())
}

```

### Example 3

you can easily convert a model to json string!

```rust
use ollama_models_info_fetcher::{convert_to_json, fetch_model_info, OResult};

#[tokio::main]
async fn main() -> OResult<()> {
    let model = fetch_model_info("deepseek-r1").await?;
    let json = convert_to_json(&model)?;
    print!("{}", json);
    Ok(())
}

```

### Example 4

this example finalize everything , here you can export every available model to json file.

```rust
use ollama_models_info_fetcher::{
    convert_to_json, fetch_all_available_models, fetch_model_info, OResult,
};

use std::{fs::File, io::Write};

#[tokio::main]
async fn main() -> OResult<()> {
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

```


 [__link0]: https://ollama.com/library
 [__link1]: https://ollama.com/library
