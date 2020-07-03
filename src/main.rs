#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod books;
mod encoder;

use rocket::response::Redirect;



#[get("/")]
fn index() -> &'static str {
    "Nothing fancy at the root yo, shoooo"
}


#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    Redirect::to(books::open_book(&cmd))
}


fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}