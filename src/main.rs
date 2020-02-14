#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate nats;

extern crate bincode;

use std::{io};
use std::str;
use nats::*;

use actix_cors::Cors;
use actix_web::{web, middleware, http, App, HttpServer, HttpRequest, HttpResponse, Result};

use serde_json::json;
use rustic_auth::establish_connection;

mod auth;

#[derive(Deserialize)]
struct Info {
    code: String,
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
//    let results = location::Location::all(&connection);
    let mut nats_client = Client::new("nats://rustic.local:4222").unwrap();

    nats_client.set_name("rustic");
    nats_client.publish("rustic.auth", "test".as_bytes()).unwrap();

    let s1 = nats_client.subscribe("rustic.*", Some("rustic")).unwrap();

    let event = nats_client.wait();

    println!("Event received: {}", str::from_utf8(&event.unwrap().msg).unwrap());
    let addr = SocketAddr::from(([0, 0, 0, 0],4114));

    HttpServer::new(
        ||
            App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("*")
                    .allowed_origin("http://rustic.local")
                    .allowed_origin("http://localhost:1977")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
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
