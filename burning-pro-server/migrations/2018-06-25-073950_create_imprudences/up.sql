-- Your SQL goes here
PRAGMA foreign_keys = ON;

-- NOTE: Use UTC datetime.

-- Not people, but "persons" for simplicity.
CREATE TABLE persons (
    person_id INTEGER NOT NULL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    modified_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    real_name VARCHAR,
    display_name VARCHAR,
    twitter VARCHAR UNIQUE
);

CREATE TABLE person_and_urls (
    person_and_url_id INTEGER NOT NULL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    modified_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    person_id INTEGER NOT NULL,
    url VARCHAR NOT NULL,
    FOREIGN KEY(person_id) REFERENCES persons(person_id),
    UNIQUE(person_id, url)
);

CREATE TABLE imprudences (
    imprudence_id INTEGER NOT NULL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    modified_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    title VARCHAR UNIQUE NOT NULL,
    phrase VARCHAR NOT NULL,
    person_id INTEGER NOT NULL,
    url VARCHAR,
    deleted BOOLEAN NOT NULL DEFAULT 0,
    published_at TIMESTAMP,
    FOREIGN KEY(person_id) REFERENCES persons(person_id),
    UNIQUE(phrase, person_id)
);

CREATE TABLE imprudence_tags (
    imprudence_tag_id INTEGER NOT NULL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    modified_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    name VARCHAR UNIQUE NOT NULL,
    description VARCHAR
);

CREATE TABLE imprudences_and_tags (
    imprudence_and_tag_id INTEGER NOT NULL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    modified_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    imprudence_id INTEGER NOT NULL,
    imprudence_tag_id INTEGER NOT NULL,
    FOREIGN KEY(imprudence_id) REFERENCES imprudences(imprudence_id),
    FOREIGN KEY(imprudence_tag_id) REFERENCES imprudence_tags(imprudence_tag_id),
    UNIQUE(imprudence_id, imprudence_tag_id)
);
