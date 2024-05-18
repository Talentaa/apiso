-- Add up migration script here

CREATE TABLE IF NOT EXISTS questions (
    question_uuid CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS answers (
    answer_uuid CHAR(36) PRIMARY KEY DEFAULT (UUID()),
    question_uuid CHAR(36) NOT NULL,
    content VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (question_uuid) REFERENCES questions (question_uuid) ON DELETE CASCADE
);