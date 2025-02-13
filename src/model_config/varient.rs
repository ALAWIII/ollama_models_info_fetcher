use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// the tag or (token_size , size) that the model may provide ,
///
/// # Example
/// ```ignore
/// ("7b","4.9GB")
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Varient {
    token_size: String,
    size: String,
}

impl Varient {
    pub fn new(token_size: &str, size: &str) -> Self {
        Self {
            token_size: token_size.to_string(),
            size: size.to_string(),
        }
    }
    /// returns the parameter size .
    ///
    /// # Example
    ///  1.5b ,7b ,....
    pub fn token_size(&self) -> &str {
        &self.token_size
    }
    /// returns the actual size in GigaByte.
    pub fn size(&self) -> &str {
        &self.size
    }
}

impl Display for Varient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.token_size(), self.size())
    }
}
