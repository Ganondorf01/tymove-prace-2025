use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use bcrypt::{verify, DEFAULT_COST, hash};
use chrono::{Utc, Duration};

// vytvoreni extremne bezpecneho klice :333

const SECRET_KEY: &[u8] = b"supersecretkey";

// vytvoreni login requestu

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

const ADMIN_USERNAME: &str = "admin";
const ADMIN_PASSWORD_HASH: &str = "$2b$12$eImiTXuWVxfM37uY4JANjQ6.D9yOGsOq3Fz9HCB1dcFP.sAp3DAmC";

pub async fn login(credentials: web::Json<LoginRequest>) -> impl Responder {
    if credentials.username != ADMIN_USERNAME {
        return HttpResponse::Unauthorized().json("Invalid username or password");
    }

    if !verify(&credentials.password, ADMIN_PASSWORD_HASH).unwrap_or(false) {
        return HttpResponse::Unauthorized().json("Invalid username or password");
    }

    let expiration = Utc::now() + Duration::hours(2);
    let claims = Claims {
        sub: credentials.username.clone(),
        exp: expiration.timestamp() as usize,
    };

    match encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY)) {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => HttpResponse::InternalServerError().json("Error generating token"),
    }
}

pub fn validate_token(req: HttpRequest) -> Result<String, HttpResponse> {
    let auth_header = req.headers().get("Authorization");
    if let Some(header_value) = auth_header {
        if let Ok(auth_str) = header_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                return decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(SECRET_KEY),
                    &Validation::new(Algorithm::HS256),
                )
                .map(|data| data.claims.sub)
                .map_err(|_| HttpResponse::Unauthorized().json("Invalid token"));
            }
        }
    }
    Err(HttpResponse::Unauthorized().json("Missing or invalid Authorization header"))
}

