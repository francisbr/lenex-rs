use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
pub struct Fee {
    r#type: String,
    value: u64,
}
