use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{DBError, Question, QuestionDetail};

pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: Uuid) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query!(
            r#"
          INSERT INTO questions ( title, description )
          VALUES ( $1, $2 )
          RETURNING *
          "#,
            question.title,
            question.description
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(QuestionDetail {
            question_uuid: record.question_uuid,
            title: record.title,
            description: record.description,
            created_at: record.created_at,
        })
    }

    async fn delete_question(&self, question_uuid: Uuid) -> Result<(), DBError> {
        sqlx::query!(
            "DELETE FROM questions WHERE question_uuid = $1",
            question_uuid
        )
        .execute(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;
        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let questions = sqlx::query_as!(QuestionDetail, r#"SELECT * FROM questions"#)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(questions)
    }
}
