// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "row_type_enum"))]
    pub struct RowTypeEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RowTypeEnum;

    queue_rows (id) {
        id -> Uuid,
        queue_id -> Uuid,
        row_type -> RowTypeEnum,
        queue_order -> Int4,
        left_player_name -> Nullable<Text>,
        right_player_name -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    queues (id) {
        id -> Uuid,
        #[max_length = 255]
        url_name -> Varchar,
        #[max_length = 255]
        display_name -> Varchar,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(queue_rows -> queues (queue_id));

diesel::allow_tables_to_appear_in_same_query!(
    queue_rows,
    queues,
);
