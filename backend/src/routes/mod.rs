use tokio::sync::mpsc::Sender;
use crate::gpio::GpioInstruction;

mod gpio;
mod static_files;

use warp::Filter;

pub async fn routes(tx: Sender<GpioInstruction>) {
    let routes = static_files::static_files()
        .or(gpio::gpio(tx));

    warp::serve(routes)
        .run(([0,0,0,0], 3030))
        .await;
}
