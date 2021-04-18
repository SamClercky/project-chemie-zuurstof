use tokio::sync::{mpsc, watch};

pub mod routes;
pub mod gpio;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let (tx, rx) = mpsc::channel(10);
    let (stx, srx) = watch::channel(gpio::get_default_state());

    tokio::join!(
        // start WARP
        routes::routes(tx, srx),
        // Start GPIO
        gpio::bootstap(rx, Some(stx))
    );
}
