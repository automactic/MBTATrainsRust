mod enums;
use enums::Line;

mod api_client;
use api_client::MBTAClient;

fn main() {
    println!("Hello, world!");
    let client = MBTAClient{};
    client.print_ip_address();
}
