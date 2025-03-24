use actix_web::{web, Responder, HttpResponse};
use sqlx::PgPool;
use csv::Writer;
use std::fs::File;

pub async fn export_data(pool: web::Data<PgPool>) -> impl Responder {
    let rows = sqlx::query!(
        "SELECT teacher_rating, school_rating, field_of_study, birth_year, visit_duration FROM votes"
    )
    .fetch_all(pool.get_ref())
    .await;

    match rows {
        Ok(data) => {
            let file = File::create("export.csv").unwrap();
            let mut wtr = Writer::from_writer(file);

            wtr.write_record(&["Teacher Rating", "School Rating", "Field of Study", "Birth Year", "Visit Duration"])
                .unwrap();

            for row in data {
                wtr.write_record(&[
                    row.teacher_rating.expect("REASON").to_string(),
                    row.school_rating.expect("REASON").to_string(),
                    row.field_of_study.expect("REASON"),
                    row.birth_year.expect("REASON").to_string(),
                    row.visit_duration.expect("REASON").to_string(),
                ])
                .unwrap();
            }
            wtr.flush().unwrap();
            HttpResponse::Ok().json("Data exported successfully")
        }
        Err(_) => HttpResponse::InternalServerError().json("Error exporting data"),
    }
}



