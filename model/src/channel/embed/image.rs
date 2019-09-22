use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EmbedImage {
    pub height: u64,
    pub proxy_url: String,
    pub url: String,
    pub width: u64,
}
