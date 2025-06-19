// @generated automatically by Diesel CLI.

diesel::table! {
    queue_rows (id) {
        id -> Uuid,
        queue_id -> Uuid,
        left_player_name -> Nullable<Text>,
        right_player_name -> Nullable<Text>,
        queue_order -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    queues (id) {
        id -> Uuid,
        #[max_length = 255]
        url_name -> Varchar,
        #[max_length = 255]
        display_name -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(queue_rows -> queues (queue_id));

diesel::allow_tables_to_appear_in_same_query!(
    queue_rows,
    queues,
);
