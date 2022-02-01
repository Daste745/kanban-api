table! {
    boards (id) {
        id -> Uuid,
        owner -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    cards (id) {
        id -> Uuid,
        list -> Uuid,
        content -> Nullable<Text>,
        labels -> Nullable<Array<Text>>,
    }
}

table! {
    lists (id) {
        id -> Uuid,
        board -> Uuid,
        name -> Text,
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
joinable!(cards -> lists (list));
joinable!(lists -> boards (board));

allow_tables_to_appear_in_same_query!(
    boards,
    cards,
    lists,
    users,
);
