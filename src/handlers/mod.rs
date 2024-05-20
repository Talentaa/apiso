use axum::{extract::State, response::IntoResponse, Json};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{models::*, AppState};

mod handlers_inner;

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    Json(QuestionDetail {
        title: question.title,
        description: question.description,
        question_uuid: Uuid::new_v4(),
        created_at: OffsetDateTime::now_utc(),
    })
}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> impl IntoResponse {
    Json(vec![QuestionDetail {
        title: "title".to_string(),
        description: "description".to_string(),
        question_uuid: Uuid::new_v4(),
        created_at: OffsetDateTime::now_utc(),
    }])
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) {
}

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    Json(AnswerDetail {
        answer_uuid: Uuid::new_v4(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: OffsetDateTime::now_utc(),
    })
}

pub async fn read_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
) -> impl IntoResponse {
    Json(vec![AnswerDetail {
        question_uuid: Uuid::new_v4(),
        content: "content".to_string(),
        answer_uuid: Uuid::new_v4(),
        created_at: OffsetDateTime::now_utc(),
    }])
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_id): Json<QuestionId>,
) -> impl IntoResponse {
}
