#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::fs::{self, File};
use std::io::{BufRead, BufReader};

#[get("/all")]
fn all_quotes() -> String {
    let file = fs::File::open("quotes.json").expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");
    serde_json::to_string_pretty(&json).unwrap()
}

#[get("/book/<id>")]
fn book_quotes(id: usize) -> String {
    let file = fs::File::open("quotes.json").expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");
    let book_titles = get_all_book_titles();
    let title = &book_titles[id];
    let quotes = get_quotes_from_book(&json, title);
    format!("{} \n {}", title, quotes)
}

fn get_all_book_titles() -> Vec<String> {
    let file = File::open("books.txt").unwrap();
    let reader = BufReader::new(file);
    let book_titles: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    book_titles
}

fn get_quotes_from_book(json: &serde_json::Value, title: &String) -> String {
    format!("{}", &json[&title])
}

fn main() {
    rocket::ignite()
        .mount("/", routes![all_quotes, book_quotes])
        .launch();
}
