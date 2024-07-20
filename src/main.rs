extern crate diesel;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use chrono::Utc;

mod models;
mod schema;

use models::{Paste, NewPaste};
use schema::pastes;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[post("/pastes")]
async fn create_paste(pool: web::Data<DbPool>, paste: web::Json<NewPaste>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let new_paste = NewPaste {
        content: paste.content.clone(),
        created_at: Some(Utc::now().naive_utc()),
    };

    diesel::insert_into(pastes::table)
        .values(&new_paste)
        .execute(&mut conn)
        .expect("Error saving new paste");

    HttpResponse::Created().finish()
}

#[get("/pastes")]
async fn get_pastes(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let results = pastes::table
        .load::<Paste>(&mut conn)
        .expect("Error loading pastes");

    HttpResponse::Ok().json(results)
}

#[get("/pastes/{id}")]
async fn get_paste(pool: web::Data<DbPool>, id: web::Path<i32>) -> impl Responder {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let result = pastes::table
        .find(id.into_inner())
        .first::<Paste>(&conn)
        .optional()
        .expect("Error loading paste");

    match result {
        Some(paste) => HttpResponse::Ok().json(paste),
        None => HttpResponse::NotFound().finish(),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(create_paste)
            .service(get_pastes)
            .service(get_paste)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
