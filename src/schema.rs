table! {
    islands (id) {
        id -> Uuid,
        owner_id -> Uuid,
        name -> Text,
        is_active -> Bool,
    }
}
