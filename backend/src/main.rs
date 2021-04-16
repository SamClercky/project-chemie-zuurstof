mod routes;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    routes::routes().await
}
