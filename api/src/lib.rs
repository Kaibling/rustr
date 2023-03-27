use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    routing::{get, post},
    Json, Router,
};
use core::repository;
use entity::{User,Event};
use std::sync::{Arc, Mutex};
use tower_http::trace::TraceLayer;
use tracing::{span, Level,event};
#[derive(Clone)]
pub struct AppState {
    user_repo: Arc<Mutex<Box<dyn repository::UserRepo + Send + Sync>>>,
    event_repo: Arc<Mutex<Box<dyn repository::EventRepo + Send + Sync>>>,
}

#[tokio::main]
pub async fn main() {
    let ur = repository::UserRepoInMemory::new();
    let er = repository::EventRepoInMemory::new();

    let app_state = AppState {
        user_repo: Arc::new(Mutex::new(Box::new(ur))),
        event_repo: Arc::new(Mutex::new(Box::new(er))),
    };
    // tracing_subscriber::fmt()
    // .with_max_level(tracing::Level::DEBUG)
    // .init();
tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/users/:id", get(read_user))
        .route("/users", get(read_users))
        .route("/users", post(save_user))
        .route("/events/:id", get(read_event))
        .route("/events", get(read_events))
        .route("/events", post(save_event))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
        .fallback(handler_404);
    let listening_string ="0.0.0.0:3000";
    let msg = format!("start listening on {}", listening_string);
    event!(Level::INFO,msg);
    // run it with hyper on localhost:3000
    axum::Server::bind(&listening_string.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
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

async fn save_user(State(state): State<AppState>, payload: axum::extract::Json<User>) -> StatusCode {
    println!("->{:?}", &payload);
    let payload: User = payload.0;
    let mut r = state.user_repo.lock().expect("mutex was poisoned");
    r.add_user(payload);
    return StatusCode::CREATED;
}



async fn read_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.user_repo.lock().expect("mutex was poisoned");
    Json(users.read_all_users())
}


async fn read_event(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Event>, (StatusCode, String)> {
    let r = state.event_repo.lock().expect("mutex was poisoned");
    let event = r.read_event(&user_id);
    if event.is_some() {
        Ok(Json(event.unwrap().to_owned()))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unknown error".to_string(),
        ))
    }
}

async fn save_event(State(state): State<AppState>, payload: axum::extract::Json<Event>) -> StatusCode {
    //println!("->{:?}", &payload);
    let payload: Event = payload.0;
    let msg = format!("save event {} content '{}'",payload.id,payload.content);
    let mut r = state.event_repo.lock().expect("mutex was poisoned");
    r.add_event(payload);
    event!(Level::INFO,msg);
    return StatusCode::CREATED;
}



async fn read_events(State(state): State<AppState>) -> Json<Vec<Event>> {
    let events = state.event_repo.lock().expect("mutex was poisoned");
    Json(events.read_all_events())
}
