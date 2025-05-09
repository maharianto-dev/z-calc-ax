use axum::{routing::{get, post}, Json, Router};
use serde::{Serialize, Deserialize};


pub struct Util {
    app: Router,
}

#[derive(Serialize)]
pub struct UtilResponse {
    is_successful: bool,
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct UtilPayload {
    name: String,
}

impl Util {
    pub fn default() -> Util {
        Util {
            app: Router::new()
                .route("/hello", get(self::hello))
                .route("/goodbye", post(self::goodbye)),
        }
    }

    pub fn app(self) -> Router {
        self.app
    }
}

pub async fn hello() -> Json<UtilResponse> {
    Json(UtilResponse {
        is_successful: true,
        message: String::from("Hello From Util App!"),
    })
}

pub async fn goodbye(Json(payload): Json<UtilPayload>) -> Json<UtilResponse> {
    Json(UtilResponse {
        is_successful: true,
        message: String::from(format!("Goodbye From Util App {}!", payload.name)),
    })
}
