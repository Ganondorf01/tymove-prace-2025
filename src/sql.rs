use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn connect_db() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    PgPoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

pub async fn init_db(pool: &PgPool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS votes (
            id SERIAL PRIMARY KEY,
            teacher_rating INT CHECK (teacher_rating BETWEEN 1 AND 5),
            school_rating INT CHECK (school_rating BETWEEN 1 AND 5),
            field_of_study VARCHAR(10) CHECK (field_of_study IN ('IT', 'SC', 'EKO', 'ELSI', 'ELSL', 'TL', 'EKL')),
            birth_year INT CHECK (birth_year BETWEEN 1920 AND 2015),
            visit_duration INT CHECK (visit_duration BETWEEN 5 AND 180),
            submitted_at TIMESTAMP DEFAULT now()
        );"
    )
    .execute(pool)
    .await
    .expect("Failed to initialize database");
}
