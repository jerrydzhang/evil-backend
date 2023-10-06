use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods};

use crate::models::product::{NewProduct, DisplayProduct};
use crate::models::catagory::Catagory;
use crate::schema::catagories;
use crate::{models::product::Product, errors::error::AppError};
use crate::schema::products::dsl::*;

pub(crate) fn db_get_all_products(
    conn: &mut PgConnection,
) -> Result<Option<Vec<DisplayProduct>>, AppError> {
    // do a left join of products and catagories
    let all_products = products
        .left_join(catagories::table)
        .load::<(Product, Option<Catagory>)>(conn)?;

    // map each catagory to the associated catagory
    let all_display_products = all_products
        .into_iter()
        // construct a DisplayProduct with the product and catagory
        .map(|(product, catagory)| {
            DisplayProduct::new(
                product,
                catagory.unwrap().name,
            )
        })
        .collect::<Vec<DisplayProduct>>();

    Ok(Some(all_display_products))
}

pub(crate) fn db_get_product_by_id(
    conn: &mut PgConnection,
    product_id: i32,
) -> Result<DisplayProduct, AppError> {
    // do an innerjoin of the product and its corresponding catagory
    let product_with_catagory = products
        .find(product_id)
        .inner_join(catagories::table)
        .first::<(Product, Catagory)>(conn)?;

    // consturct a DisplayProduct with with product and catagory
    let display_product = DisplayProduct::new(
        product_with_catagory.0,
        product_with_catagory.1.name,
    );
    
    // return the display product
    Ok(display_product)
}

pub(crate) fn db_get_multiple_products_by_id(
    conn: &mut PgConnection,
    product_ids: Vec<i32>,
) -> Result<Vec<DisplayProduct>, AppError> {
    // filter products and do an inner join with catagories
    let products_with_catagory = products
        .filter(id.eq_any(product_ids))
        .inner_join(catagories::table)
        .load::<(Product, Catagory)>(conn)?;

    // combine each product with the corresponding display
    let display_products = products_with_catagory
        .into_iter()
        // construct a DisplayProduct with the product and catagory
        .map(|(product, catagory)| {
            DisplayProduct::new(
                product,
                catagory.name,
            )
        })
    .collect::<Vec<DisplayProduct>>();

    Ok(display_products)
}

pub(crate) fn db_get_products_by_catagory(
    conn: &mut PgConnection,
    id_catagory: i32,
) -> Result<Option<Vec<DisplayProduct>>, AppError> {
    // do an inner join of the product and its corresponding catagory
    let products_with_catagory = products
        .filter(catagory_id.eq(id_catagory))
        .inner_join(catagories::table)
        .load::<(Product, Catagory)>(conn)?;

    // map each catagory to the associated catagory
    let display_products = products_with_catagory
        .into_iter()
        // construct a DisplayProduct with the product and catagory
        .map(|(product, catagory)| {
            DisplayProduct::new(
                product,
                catagory.name,
            )
        })
        .collect::<Vec<DisplayProduct>>();

    Ok(Some(display_products))
}

pub(crate) fn db_create_product(
    conn: &mut PgConnection,
    new_product: NewProduct,
) -> Result<Product, AppError> {
    let product = diesel::insert_into(products)
        .values(&new_product)
        .get_result::<Product>(conn)?;

    Ok(product)
}

pub(crate) fn db_update_product(
    conn: &mut PgConnection,
    product_id: i32,
    new_product: NewProduct,
) -> Result<Product, AppError> {
    let current_time = chrono::Local::now().naive_local();

    diesel::update(products.find(product_id))
        .set(last_updated.eq(current_time))
        .execute(conn)?;
    
    let product = diesel::update(products.find(product_id))
        .set(&new_product)
        .get_result::<Product>(conn)?;

    Ok(product)
}

pub(crate) fn db_delete_product(
    conn: &mut PgConnection,
    product_id: i32,
) -> Result<usize, AppError> {
    let res = diesel::delete(products.find(product_id))
        .execute(conn)?;

    Ok(res)
}