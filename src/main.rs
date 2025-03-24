use actix_web::{web, App, HttpServer, middleware::Logger};
use sqlx::PgPool;
use std::{env, sync::Arc};
use tokio::sync::Mutex;
use governor::{Quota, RateLimiter};
use std::num::NonZeroU32;
use log::{debug, error, log_enabled, info, Level};

mod sql;
mod vote;
mod admin;
mod auth;
// mod export;

use sql::{connect_db, init_db};
use vote::submit_vote;
//use admin::{get_stats, admin_export, delete_votes};
use admin::{get_stats, delete_votes};
use auth::{login_admin, register_admin};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let pool = connect_db().await;
    init_db(&pool).await;

    let limiter = Arc::new(RateLimiter::direct(Quota::per_hour(NonZeroU32::new(900).unwrap())));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(limiter.clone()))

            .route("/vote", web::post().to(submit_vote))
            
            .route("/admin/stats", web::get().to(get_stats))
            //.route("/admin/export", web::get().to(admin_export))
            .route("/admin/delete", web::delete().to(delete_votes))
            
            //.route("/admin/login", web::post().to(login_admin))
            //.route("/admin/register", web::post().to(register_admin))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

