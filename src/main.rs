#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = PgConnection::establish(&db_url).expect("Error connecting to database");

    use self::models::Post;

    use self::schema::posts::dsl::*;

    let results = posts
        .load::<Post>(&connection)
        .expect("Error loading posts")
        .iter()
        .map(|post| {
            println!("{}", post.title);
        });
}
