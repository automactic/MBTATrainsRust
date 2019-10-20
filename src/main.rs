mod enums;
use enums::Line;

mod api_client;
use api_client::MBTAClient;

// #[macro_use] extern crate log;

fn main() {
    println!("Hello, world!");
    let client = MBTAClient{};
    let vehicles = client.get_vehicles();
    println!("{}", vehicles[0]);
}
