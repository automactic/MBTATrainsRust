use reqwest;


pub struct MBTAClient {
    
}

impl MBTAClient {
    pub fn print_ip_address(&self) {
        let url = "https://httpbin.org/ip";
        let response_body = self.get(url);
        println!("{}", response_body.unwrap());
    }

    fn get(&self, url: &str) -> Option<String> {
        match reqwest::get(url) {
            Ok(mut response) => {
                match response.text() {
                    Ok(response_body) => Some(response_body),
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }
}
