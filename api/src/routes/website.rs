use core::arch;
use std::sync::{Arc, Mutex};

use poem::{
    get, handler, listener::TcpListener, post, web::{ Data, Json, Path}, EndpointExt, Route, Server
};
use store::{ store::Store};



use crate::{request_inputs::{CreateUserInput, CreateWebsiteInput}, request_outputs::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput}};
#[handler]
pub  fn get_websites(Path(id): Path<String>, Data(s): Data<&Arc<Mutex<Store>>>) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap(); // Lock the Store for thread safety
    let website = locked_s.get_website(id).unwrap(); // assumes get_website also returns Result<_, _>

    
    Json(GetWebsiteOutput { url: website.url })
}


#[handler]
pub fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    
    let mut s = Store::new().unwrap();

    let new_website  = s.create_website(data.url , String::from("6936bcf5-a9fe-468f-8521-a7692911ead0")).unwrap();

    let response = CreateWebsiteOutput {
       id: new_website.id,
    };
    Json(response)
}


