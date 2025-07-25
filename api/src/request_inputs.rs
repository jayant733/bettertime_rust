use serde::{Deserialize, Serialize};
// this serialization and deserialization is used to convert structs to json and vice versa
// serde is a framework for this purpose

#[derive(Serialize,Deserialize)]
pub struct CreateWebsiteInput{
   pub url : String,


}


#[derive(Serialize,Deserialize)]
pub struct CreateUserInput {
    pub username : String,
    pub password : String
}