extern crate percent_encoding;


use percent_encoding::{utf8_percent_encode as encode, AsciiSet, CONTROLS};


// What characters to encode
const CHARACTERS: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'/');


pub fn to_google_search_url(query: &str) -> String {
    let encoded = encode(query, CHARACTERS).to_string();
    let search_url = format!("https://google.com/search?q={}", encoded);

    search_url
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_google_search_url() {
        let fake_query = "hello";
        assert_eq!(to_google_search_url(fake_query), "https://google.com/search?q=hello");
    }

    #[test]
    fn test_construct_google_search_url_with_encoding() {
        let fake_query = "hello world";
        assert_eq!(to_google_search_url(fake_query), "https://google.com/search?q=hello%20world");
    }
}