use api_client::MBTAClient;
use enums::Line;

mod api_client;
mod entities;
mod enums;
mod database;

fn main() {
    println!("Hello, world!");
    let client = MBTAClient{};
    let vehicles = client.get_vehicles(Line::Orange);
    println!("{}", vehicles[0]);
}
