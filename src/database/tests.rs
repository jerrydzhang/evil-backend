#[cfg(test)]
mod test{
    use std::env;

    use diesel::{PgConnection, Connection};
    use dotenv::dotenv;
    use crate::{database::{products::{db_get_all_products, db_create_product, db_delete_product}, catagories::{db_get_all_catagories, db_create_catagory, db_delete_catagory}}, models::{product::NewProduct, catagory::NewCatagory}};

    #[test]
    fn catagories_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = &mut PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        let new_catagory1 = NewCatagory{ name: "shirts".to_string() };
        let new_catagory2 = NewCatagory{ name: "pants".to_string() };
        let new_catagory3 = NewCatagory{ name: "shoes".to_string() };

        let created_catagory1 = db_create_catagory(conn, new_catagory1);
        let created_catagory2 = db_create_catagory(conn, new_catagory2);
        let created_catagory3 = db_create_catagory(conn, new_catagory3);

        assert!(created_catagory1.is_ok());
        assert!(created_catagory2.is_ok());
        assert!(created_catagory3.is_ok());

        let catagories = db_get_all_catagories(conn).unwrap();

        assert!(catagories.is_some());


        let new_catagory4 = NewCatagory{ name: "hats".to_string()};
        let created_catagory4 = db_create_catagory(conn, new_catagory4);
        assert!(created_catagory4.is_ok());

        let deleted_catagory4 = db_delete_catagory(conn, 4);
        assert!(deleted_catagory4.is_ok());
    }

    #[test]
    fn products_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = &mut PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        let new_product1 = NewProduct{
            name: "test_name".to_string(),
            description: Some("test_description".to_string()),
            catagory_id: 1,
            price: bigdecimal::BigDecimal::from(10),
            inventory: 10,
        };

        let new_product2 = NewProduct{
            name: "test_name2".to_string(),
            description: Some("test_description2".to_string()),
            catagory_id: 2,
            price: bigdecimal::BigDecimal::from(11),
            inventory: 11,
        };

        let new_product3 = NewProduct{
            name: "test_name3".to_string(),
            description: Some("test_description3".to_string()),
            catagory_id: 3,
            price: bigdecimal::BigDecimal::from(12),
            inventory: 12,
        };

        let created_product1 = db_create_product(conn, new_product1);
        let created_product2 = db_create_product(conn, new_product2);
        let created_product3 = db_create_product(conn, new_product3);

        assert!(created_product1.is_ok());
        assert!(created_product2.is_ok());
        assert!(created_product3.is_ok());

        let products = db_get_all_products(conn).unwrap();

        assert!(products.is_some());

        let deleted_product1 = db_delete_product(conn, 1);
        let deleted_product2 = db_delete_product(conn, 2);

        assert!(deleted_product1.is_ok());
        assert!(deleted_product2.is_ok());

        let new_product4 = NewProduct{
            name: "test_name4".to_string(),
            description: Some("test_description4".to_string()),
            catagory_id: 3,
            price: bigdecimal::BigDecimal::from(13),
            inventory: 13,
        };

        let create_product4 = db_create_product(conn, new_product4);

        assert!(create_product4.is_ok());
    }
}