/* Serves as the basic communication method for the window manager */
use client::Client;
use crate::client::Reduc;

pub mod client;

fn main() {
    // Create the master client a facilitates all other clients
    let mut client = Client::default();
    client.connect_client();
}
