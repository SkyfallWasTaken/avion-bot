-- Up migration
CREATE INDEX users_id_index ON users (user_id, guild_id);