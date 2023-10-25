// #[cfg(test)]
// mod test{
//     use std::env;

//     use diesel::{PgConnection, Connection};
//     use dotenv::dotenv;
//     use crate::{database::{products::{db_get_all_products, db_create_product, db_delete_product}, catagories::{db_get_all_catagories, db_create_category, db_delete_category}}, models::{product::NewProduct, category::Newcategory}};

//     #[test]
//     fn catagories_test() {
//         dotenv().ok();

//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         let conn = &mut PgConnection::establish(&database_url)
//             .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

//         let new_category1 = Newcategory{ name: "shirts".to_string() };
//         let new_category2 = Newcategory{ name: "pants".to_string() };
//         let new_category3 = Newcategory{ name: "shoes".to_string() };

//         let created_category1 = db_create_category(conn, new_category1);
//         let created_category2 = db_create_category(conn, new_category2);
//         let created_category3 = db_create_category(conn, new_category3);

//         assert!(created_category1.is_ok());
//         assert!(created_category2.is_ok());
//         assert!(created_category3.is_ok());

//         let catagories = db_get_all_catagories(conn).unwrap();

//         assert!(catagories.is_some());


//         let new_category4 = Newcategory{ name: "hats".to_string()};
//         let created_category4 = db_create_category(conn, new_category4);
//         assert!(created_category4.is_ok());

//         let deleted_category4 = db_delete_category(conn, 4);
//         assert!(deleted_category4.is_ok());
//     }

//     #[test]
//     fn products_test() {
//         dotenv().ok();

//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         let conn = &mut PgConnection::establish(&database_url)
//             .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

//         let new_product1 = NewProduct{
//             name: "test_name".to_string(),
//             description: Some("test_description".to_string()),
//             category_id: 1,
//             price: bigdecimal::BigDecimal::from(10),
//             inventory: 10,
//         };

//         let new_product2 = NewProduct{
//             name: "test_name2".to_string(),
//             description: Some("test_description2".to_string()),
//             category_id: 2,
//             price: bigdecimal::BigDecimal::from(11),
//             inventory: 11,
//         };

//         let new_product3 = NewProduct{
//             name: "test_name3".to_string(),
//             description: Some("test_description3".to_string()),
//             category_id: 3,
//             price: bigdecimal::BigDecimal::from(12),
//             inventory: 12,
//         };

//         let created_product1 = db_create_product(conn, new_product1);
//         let created_product2 = db_create_product(conn, new_product2);
//         let created_product3 = db_create_product(conn, new_product3);

//         assert!(created_product1.is_ok());
//         assert!(created_product2.is_ok());
//         assert!(created_product3.is_ok());

//         let products = db_get_all_products(conn).unwrap();

//         assert!(products.is_some());

//         let deleted_product1 = db_delete_product(conn, "1".to_string());
//         let deleted_product2 = db_delete_product(conn, "2".to_string());

//         assert!(deleted_product1.is_ok());
//         assert!(deleted_product2.is_ok());

//         let new_product4 = NewProduct{
//             name: "test_name4".to_string(),
//             description: Some("test_description4".to_string()),
//             category_id: 3,
//             price: bigdecimal::BigDecimal::from(13),
//             inventory: 13,
//         };

//         let create_product4 = db_create_product(conn, new_product4);

//         assert!(create_product4.is_ok());
//     }
// }