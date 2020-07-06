use std::collections::HashMap;



#[derive(Deserialize, Debug)]
pub struct Book {
    pub alias: String,
    pub default: String,
    pub pages: HashMap<String, Page>
}

#[derive(Deserialize, Debug)]
pub struct Page {
    pub prefix: String,
    pub url: String
}