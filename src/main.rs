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

    use self::models::{NewPost, Post};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    let new_post = NewPost {
        title: "A new post title",
        slug: "a-new-post",
        body: "This is a new post",
    };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<Post>(&connection)
        .expect("Error saving new post");

    let results = posts
        .load::<Post>(&connection)
        .expect("Error loading posts");

    for post in results {
        println!("{}", post.title);
    }
}
