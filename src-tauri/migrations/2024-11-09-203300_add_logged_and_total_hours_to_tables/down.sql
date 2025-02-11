-- This file should undo anything in `up.sql`
ALTER TABLE epics
DROP COLUMN logged_hours,
DROP COLUMN total_hours;

ALTER TABLE tickets
DROP COLUMN logged_hours,
DROP COLUMN total_hours;

ALTER TABLE projects
DROP COLUMN logged_hours,
DROP COLUMN total_hours;
