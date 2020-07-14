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