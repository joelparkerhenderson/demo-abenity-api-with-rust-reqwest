// Demo of Abenity API with Rust reqwest crate
//
// Example API calls:
//
//    curl -X GET -u X3dh4BdWpzny9Vm65ZnwIsRTkdMX5UAw:clientapp \
//    https://acme.abenity.com/perks/api/v2/feed/categories.json
//
//    curl -X GET -u X3dh4BdWpzny9Vm65ZnwIsRTkdMX5UAw:clientapp \
//    https://acme.abenity.com/perks/api/v2/feed/offers.json?category_id=4

extern crate futures;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate reqwest;
use futures::executor::block_on;
pub mod abenity;

async fn async_main() {
    let client = reqwest::Client::new();
    let config = crate::abenity::perks::api::config::Config::from_env();
    // let categories: Vec<crate::abenity::perks::api::entities::category::Category> = abenity::perks::api::endpoints::categories::get_categories(client, base).await;
    // println!("{:#?}", categories);
}
//#[tokio::main]
fn main() {
    block_on(async_main())
}

#[cfg(test)]
mod tests {

    // use super::*;

    // #[test]
    // fn test_example() {
    //     assert!(true);
    // }
    
}

