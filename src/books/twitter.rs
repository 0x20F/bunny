const TWITTER_URL: &str = "https://twitter.com";
const USER_INDICATOR: char = '@';




pub fn construct_twitter_url(params: &str) -> String {
    if params.is_empty() {
        return TWITTER_URL.to_string();
    }

    if params.starts_with(USER_INDICATOR) {
        return construct_twitter_profile_url(&params[1..])
    }

    construct_twitter_search_url(&params)
}



fn construct_twitter_profile_url(profile: &str) -> String {
    format!("{}/{}", TWITTER_URL, profile)
}



fn construct_twitter_search_url(query: &str) -> String {
    let encoded = crate::encoder::encode(query);
    let search_url = format!("{}/search?q={}", TWITTER_URL, encoded);

    search_url
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_twitter_url() {
        let fake_query = "";
        assert_eq!(construct_twitter_url(fake_query), TWITTER_URL);
    }

    #[test]
    fn test_construct_twitter_url_query() {
        let fake_query = "hello world";
        assert_eq!(construct_twitter_url(fake_query), format!("{}/search?q=hello%20world", TWITTER_URL));
    }

    #[test]
    fn test_construct_twitter_url_profile() {
        let fake_query = "@water";
        assert_eq!(construct_twitter_url(fake_query), format!("{}/water", TWITTER_URL));
    }

    #[test]
    fn test_construct_twitter_profile_url() {
        let fake_profile = "abcde";
        assert_eq!(construct_twitter_profile_url(fake_profile), format!("{}/abcde", TWITTER_URL));
    }

    #[test]
    fn test_construct_twitter_search_url() {
        let fake_query = "hello world";
        assert_eq!(construct_twitter_search_url(fake_query), format!("{}/search?q=hello%20world", TWITTER_URL));
    }
}

