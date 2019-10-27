use std::env;

use reqwest;
use serde_json::Value;

use super::entities::Vehicle;
use super::enums::Line;

pub struct MBTAClient {
    
}

impl MBTAClient {
    pub fn get_vehicles(&self, line: Line) -> Vec<Vehicle> {
        let url = "https://api-v3.mbta.com/vehicles";
        let query = [(String::from("filter[route]"), String::from(line.as_str()))];
        let response_json = match MBTAClient::get(url, &query) {
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

    fn get(url: &str, query: &[(String, String)]) -> Option<Value> {
        let client = reqwest::Client::new();
        let mut request = client.get(url).query(query);
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
