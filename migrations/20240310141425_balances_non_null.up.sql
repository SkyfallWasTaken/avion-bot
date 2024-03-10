ALTER TABLE users
ALTER COLUMN wallet_balance SET NOT NULL,
ALTER COLUMN wallet_balance SET DEFAULT 0,
ALTER COLUMN wallet_balance TYPE int,
ADD CONSTRAINT wallet_balance_check CHECK (wallet_balance >= 0);

ALTER TABLE users
ALTER COLUMN bank_balance SET NOT NULL,
ALTER COLUMN bank_balance SET DEFAULT 0,
ALTER COLUMN bank_balance TYPE int,
ADD CONSTRAINT bank_balance_check CHECK (bank_balance >= 0);
