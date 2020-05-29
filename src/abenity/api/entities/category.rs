#[derive(Deserialize, Debug)]
pub struct Category {
    id: Option<String>,
    key: Option<String>,
    title: Option<String>,
    offer_count: Option<u16>,
    children: Option<Vec<crate::abenity::api::entities::category::Category>>,
}

#[cfg(test)]
mod tests {

    use crate::abenity::api::entities::category::Category;

    #[test]
    fn test_parse_json_str_to_value() {
        let json_str = r#"
            {
                "id": "322",
                "key": "Attractions_and_Tours",
                "title": "Attractions & Tours",
                "offer_count": 406,
                "children": [
                    {
                        "id": "339",
                        "key": "Attractions_and_Tours/Dinner_Theaters",
                        "title": "Dinner Theaters",
                        "offer_count": 9,
                        "children": null
                    }
                ]
            }
        "#;
        let v: serde_json::Value = serde_json::from_str(json_str).expect("from_str");
        assert_eq!(v["id"], "322", "id");
        let result: serde_json::Result<Category> = serde_json::from_str(json_str);
        assert!(result.is_ok());
    }

}