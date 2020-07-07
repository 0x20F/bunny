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
        let config_path = format!("{}/{}", dirs::home_dir().unwrap().display(), "bookmarks.toml");

        let contents = fs::read_to_string(config_path)
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
    fn test_search_engine_query() {
        let text = "hello world";
        let query = Library::construct_search_engine_query(text);

        assert_eq!(query, "https://google.com/search?q=hello%20world");
    }
}