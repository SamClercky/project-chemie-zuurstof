use warp::{Filter, Rejection};

const STATIC_FILES_DIR: &str = "";
const STATIC_HOME: &str = "";

pub fn static_files() -> impl Filter {
    let static_route = warp::path("static")
        .and(warp::fs::dir(STATIC_FILES_DIR));

    let home = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/{}", STATIC_FILES_DIR, STATIC_HOME)));

    static_route.or(home)
}
