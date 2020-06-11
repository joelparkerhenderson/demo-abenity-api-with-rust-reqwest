use reqwest::Url;

#[derive(Debug)]
pub struct Config {
    pub base: Url,
    pub username: String,
    pub password: String,
}

impl Config {

    pub fn from_env() -> Config {
        Config {
            base: Url::parse(&std::env::var("ABENITY_PERKS_API_BASE").expect("ABENITY_PERKS_API_BASE")).expect("url"),
            username: std::env::var("ABENITY_PERKS_API_USERNAME").expect("ABENITY_PERKS_API_USERNAME"),
            password: std::env::var("ABENITY_PERKS_API_PASSWORD").expect("ABENITY_PERKS_API_PASSWORD"),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::abenity::perks::api::config::*;
    use reqwest::Url;

    #[test]
    fn test_from_env() {
        std::env::set_var("ABENITY_PERKS_API_BASE", "https://example.com/");
        std::env::set_var("ABENITY_PERKS_API_USERNAME", "my_username");
        std::env::set_var("ABENITY_PERKS_API_PASSWORD", "my_password");
        let config = Config::from_env();
        assert_eq!(config.base.as_str(), "https://example.com/");
        assert_eq!(config.username, "my_username");
        assert_eq!(config.password, "my_password");
    }

}