// @generated automatically by Diesel CLI.

diesel::table! {
    carts (id) {
        id -> Int4,
        user_id -> Varchar,
        product_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    catagories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        catagory_id -> Int4,
        price -> Numeric,
        inventory -> Int4,
        last_updated -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(carts -> products (product_id));
diesel::joinable!(carts -> users (user_id));
diesel::joinable!(products -> catagories (catagory_id));

diesel::allow_tables_to_appear_in_same_query!(
    carts,
    catagories,
    products,
    users,
);
