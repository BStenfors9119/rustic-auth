#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use std::{io};

use actix_cors::Cors;
use actix_web::{web, middleware, http, App, HttpServer, HttpRequest, HttpResponse, Result};

use serde_json::json;
use rustic_auth::establish_connection;

pub mod schema;
mod location;
mod auth;

#[derive(Deserialize)]
struct Info {
    code: String,
}

fn all_locations() -> HttpResponse {

    let connection = establish_connection();
    let results = location::Location::all(&connection);

    HttpResponse::Ok().json(&results)
}

fn log_in() -> HttpResponse {
    let auth_url = auth::Auth::auth();

    println!("Auth URL to be used {}", auth_url);

    let resp = json!({
        "authUrl": auth_url
    });

    HttpResponse::Ok().json(&resp)
}

fn call_back(params: web::Json<Info>) -> HttpResponse {
    println!("Auth code received {}", params.code);

    let token_result = auth::Auth::exchange(params.code.to_string());

    match token_result {
        Ok(t) => {
            auth::Auth::get_user_github_info(t.access_token.to_string());
            HttpResponse::Ok().json(t)
        },
        Err(e) => HttpResponse::Ok().json(e),
    }
}

fn test_grpc() {
    let token = String::new();
    auth::Auth::get_user_github_info(token);
}

fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    use std::net::{SocketAddr};
    let connection = establish_connection();
    let results = location::Location::all(&connection);

    println!("Number of locations: {}", results.len());
    let addr = SocketAddr::from(([0, 0, 0, 0],4114));

    HttpServer::new(
        ||
            App::new()
            .wrap(
                Cors::new().supports_credentials()
            )
            .wrap(middleware::Logger::default())

            .service(
        web::resource("/login").to(log_in)
            )
            .service(
        web::resource("/callback").to(call_back)
            )
            .service(
        web::resource("/health").to(|| HttpResponse::Ok())
            )
    )
    .bind(addr)?
    .run()
}
