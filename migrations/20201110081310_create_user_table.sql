-- migrations/{timestamp}_create_subscriptions_table.sql -- Create Subscriptions Table
CREATE TABLE "user" (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL
);