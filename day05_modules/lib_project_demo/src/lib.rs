// Declaring a sub-module

pub mod network;

pub fn connect() {
    network::server::init();        //This is a code inside network module
}
