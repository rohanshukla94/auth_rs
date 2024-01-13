#![allow(dead_code, unused_variables)]
mod database;
mod auth;

pub use auth::models::Credentials;
use database::Status;

use rand::prelude::*;

pub fn authenticate(creds: Credentials) {

    let timeout = thread_rng().gen_range(100..500);

    println!("the time out is {timeout}");
    if let Status::Connected = database::connect_db() {
        auth::login(creds);

        println!("You are successfully logged in");
    }
}