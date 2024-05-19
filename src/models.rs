use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct QuestionDetail {
    pub question_uuid: Uuid,
    pub title: String,
    pub description: String,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct QuestionId {
    pub question_uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub question_uuid: Uuid,
    pub content: String,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AnswerDetail {
    pub answer_uuid: Uuid,
    pub question_uuid: Uuid,
    pub content: String,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AnswerId {
    pub answer_uuid: Uuid,
}

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Database error occurred")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub mod mysql_error_codes {
    pub const FORIGN_KEY_VIOLATION: &str = "23503";
}
