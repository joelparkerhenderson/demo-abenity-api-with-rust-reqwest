#[derive(Deserialize, Debug)]
pub struct Offer {
    id: Option<String>,
    category_id: Option<String>,
    exp_date: Option<String>,
    locations: Option<Vec<crate::abenity::api::entities::location::Location>>,
    days_old: Option<u16>,
    title: Option<String>,
    link: Option<String>,
}

#[cfg(test)]
mod tests {

    use crate::abenity::api::entities::offer::Offer;

    #[test]
    fn test_parse_json_str_to_value() {
        let json_str = r#"
            {
                "category_id": "50",
                "exp_date": "2017-07-03",
                "locations": [
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
                ],
                "days_old": 65,
                "id": "106398676",
                "title": "FREE Chip Repair with your comprehensive insurance",
                "link": "https://acme.abenity.com/perks/offer/10:1662479"
            }
        "#;
        let v: serde_json::Value = serde_json::from_str(json_str).expect("from_str");
        assert_eq!(v["id"], "106398676", "id");
        let result: serde_json::Result<Offer> = serde_json::from_str(json_str);
        assert!(result.is_ok());
    }

}
