use super::schema::food;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Insertable)]
#[table_name = "food"]
pub struct NewFood {
    name: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Food {
    id: i32,
    name: String,
}
