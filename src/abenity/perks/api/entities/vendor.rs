#[derive(Deserialize, Debug)]
pub struct Vendor {
    id: Option<String>,
    name: Option<String>,
    logo: Option<crate::abenity::perks::api::entities::logo::Logo>,
}

#[cfg(test)]
mod tests {

    use crate::abenity::perks::api::entities::vendor::Vendor;

    #[test]
    fn test_abenity_api_vendor_x_parse_json_str_to_value() {
        let json_str = r#"
            {
                "name": "Windshield Replacers",
                "id": "123",
                "logo": {
                    "url": "http://abenity.s3.amazonaws.com/vendor/assets/logos/local-merchant@2x.png",
                    "width": "306",
                    "height": "96"
                }
            }
        "#;
        let v: serde_json::Value = serde_json::from_str(json_str).expect("from_str");
        assert_eq!(v["id"], "123", "id");
        let result: serde_json::Result<Vendor> = serde_json::from_str(json_str);
        assert!(result.is_ok());
    }

}
