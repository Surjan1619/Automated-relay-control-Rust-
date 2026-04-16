use std::io::Write;
use tokio::time::{sleep, Duration };

use futures_util::{SinkExt, StreamExt};
use reqwest::Client;
use serialport::SerialPort;


struct RelayConnection{
    port : Box<dyn SerialPort>,
}
impl RelayConnection {
    pub fn new(port_name: &str, baund_rate : u32) -> Self {
        let port = serialport::new(port_name, baund_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open serial port");
        println!("Opened serial port in bound - {}", baund_rate);
    RelayConnection{port}
    }
    pub fn turn_on(&mut self) {
        self.port.write_all(b"1").expect("Failed to turn on");
    }
    pub fn turn_off(&mut self) {
        self.port.write_all(b"0").expect("Failed to turn off");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut relay = RelayConnection::new("COM3", 9600);
    let mut relay_status = "OFF";
    loop {
        let response = client.get("http://localhost:3000/get-state").body("helllo friend").send().await?;
        let command = response.text().await?;
        if relay_status == command {
            println!("Got relay same status - {}", command);
            sleep(Duration::from_secs(5)).await;
        }else{
            if command == "OFF" {
                println!("Got off command");
                relay.turn_off();
                relay_status = "OFF";
            }else{
                println!("Got ON command");
                relay.turn_on();
                relay_status = "ON";

            }
        }
    sleep(Duration::from_secs(1)).await;
    }



}
