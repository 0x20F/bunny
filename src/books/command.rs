use key_list::KeyList;




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


    pub fn encode_url_no_prefix(&mut self, url: &str, prefix: &str) -> String {
        self.remove_prefix(prefix);
        self.encode_url(url)
    }


    pub fn encode_url(&mut self, url: &str) -> String {
        let keys = KeyList::new(url, '{', '}');
        let mut clean = url.to_string();

        for key in keys {
            match key {
                "{encoded}" => clean = clean.replace(
                    key,
                    crate::encoder::encode(self.all_segments()).as_ref()
                ),
                "{raw}" => clean = clean.replace(key, self.all_segments()),
                "{0}" => clean = clean.replace(key, self.get_segment(0)),
                "{1}" => clean = clean.replace(key, self.get_segment(1)),
                _ => ()
            }
        }

        clean
    }


    fn remove_prefix(&mut self, prefix: &str) {
        let prefix_length = prefix.len();
        self.params = &self.params[prefix_length..].trim();
    }


    fn get_segment(&self, index: usize) -> &str {
        let mut segments = self.params.split_ascii_whitespace();

        match segments.nth(index) {
            Some(seg) => seg,
            None => ""
        }
    }


    fn all_segments(&self) -> &str {
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
    fn test_url_encoding_with_prefix_removal() {
        let mut command = command("tw -s extra things");
        let url = "{0}";

        assert_eq!(command.encode_url(url), "-s");
        assert_eq!(command.encode_url_no_prefix(url, "-s"), "extra");
    }



    #[test]
    fn test_url_encoding_encoded() {
        let mut command = command("tw hello world");
        let url = "{encoded}";

        assert_eq!(command.encode_url(url), "hello%20world");
    }

    #[test]
    fn test_url_encoding_raw() {
        let mut command = command("tw hello world");
        let url = "{raw}";

        assert_eq!(command.encode_url(url), "hello world");
    }

    #[test]
    fn test_url_encoding_with_indexes() {
        let mut command = command("tw hello world");
        let url = "{0}";

        assert_eq!(command.encode_url(url), "hello");
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
        assert_eq!(command.all_segments(), "one two three");
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