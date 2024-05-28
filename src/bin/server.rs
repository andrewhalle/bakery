use std::{
    env, fs,
    sync::{Arc, Mutex},
};

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

async fn create_cart(State(state): State<Arc<Mutex<AppState>>>) {
    let _guard = dbg!(state.lock().unwrap());
}

async fn get_cart_price() {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Cart {
    items: Vec<ItemIdAndAmount>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ItemIdAndAmount {
    item_id: u64,
    amount: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Applies {
    Day(chrono::Weekday),
    Date(chrono::Month, u16),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Sale {
    id: u64,
    filter: Applies,
    sale: bakery_test::Sale,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppState {
    treats: Vec<bakery_test::Item>,
    sales: Vec<Sale>,
    #[serde(skip)]
    carts: Vec<Cart>,
}

#[tokio::main]
async fn main() {
    // TODO: database
    // TODO: unwrap
    let state: AppState = serde_json::from_str(
        &fs::read_to_string(env::var("BAKERY_TEST_DATAFILE").unwrap()).unwrap(),
    )
    .unwrap();
    // TODO: Putting the global state in a mutex limits the concurrency to one at a time. This will
    // be fixed by getting the data from a database.
    let state = Arc::new(Mutex::new(state));
    // build our application with a single route
    let app = Router::new()
        .route("/cart", post(create_cart))
        .route("/cart/:id/price", get(get_cart_price))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
