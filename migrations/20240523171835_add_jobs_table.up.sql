-- Up migration
CREATE TABLE jobs (
    guild_id TEXT NOT NULL,
    job_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    salary_per_hour INTEGER NOT NULL,
    PRIMARY KEY (guild_id, job_id)
);

CREATE INDEX jobs_guild_id_index ON jobs (guild_id);
