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

CREATE TABLE person_urls (
    person_url_id INTEGER NOT NULL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    modified_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    person_id INTEGER NOT NULL,
    url VARCHAR NOT NULL,
    FOREIGN KEY(person_id) REFERENCES persons(person_id),
    UNIQUE(person_id, url)
);

CREATE TABLE good_phrases (
    good_phrase_id INTEGER NOT NULL PRIMARY KEY,
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

CREATE TABLE good_phrase_tags (
    good_phrase_tag_id INTEGER NOT NULL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    modified_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    name VARCHAR UNIQUE NOT NULL,
    description VARCHAR
);

CREATE TABLE good_phrases_and_tags (
    good_phrase_and_tag_id INTEGER NOT NULL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    modified_at TIMESTAMP NOT NULL DEFAULT (DATETIME('now')),
    good_phrase_id INTEGER NOT NULL,
    good_phrase_tag_id INTEGER NOT NULL,
    FOREIGN KEY(good_phrase_id) REFERENCES good_phrases(good_phrase_id),
    FOREIGN KEY(good_phrase_tag_id) REFERENCES good_phrase_tags(good_phrase_tag_id),
    UNIQUE(good_phrase_id, good_phrase_tag_id)
);
