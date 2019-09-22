use serde::{Deserialize, Serialize};
use super::MessageActivityType;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageActivity {
    pub kind: MessageActivityType,
    pub party_id: Option<String>,
}
