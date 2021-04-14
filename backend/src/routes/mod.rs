pub mod gpio;
pub mod static_files;

use std::convert::Infallible;
use warp::{self, Filter, Reply};

fn hello() 
    -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone{
    warp::any().map(|| "Hallo world")
}

pub async fn routes() {
    let routes = hello();

    warp::serve(routes)
        .run(([0,0,0,0], 3030))
        .await;
}
