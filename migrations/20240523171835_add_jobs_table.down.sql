-- Down migration
DROP TABLE jobs;
DROP INDEX [IF_EXISTS] jobs_guild_id_index;