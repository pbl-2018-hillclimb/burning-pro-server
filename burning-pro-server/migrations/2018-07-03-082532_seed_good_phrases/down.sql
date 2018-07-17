-- This file should undo anything in `up.sql`
PRAGMA foreign_keys = ON;

DELETE FROM good_phrases_and_tags
WHERE good_phrase_and_tag_id = 0;

DELETE FROM good_phrase_tags
WHERE good_phrase_tag_id = 0;

DELETE FROM good_phrases
WHERE good_phrase_id = 0;

DELETE FROM person_urls
WHERE person_url_id = 0;

DELETE FROM persons
WHERE person_id = 0;
