use std::fs;
use std::borrow::Borrow;
use std::collections::HashMap;
use crate::books::config::Book;
use crate::books::command::Command;


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
        let books = self.books.borrow().iter();

        for (_, book) in books {
            if command.alias != book.alias {
                continue;
            }

            if command.params.is_empty() {
                return Some(book.get_default());
            }

            return Some(self.get_page(book, command));
        }

        None
    }



    pub fn get_page(&self, book: &Book, command: &mut Command) -> String {
        for (_, page) in book.pages.borrow() {
            let prefix = &page.prefix;

            if let Ok(url) = page.handle_special_prefix(command) {
                return url;
            }

            if !command.params.starts_with(prefix) {
                continue;
            }

            command.remove_prefix(prefix);
            return command.encode_url(&page.url);
        }

        // If no page was found, use the default one
        book.get_default()
    }
}