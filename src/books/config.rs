use std::collections::HashMap;
use key_list::KeyList;



#[derive(Deserialize, Debug)]
pub struct Book {
    pub alias: String,
    pub default: String,
    pub pages: HashMap<String, Page>
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
}


#[derive(Deserialize, Debug, Default)]
pub struct Page {
    pub prefix: String,
    pub url: String
}

impl Page {
    pub fn construct_url(&self, data: &str) -> String {
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


    #[test]
    fn test_construct_url_raw_data() {
        let data = "replaced";
        let page = simple_page(".", "{raw}");

        assert_eq!(page.construct_url(data), "replaced");
    }

    #[test]
    fn test_construct_url_encoded_data() {
        let data = "hello world";
        let page = simple_page(".", "{encoded}");

        assert_eq!(page.construct_url(data), "hello%20world");
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
}