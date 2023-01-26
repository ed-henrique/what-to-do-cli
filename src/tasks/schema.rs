// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        task_name -> Text,
    }
}
