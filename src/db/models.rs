use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::queues)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Queue {
    pub id: Uuid,
    pub url_name: String,
    pub display_name: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::queue_rows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QueueRow {
    pub id: Uuid,
    pub queue_id: Uuid,
    pub left_player_name: Option<String>,
    pub right_player_name: Option<String>,
    pub queue_order: i32,
    pub created_at: chrono::DateTime<Utc>,
}
