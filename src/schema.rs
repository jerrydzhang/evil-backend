// @generated automatically by Diesel CLI.

diesel::table! {
    carts (id) {
        id -> Int4,
        user_id -> Varchar,
        product_id -> Varchar,
        quantity -> Int4,
    }
}

diesel::table! {
    orders (id) {
        id -> Varchar,
        user_id -> Varchar,
        products -> Jsonb,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
        category -> Nullable<Varchar>,
        price -> Nullable<Numeric>,
        inventory -> Int4,
        last_updated -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        images -> Nullable<Array<Nullable<Text>>>,
        price_id -> Nullable<Varchar>,
        active -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        roles -> Nullable<Array<Nullable<Text>>>,
        stripe_id -> Nullable<Varchar>,
    }
}

diesel::joinable!(carts -> products (product_id));
diesel::joinable!(carts -> users (user_id));
diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    carts,
    orders,
    products,
    users,
);
