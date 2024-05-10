use std::sync::Arc;

use axum::{
    extract::{ConnectInfo, State}, routing::get, Router
};
use tokio::sync::Mutex;
use std::net::SocketAddr;
use axum_macros::debug_handler;

#[tokio::main]
async fn main() {
    let state: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let app = Router::new()
    .route("/log", get(log))
    .route("/", get(root))
    .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2000").await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap()

}


async fn root(State(state): State<Arc<Mutex<Vec<String>>>>) -> String  {
    let inner = state.lock().await;
    format!("{:#?}", inner)

}

#[debug_handler]
async fn log(State(state): State<Arc<Mutex<Vec<String>>>>, ConnectInfo(connect_addr) : ConnectInfo<SocketAddr>) -> String {
    let stri = format!("{:?}", connect_addr);
    let mut inner = state.lock().await;
    (*inner).push(stri.clone());
    stri
}