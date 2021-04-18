use tokio::sync::{mpsc, watch};
use crate::gpio::{GpioInstruction, ValveState};

mod gpio;
mod static_files;

use warp::Filter;

pub async fn routes(tx: mpsc::Sender<GpioInstruction>,
                    srx: watch::Receiver<ValveState>) {
    let routes = static_files::static_files()
        .or(gpio::gpio_instruction(tx));

    warp::serve(routes)
        .run(([0,0,0,0], 3030))
        .await;
}
