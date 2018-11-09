ALTER TABLE good_phrase_requests RENAME TO old_good_phrase_requests;
CREATE TABLE good_phrase_requests (
    good_phrase_request_id INTEGER NOT NULL PRIMARY KEY,
    phrase VARCHAR NOT NULL,
    person VARCHAR NOT NULL,
    url VARCHAR,
    deleted BOOLEAN NOT NULL DEFAULT 0,
    published_at TIMESTAMP
);
INSERT INTO good_phrase_requests(good_phrase_request_id,phrase,person,url,deleted,published_at) SELECT good_phrase_request_id,phrase,person,url,deleted,published_at FROM old_good_phrase_requests;
DROP TABLE old_good_phrase_requests;
