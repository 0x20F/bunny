mod config;
mod library;

use library::Library;
use crate::command::Command;



pub fn open_book(cmd: &mut Command) -> String {
    let lib = Library::new();

    match lib.get_url(cmd) {
        Some(url) => url,
        None => {
            let query = format!("{} {}", cmd.alias, cmd.params);
            construct_search_engine_query(&query)
        }
    }
}



fn construct_search_engine_query(data: &str) -> String {
    let encoded = crate::encoder::encode(&data);

    format!("https://google.com/search?q={}", encoded)
}











#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_engine_query() {
        let text = "hello world";
        let query = construct_search_engine_query(text);

        assert_eq!(query, "https://google.com/search?q=hello%20world");
    }
}