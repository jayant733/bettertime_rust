use crate::{store::Store};

use diesel::{prelude::*};
use uuid::Uuid;


#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::websites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website { 
    pub id : String,
    pub url : String,
    pub user_id : String,
    pub date_time : Option<chrono::NaiveDateTime>
}

impl Store {
   pub fn create_website(&mut self,input_url: String,input_user_id: String,
) -> Result<Website, diesel::result::Error> {
    let id = Uuid::new_v4();

    let website = Website {
        id: id.to_string(),
        url: input_url,
        user_id: input_user_id,
        date_time: Some(chrono::Utc::now().naive_utc()),
    };

    let result: Result<Website, diesel::result::Error> = diesel::insert_into(crate::schema::websites::table)
        .values(&website)
        .returning(Website::as_returning())
        .get_result(&mut self.conn);

    println!("Website created");

    result
    
    
    // ✅ This is the actual return
}


    pub fn get_website(&mut self , input_id : String) -> Result<Website, diesel::result::Error> {
            use crate::schema::websites::dsl::*;

            let website: Result<Website, diesel::result::Error> = websites.filter(id.eq(input_id))
            .select(Website::as_select())
            .first(&mut self.conn);



    website

        
    }
}