table! {
    posts (id) {
        id -> Int4,
        body -> Nullable<Varchar>,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
