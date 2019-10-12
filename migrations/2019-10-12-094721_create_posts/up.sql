-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    body VARCHAR,
    user_id INT NOT NULL,
    created_at timestamp,
    updated_at timestamp
);