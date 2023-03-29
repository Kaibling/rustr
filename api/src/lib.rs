use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    routing::{get, post},
    Json, Router,
};
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::Response;
use core::repository::{SessionRepo,EventRepo,UserRepo};
use core::repository::{SessionRepoInMemory,EventRepoInMemory,UserRepoInMemory};

use entity::{User,Event, Session};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use tracing::{Level,event};
#[derive(Clone)]
pub struct AppState {
    user_repo: Arc<Mutex<Box<dyn UserRepo + Send + Sync>>>,
    event_repo: Arc<Mutex<Box<dyn EventRepo + Send + Sync>>>,
    session_repo: Arc<Mutex<Box<dyn SessionRepo + Send + Sync>>>,
    //context : Arc<Mutex<Context>>,
}
#[derive(Clone)]
struct Context {
    pub private_key : Option<String>
}

#[tokio::main]
pub async fn main() {
    let ur = UserRepoInMemory::new();
    let er = EventRepoInMemory::new();
    let sr = SessionRepoInMemory::new();
    let context = Context{private_key: None};
    
    let app_state = AppState {
        user_repo: Arc::new(Mutex::new(Box::new(ur))),
        event_repo: Arc::new(Mutex::new(Box::new(er))),
        session_repo: Arc::new(Mutex::new(Box::new(sr))),
       //context: Arc::new(Mutex::new(context)),
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
        .layer(middleware::from_fn_with_state(app_state.clone(),test_middleware_mutex))
        .route("/authenticate", post(save_session))
        //authenticate
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
    let r = state.user_repo.lock().await;//.expect("mutex was poisoned");
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
    let mut r = state.user_repo.lock().await;//.expect("mutex was poisoned");
    r.add_user(payload);
    return StatusCode::CREATED;
}



async fn read_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.user_repo.lock().await;//.expect("mutex was poisoned");
    Json(users.read_all_users())
}


async fn read_event(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Event>, (StatusCode, String)> {
    let mut r = state.event_repo.lock().await;//.expect("mutex was poisoned");
    match r.read(&id){
        Some(event) => { 
        return Ok(Json(event.to_owned()))},
        None => return Err((
            StatusCode::NOT_FOUND,
            "event not found".to_string(),
        ))
    }
}

async fn save_event(State(state): State<AppState>, payload: axum::extract::Json<Event>) -> StatusCode {
    let payload: Event = payload.0;
    let msg = format!("save event {} content '{}'",payload.id,payload.content);
    let mut r = state.event_repo.lock().await;//.expect("mutex was poisoned");
    r.add(payload);
    event!(Level::INFO,msg);
    return StatusCode::CREATED;
}



async fn read_events(State(state): State<AppState>) -> Json<Vec<Event>> {
    let mut events = state.event_repo.lock().await;//.expect("mutex was poisoned");
    Json(events.read_all())
}

async fn save_session(headers: HeaderMap,State(state): State<AppState>) -> StatusCode {
    let auth_header = headers.get("authorization");
    let auth_parts = if auth_header.is_some(){
        auth_header.unwrap().to_str()
    } else {
        return StatusCode::BAD_REQUEST
    };

    let auth_part = match  auth_parts {
        Ok(v) => v.split(" ").nth(1),
        _ => return StatusCode::BAD_REQUEST,
    };

    let token = match  auth_part {
        Some(v) => v,
        _ => return StatusCode::BAD_REQUEST,
    };

    println!("{:?}",token);
    // let ctx = state.context.lock().await;//.expect("mutex was poisoned");
    // println!("session {}",ctx.private_key.as_ref().unwrap());

    let mut session_repo = state.session_repo.lock().await;//.expect("mutex was poisoned");

    session_repo.add(Session::new(token.to_string(), 0));
    let msg = format!("new session for  {}", token.to_string());
    event!(Level::INFO,msg);
    return StatusCode::CREATED;
}


use axum::http::HeaderMap;

async fn test_middleware_mutex<B>(
    headers: HeaderMap,
    State(state): State<AppState>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {

    let auth_header = headers.get("authorization");
    let auth_parts = if auth_header.is_some(){
        auth_header.unwrap().to_str()
    } else {
        return Err(StatusCode::BAD_REQUEST)
    };

    let auth_part = match  auth_parts {
        Ok(v) => v.split(" ").nth(1),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    let token = match  auth_part {
        Some(v) => v,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    println!("middleware {:?}",token);

    let mut sr = state.session_repo.lock().await;
    let  s =  sr.read(&token.to_string());
    if s.is_some() {
        println!("is da");
    } else {
        return Err(StatusCode::UNAUTHORIZED)
    }
    let response = next.run(request).await;
        Ok(response)
}
