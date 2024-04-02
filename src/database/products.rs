use diesel::result::Error;
use diesel::{ExpressionMethods, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl};

use crate::models::product::{NewProduct, Product};
use crate::schema::products::dsl::*;

use super::carts::db_delete_cart_items_by_product;

pub(crate) fn db_get_all_products(conn: &mut PgConnection) -> Result<Option<Vec<Product>>, Error> {
    // do a left join of products and categories
    let all_products = products.load::<Product>(conn)?;

    Ok(Some(all_products))
}

pub(crate) fn db_get_active_products(
    conn: &mut PgConnection,
) -> Result<Option<Vec<Product>>, Error> {
    // do a left join of products and categories
    let all_products = products.filter(active.eq(true)).load::<Product>(conn)?;

    Ok(Some(all_products))
}

pub(crate) fn db_get_product_by_name(
    conn: &mut PgConnection,
    product_name: String,
) -> Result<Option<Vec<Product>>, Error> {
    // do an innerjoin of the product and its corresponding category
    let all_products = products
        .filter(active.eq(true))
        .filter(name.ilike(product_name))
        .load::<Product>(conn)?;

    // return the display product
    Ok(Some(all_products))
}

pub(crate) fn db_get_product_by_id(
    conn: &mut PgConnection,
    product_id: String,
) -> Result<Product, Error> {
    // do an innerjoin of the product and its corresponding category
    let product = products.filter(id.eq(product_id)).first::<Product>(conn)?;

    // return the display product
    Ok(product)
}

pub(crate) fn db_get_multiple_products_by_id(
    conn: &mut PgConnection,
    product_ids: Vec<String>,
) -> Result<Vec<Product>, Error> {
    // filter products and do an inner join with categories
    let products_by_id = products
        .filter(id.eq_any(product_ids))
        .load::<Product>(conn)?;

    Ok(products_by_id)
}

pub(crate) fn db_get_categories(conn: &mut PgConnection) -> Result<Option<Vec<String>>, Error> {
    let load = products
        .filter(active.eq(true))
        .select(category)
        .distinct()
        .load::<Option<String>>(conn)?;

    let categories = load.into_iter().filter_map(|x| x).collect();

    Ok(Some(categories))
}

pub(crate) fn db_get_products_by_category(
    conn: &mut PgConnection,
    category_name: String,
) -> Result<Option<Vec<Product>>, Error> {
    let products_with_category = products
        .filter(category.eq(category_name))
        .load::<Product>(conn)?;

    Ok(Some(products_with_category))
}

pub(crate) fn db_get_active_products_by_category(
    conn: &mut PgConnection,
    category_name: String,
) -> Result<Option<Vec<Product>>, Error> {
    let products_with_category = products
        .filter(category.eq(category_name))
        .filter(active.eq(true))
        .load::<Product>(conn)?;

    Ok(Some(products_with_category))
}

pub(crate) fn db_expand_products(
    conn: &mut PgConnection,
    product_ids: Vec<String>,
) -> Result<Vec<Product>, Error> {
    let expanded_products = products
        .filter(id.eq_any(product_ids))
        .load::<Product>(conn)?;

    Ok(expanded_products)
}

pub(crate) fn db_create_product(
    conn: &mut PgConnection,
    new_product: Product,
) -> Result<Product, Error> {
    let product = diesel::insert_into(products)
        .values(&new_product)
        .get_result::<Product>(conn)?;

    Ok(product)
}

pub(crate) fn db_update_product(
    conn: &mut PgConnection,
    new_product: NewProduct,
) -> Result<Product, Error> {
    diesel::update(products.find(new_product.id.clone().unwrap()))
        .set(&new_product)
        .get_result::<Product>(conn)?;

    let current_time = chrono::Local::now().naive_local();
    let product = diesel::update(products.find(new_product.id.clone().unwrap()))
        .set(last_updated.eq(current_time))
        .get_result::<Product>(conn)?;

    Ok(product)
}

pub(crate) fn db_delete_product(
    conn: &mut PgConnection,
    product_id: String,
) -> Result<usize, Error> {
    // delete all cart items associated with the product
    db_delete_cart_items_by_product(conn, product_id.clone())?;

    // delete the product
    let res = diesel::delete(products.find(product_id.clone())).execute(conn)?;

    Ok(res)
}
