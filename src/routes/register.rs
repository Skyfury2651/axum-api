use axum::Router;

use crate::model;

pub fn create_router() -> Router {
    let db = model::todo_db();

    Router::new()
        // .route("/api/healthchecker", get(health_checker_handler))
        // .route(
        //     "/api/todos",
        //     post(create_todo_handler).get(todos_list_handler),
        // )
        // .route(
        //     "/api/todos/:id",
        //     get(get_todo_handler)
        //         .patch(edit_todo_handler)
        //         .delete(delete_todo_handler),
        // )
        .with_state(db)
}
