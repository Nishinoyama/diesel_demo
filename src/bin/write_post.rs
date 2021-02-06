extern crate diesel;
extern crate diesel_demo;

use diesel_demo::*;
use std::io::{stdin};

fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];
    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_line(&mut body).unwrap();

    let _result = create_post(&connection, title, &body);
    println!("\nSaved draft successfully!");
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";