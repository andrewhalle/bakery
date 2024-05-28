use axum::{
    routing::{get, post},
    Router,
};

async fn create_cart() {}

async fn get_cart_price() {}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/cart", post(create_cart))
        .route("/cart/price", get(get_cart_price));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
