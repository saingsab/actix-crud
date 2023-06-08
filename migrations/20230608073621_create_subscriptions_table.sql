-- Add migration script here
-- Create Subscriptions Table
CREATE TABLE tbl_todolist (
id uuid NOT NULL,
PRIMARY KEY (id),
title TEXT NOT NULL,
create_at timestamptz NOT NULL,
update_at timestamptz NOT NULL );