use diesel::prelude::*;
use super::schema::tasks;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub task_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub task_name: &'a str,
}