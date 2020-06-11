#[derive(Deserialize, Debug)]
pub struct Offer {
    id: Option<String>,
    category_id: Option<String>,
    exp_date: Option<String>,
    alternate_vendor_name: Option<String>,
    days_old: Option<u16>,
    title: Option<String>,
    link: Option<String>,
    offer_category_list: Option<Vec<crate::abenity::perks::api::entities::offer_category::OfferCategory>>,
    location_list: Option<Vec<crate::abenity::perks::api::entities::location::Location>>,
}

#[cfg(test)]
mod tests {

    use crate::abenity::perks::api::entities::offer::Offer;

    #[test]
    fn test_parse_json_str_to_value() {
        let json_str = r#"
            {
                "id": "106398676",
                "category_id": "50",
                "alternate_vendor_name": "Roadrunner",
                "exp_date": "2017-07-03",
                "days_old": 65,
                "title": "FREE Chip Repair with your comprehensive insurance",
                "link": "https://acme.abenity.com/perks/offer/10:1662479",
                "category": [
                    {
                        "category_id": "165",
                        "parent_id": "4",
                        "category_key": "Automotive\/Gas_Stations",
                        "main_cat": "Automotive",
                        "sub_cat": "Gas Stations",
                        "top_offers": "",
                        "provider_id_restricted": "0"
                    }
                ],
                "locations": [
                    {
                        "id": "107508475",
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
                        "fax": ""
                    }
                ]
            }
        "#;
        let v: serde_json::Value = serde_json::from_str(json_str).expect("from_str");
        assert_eq!(v["id"], "106398676", "id");
        let result: serde_json::Result<Offer> = serde_json::from_str(json_str);
        assert!(result.is_ok());
    }

}
