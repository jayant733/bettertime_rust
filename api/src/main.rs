use core::arch;
use std::sync::{Arc, Mutex};

use poem::{
    get, handler, listener::TcpListener, post, web::{ Data, Json, Path}, EndpointExt, Route, Server
};
use store::{ store::Store};



use crate::{request_inputs::{CreateUserInput, CreateWebsiteInput}, request_outputs::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput}, routes::{user::{sign_in_user, sign_up_user}, website::{create_website, get_websites}}};

pub mod request_inputs;
pub mod request_outputs;
pub mod routes;




#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

   

    let mut s = Arc::new(Mutex::new(Store::new().unwrap()));
    let app = Route::new()
        .at("/website/:id", get(get_websites))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up_user))
        .at("/user/signin", post(sign_in_user))
        .data(s);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
 