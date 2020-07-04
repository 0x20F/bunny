#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod books;
mod encoder;

use rocket::response::Redirect;


#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    Redirect::to(books::open_book(&cmd))
}


fn main() {
    rocket::ignite().mount("/", routes![search]).launch();
}