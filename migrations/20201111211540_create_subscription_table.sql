--  Create Subscription Table
CREATE TABLE subscription(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL
);