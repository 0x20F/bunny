use std::collections::HashMap;


#[derive(Deserialize, Debug, Default)]
pub struct Book {
    pub alias: String,
    pub pages: HashMap<String, Page>,

    default: String
}

impl Book {
    pub fn get_default(&self) -> String {
        self.default.clone()
    }
}



#[derive(Deserialize, Debug, Default)]
pub struct Page {
    pub prefix: String,
    pub url: String
}