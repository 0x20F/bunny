#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

mod books;
mod command;
mod encoder;

use command::Command;
use rocket::response::Redirect;


#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let mut command = Command::new(&cmd);
    let url = books::open_book(&mut command);

    Redirect::to(url)
}


fn main() {
    rocket::ignite().mount("/", routes![search]).launch();
}