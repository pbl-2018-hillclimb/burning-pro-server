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

INSERT INTO person_and_urls (
    person_and_url_id,
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

INSERT INTO imprudences (
    imprudence_id,
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
    false,
    "2016-01-18T00:23:05.319"
);

INSERT INTO imprudence_tags (
    imprudence_tag_id,
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

INSERT INTO imprudences_and_tags (
    imprudence_and_tag_id,
    created_at,
    modified_at,
    imprudence_id,
    imprudence_tag_id
) VALUES (
    0,
    "2018-07-03T08:32:34",
    "2018-07-03T08:32:34",
    0,
    0
);
