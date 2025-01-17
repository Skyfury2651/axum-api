use crate::{handler::todo_handler, state::user_state::UserState};
use axum::{ routing::post, Router };

pub fn routes() -> Router<UserState> {
    let router = Router::new().route("/todo", post(todo_handler::create(validated_request)));
    return router;
}
