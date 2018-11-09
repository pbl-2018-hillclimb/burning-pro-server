ALTER TABLE good_phrase_requests RENAME TO new_good_phrase_requests;
CREATE TABLE good_phrase_requests(
    good_phrase_request_id INTEGER NOT NULL PRIMARY KEY,
    title VARCHAR NOT NULL,
    phrase VARCHAR NOT NULL,
    person VARCHAR NOT NULL,
    url VARCHAR,
    deleted BOOLEAN NOT NULL DEFAULT 0,
    published_at TIMESTAMP,
    tags VARCHAR
);
INSERT INTO good_phrase_requests(good_phrase_request_id,title,phrase,person,url,deleted,published_at,tags) SELECT good_phrase_request_id,"TITLE",phrase,person,url,deleted,published_at,"" FROM new_good_phrase_requests;
DROP TABLE new_good_phrase_requests;
