CREATE TABLE subscriptions(
    id UUID NOT NULL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NULL
);
