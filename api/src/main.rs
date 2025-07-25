use poem::{
    get, handler, listener::TcpListener, post, web::{ Json, Path}, Route, Server
};
use store::{ store::Store};



use crate::{request_inputs::{CreateUserInput, CreateWebsiteInput}, request_outputs::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput}};

pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_websites(Path(id): Path<String>) -> Json<GetWebsiteOutput> {
    let mut s = Store::new().unwrap(); // assumes Store::default() returns Result<_, _>
    let website = s.get_website(id).unwrap(); // assumes get_website also returns Result<_, _>

    
    Json(GetWebsiteOutput { url: website.url })
}


#[handler]
fn sign_up_user(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput>{
        let mut s = Store::new().unwrap();

        let id = s.create_user(data.username , data.password).unwrap();
    

        let response = CreateUserOutput { 
            id 
        };
        Json(response)
}

#[handler]
fn sign_in_user(Json(data): Json<CreateUserInput>) -> Json<SigninOutput>{
        let mut s = Store::new().unwrap();

         s.get_users(data.username , data.password).unwrap();
    

        let response = SigninOutput{ 
           
            jwt : String::from("dummy_jwt")
        };

        Json(response)
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    
    let mut s = Store::new().unwrap();

    let new_website  = s.create_website(data.url , String::from("6936bcf5-a9fe-468f-8521-a7692911ead0")).unwrap();

    let response = CreateWebsiteOutput {
       id: new_website.id,
    };
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/website/:id", get(get_websites))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up_user))
        .at("/user/signin", post(sign_in_user));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
