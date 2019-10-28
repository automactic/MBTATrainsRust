#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

use api_client::MBTAClient;
use database::Database;
use entities::Vehicle;
use enums::Line;

mod api_client;
mod entities;
mod enums;
mod database;
mod schema;

fn main() {
    let database = Database{};
    let vehicles = fetch_vehicles();
    database.update_vehicles(vehicles);
    database.get_vehicle_count();
}

fn fetch_vehicles() -> Vec<Vehicle> {
    let client = MBTAClient{};
    client.get_vehicles(Line::Orange)
}
