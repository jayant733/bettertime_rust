use crate::store::Store;

use diesel::{prelude::*};
use uuid::Uuid;


#[derive(Queryable, Insertable , Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User { 
    id : String,
    username : String,
    password : String
}

impl Store {
    pub fn create_user(&mut self, username : String , password : String)-> Result<String, diesel::result::Error>{
        let id = Uuid::new_v4();
        let u = User {
        username,
        password,
           id : id.to_string(),
        };
       diesel::insert_into(crate::schema::users::table)
             .values(&u ).returning(User::as_returning())
             .get_result(&mut self.conn)?;
        
        Ok(id.to_string())
    }

    //this is the sign in function 
    pub fn get_users(&mut self, input_username : String, input_password : String) -> Result<bool, diesel::result::Error> {
        use crate::schema::users::dsl::*;

        let user = users.filter(username.eq(input_username))
        .select(User::as_select()).first(&mut self.conn)?;


        if user.password!= input_password{
                return  Ok(false); //we have to do hashing and then cheeck the wrokd
        }
        Ok(true)

     
    }
}