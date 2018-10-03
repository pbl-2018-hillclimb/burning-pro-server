-- Your SQL goes here
PRAGMA foreign_keys = ON;

INSERT INTO persons (
    person_id,
    created_at,
    modified_at,
    real_name,
    display_name,
    twitter
) VALUES (
    0,
    "2018-07-03T08:32:34",
    "2018-07-03T08:32:34",
    NULL,
    "wtakuo",
    "wtakuo"
);

INSERT INTO person_urls (
    person_url_id,
    created_at,
    modified_at,
    person_id,
    url
) VALUES (
    0,
    "2018-07-03T08:32:34",
    "2018-07-03T08:32:34",
    0,
    "https://twitter.com/wtakuo"
);

INSERT INTO good_phrases (
    good_phrase_id,
    created_at,
    modified_at,
    title,
    phrase,
    person_id,
    url,
    deleted,
    published_at
) VALUES (
    0,
    "2018-07-03T08:32:34",
    "2018-07-03T08:32:34",
    "大した問題じゃないでしょう",
    "{大学}から{休講情報}が{出ない}という文句のツイートが散見されますが，{大学生}なんだから自分で判断して{休みたかったら休めばいい}のではと思ってしまいます．{授業}に{一回}くらい{出なくた}って大した問題じゃないでしょう．",
    0,
    "https://twitter.com/wtakuo/status/688879244567445504",
    0,
    "2016-01-18T00:23:05.319"
);

INSERT INTO good_phrase_tags (
    good_phrase_tag_id,
    created_at,
    modified_at,
    name,
    description
) VALUES (
    0,
    "2018-07-03T08:32:34",
    "2018-07-03T08:32:34",
    "炎上実績",
    "炎上した過去を持つ発言"
);

INSERT INTO good_phrases_and_tags (
    good_phrase_and_tag_id,
    created_at,
    modified_at,
    good_phrase_id,
    good_phrase_tag_id
) VALUES (
    0,
    "2018-07-03T08:32:34",
    "2018-07-03T08:32:34",
    0,
    0
);
