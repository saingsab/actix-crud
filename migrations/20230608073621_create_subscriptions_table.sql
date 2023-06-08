-- Add migration script here
-- Create Subscriptions Table
CREATE TABLE tbl_todolist (
id uuid NOT NULL,
PRIMARY KEY (id),
title TEXT NOT NULL,
created_at timestamptz NOT NULL default current_timestamp,
updated_at timestamptz);