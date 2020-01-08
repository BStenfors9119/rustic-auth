extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate url;
extern crate dotenv;
extern crate rustic_users;

use oauth2::{Config, Token, TokenError};
use url::Url;
use dotenv::dotenv;
use std::env;
use actix_web::client;

use std::sync::Arc;

use grpc::{Client, SingleResponse};
use grpc::ClientStub;

use rustic_users::rustic::*;
use rustic_users::rustic_grpc::*;

pub struct Auth{}

impl Auth {
    pub fn auth() -> String {
        dotenv().ok();
        let mut client_id = env::var("CLIENT_ID").expect("Client ID is missing");
        let mut client_secret = env::var("CLIENT_SECRET").expect("Client secret is missing");
        let mut auth_url = env::var("AUTH_URL").expect("Auth URL is missing");
        let mut token_url = env::var("TOKEN_URL").expect("Token URL is missing");

        let mut config = Config::new(client_id, client_secret, auth_url, token_url);

        config = config.add_scope("user");
//        config = config.add_scope("email");

        config = config.set_redirect_url("http://rustic.local/callback");

        config = config.set_state("1234");

//        config = config.set_response_type(ResponseType::Token);

        println!("Browse to: {}", config.authorize_url());

        return config.authorize_url().to_string();
//        let token_result = config.exchange_code("Some auth code");
    }

    pub fn exchange(code: String) -> Result<Token, TokenError> {
        dotenv().ok();
        let mut client_id = env::var("CLIENT_ID").expect("Client ID is missing");
        let mut client_secret = env::var("CLIENT_SECRET").expect("Client secret is missing");
        let mut auth_url = env::var("AUTH_URL").expect("Auth URL is missing");
        let mut token_url = env::var("TOKEN_URL").expect("Token URL is missing");

        let mut config = Config::new(client_id, client_secret, auth_url, token_url);

        let token_result = config.exchange_code(code);

        return token_result;
    }

    pub fn get_user_github_info(code: String){
        println!("{}", code);
//        let _port = 50051;
//        let mut users_url = env::var("USERS_URL").expect("Users URL is missing");
//
//        println!("{}", users_url);
//        //    let client_conf = Default::default();
//
//        //    let grpc_client = Client::new_plain("::1", 50051, client_conf);
//        let client = Arc::new(Client::new_plain(&users_url.to_string(), _port, Default::default()).unwrap());
//
//        let user_client = UserServiceClient::with_client(client);
//
//        let mut req = GetUserGithubInfoRequest::new();
//
//        req.set_token(code);
//
//        let resp = user_client.get_user_github_info(grpc::RequestOptions::new(), req);
//
//
//        println!("{:?}", resp.wait());




    }
}
