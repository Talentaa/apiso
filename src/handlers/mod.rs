use axum::{response::IntoResponse, Json};
use uuid::Uuid;

use crate::models::*;

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    Json(QuestionDetail {
        title: question.title,
        description: question.description,
        question_uuid: Uuid::new_v4(),
        created_at: "created_at".to_string(),
    })
}

pub async fn read_questions() -> impl IntoResponse {
    Json(vec![QuestionDetail {
        title: "title".to_string(),
        description: "description".to_string(),
        question_uuid: Uuid::new_v4(),
        created_at: "created_at".to_string(),
    }])
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {}

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    Json(AnswerDetail {
        answer_uuid: Uuid::new_v4(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: "created_at".to_string(),
    })
}

pub async fn read_answers() -> impl IntoResponse {
    Json(vec![AnswerDetail {
        question_uuid: Uuid::new_v4(),
        content: "content".to_string(),
        answer_uuid: Uuid::new_v4(),
        created_at: "created_at".to_string(),
    }])
}

pub async fn delete_answer(Json(question_id): Json<QuestionId>) -> impl IntoResponse {}
