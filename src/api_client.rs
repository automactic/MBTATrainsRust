use std::fmt;
use std::env;
use reqwest;
use serde_json::{Value};


pub struct MBTAClient {
    
}

impl MBTAClient {
    pub fn get_vehicles(&self) -> Vec<Vehicle> {
        let url = "https://api-v3.mbta.com/vehicles";
        let response_json = match MBTAClient::get(url) {
            Some(response_json) => response_json,
            None=> return Vec::new()
        };
        let data = match response_json["data"].as_array() {
            Some(data) => data,
            None=> return Vec::new()
        };

        let mut vehicles: Vec<Vehicle> = Vec::new();
        for item in data {
            if let Some(vehicle) = Vehicle::from(item) {
                vehicles.push(vehicle)
            }
        }
        vehicles
    }

    fn get(url: &str) -> Option<Value> {
        let client = reqwest::Client::new();
        let mut request = client.get(url);
        if let Ok(token) = env::var("API_TOKEN") {
            request = request.header("x-api-key", token);
        }
        let mut response = match request.send() {
            Ok(response) => response,
            Err(_) => {
                // error!("HTTP Request Error -- {:?}", error);
                return None;
            }
        };
        let response_body = match response.text() {
            Ok(response_body) => response_body,
            Err(_) => return None
        };
        let response_json = match serde_json::from_str(&response_body) {
            Ok(response_json) => Some(response_json),
            Err(_) => None
        };
        response_json
    }
}

pub struct Vehicle {
    id: String,
    label: String,
}

impl Vehicle {
    fn from(json: &Value) -> Option<Self> {
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