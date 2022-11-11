mod client;
pub mod edsig;
pub mod messages;
pub mod proto;
mod server;
pub mod statemachine;
pub mod types;

use crate::server::setup_server;

fn main() {
    setup_server();
    println!("Hello, world!");
}
