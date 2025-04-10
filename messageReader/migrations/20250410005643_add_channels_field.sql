-- Add migration script here
ALTER TABLE messages ADD COLUMN channel TEXT DEFAULT '-1' NOT NULL;