use tokio::sync::{mpsc, watch};
use crate::gpio::{GpioInstruction, GpioEvent};

mod gpio;
mod static_files;

use warp::Filter;

pub async fn routes(tx: mpsc::Sender<GpioInstruction>,
                    srx: watch::Receiver<GpioEvent>) {
    let routes = static_files::static_files()
        .or(gpio::instruction(tx))
        .or(gpio::get_status(srx.to_owned()))
        .or(gpio::ws_status(srx.to_owned()));

    warp::serve(routes)
        .run(([0,0,0,0], 3030))
        .await;
}
