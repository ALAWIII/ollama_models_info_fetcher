use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// represents different tags that the model may be marked with or have the capability to perform!
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Tool,
    Vision,
    Embedding,
    Other,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            Self::Tool => "tool",
            Self::Vision => "vision",
            Self::Embedding => "embedding",
            _ => "other",
        }
        .to_string();
        write!(f, "{}", kind)
    }
}
