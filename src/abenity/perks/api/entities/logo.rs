#[derive(Deserialize, Debug)]
pub struct Logo {
    url: Option<String>,
    width: Option<String>,
    height: Option<String>,
}

#[cfg(test)]
mod tests {

    use crate::abenity::perks::api::entities::logo::Logo;

    #[test]
    fn test_parse_json_str_to_value() {
        let json_str = r#"
            {
                "url": "http://abenity.s3.amazonaws.com/vendor/assets/logos/local-merchant@2x.png",
                "width": "306",
                "height": "96"
            }
        "#;
        let v: serde_json::Value = serde_json::from_str(json_str).expect("from_str");
        assert_eq!(v["width"], "306", "width");
        let result: serde_json::Result<Logo> = serde_json::from_str(json_str);
        assert!(result.is_ok());
    }

}
