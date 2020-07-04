mod google;
mod twitter;
mod github;





pub fn open_book(query: &str) -> String {
    let command = command_from_query(&query);

    let redirect_url = match command.as_ref() {
        twitter::ALIAS => twitter::to_twitter_url(&query),
        _ => google::to_google_search_url(&query)
    };

    redirect_url
}


fn command_from_query(query: &str) -> &str {
    if query.contains(' ') {
        let first_space = query.find(' ').unwrap_or(0);
        return &query[..first_space];
    }

    query
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_no_whitespace() {
        let actual = command_from_query("gh");
        let expected = "gh";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_from_query_with_whitespace() {
        let actual = command_from_query("gh 0x20F/paris");
        let expected = "gh";

        assert_eq!(actual, expected);
    }
}