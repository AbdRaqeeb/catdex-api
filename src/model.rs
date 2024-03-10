use crate::schema::cats;
use diesel::{prelude::Insertable, Queryable};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = cats)]
pub struct NewCat {
    pub name: String,
    pub image_path: String,
}
