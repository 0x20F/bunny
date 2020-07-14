use std::fs;
use std::borrow::Borrow;
use std::collections::HashMap;
use crate::books::config::Book;
use crate::books::command::Command;


pub struct Library<'a> {
    books: HashMap<String, Book>,
    command: &'a Command<'a>,
}


impl<'a> Library<'a> {
    pub fn with_command(cmd: &'a Command) -> Self {
        let config_path = format!("{}/{}", dirs::home_dir().unwrap().display(), "bookmarks.toml");

        let contents = fs::read_to_string(config_path)
            .expect("Could not read the bookmarks file!");

        Self {
            books: toml::from_str(&contents).unwrap(),
            command: cmd,
        }
    }



    pub fn get_url(&self) -> Option<String> {
        let books = self.books.borrow().iter();

        for (_, book) in books {
            if self.command.alias != book.alias {
                continue;
            }

            return Some(self.get_page(book));
        }

        None
    }



    pub fn get_page(&self, book: &Book) -> String {
        let params = self.command.params;

        // If no params passed, it's default
        if params.is_empty() {
            return book.get_default();
        }

        for prefix in book.get_prefixes() {
            let page = book.get_page_by_prefix(prefix).unwrap();

            // For special cases
            match prefix {
                "NONE" => return page.encode_url(params),
                "CAPS" => (), // Idk just as reminder
                 _ => ()
            }

            if !params.starts_with(prefix) {
                continue;
            }

            let query = page.remove_prefix(params);
            let url = page.encode_url(query);

            return url;
        }

        // If no page was found, use the default one
        book.get_default()
    }
}