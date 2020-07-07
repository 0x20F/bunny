use std::fs;
use std::borrow::Borrow;
use std::collections::HashMap;
use crate::books::config::Book;





pub struct Library {
    books: HashMap<String, Book>,
    command: String,
    params: String
}


impl Library {
    pub fn new(command: &str, params: &str) -> Self {
        let contents = fs::read_to_string("books.toml")
            .expect("Could not read the bookmarks file!");

        Library {
            books: toml::from_str(&contents).unwrap(),
            command: command.to_owned(),
            params: params.to_owned()
        }
    }



    pub fn get_url(&self) -> String {
        let books = self.books.borrow().iter();

        for (_, book) in books {
            if self.command != book.alias {
                continue;
            }

            return self.get_page(book);
        }

        // If no alias was found, just search for the given query
        let query = format!("{} {}", self.command, self.params);
        Library::construct_search_engine_query(&query)
    }



    pub fn get_page(&self, book: &Book) -> String {
        let params = &self.params;

        // If no params passed, it's default
        if params.is_empty() {
            return book.default.to_owned();
        }

        for prefix in book.get_prefixes() {
            let page = book.get_page_by_prefix(prefix).unwrap();

            // For special cases
            match prefix {
                "NONE" => return page.construct_url(params),
                "CAPS" => (), // Idk just as reminder
                 _ => ()
            }

            if !params.starts_with(prefix) {
                continue;
            }

            let query = page.remove_prefix(params);
            let url = page.construct_url(query);

            return url;
        }

        // If no page was found, use the default one
        book.default.to_owned()
    }



    fn construct_search_engine_query(data: &str) -> String {
        let encoded = crate::encoder::encode(&data);

        format!("https://google.com/search?q={}", encoded)
    }
}










#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_keys_raw_data() {
        let text = "{raw}";
        let data = "replaced";
        let replaced = Library::replace_keys(text, data);

        assert_eq!(replaced, "replaced");
    }

    #[test]
    fn test_replace_keys_encoded_data() {
        let text = "{encoded}";
        let data = "hello world";
        let replaced = Library::replace_keys(text, data);

        assert_eq!(replaced, "hello%20world");
    }

    #[test]
    fn test_search_engine_query() {
        let text = "hello world";
        let query = Library::construct_search_engine_query(text);

        assert_eq!(query, "https://google.com/search?q=hello%20world");
    }

    #[test]
    fn remove_prefix_single_character() {
        let text = "@lmao";
        let actual = Library::remove_prefix(text, "@");

        assert_eq!(actual, "lmao");
    }

    #[test]
    fn remove_prefix_multiple_characters() {
        let text = "----s lmao";
        let actual = Library::remove_prefix(text, "----s");

        assert_eq!(actual, "lmao");
    }

    #[test]
    fn remove_prefix_from_start_with_space() {
        let text = "-s with space";
        let actual = Library::remove_prefix(text, "-s");

        assert_eq!(actual, "with space");
    }

    #[test]
    fn remove_prefix_from_start_without_space() {
        let text = "-swithout space";
        let actual = Library::remove_prefix(text, "-s");

        assert_eq!(actual, "without space");
    }
}