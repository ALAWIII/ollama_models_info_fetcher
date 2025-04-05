//! ollama model information fetcher.
//!
//! for getting every single info about ollama models,  those models are hosted on ***[Ollama](https://ollama.com/library)***
//!
//! # Examples
//!
//! these examples are the most you will need to use !!
//!
//! ## Example 1
//! You can get all models names available that are hosted on the ***[Ollama](https://ollama.com/library)*** website.
//!```no_run
#![doc=include_str!("../examples/example1.rs")]
//!```
//! ## Example 2
//! you can fetch and get a model info by feeding it a name !
//! ```no_run
#![doc=include_str!("../examples/example2.rs")]
//! ```
//! ## Example 3
//!
//! you can easily convert a model to json string!
//! ```no_run
#![doc=include_str!("../examples/example3.rs")]
//! ```
//!
//! ## Example 4
//!
//! this example finalize everything , here you can export every available model to json file.
//!```no_run
#![doc=include_str!("../examples/example4.rs")]
//!```
pub use anyhow::{anyhow, Result};
mod errors;
pub use errors::*;
mod model_config;
pub use model_config::*;
mod utils;
use scraper::Selector;
pub use utils::*;

pub(crate) fn create_selector(selector_name: &str) -> Result<Selector> {
    Selector::parse(selector_name).map_err(|e| anyhow!(format!("Selector parse error: {e}")))
}
