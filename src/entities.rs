use std::fmt;

use serde_json::Value;
use super::schema::vehicles;
use diesel::sql_types::Timestamp;
use chrono::{NaiveDateTime, DateTime, Utc, TimeZone};

#[derive(Queryable, Insertable, AsChangeset)]
#[table_name="vehicles"]
pub struct Vehicle {
    id: String,
    label: String,
    updated_at: NaiveDateTime,
}

impl Vehicle {
    pub fn from(json: &Value) -> Option<Self> {
        let id = json["id"].as_str()?;
        let label = json["attributes"]["label"].as_str()?;
        let updated_at = json["attributes"]["updated_at"].as_str()?;
        Some(Vehicle {
            id: String::from(id),
            label: String::from(label),
            updated_at: DateTime::parse_from_rfc3339(&updated_at).ok()?.naive_utc()
        })
    }
}

impl fmt::Display for Vehicle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.updated_at)
    }
}