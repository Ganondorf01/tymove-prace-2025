use sqlx::{MySql, Pool};
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

