use std::fs;
use toml::Value;
use toml::value::Table;
use key_list::KeyList;




pub fn open_book(query: &str) -> String {
    let (command, params) = command_from_query(&query);

    let contents = fs::read_to_string("books.toml")
        .expect("Something went wrong when reading the file");

    let config = contents.parse::<Value>().unwrap();
    let books = config.as_table().unwrap();

    resolve_book_url(books, command, params)
}


fn command_from_query(query: &str) -> (&str, &str) {
    let clean = query.trim();

    if clean.contains(' ') {
        let space = clean.find(' ').unwrap_or(0);

        let command = &clean[..space];
        let params = &clean[space..];
        return (command, params.trim());
    }

    (clean, "")
}


fn resolve_book_url(books: &Table, command: &str, params: &str) -> String {
    for (_, book) in books.iter() {
        let alias = value_as_str(book, "alias");

        if command == alias {
            return resolve_correct_page(book, params);
        }
    }

    // If nothing was returned in the earlier stages
    // just forward whatever was typed to a search engine
    search_engine_query(format!("{} {}", command, params).as_ref())
}


fn resolve_correct_page(book: &Value, params: &str) -> String {
    let pages = value_as_table(book, "pages");

    for (_, page) in pages.iter() {
        let prefix = value_as_str(page, "prefix");
        let url = value_as_str(page, "url");

        if params.starts_with(prefix) {
            let url = replace_keys(url, remove_prefix(params, prefix));

            return url;
        }
    }

    // If it made it all the way here, no other pages matched
    // just use default
    value_as_str(book, "default").to_owned()
}


fn value_as_str<'a>(value: &'a Value, key: &str) -> &'a str {
    match value.get(key) {
        Some(v) => v.as_str().unwrap(),
        None => ""
    }
}


fn value_as_table<'a>(value: &'a Value, key: &str) -> &'a Table {
    value.get(key)
        .unwrap()
        .as_table()
        .unwrap()
}


fn remove_prefix<'a>(text: &'a str, prefix: &'a str) -> &'a str {
    &text[prefix.len()..].trim()
}


fn replace_keys(text: &str, data: &str) -> String {
    let keys = KeyList::new(text, '{', '}');
    let mut clean = String::from(text);

    for key in keys {
        match key {
            "{encoded}" => clean = text.replace(
                key,
                crate::encoder::encode(data).as_ref()
            ),
            "{raw}" => clean = text.replace(key, data),
            _ => ()
        }
    }

    clean
}


fn search_engine_query(data: &str) -> String {
    // google, for now
    let encoded = crate::encoder::encode(data);
    format!("https://google.com/search?q={}", encoded)
}








#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_no_whitespace() {
        let actual = command_from_query("gh");
        let expected = ("gh", "");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_from_query_with_whitespace() {
        let actual = command_from_query("gh 0x20F/paris");
        let expected = ("gh", "0x20F/paris");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_replace_keys_raw_data() {
        let text = "{raw}";
        let data = "replaced";
        let replaced = replace_keys(text, data);

        assert_eq!(replaced, "replaced");
    }

    #[test]
    fn test_replace_keys_encoded_data() {
        let text = "{encoded}";
        let data = "hello world";
        let replaced = replace_keys(text, data);

        assert_eq!(replaced, "hello%20world");
    }

    #[test]
    fn test_search_engine_query() {
        let text = "hello world";
        let query = search_engine_query(text);

        assert_eq!(query, "https://google.com/search?q=hello%20world");
    }

    #[test]
    fn remove_prefix_single_character() {
        let text = "@lmao";
        let actual = remove_prefix(text, "@");

        assert_eq!(actual, "lmao");
    }

    #[test]
    fn remove_prefix_multiple_characters() {
        let text = "----s lmao";
        let actual = remove_prefix(text, "----s");

        assert_eq!(actual, "lmao");
    }

    #[test]
    fn remove_prefix_from_start_with_space() {
        let text = "-s with space";
        let actual = remove_prefix(text, "-s");

        assert_eq!(actual, "with space");
    }

    #[test]
    fn remove_prefix_from_start_without_space() {
        let text = "-swithout space";
        let actual = remove_prefix(text, "-s");

        assert_eq!(actual, "without space");
    }
}