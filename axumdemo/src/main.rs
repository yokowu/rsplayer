use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(test));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

enum AppError {
    Invalid,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Invalid => Resp::<()>::new(-1000, "无效的请求".to_string()).into_response(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
struct Resp<T> {
    code: Option<i32>,
    data: Option<T>,
    msg: Option<String>,
}

impl<T> Resp<T>
where
    T: Serialize + Clone,
{
    fn new(code: i32, msg: String) -> Self {
        Self {
            code: Some(code),
            data: None,
            msg: Some(msg),
        }
    }

    fn sucess(d: &T) -> Self {
        Self {
            code: Some(0),
            data: Some(d.clone()),
            msg: Some("success".to_string()),
        }
    }
}

impl<T> IntoResponse for Resp<T>
where
    T: Serialize + Clone,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Clone)]
struct User {
    name: String,
}

impl IntoResponse for User {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

async fn test() -> Result<(), AppError> {
    Err(AppError::Invalid)
}
