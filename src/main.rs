use std::sync::mpsc;
use std::thread;
use dotenv::dotenv;

mod status;
mod data_collector;


fn main() {
    dotenv().ok();

    let (tx, rx) = mpsc::channel();

    // Start the status polling service
    thread::spawn(move || {
        status::run(tx);
    });

    // Start the data collection
    data_collector::run(rx);
}
