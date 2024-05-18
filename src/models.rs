use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    pub question_uuid: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionId {
    pub question_uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub question_uuid: Uuid,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerDetail {
    pub answer_uuid: Uuid,
    pub question_uuid: Uuid,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerId {
    pub answer_uuid: Uuid,
}