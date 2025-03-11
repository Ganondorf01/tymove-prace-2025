mod db;
mod routes;
mod models;

use actix_web::{web, App, HttpServer};
use db::establish_connection;
use routes::{add_question, get_questions, submit_vote};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/questions", web::get().to(get_questions))
            .route("/questions", web::post().to(add_question))
            .route("/vote", web::post().to(submit_vote))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

