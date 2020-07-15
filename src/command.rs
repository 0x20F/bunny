use key_list::KeyList;
use crate::books::config::{Book, Page};




pub struct Command<'a> {
    pub alias: &'a str,
    pub params: &'a str
}

impl<'a> Command<'a> {
    pub fn new(query: &'a str) -> Self {
        let (command, params) = Self::from_query(query);

        Self {
            alias: command,
            params
        }
    }


    pub fn encode_url(&mut self, page: &Page, book: &Book) -> String {
        let url = &page.url;

        let keys = KeyList::new(url, '{', '}');
        let mut clean = url.to_string();

        for key in keys {
            clean = match key {
                "{default}" => clean.replace(key, &book.get_default()),
                "{encoded}" => clean.replace(
                    key,
                    crate::encoder::encode(self.get_all_segments()).as_ref()
                ),
                "{raw}" => clean.replace(key, self.get_all_segments()),
                "{0}" => clean.replace(key, self.get_segment(0)),
                "{1}" => clean.replace(key, self.get_segment(1)),
                "{2}" => clean.replace(key, self.get_segment(2)),
                "{3}" => clean.replace(key, self.get_segment(3)),
                _ => clean
            }
        }

        clean
    }


    pub fn remove_prefix(&mut self, prefix: &str) -> &mut Self {
        let prefix_length = prefix.len();
        self.params = &self.params[prefix_length..].trim();

        self
    }


    fn get_segment(&self, index: usize) -> &str {
        let mut segments = self.params.split_ascii_whitespace();

        match segments.nth(index) {
            Some(seg) => seg,
            None => ""
        }
    }


    fn get_all_segments(&self) -> &str {
        self.params
    }


    fn from_query(query: &str) -> (&str, &str) {
        let clean = query.trim();

        if clean.contains(' ') {
            let space = clean.find(' ').unwrap_or(0);

            let command = &clean[..space];
            let params = &clean[space..];
            return (command, params.trim());
        }

        (clean, "")
    }
}











#[cfg(test)]
mod tests {
    use super::*;

    fn command(query: &str) -> Command {
        Command::new(query)
    }

    #[test]
    fn test_alias_gets_parsed_correctly() {
        let command = command("tw la la la ");
        assert_eq!(command.alias, "tw");
    }

    #[test]
    fn test_params_get_parsed_correctly() {
        let command = command("tw extra-things");
        assert_eq!(command.params, "extra-things");
    }



    #[test]
    fn test_remove_prefix_with_space() {
        let mut command = command("tw prefix after-prefix");
        command.remove_prefix("prefix");

        assert_eq!(command.params, "after-prefix");
    }

    #[test]
    fn test_remove_prefix_no_space() {
        let mut command = command("tw prefixafter-prefix");
        command.remove_prefix("prefix");

        assert_eq!(command.params, "after-prefix");
    }



    #[test]
    fn test_get_segment() {
        let command = command("tw one two three");
        assert_eq!(command.get_segment(0), "one");
    }

    #[test]
    fn test_get_segment_out_of_bounds() {
        let command = command("tw one two three");
        assert_eq!(command.get_segment(200), "");
    }

    #[test]
    fn test_get_all_segments() {
        let command = command("tw one two three");
        assert_eq!(command.get_all_segments(), "one two three");
    }



    #[test]
    fn test_get_command_from_query_no_whitespace() {
        let actual = Command::from_query("gh");
        let expected = ("gh", "");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_command_from_query_with_whitespace() {
        let actual = Command::from_query("gh 0x20F/paris");
        let expected = ("gh", "0x20F/paris");

        assert_eq!(actual, expected);
    }
}