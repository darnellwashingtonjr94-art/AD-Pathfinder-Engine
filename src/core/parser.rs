use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ADObject {
    pub id: String,
    pub name: String,
    pub object_type: String,
}

pub fn parse_payload(data: &str) -> Result<Vec<ADObject>, serde_json::Error> {
    serde_json::from_str(data)
}
