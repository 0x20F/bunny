use key_list::KeyList;




pub struct Command<'a> {
    pub alias: &'a str,
    pub params: &'a str
}

impl<'a> Command<'a> {
    pub fn new(query: &'a str) -> Self {
        let (command, params) = Self::command_from_query(query);

        Self {
            alias: command,
            params
        }
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


    pub fn remove_prefix(&mut self, prefix: &str) {
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
}