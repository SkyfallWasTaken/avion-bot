CREATE TABLE users (
    user_id text PRIMARY KEY NOT NULL,
    wallet_balance int DEFAULT 0 CHECK (wallet_balance >= 0),
    bank_balance int DEFAULT 0 CHECK (bank_balance >= 0),
    job text
);