use std::fmt;

use serde_json::Value;

pub struct Vehicle {
    id: String,
    label: String,
}

impl Vehicle {
    pub fn from(json: &Value) -> Option<Self> {
        let id = json["id"].as_str()?;
        let label = json["attributes"]["label"].as_str()?;
        Some(Vehicle {
            id: String::from(id),
            label: String::from(label),
        })
    }
}

impl fmt::Display for Vehicle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.label)
    }
}