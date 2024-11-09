-- Your SQL goes here
CREATE TABLE projects (
                       id SERIAL PRIMARY KEY,
                       title VARCHAR NOT NULL,
                       description TEXT,
                       status VARCHAR NOT NULL DEFAULT 'New',
                       created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
