
use axum::{
    routing::{get},
    Router,
};
use crate::handlers::{get_ip, health_checker_handler};

// const API_V1_BASE: &str = "/api/v1";

pub fn create_router() -> Router {
    // let db = model::todo_db();

    Router::new()
        .route("/health", get(health_checker_handler))
        .route("/ip/:ip", get(get_ip)
        )
        // .route(
        //     "/api/todos",
        //     get(todos_list_handler),
        // .route(
        //     "/api/todos/:id",
        //     get(get_todo_handler)
        //         .patch(edit_todo_handler)
        //         .delete(delete_todo_handler),
        // )
        // .with_state(db)
}