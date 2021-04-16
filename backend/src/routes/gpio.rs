use tokio::sync::mpsc::Sender;
use warp::{self, Filter};
use warp::Rejection;
use crate::gpio::GpioInstruction;

static mut GPIO_TX: Option<Sender<GpioInstruction>> = None;
fn get_gpio_tx() -> Option<Sender<GpioInstruction>> {
    unsafe {
        GPIO_TX.to_owned()
    }
}

pub fn gpio(tx: Sender<GpioInstruction>) -> impl Filter<Extract=(warp::reply::Json,), Error=Rejection> + Clone {
    unsafe {GPIO_TX = Some(tx);} // This is ok, because no other writes will happen

    return warp::post()
        .and(warp::path("gpio"))
        .and(warp::body::content_length_limit(1024*16))
        .and(warp::body::json())
        .map(|instruction: GpioInstruction| {
            let instr = instruction.clone();
            tokio::spawn(async move {
                if let Some(tx) = get_gpio_tx() {
                    tx.send(instr).await
                        .map_err(|_| "Could not write to GPIO")
                        .unwrap();
                }
            });
            warp::reply::json(&instruction)
        })
}
