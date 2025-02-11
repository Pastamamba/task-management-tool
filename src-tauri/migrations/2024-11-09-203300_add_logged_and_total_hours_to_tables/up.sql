-- Your SQL goes here
ALTER TABLE epics
    ADD COLUMN logged_hours JSONB,
ADD COLUMN total_hours INTEGER DEFAULT 0;

ALTER TABLE tickets
    ADD COLUMN logged_hours JSONB,
ADD COLUMN total_hours INTEGER DEFAULT 0;

ALTER TABLE projects
    ADD COLUMN logged_hours JSONB,
ADD COLUMN total_hours INTEGER DEFAULT 0;
