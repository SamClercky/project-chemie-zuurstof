use tokio::sync::mpsc;

pub mod routes;
pub mod gpio;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let (tx, rx) = mpsc::channel(10);

    tokio::join!(
        // start WARP
        routes::routes(tx),
        // Start GPIO
        gpio::bootstap(rx)
    );
}
