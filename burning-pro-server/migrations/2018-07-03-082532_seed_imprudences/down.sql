-- This file should undo anything in `up.sql`
PRAGMA foreign_keys = ON;

DELETE FROM imprudences_and_tags
WHERE imprudence_and_tag_id = 0;

DELETE FROM imprudence_tags
WHERE imprudence_tag_id = 0;

DELETE FROM imprudences
WHERE imprudence_id = 0;

DELETE FROM person_and_urls
WHERE person_and_url_id = 0;

DELETE FROM persons
WHERE person_id = 0;
