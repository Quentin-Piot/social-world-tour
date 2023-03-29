use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    routing::post,
    Router,
};
use tower_cookies::Cookies;

use crate::{AppState, FlashData};
use entity::users;
use social_world_tour_core::users::Mutation as MutationCore;

use crate::flash::{post_response, PostResponse};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/", post(create_user))
        .route("/users/:id", post(update_user))
        .route("/users/delete/:id", post(delete_user))
}

async fn create_user(
    state: State<AppState>,
    mut cookies: Cookies,
    form: Form<users::Model>,
) -> Result<PostResponse, (StatusCode, &'static str)> {
    let form = form.0;

    MutationCore::create_user(&state.conn, form)
        .await
        .expect("could not insert post");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "User succcessfully added".to_owned(),
    };

    Ok(post_response(&mut cookies, data))
}

async fn update_user(
    state: State<AppState>,
    Path(id): Path<i32>,
    mut cookies: Cookies,
    form: Form<users::Model>,
) -> Result<PostResponse, (StatusCode, String)> {
    let form = form.0;

    MutationCore::update_user_by_id(&state.conn, id, form)
        .await
        .expect("could not edit user");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "User succcessfully updated".to_owned(),
    };

    Ok(post_response(&mut cookies, data))
}

async fn delete_user(
    state: State<AppState>,
    Path(id): Path<i32>,
    mut cookies: Cookies,
) -> Result<PostResponse, (StatusCode, &'static str)> {
    MutationCore::delete_user(&state.conn, id)
        .await
        .expect("could not delete user");

    let data = FlashData {
        kind: "success".to_owned(),
        message: "User succcessfully deleted".to_owned(),
    };

    Ok(post_response(&mut cookies, data))
}
