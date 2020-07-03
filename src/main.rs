#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod utils;

use rocket::response::Redirect;



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You typed in: {}", cmd);

    let command = utils::command_from_query(&cmd);

    let redirect_url = match command.as_ref() {
        "tw" => utils::twitter::to_twitter_url(&cmd),
        _ => utils::google::to_google_search_url(&cmd)
    };

    Redirect::to(redirect_url)
}


fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}