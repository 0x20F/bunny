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


#[derive(Deserialize, Debug)]
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