use diesel::table;

table! {
    users (id) {
        id -> Integer,
        lang -> Text,
    }
}
