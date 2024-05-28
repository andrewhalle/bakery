use std::{
    collections::HashMap,
    env, fs,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use bakery_test::Parcel;
use chrono::{DateTime, Datelike as _, Utc};
use serde::{Deserialize, Serialize};

async fn create_cart(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(cart): Json<Cart>,
) -> Json<CreatedCart> {
    let mut state = state.lock().unwrap();
    state.carts.push(cart);
    let cart_id = state.carts.len() as u64;
    Json(CreatedCart { cart_id })
}

async fn get_cart_price(
    Path(id): Path<u64>,
    Query(DateQuery { date }): Query<DateQuery>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> Json<Price> {
    let state = state.lock().unwrap();
    let cart = &state.carts[(id - 1) as usize];
    let parcels: Vec<bakery_test::Parcel> = cart
        .items
        .iter()
        .map(
            |ItemIdAndAmount {
                 item_id,
                 amount: count,
             }| {
                let item = state
                    .treats
                    .iter()
                    .find(|&treat| treat.id == *item_id)
                    .unwrap()
                    .clone();
                Parcel {
                    item,
                    count: *count,
                }
            },
        )
        .collect();
    let cart = bakery_test::Cart { parcels };
    let sales: HashMap<u64, bakery_test::Sale> = state
        .sales
        .iter()
        .filter(|sale| sale.applies(date))
        .map(|sale| (sale.item_id, sale.sale.clone()))
        .collect();
    Json(Price {
        price: cart.price(&sales),
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct Price {
    price: f64,
}

#[derive(Debug, Deserialize)]
struct DateQuery {
    date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Cart {
    items: Vec<ItemIdAndAmount>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatedCart {
    cart_id: u64,
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
    Date(chrono::Month, u32),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Sale {
    id: u64,
    item_id: u64,
    filter: Applies,
    sale: bakery_test::Sale,
}

impl Sale {
    fn applies(&self, date: DateTime<Utc>) -> bool {
        match &self.filter {
            Applies::Day(day) => date.weekday() == *day,
            Applies::Date(month, day) => {
                date.month() == month.number_from_month() && date.day() == *day
            }
        }
    }
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
