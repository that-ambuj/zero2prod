-- Create Subscriptions Table
CREATE TABLE subscriptions (
    id uuid NOT NUll,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
)
