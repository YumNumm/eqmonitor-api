
use axum::{routing::get, Router};
use shuttle_service::tracing::log::logger;
use sync_wrapper::SyncWrapper;

async fn hello_world() -> &'static str {
    // print Hello world to console
    println!("Hello world");
    "Hello world"
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new().route("/hello", get(hello_world));
    let sync_wrapper = SyncWrapper::new(router);
    

    Ok(sync_wrapper)
}
