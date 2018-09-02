table! {
    users (id) {
        id -> Int4,
        email -> Citext,
        passhash -> Text,
        user_id -> Uuid,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
    }
}
