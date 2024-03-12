ALTER TABLE users
ALTER column user_id
SET NOT NULL;

ALTER TABLE users
DROP CONSTRAINT users_pkey;

ALTER TABLE users
ADD COLUMN guild_id text NOT NULL DEFAULT '';

ALTER TABLE users
ADD PRIMARY KEY (user_id, guild_id);