#[derive(Deserialize, Debug)]
pub struct Location {
    id: Option<String>,
    name: Option<String>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    country: Option<String>,
    latitude: Option<String>,
    longitude: Option<String>,
    coordinate_accuracy: Option<String>,
    arcgis_score: Option<String>,
    phone: Option<String>,
    fax: Option<String>,
}

#[cfg(test)]
mod tests {

    use crate::abenity::api::entities::location::Location;
    
    #[test]
    fn test_parse_json_str_to_value() {
        let json_str = r#"
            {
                "name": "",
                "address": "495 1st Street",
                "city": "Idaho Falls",
                "state": "ID",
                "zip": "83401",
                "country": "US",
                "latitude": "43.497135",
                "longitude": "-112.024162",
                "coordinate_accuracy": "address",
                "arcgis_score": null,
                "phone": "208-525-3230",
                "fax": "",
                "id": "107508475"
            }
        "#;
        let v: serde_json::Value = serde_json::from_str(json_str).expect("from_str");
        assert_eq!(v["id"], "107508475", "id");
        let result: serde_json::Result<Location> = serde_json::from_str(json_str);
        assert!(result.is_ok());
    }

}
