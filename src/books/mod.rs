mod config;
mod library;

use library::Library;




pub fn open_book(query: &str) -> String {
    let (command, params) = command_from_query(&query);

    let lib = Library::new(command, params);
    lib.get_url()
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