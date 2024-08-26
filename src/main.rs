// #![feature(proc_macro_hygiene, decl_macro)]

#![macro_use]
extern crate rocket;

use rocket::{get, launch, response::Redirect, routes};

pub mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You types in: {}", cmd);
    let command = utils::get_command_from_query_string(&cmd);
    let redirect_url = match command {
        "gh" => utils::github::construct_github_url(&cmd),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd),
    };
    Redirect::to(redirect_url)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search])
}
