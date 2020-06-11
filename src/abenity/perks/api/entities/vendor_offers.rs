#[derive(Deserialize, Debug)]
pub struct VendorOffers {
    vendor: Option<crate::abenity::perks::api::entities::vendor::Vendor>,
    offers: Option<Vec<crate::abenity::perks::api::entities::offer::Offer>>,
}

#[cfg(test)]
mod tests {

    use crate::abenity::perks::api::entities::vendor_offers::VendorOffers;

    #[test]
    fn test_parse_json_str_to_value() {
        let json_str = r#"
            {
                "vendor": {
                    "name": "Windshield Replacers",
                    "id": "123",
                    "logo": {
                        "url": "http://abenity.s3.amazonaws.com/vendor/assets/logos/local-merchant@2x.png",
                        "width": "306",
                        "height": "96"
                    }
                },
                "offers": [
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
                ]
            }
        "#;
        let v: serde_json::Value = serde_json::from_str(json_str).expect("from_str");
        assert_eq!(v["vendor"]["id"], "123", "vendor");
        assert_eq!(v["offers"][0]["id"], "106398676", "offer");
        assert_eq!(v["offers"][0]["locations"][0]["id"], "107508475", "location");
        let result: serde_json::Result<VendorOffers> = serde_json::from_str(json_str);
        assert!(result.is_ok());
    }

}
