use warp::{Filter, Reply};

const STATIC_FILES_DIR: &str = "/tmp/www";
const STATIC_HOME: &str = "index.html";

pub fn static_files() 
    -> impl Filter<Extract = impl Reply, Error=warp::reject::Rejection> + Clone {
    let static_route = warp::path("static")
        .and(warp::fs::dir(STATIC_FILES_DIR));

    let home = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/{}", STATIC_FILES_DIR, STATIC_HOME)));

    static_route.or(home)
}
