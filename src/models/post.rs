use crate::schema::posts;
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, AsChangeset)]
pub struct Post {
    pub id: i32,
    pub body: Option<String>,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub body: Option<String>,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl NewPost {
    pub fn with_time(mut self) -> Self {
        let now = Local::now().naive_local();

        self.created_at= Some(now);
        self.updated_at= Some(now);
        self
    }
}

#[derive(Deserialize)]
pub struct DeletePost {
    pub id: i32,
}

/*id SERIAL PRIMARY KEY,
   body VARCHAR,
   user_id INT,
   created_at timestamp,
   updated_at timestamp
*/
