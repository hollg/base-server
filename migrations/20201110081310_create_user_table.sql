-- Add migration script here
CREATE TABLE user(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL
);