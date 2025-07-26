use core::arch;
use std::sync::{Arc, Mutex};

use poem::{
    get, handler, listener::TcpListener, post, web::{ Data, Json, Path}, EndpointExt, Route, Server
};
use store::{ store::Store};



use crate::{request_inputs::{CreateUserInput, CreateWebsiteInput}, request_outputs::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput}};



#[handler]
pub fn sign_up_user(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput>{
        let mut s = Store::new().unwrap();

        let id = s.create_user(data.username , data.password).unwrap();
    

        let response = CreateUserOutput { 
            id 
        };
        Json(response)
}

#[handler]
pub fn sign_in_user(Json(data): Json<CreateUserInput>) -> Json<SigninOutput>{
        let mut s = Store::new().unwrap();

         s.get_users(data.username , data.password).unwrap();
    

        let response = SigninOutput{ 
           
            jwt : String::from("dummy_jwt")
        };

        Json(response)
}
