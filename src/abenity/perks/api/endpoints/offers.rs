use reqwest::Url;

pub fn get_offers_url(base: Url, category_id: &str) -> Url {
    base.join(&format!("offers.json?category={}", category_id)).expect("url")
}

pub async fn get_offers(
    config: crate::abenity::perks::api::config::Config,
    client: reqwest::Client, 
    base: Url, 
    category_id: &str
) -> Vec<crate::abenity::perks::api::entities::vendor_offers::VendorOffers> {
    let url = get_offers_url(config.base, category_id);
    client
    .get(url)
    .basic_auth(config.username, Some(config.password))
    .send().await.expect("send")
    .json::<Vec<crate::abenity::perks::api::entities::vendor_offers::VendorOffers>>().await.expect("json")
}

#[cfg(test)]
mod tests {

    use crate::abenity::perks::api::endpoints::offers::*;
    use reqwest::Url;

    #[test]
    fn test_get_offers_url() {
        let base = Url::parse("https://acme.abenity.com/perks/api/v2/feed/").expect("url");
        let category_id = "my_category_id";
        let url = get_offers_url(base, category_id);
        assert_eq!(url.as_str(), "https://acme.abenity.com/perks/api/v2/feed/offers.json?category=my_category_id");
    }

}
