-- Add down migration script here
CREATE TABLE mod_gd_versions (
    id SERIAL PRIMARY KEY,
    mod_id INTEGER NOT NULL REFERENCES mod_versions(id),
    gd INTEGER NOT NULL,
    platform TEXT NOT NULL
);
