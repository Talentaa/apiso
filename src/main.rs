use axum::{
    routing::{delete, get, post},
    Router,
};
use handlers::*;
use tokio::net::TcpListener;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/question", post(create_question))
        .route("/question", delete(delete_question))
        .route("/questions", get(read_questions))
        .route("/answer", post(create_answer))
        .route("/answer", delete(delete_answer))
        .route("/answers", get(read_answers));

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
