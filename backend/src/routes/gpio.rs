use std::convert::Infallible;
use tokio::sync::{mpsc, watch};
use tokio_stream::wrappers::WatchStream;
use futures::{FutureExt, StreamExt};
use warp::{self, Filter, ws::Message};
use warp::Rejection;
use crate::gpio::{GpioInstruction, GpioEvent};
use log::*;

/// Middleware to be able to send new instructions to gpio
fn with_gpio(tx: mpsc::Sender<GpioInstruction>) 
    -> impl Filter<Extract=(mpsc::Sender<GpioInstruction>,), Error = Infallible> + Clone {
    warp::any().map(move || tx.clone())
}

/// Send new isntruction
pub fn instruction(tx: mpsc::Sender<GpioInstruction>) 
    -> impl Filter<Extract=(warp::reply::Json,), Error=Rejection> + Clone {
    return warp::post()
        .and(warp::path!("gpio" / "instruction"))
        .and(warp::body::content_length_limit(1024*16))
        .and(warp::body::json())
        .and(with_gpio(tx)) // send along tx
        .map(|instruction: GpioInstruction, tx: mpsc::Sender<GpioInstruction>| {
            let instr = instruction.clone();
            tokio::spawn(async move {
                tx.send(instr).await
                    .map_err(|_| "Could not write to GPIO")
                    .unwrap();
            });
            warp::reply::json(&instruction)
        })
}

/// Middleware to conenct the server with internal gpio
fn with_gpio_status(rx: watch::Receiver<GpioEvent>)
    -> impl Filter<Extract=(watch::Receiver<GpioEvent>,), Error=Infallible> + Clone {
    warp::any().map(move || rx.clone())
}

/// One off status message
pub fn get_status(rx: watch::Receiver<GpioEvent>)
    -> impl Filter<Extract=(warp::reply::Json,), Error=Rejection> + Clone {
    warp::get()
        .and(warp::path!("gpio" / "status"))
        .and(with_gpio_status(rx))
        .map(|status: watch::Receiver<GpioEvent>| {
            warp::reply::json(&*status.borrow())
        })
}

pub fn ws_status(rx: watch::Receiver<GpioEvent>) 
    -> impl Filter<Extract=impl warp::reply::Reply, Error=Rejection> + Clone {
    warp::get()
        .and(warp::path!("gpio" / "ws"))
        .and(warp::ws())
        .and(with_gpio_status(rx))
        .map(|ws: warp::ws::Ws, status: watch::Receiver<GpioEvent>| {
            ws.on_upgrade(move |websocket| {
                handle_ws(websocket, status)
            })
        })
}

async fn handle_ws(websocket: warp::ws::WebSocket,
                  status: watch::Receiver<GpioEvent>) {
    let (tx, _rx) = websocket.split();
    let status = WatchStream::new(status);

    status
        .map(|v| { Ok(Message::text(serde_json::to_string(&v).unwrap()))})
        .forward(tx)
        .map(|result| { 
            if let Err(e) = result {
                error!("WebSocket error: {}", e);
            }
        }).await;
}
