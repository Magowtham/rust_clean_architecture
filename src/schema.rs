// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 20]
        name -> Varchar,
        #[max_length = 20]
        email -> Varchar,
        #[max_length = 20]
        phone -> Varchar,
        #[max_length = 40]
        address -> Varchar,
    }
}
