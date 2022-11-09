mod server;
mod client;
pub mod edsig;
pub mod messages;
pub mod statemachine;
pub mod types;
pub mod proto;

use crate::server::setup_server;

fn main() {
    setup_server();
    println!("Hello, world!");
}