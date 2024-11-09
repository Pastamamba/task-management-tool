-- Your SQL goes here
ALTER TABLE epics ADD COLUMN project_id INTEGER REFERENCES projects(id);
