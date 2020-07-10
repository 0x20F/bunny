use std::collections::HashMap;
use key_list::KeyList;



#[derive(Deserialize, Debug, Default)]
pub struct Book {
    pub alias: String,
    pub pages: HashMap<String, Page>,

    default: String
}

impl Book {
    pub fn get_prefixes(&self) -> Vec<&str> {
        let mut prefixes: Vec<&str> = Vec::with_capacity(4);

        for (_, page) in self.pages.iter() {
            prefixes.push(&page.prefix);
        }

        prefixes
    }

    pub fn get_page_by_prefix(&self, prefix: &str) -> Option<&Page> {
        for (_, page) in self.pages.iter() {
            if page.prefix == prefix {
                return Some(page);
            }
        }

        None
    }

    pub fn get_default(&self) -> String {
        self.default.clone()
    }
}


#[derive(Deserialize, Debug, Default)]
pub struct Page {
    pub prefix: String,
    pub url: String
}

impl Page {
    pub fn encode_url(&self, data: &str) -> String {
        let keys = KeyList::new(&self.url, '{', '}');
        let mut clean = self.url.clone();

        for key in keys {
            match key {
                "{encoded}" => clean = clean.replace(
                    key,
                    crate::encoder::encode(data).as_ref()
                ),
                "{raw}" => clean = clean.replace(key, data),
                _ => ()
            }
        }

        clean
    }

    pub fn remove_prefix<'a>(&self, data: &'a str) -> &'a str {
        let prefix_length = self.prefix.len();
        &data[prefix_length..].trim()
    }
}










#[cfg(test)]
mod tests {
    use super::*;

    fn simple_page(prefix: &str, url: &str) -> Page {
        let mut page = Page::default();
        page.prefix = String::from(prefix);
        page.url = String::from(url);

        page
    }

    fn simple_book() -> Book {
        let mut pages: HashMap<String, Page> = HashMap::new();
        pages.insert(
            "test1".to_string(),
            simple_page("-s", "first page")
        );
        pages.insert(
            "test2".to_string(),
            simple_page("-a", "second page")
        );

        let mut book = Book::default();
        book.pages = pages;

        book
    }


    #[test]
    fn test_construct_url_raw_data() {
        let data = "replaced";
        let page = simple_page(".", "{raw}");

        assert_eq!(page.encode_url(data), "replaced");
    }

    #[test]
    fn test_construct_url_encoded_data() {
        let data = "hello world";
        let page = simple_page(".", "{encoded}");

        assert_eq!(page.encode_url(data), "hello%20world");
    }

    #[test]
    fn remove_prefix_single_character() {
        let text = "@lmao";
        let page = simple_page("@", "");
        let actual = page.remove_prefix(text);

        assert_eq!(actual, "lmao");
    }

    #[test]
    fn remove_prefix_multiple_characters() {
        let text = "----s lmao";
        let page = simple_page("----s", "");
        let actual = page.remove_prefix(text);

        assert_eq!(actual, "lmao");
    }

    #[test]
    fn remove_prefix_from_start_with_space() {
        let text = "-s with space";
        let page = simple_page("-s", "");
        let actual = page.remove_prefix(text);

        assert_eq!(actual, "with space");
    }

    #[test]
    fn remove_prefix_from_start_without_space() {
        let text = "-swithout space";
        let page = simple_page("-s", "");
        let actual = page.remove_prefix(text);

        assert_eq!(actual, "without space");
    }

    #[test]
    fn book_has_access_to_all_prefixes() {
        let book = simple_book();

        let actual = book.get_prefixes();
        let expected = vec![ "-s", "-a" ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn book_can_retrieve_specific_page_by_prefix() {
        let book = simple_book();

        let page = book.get_page_by_prefix("-s").unwrap();
        assert_eq!(page.url, "first page".to_string());
    }
}