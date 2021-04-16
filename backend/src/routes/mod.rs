mod gpio;
mod static_files;

use warp::Filter;

pub async fn routes() {
    let routes = static_files::static_files()
        .or(gpio::gpio());

    warp::serve(routes)
        .run(([0,0,0,0], 3030))
        .await;
}
