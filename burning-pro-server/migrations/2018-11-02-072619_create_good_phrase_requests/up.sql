CREATE TABLE good_phrase_requests (
    good_phrase_request_id INTEGER NOT NULL PRIMARY KEY,
    title VARCHAR NOT NULL,
    phrase VARCHAR NOT NULL,
    person VARCHAR NOT NULL,
    url VARCHAR,
    deleted BOOLEAN NOT NULL DEFAULT 0,
    published_at TIMESTAMP,
    tags VARCHAR,
    UNIQUE(phrase)
);
