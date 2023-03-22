use axum::{
    routing::{get,get_service, post},
    Router, Json,
};
use axum::{extract::{Form, Path, Query, State}};
use core::repository;
use entity::User;

#[derive(Clone)]
pub struct AppState {
    user_repo: Box<dyn repository::UserRepo + Send + Sync >,
    //event_repo: Mutex<Box<dyn repository::EventRepo + Send + Sync>>,
}


#[tokio::main]
pub async fn main() {

    let ur = repository::UserRepoInMemory::new();
    //let er = repository::EventRepoInMemory::new();

    let app_state = AppState {
        user_repo:Box::new(ur),
       // event_repo: Mutex::new(Box::new(er)),
    };

    let app = Router::new().
    route("/ping", get(|| async { "pong" })).
    route("/users/:id", get(read_user)).
    route("/users", post(save_user))
    .with_state(app_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


// async fn read_all_users(
//     state: State<AppState>
// ) -> Json<User> {
//     let users = state.user_repo.read_all_user();
//     Json(users)
// }
//#[axum_macros::debug_handler]
async fn read_user(
    state: State<AppState>,
    Path(user_id) : Path<String>
) -> Json<User> {
    let user = state.user_repo.read_user(&user_id);
    // if user.is_some() {
    //     Json(user.unwrap())
    // } else {

    // }
    Json(user.unwrap().to_owned())
}

async fn save_user(
    mut state: State<AppState>,
    payload: axum::extract::Json<User>
)  {
    let payload: User = payload.0;
    state.user_repo.add_user(payload);
    // if user.is_some() {
    //     Json(user.unwrap())
    // } else {

    // }
    //Json(user.unwrap().to_owned())
}
