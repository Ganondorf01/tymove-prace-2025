use actix_web::{web, Responder, HttpResponse};
use sqlx::{MySql, Pool};
use crate::models::{Question, Vote};

pub async fn get_questions(pool: web::Data<Pool<MySql>>) -> impl Responder {
    let result = sqlx::query_as!(Question, "SELECT id, text FROM questions")
        .fetch_all(pool.get_ref())
        .await;
    
    match result {
        Ok(questions) => HttpResponse::Ok().json(questions),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching questions"),
    }
}

pub async fn add_question(pool: web::Data<Pool<MySql>>, question: web::Json<Question>) -> impl Responder {
    let result = sqlx::query!("INSERT INTO questions (text) VALUES (?)", question.text)
        .execute(pool.get_ref())
        .await;
    
    match result {
        Ok(_) => HttpResponse::Ok().body("Question added"),
        Err(_) => HttpResponse::InternalServerError().body("Error adding question"),
    }
}

pub async fn submit_vote(pool: web::Data<Pool<MySql>>, vote: web::Json<Vote>) -> impl Responder {
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

