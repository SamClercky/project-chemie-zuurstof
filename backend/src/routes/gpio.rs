use warp::{self, Filter, Reply};
use warp::Rejection;
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct GpioInstruction {
    feed_sec: u32,
    delay_sec: u32,
    exhaust_sec: u32
}

pub fn gpio() -> impl Filter<Extract=(warp::reply::Json,), Error=Rejection> + Clone {
    return warp::post()
        .and(warp::path("gpio"))
        .and(warp::body::content_length_limit(1024*16))
        .and(warp::body::json())
        .map(|mut instruction: GpioInstruction| {
            warp::reply::json(&instruction)
        })
}
