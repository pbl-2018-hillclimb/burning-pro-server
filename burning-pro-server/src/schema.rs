table! {
    imprudence_tags (imprudence_tag_id) {
        imprudence_tag_id -> Integer,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    imprudences (imprudence_id) {
        imprudence_id -> Integer,
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
    imprudences_and_tags (imprudence_and_tag_id) {
        imprudence_and_tag_id -> Integer,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        imprudence_id -> Integer,
        imprudence_tag_id -> Integer,
    }
}

table! {
    person_and_urls (person_and_url_id) {
        person_and_url_id -> Integer,
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
        display_name -> Nullable<Text>,
        twitter -> Nullable<Text>,
    }
}

joinable!(imprudences -> persons (person_id));
joinable!(imprudences_and_tags -> imprudence_tags (imprudence_tag_id));
joinable!(imprudences_and_tags -> imprudences (imprudence_id));
joinable!(person_and_urls -> persons (person_id));

allow_tables_to_appear_in_same_query!(
    imprudence_tags,
    imprudences,
    imprudences_and_tags,
    person_and_urls,
    persons,
);
