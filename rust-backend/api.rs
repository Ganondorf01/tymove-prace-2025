use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};



#[derive(Serialize, Deserialize)]

struct Vote {
    question_id: i32,
    rating: i32,
    comment: Option<String>,

}



async fn submit_vote(pool: web::Data<Pool<MySql>>, vote: web::Json<Vote>) -> impl Responder {

    let result = sqlx::query!(

        "INSERT INTO votes (question_id, rating, comment) VALUES (?, ?, ?)",
        vote.question_id,
        vote.rating,
        vote.comment

    )

    .execute(pool.get_ref())

    .await;



    match result {

        Ok(_) => HttpResponse::Ok().body("Vote submitted"),
        Err(_) => HttpResponse::InternalServerError().body("Error saving vote"),

    }

}



#[actix_web::main]

async fn main() -> std::io::Result<()> {

    let pool = Pool::<MySql>::connect("mysql://user:password@localhost/voting_db").await.unwrap();

    

    HttpServer::new(move || {

        App::new()

            .app_data(web::Data::new(pool.clone()))
            .route("/vote", web::post().to(submit_vote))

    })

    .bind("127.0.0.1:8080")?

    .run()

    .await

}

sql_api.rs
use sqlx::{MySql, Pool, Executor};

use std::env;



pub async fn establish_connection() -> Pool<MySql> {

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Pool::<MySql>::connect(&database_url).await.expect("Failed to connect to database")

}



pub async fn run_migrations(pool: &Pool<MySql>) {

    sqlx::query(

        "CREATE TABLE IF NOT EXISTS questions (
            id INT AUTO_INCREMENT PRIMARY KEY,
            text VARCHAR(255) NOT NULL
        );"

    )

    .execute(pool)
    .await
    .expect("Failed to run migrations");

    

    sqlx::query(

        "CREATE TABLE IF NOT EXISTS votes (
            id INT AUTO_INCREMENT PRIMARY KEY,
            question_id INT NOT NULL,
            rating INT NOT NULL,
            comment TEXT,
            FOREIGN KEY (question_id) REFERENCES questions(id)
        );"

    )

    .execute(pool)

    .await

    .expect("Failed to run migrations");

}
admin_api.rs
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};



#[derive(Serialize, Deserialize)]

struct Question {

    id: i32,
    text: String,

}



async fn get_questions(pool: web::Data<Pool<MySql>>) -> impl Responder {

    let result = sqlx::query_as!(Question, "SELECT id, text FROM questions")

        .fetch_all(pool.get_ref())
        .await;

    

    match result {

        Ok(questions) => HttpResponse::Ok().json(questions),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching questions"),

    }
}



async fn add_question(pool: web::Data<Pool<MySql>>, question: web::Json<Question>) -> impl Responder {

    let result = sqlx::query!("INSERT INTO questions (text) VALUES (?)", question.text)

        .execute(pool.get_ref())
        .await;

    

    match result {

        Ok(_) => HttpResponse::Ok().body("Question added"),
        Err(_) => HttpResponse::InternalServerError().body("Error adding question"),

    }
}



#[actix_web::main]

async fn main() -> std::io::Result<()> {

    let pool = Pool::<MySql>::connect("mysql://user:password@localhost/voting_db").await.unwrap();

    

    HttpServer::new(move || {

        App::new()

            .app_data(web::Data::new(pool.clone()))

            .route("/questions", web::get().to(get_questions))

            .route("/questions", web::post().to(add_question))

    })

    .bind("127.0.0.1:8081")?

    .run()

    .await

}

