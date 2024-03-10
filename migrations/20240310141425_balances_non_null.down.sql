ALTER TABLE users ALTER COLUMN wallet_balance TYPE int DEFAULT 0 CHECK (wallet_balance >= 0);
ALTER TABLE users ALTER COLUMN bank_balance TYPE int DEFAULT 0 CHECK (bank_balance >= 0);