use serde::{Deserialize, Serialize};

use super::{Category, Varient};

/// represents and stores the required information!
///
/// # Examples
/// ```ignore
/// let model = Model::default();
/// ```
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    name: String,
    varients: Vec<Varient>,
    category: Category,

    summary_content: String,
    readme_content: String,
}

impl Model {
    /// getting the name of the model.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// getting the varients that represent the model (token_size , size in GB).
    /// # Example
    /// ```ignore
    /// ("7b","4.9GB")
    /// ```
    pub fn varients(&self) -> &Vec<Varient> {
        &self.varients
    }
    /// getting the category for which the model is classified.
    pub fn category(&self) -> &Category {
        &self.category
    }
    /// the summary describtion for the model in its top page.
    pub fn summary_content(&self) -> &str {
        &self.summary_content
    }
    /// the readme or the long describtion for the model .
    pub fn readme_content(&self) -> &str {
        &self.readme_content
    }
}
impl Default for Model {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            varients: vec![],
            category: Category::Other,

            summary_content: "".to_owned(),
            readme_content: "".to_owned(),
        }
    }
}

/// This is used to quickly building and instantiating the ***Model*** struct .
#[derive(Debug, Default)]
pub struct ModelBuilder {
    model: Model,
}
impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            model: Model::default(),
        }
    }
    pub fn name(mut self, model_name: &str) -> Self {
        self.model.name = model_name.to_string();
        self
    }
    pub fn varients(mut self, varients: Vec<Varient>) -> Self {
        self.model.varients = varients;
        self
    }
    pub fn category(mut self, categ: Category) -> Self {
        self.model.category = categ;
        self
    }
    pub fn summary_content(mut self, content: &str) -> Self {
        self.model.summary_content = content.to_string();

        self
    }
    pub fn readme_content(mut self, content: &str) -> Self {
        self.model.readme_content = content.to_string();
        self
    }
    pub fn build(self) -> Model {
        self.model
    }
}
