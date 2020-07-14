use std::collections::HashMap;
use crate::command::Command;


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

impl Page {
    pub fn handle_special_prefix(&self, cmd: &mut Command) -> Result<String, ()> {
        match self.prefix.as_str() {
            "NONE" | "CAPS" => Ok(cmd.encode_url(&self.url)),
            _ => Err(())
        }
    }
}