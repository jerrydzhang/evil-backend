// use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
// use stripe::{Product, Price, CreateProduct, CreatePrice, Currency, IdOrCreate, StripeError, Client};

// use crate::models::product::NewProduct;


// pub(crate) async fn stripe_create_product(
//     stripe_client: &Client,
//     product: NewProduct,
// ) -> Result<Product, StripeError> {
//     let stripe_product = {
//         let mut create_product = CreateProduct::new(&product.name);
//         create_product.description = product.description.as_deref();
//         create_product.images = match product.images {
//             Some(_) => {
//                 product.clone().images.map(|images| images.iter().flatten().cloned().collect())
//             },
//             None => None,
//         };
//         create_product.metadata = Some(std::collections::HashMap::from([(
//             String::from("async-stripe"),
//             String::from("true"),
//         )]));
//         Product::create(&stripe_client, create_product).await?
//     };

//     let price = {
//         let mut create_price = CreatePrice::new(Currency::USD);
//         create_price.product = Some(IdOrCreate::Id(&stripe_product.id));
//         create_price.metadata = Some(std::collections::HashMap::from([(
//             String::from("async-stripe"),
//             String::from("true"),
//         )]));
//         create_price.unit_amount = Some((product.clone().price.unwrap() * BigDecimal::from_i32(100).unwrap()).to_i64().unwrap());
//         create_price.expand = &["product"];
//         Price::create(&stripe_client, create_price).await?
//     };

//     log::info!("created a product {:?} at price {} {}",
//         stripe_product.name.clone().unwrap(),
//         price.unit_amount.unwrap() / 100,
//         price.currency.unwrap()
//     );
    
//     Ok(stripe_product)
// }