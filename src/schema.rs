table! {
    boards (id) {
        id -> Uuid,
        owner -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        mail -> Text,
        password -> Text,
    }
}

joinable!(boards -> users (owner));

allow_tables_to_appear_in_same_query!(
    boards,
    users,
);
