use axum::{
    routing::{delete, get, post},
    Router,
};
use dotenvy::dotenv;
use handlers::*;
use log::info;
use sqlx::mysql::MySqlPoolOptions;
use tokio::net::TcpListener;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().expect(".env file not found");

    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Connot connect to Database");

    let recs = sqlx::query!(r#"select * from questions"#)
        .fetch_all(&pool)
        .await
        .unwrap();

    info!("********* Question Records ********");
    info!("{:?}", recs);

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
