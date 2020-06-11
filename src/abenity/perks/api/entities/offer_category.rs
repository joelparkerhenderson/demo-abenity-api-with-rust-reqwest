#[derive(Deserialize, Debug)]
pub struct OfferCategory {
    category_id: Option<String>,
    parent_id: Option<String>,
    category_key: Option<String>,
    main_cat: Option<String>,
    sub_cat: Option<String>,
    top_offers: Option<String>,
    provider_id_restricted: Option<String>,
}

#[cfg(test)]
mod tests {

    use crate::abenity::perks::api::entities::offer_category::OfferCategory;

    #[test]
    fn test_parse_json_str_to_value() {
        let json_str = r#"
            {
                "category_id": "51",
                "parent_id": "4",
                "category_key": "Automotive/New_and_Used_Cars",
                "main_cat": "Automotive",
                "sub_cat": "New & Used Cars",
                "top_offers": "334",
                "provider_id_restricted": "0"
            }
        "#;
        let x: serde_json::Value = serde_json::from_str(json_str).expect("from_str");
        assert_eq!(x["category_id"], "51", "category_id");
        let result: serde_json::Result<OfferCategory> = serde_json::from_str(json_str);
        assert!(result.is_ok());
    }

}