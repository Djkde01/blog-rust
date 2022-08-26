#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use dotenv::dotenv;
use std::env;
use tera::Tera;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::{NewPost, NewPostHandler, Post};
use self::schema::posts;
use self::schema::posts::dsl::*;

#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let conn = pool.get().expect("Error connecting db");

    match web::block(move || posts.load::<Post>(&conn)).await {
        Ok(data) => {
            let posts_data = data.unwrap();
            let mut context = tera::Context::new();

            context.insert("posts", &posts_data);
            HttpResponse::Ok().body(template_manager.render("index.html", &context).unwrap())
        }
        Err(err) => HttpResponse::Ok().body("Error receiving data"),
    }
}

#[get("/blog/{blog_slug}")]
async fn get_post(
    pool: web::Data<DbPool>,
    template_manager: web::Data<tera::Tera>,
    blog_slug: web::Path<String>,
) -> impl Responder {
    let conn = pool.get().expect("Error connecting db");

    let url_slug = blog_slug.into_inner();

    match web::block(move || posts.filter(slug.eq(url_slug)).load::<Post>(&conn)).await {
        Ok(data) => {
            let data = data.unwrap();

            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }
            let data = &data[0];
            let mut context = tera::Context::new();

            context.insert("post", data);
            HttpResponse::Ok().body(template_manager.render("post.html", &context).unwrap())
        }
        Err(err) => HttpResponse::Ok().body("Error receiving data"),
    }
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("Error connecting db");

    println!("{:?}", item);

    match web::block(move || Post::create_post(&conn, &item)).await {
        Ok(data) => HttpResponse::Ok().body(format!("{:?}", data)),
        Err(err) => HttpResponse::Ok().body("Error receiving data"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db url not found");
    let port = env::var("PORT").expect("port not found");
    let port: u16 = port.parse().unwrap();

    let connection = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder()
        .build(connection)
        .expect("Pool can't be created");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .service(index)
            .service(new_post)
            .service(get_post)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera))
    })
    .bind(("0.0.0.0", port))
    .unwrap()
    .run()
    .await
}
