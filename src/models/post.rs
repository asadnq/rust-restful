use std::time::SystemTime;
use serde::Serialize;
use diesel::sql_types::*;
use chrono::NaiveDateTime;
#[derive(Queryable, Serialize, Debug)]
pub struct Post {
    pub id: i32,
    pub body: Option<String>,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/*id SERIAL PRIMARY KEY,
   body VARCHAR,
   user_id INT,
   created_at timestamp,
   updated_at timestamp
*/
