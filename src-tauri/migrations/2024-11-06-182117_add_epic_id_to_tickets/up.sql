ALTER TABLE tickets ADD COLUMN epic_id INTEGER REFERENCES epics(id);
