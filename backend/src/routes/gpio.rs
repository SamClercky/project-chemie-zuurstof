use std::convert::Infallible;
use tokio::sync::mpsc::Sender;
use warp::{self, Filter};
use warp::Rejection;
use crate::gpio::GpioInstruction;

fn with_gpio(tx: Sender<GpioInstruction>) 
    -> impl Filter<Extract=(Sender<GpioInstruction>,), Error = Infallible> + Clone {
    warp::any().map(move || tx.clone())
}

pub fn gpio(tx: Sender<GpioInstruction>) 
    -> impl Filter<Extract=(warp::reply::Json,), Error=Rejection> + Clone {
    return warp::post()
        .and(warp::path("gpio"))
        .and(warp::body::content_length_limit(1024*16))
        .and(warp::body::json())
        .and(with_gpio(tx)) // send along tx
        .map(|instruction: GpioInstruction, tx: Sender<GpioInstruction>| {
            let instr = instruction.clone();
            tokio::spawn(async move {
                tx.send(instr).await
                    .map_err(|_| "Could not write to GPIO")
                    .unwrap();
            });
            warp::reply::json(&instruction)
        })
}
