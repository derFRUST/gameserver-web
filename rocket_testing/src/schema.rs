table! {
    games (id) {
        id -> Integer,
        name -> Text,
        image -> Text,
    }
}

table! {
    servers (id) {
        id -> Integer,
        name -> Text,
        game_id -> Integer,
        status -> Text,
    }
}

joinable!(servers -> games (game_id));

allow_tables_to_appear_in_same_query!(
    games,
    servers,
);
