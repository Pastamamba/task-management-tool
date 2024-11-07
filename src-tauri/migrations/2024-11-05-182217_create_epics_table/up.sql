CREATE TABLE epics (
                       id SERIAL PRIMARY KEY,
                       title VARCHAR NOT NULL,
                       status VARCHAR NOT NULL,
                       description TEXT
);
