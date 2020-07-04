pub const ALIAS: &str = "gh";

const GITHUB_URL: &str = "https://github.com";
const USER_INDICATOR: char = '@';




pub fn to_github_url(query: &str) -> String {
    let params = &query[ALIAS.len()..];
    let params = params.trim();

    if params.is_empty() {
        return GITHUB_URL.to_string();
    }

    if params.starts_with(USER_INDICATOR) {
        return to_github_profile_url(&params[1..]);
    }

    to_github_search_url(&params)
}



fn to_github_profile_url(profile: &str) -> String {
    format!("{}/{}", GITHUB_URL, profile)
}



fn to_github_search_url(query: &str) -> String {
    let encoded = crate::encoder::encode(query);
    let search_url = format!("{}/search?q={}", GITHUB_URL, encoded);

    search_url
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_twitter_url() {
        let fake_query = "tw";
        assert_eq!(to_github_url(fake_query), GITHUB_URL);
    }

    #[test]
    fn test_construct_twitter_url_query() {
        let fake_query = "tw hello world";
        assert_eq!(to_github_url(fake_query), format!("{}/search?q=hello%20world", GITHUB_URL));
    }

    #[test]
    fn test_construct_twitter_url_profile() {
        let fake_query = "tw @water";
        assert_eq!(to_github_url(fake_query), format!("{}/water", GITHUB_URL));
    }

    #[test]
    fn test_construct_twitter_profile_url() {
        let fake_profile = "abcde";
        assert_eq!(to_github_profile_url(fake_profile), format!("{}/abcde", GITHUB_URL));
    }

    #[test]
    fn test_construct_twitter_search_url() {
        let fake_query = "hello world";
        assert_eq!(to_github_search_url(fake_query), format!("{}/search?q=hello%20world", GITHUB_URL));
    }
}