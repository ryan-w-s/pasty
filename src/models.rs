use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use crate::schema::pastes;

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = pastes)]
pub struct Paste {
    pub id: i32,
    pub content: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = pastes)]
pub struct NewPaste {
    pub content: String,
}
