use reqwest::Url;

pub fn get_categories_url(base: Url) -> Url {
    base.join("categories.json").expect("url")
}

pub async fn get_categories(
    config: crate::abenity::api::config::Config,
    client: reqwest::Client, 
    base: Url
) -> Vec<crate::abenity::api::entities::category::Category> {
    let url = get_categories_url(base);
    client
    .get(url)
    .basic_auth(config.username, Some(config.password))
    .send().await.expect("send")
    .json::<Vec<crate::abenity::api::entities::category::Category>>().await.expect("json")
}

#[cfg(test)]
mod tests {

    use crate::abenity::api::endpoints::categories::*;
    use reqwest::Url;

    #[test]
    fn test_get_categories_url() {
        let base = Url::parse("https://acme.abenity.com/perks/api/v2/feed/").expect("url");
        let url = get_categories_url(base);
        assert_eq!(url.as_str(), "https://acme.abenity.com/perks/api/v2/feed/categories.json");
    }

}
