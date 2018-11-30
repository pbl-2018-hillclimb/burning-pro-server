table! {
    good_phrase_requests (good_phrase_request_id) {
        good_phrase_request_id -> Integer,
        phrase -> Text,
        person -> Text,
        url -> Nullable<Text>,
        deleted -> Bool,
        published_at -> Nullable<Timestamp>,
    }
}

table! {
    good_phrase_tags (good_phrase_tag_id) {
        good_phrase_tag_id -> Integer,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    good_phrases (good_phrase_id) {
        good_phrase_id -> Integer,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        title -> Text,
        phrase -> Text,
        person_id -> Integer,
        url -> Nullable<Text>,
        deleted -> Bool,
        published_at -> Nullable<Timestamp>,
    }
}

table! {
    good_phrases_and_tags (good_phrase_and_tag_id) {
        good_phrase_and_tag_id -> Integer,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        good_phrase_id -> Integer,
        good_phrase_tag_id -> Integer,
    }
}

table! {
    person_urls (person_url_id) {
        person_url_id -> Integer,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        person_id -> Integer,
        url -> Text,
    }
}

table! {
    persons (person_id) {
        person_id -> Integer,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        real_name -> Nullable<Text>,
        display_name -> Text,
        twitter -> Nullable<Text>,
    }
}

joinable!(good_phrases -> persons (person_id));
joinable!(good_phrases_and_tags -> good_phrase_tags (good_phrase_tag_id));
joinable!(good_phrases_and_tags -> good_phrases (good_phrase_id));
joinable!(person_urls -> persons (person_id));

allow_tables_to_appear_in_same_query!(
    good_phrase_requests,
    good_phrase_tags,
    good_phrases,
    good_phrases_and_tags,
    person_urls,
    persons,
);
