-- Add migration script here
ALTER TABLE skills
ADD COLUMN computed_value INT;
ALTER TABLE stats
ADD COLUMN computed_value INT;