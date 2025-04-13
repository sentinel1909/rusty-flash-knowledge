-- Add up migration script here
CREATE TABLE IF NOT EXISTS flashcards (
    id UUID PRIMARY KEY,
    question TEXT NOT NULL UNIQUE,
    answer TEXT NOT NULL,
    topic TEXT,
    tags TEXT[],
    difficulty INT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ
);

CREATE INDEX idx_flashcards_topic ON flashcards(topic);
CREATE INDEX idx_flashcards_tags ON flashcards USING GIN (tags);
