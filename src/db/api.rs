use crate::db;
use crate::queue::*;
use std::collections::VecDeque;
use uuid::Uuid;

pub async fn queue_from_url_name(url_name: &str) -> Option<QueueData> {
    // Diesel stuff goes here?
    // Select the db::Queue with the url_name = name.
    // My cat had this to say: =----r4eghf

    todo!()
}

pub async fn get_queue_rows(id: Uuid) -> Option<VecDeque<RowData>> {
    // Here is where we're gonna get the QueueData
    // I imagine there's going to be some trasnformation from
    // Iterable<db::QueueRow> to VecDeque<RowData>?

    todo!()
}
