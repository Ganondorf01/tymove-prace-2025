use actix_web::{web, Responder, HttpResponse, HttpRequest};
//use sqlx::PgPooluse;
//use crate::export::export_data;
use crate::auth::validate_token;

#[derive(serde::Serialize)]
    struct AdminResponse {
        message: String,
        total_votes: i64,
 }

    // funkce pro vypsani hlasu 

    pub async fn get_stats(pool: web::Data<PgPool>, req: HttpRequest) -> impl Responder {
        match validate_token(req) {
            Ok(_) => {
                let count = sqlx::query!("SELECT COUNT(*) as count FROM votes")
                    .fetch_one(pool.get_ref())
                    .await;

                match count {
                    Ok(res) => HttpResponse::Ok().json(AdminResponse {
                        message: "Total votes collected".to_string(),
                        total_votes: res.count.unwrap_or(0),
                    }),
                    Err(_) => HttpResponse::InternalServerError().json("Error fetching stats"),
                }
            }
            Err(err) => err,
        }
    }
/*
    //funkce pro vypsani hlasu za pomoci exportovani dat - vypisuje v JSON souboru 
    
    pub async fn admin_export(pool: web::Data<PgPool>, req: HttpRequest) -> impl Responder {
        match validate_token(req) {
            Ok(_) => export_data(pool).await,
            Err(err) => err,
            }
}
*/
// funkce pro smazani hlasu 

pub async fn delete_votes(pool: web::Data<PgPool>, req: HttpRequest) -> impl Resnr {   
    match validate_token(req) {
        Ok(_) => {
            let result = sqlx::query!("DELETE FROM votes").execute(pool.get_ref()).await;

            match result {
                Ok(_) => HttpResponse::Ok().json("All votes deleted successfully"),
                Err(_) => HttpResponse::InternalServerError().json("Error deleting votes"),
            }
        }
        Err(err) => err,
    }
}
