CREATE TABLE chat_history (
    id SERIAL PRIMARY KEY,
    user_query TEXT NOT NULL,
    ai_response TEXT NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT NOW()
);