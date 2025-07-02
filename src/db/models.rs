use crate::db::schema::{queue_rows, queues};
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = queues)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Queue {
    pub id: Uuid,
    pub url_name: String,
    pub display_name: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = queues)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewQueue {
    pub url_name: String,
    pub display_name: String,
}

#[derive(Queryable, Selectable, AsChangeset)]
#[diesel(table_name = queue_rows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(treat_none_as_null = true)]
pub struct QueueRow {
    pub id: Uuid,
    pub queue_id: Uuid,
    pub left_player_name: Option<String>,
    pub right_player_name: Option<String>,
    pub queue_order: i32,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = queue_rows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewQueueRow {
    pub queue_id: Uuid,
    pub left_player_name: Option<String>,
    pub right_player_name: Option<String>,
    pub queue_order: i32,
}
