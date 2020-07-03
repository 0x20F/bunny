extern crate percent_encoding;


use percent_encoding::{utf8_percent_encode as encode, AsciiSet, CONTROLS};


// What characters to encode
const CHARACTERS: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'/');




pub fn to_twitter_url(query: &str) -> String {
    if query == "tw" {
        let url = "https://twitter.com";

        url.to_string()
    } else if &query[..4] == "tw @" {
        to_twitter_profile_url(&query[4..])
    } else {
        to_twitter_search_url(&query[3..])
    }
}


fn to_twitter_profile_url(profile: &str) -> String {
    format!("https://twitter.com/{}", profile)
}


fn to_twitter_search_url(query: &str) -> String {
    let encoded = encode(query, CHARACTERS).to_string();
    let search_url = format!("https://twitter.com/search?q={}", encoded);

    search_url
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_twitter_url() {
        let fake_query = "tw";
        assert_eq!(to_twitter_url(fake_query), "https://twitter.com");
    }

    #[test]
    fn test_construct_twitter_url_query() {
        let fake_query = "tw hello world";
        assert_eq!(to_twitter_url(fake_query), "https://twitter.com/search?q=hello%20world");
    }

    #[test]
    fn test_construct_twitter_url_profile() {
        let fake_query = "tw @water";
        assert_eq!(to_twitter_url(fake_query), "https://twitter.com/water");
    }

    #[test]
    fn test_construct_twitter_profile_url() {
        let fake_profile = "abcde";
        assert_eq!(to_twitter_profile_url(fake_profile), "https://twitter.com/abcde");
    }

    #[test]
    fn test_construct_twitter_search_url() {
        let fake_query = "hello world";
        assert_eq!(to_twitter_search_url(fake_query), "https://twitter.com/search?q=hello%20world");
    }
}

