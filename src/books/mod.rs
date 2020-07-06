mod google;
mod twitter;
mod github;

use std::fs;
use toml::Value;
use toml::value::Table;
use key_list::KeyList;


pub fn open_book(query: &str) -> String {
    let (command, params) = command_from_query(&query);
    resolve_book_url(command, params)
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


fn resolve_book_url(command: &str, params: &str) -> String {
    let contents = fs::read_to_string("books.toml")
        .expect("Something went wrong when reading the file");

    let config = contents.parse::<Value>().unwrap();
    let books = config.as_table().unwrap();


    for (_, book) in books.iter() {
        let alias = value_as_str(book, "alias");

        if command == alias {
            let url = resolve_correct_option(book, params);

            if let Some(url) = url {
                return url;
            }
        }
    }

    // If nothing was found, go to a search engine
    search_engine_query(format!("{} {}", command, params).as_ref())
}


fn resolve_correct_option(book: &Value, params: &str) -> Option<String> {
    let options = value_as_table(book, "options");
    let mut default: Option<&Value> = None;

    for (name, option) in options.iter() {
        if name == "default" {
            default = Some(option);
            continue;
        }

        let prefix = value_as_str(option, "prefix");
        let url = value_as_str(option, "url");

        if params.starts_with(prefix) {
            let params = params.replace(prefix, "");
            let url = replace_keys(url, params.trim());

            return Some(url);
        }
    }

    match default {
        Some(v) => Some(value_as_str(v, "url").to_owned()),
        None => None
    }
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
}