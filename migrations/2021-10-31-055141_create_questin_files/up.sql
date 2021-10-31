-- Your SQL goes here
CREATE TABLE question_files(
    ID BIGSERIAL PRIMARY KEY,
    file_id UUID NOT NULL,
    question_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(file_id) REFERENCES files(id),
    FOREIGN KEY(question_id) REFERENCES questions(id)
);

CREATE INDEX question_files_question_id_index ON question_files(question_id);
