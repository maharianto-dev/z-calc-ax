use axum::{Json, Router, routing::get};
use models::envelope::Envelope;
mod models;
mod routers;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(hello))
        .nest("/util", routers::util::Util::default().app());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> Json<Envelope> {
    Json(Envelope {
        is_successful: true,
        message: String::from("Hello From Root!"),
    })
}
