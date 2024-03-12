ALTER TABLE users
DROP column guild_id;

ALTER TABLE users
ADD PRIMARY KEY (user_id);