mod google;
mod twitter;
mod github;

use std::fs;
use toml::Value;



pub fn open_book(query: &str) -> String {
    let (command, params) = command_from_query(&query);

    resolve_book_url(command, params);

    match command {
        "gh" => github::construct_github_url(params),
        "tw" => twitter::construct_twitter_url(params),
        _ => google::construct_google_search_url(params)
    }
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


fn resolve_book_url(command: &str, params: &str) {
    let contents = fs::read_to_string("books.toml")
        .expect("Something went wrong when reading the file");

    let config = contents.parse::<Value>().unwrap();
    let table = config.as_table().unwrap();

    for (_, value) in table.iter() {
        let alias = value.get("alias")
            .unwrap()
            .as_str()
            .unwrap();

        if command != alias {
            continue;
        }

        println!("Command '{}' was given and found", command);

        let books = value.get("books")
            .unwrap()
            .as_table()
            .unwrap();

        for (book, value) in books.iter() {
            let format = value.get("format").unwrap();
            let url = value.get("url").unwrap();

            println!(
                "Gotta get the format '{}' to somehow check validity of passed in params '{}'",
                format,
                params
            );
        }
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