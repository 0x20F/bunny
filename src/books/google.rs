const GOOGLE_URL: &str = "https://google.com";



pub fn construct_google_search_url(query: &str) -> String {
    let encoded = crate::encoder::encode(query);
    let search_url = format!("{}/search?q={}", GOOGLE_URL, encoded);

    search_url
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url() {
        let fake_query = "hello";
        assert_eq!(construct_google_search_url(fake_query), format!("{}/search?q=hello", GOOGLE_URL));
    }

    #[test]
    fn test_construct_google_search_url_with_encoding() {
        let fake_query = "hello world";
        assert_eq!(construct_google_search_url(fake_query), format!("{}/search?q=hello%20world", GOOGLE_URL));
    }
}