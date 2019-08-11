#![feature(proc_macro_hygiene)]

// Rocket web server
#![feature(decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

// Diesel ORM
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod archive;
mod schema;
use self::archive::{Archive};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::uuid::Uuid as RocketUUID;
use chrono::prelude::{Utc};
use uuid::Uuid;

#[post("/new", format="json", data="<target_url>")]
fn new(target_url: String) -> JsonValue {
    let new_archive = Archive::new(target_url);
    println!("would create: {:#?}", new_archive);
    json!({ "status": "ok" })
}

#[get("/<id>", format="json")]
fn get(id: RocketUUID) -> Json<Archive> {
    Json(Archive{
        id: Uuid::new_v4(),
        original_link: String::from("something"),
        archive_timestamp: Utc::now(),
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/archives", routes![new, get])
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use self::schema::archives::dsl::*;
    let connection = establish_connection();
    let results = archives.limit(5)
        .load::<Archive>(&connection)
        .expect("Error loading archives");

    println!("Displaying {} archives", results.len());
    for arch in results {
        println!("{}", arch.original_link);
        println!("----------\n");
        println!("{}", arch.archive_timestamp);
    }

    rocket().launch();
}
