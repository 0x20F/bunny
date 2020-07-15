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

            if !command.params.starts_with(prefix) {
                if prefix == "NONE" {
                    return command.encode_url(page, book);
                }

                continue;
            }

            return command
                .remove_prefix(prefix)
                .encode_url(page, book);
        }

        // If no page was found, use the default one
        book.get_default()
    }
}