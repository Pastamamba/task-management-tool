-- This file should undo anything in `up.sql`
ALTER TABLE tickets
DROP COLUMN created_at;

ALTER TABLE epics
DROP COLUMN created_at;
