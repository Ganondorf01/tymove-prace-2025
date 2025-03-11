use actix_web::{web, Responder, HttpResponse};
use sqlx::PgPool;
use serde::Deserialize;
use std::sync::Arc;
use governor::RateLimiter;
use chrono::Utc;

#[derive(Deserialize)]
pub struct Vote {
    pub teacher_rating: u8,
    pub school_rating: u8,
    pub field_of_study: String,
    pub birth_year: u16,
    pub visit_duration: u16,
}

pub async fn submit_vote(
    pool: web::Data<PgPool>, 
    vote: web::Json<Vote>, 
    limiter: web::Data<Arc<RateLimiter>>
) -> impl Responder {
    
    if !(1..=5).contains(&vote.teacher_rating) {
        return HttpResponse::BadRequest().json("Invalid teacher rating (must be 1-5)");
    }
    if !(1..=5).contains(&vote.school_rating) {
        return HttpResponse::BadRequest().json("Invalid school rating (must be 1-5)");
    }
    let allowed_fields = ["IT", "SC", "EKO", "ELSI", "ELSL", "TL", "EKL"];
    if !allowed_fields.contains(&vote.field_of_study.as_str()) {
        return HttpResponse::BadRequest().json("Invalid field of study");
    }
    if !(2008..=2012).contains(&vote.birth_year) {
        return HttpResponse::BadRequest().json("Invalid birth year (must be 2008-2012)");
    }
    if !(5..=180).contains(&vote.visit_duration) {
        return HttpResponse::BadRequest().json("Invalid visit duration (must be 5-180 minutes)");
    }

    if limiter.check_key(&"global").is_err() {
        return HttpResponse::TooManyRequests().json("Rate limit exceeded");
    }

    let result = sqlx::query!(
        "INSERT INTO votes (teacher_rating, school_rating, field_of_study, birth_year, visit_duration, submitted_at) 
         VALUES ($1, $2, $3, $4, $5, $6)",
        vote.teacher_rating,
        vote.school_rating,
        vote.field_of_study,
        vote.birth_year,
        vote.visit_duration,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Vote submitted successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Error submitting vote"),
    }
}

