use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

async fn login() -> Result<impl Responder, actix_web::Error> {
    let claims = Claims {
        sub: "user123".to_string(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())) {
        Ok(t) => t,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Token generation failed")),
    };

    Ok(HttpResponse::Ok()
        .cookie(
            Cookie::build("auth-token", token)
                .secure(true)
                .http_only(true)
                .finish(),
        )
        .body("Login successful. Token set in cookie."))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

