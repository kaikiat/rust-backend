// @generated automatically by Diesel CLI.

diesel::table! {
    solutions (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        code -> Text,
        created_on -> Timestamp,
        modified_on -> Nullable<Timestamp>,
    }
}
