use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    routing::{get, post},
    Json, Router,
};
use core::repository;
use entity::User;
use std::sync::{Arc, Mutex};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    user_repo: Arc<Mutex<Box<dyn repository::UserRepo + Send + Sync>>>,
    //event_repo: Mutex<Box<dyn repository::EventRepo + Send + Sync>>,
}

#[tokio::main]
pub async fn main() {
    let ur = repository::UserRepoInMemory::new();
    //let er = repository::EventRepoInMemory::new();

    let app_state = AppState {
        user_repo: Arc::new(Mutex::new(Box::new(ur))),
        // event_repo: Mutex::new(Box::new(er)),
    };
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/users/:id", get(read_user))
        .route("/users", get(read_users))
        .route("/users", post(save_user))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
        .fallback(handler_404);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//#[axum_macros::debug_handler]
async fn read_user(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<User>, (StatusCode, String)> {
    let r = state.user_repo.lock().expect("mutex was poisoned");
    let user = r.read_user(&user_id);
    if user.is_some() {
        Ok(Json(user.unwrap().to_owned()))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unknown error".to_string(),
        ))
    }
}

async fn save_user(State(state): State<AppState>, payload: axum::extract::Json<User>) {
    println!("->{:?}", &payload);
    let payload: User = payload.0;
    let mut r = state.user_repo.lock().expect("mutex was poisoned");
    r.add_user(payload);
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn read_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.user_repo.lock().expect("mutex was poisoned");
    Json(users.read_all_users())
}
