use std::fs;
use std::borrow::Borrow;
use std::collections::HashMap;
use crate::books::config::Book;
use crate::command::Command;


pub struct Library {
    books: HashMap<String, Book>
}


impl Library {
    pub fn new() -> Self {
        let config_path = format!("{}/{}", dirs::home_dir().unwrap().display(), "bookmarks.toml");

        let contents = fs::read_to_string(config_path)
            .expect("Could not read the bookmarks file!");

        Self {
            books: toml::from_str(&contents).unwrap()
        }
    }



    pub fn get_url(&self, command: &mut Command) -> Option<String> {
        for book in self.books.borrow().values() {
            // If input alias doesn't match
            if command.alias != book.alias {
                continue;
            }

            // If no other params were passed besides the alias
            if command.params.is_empty() {
                return Some(book.get_default());
            }

            return Some(self.get_page(book, command));
        }

        None
    }



    pub fn get_page(&self, book: &Book, command: &mut Command) -> String {
        for page in book.pages.borrow().values() {
            let prefix = &page.prefix;

            // Match against special prefixes
            // if the page has any, do what those prefixes
            // require
            if let Ok(url) = page.handle_special_prefix(command) {
                return url;
            }

            if !command.params.starts_with(prefix) {
                continue;
            }

            return command.encode_url_no_prefix(&page.url, prefix);
        }

        // If no page was found, use the default one
        book.get_default()
    }
}