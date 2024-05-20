use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: Uuid) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: Uuid) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let answer = sqlx::query_as!(
            AnswerDetail,
            r#"
            INSERT INTO answers ( question_uuid, content )
            VALUES ( $1, $2 )
            RETURNING *
            "#,
            answer.question_uuid,
            answer.content
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(answer)
    }

    async fn delete_answer(&self, answer_uuid: Uuid) -> Result<(), DBError> {
        sqlx::query!(r#"DELETE FROM answers WHERE answer_uuid = $1"#, answer_uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: Uuid) -> Result<Vec<AnswerDetail>, DBError> {
        let answers = sqlx::query_as!(
            AnswerDetail,
            r#"SELECT * FROM answers WHERE question_uuid = $1"#,
            question_uuid
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(answers)
    }
}
