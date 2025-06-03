CREATE TYPE row_type_enum AS ENUM ('LeftOnly', 'RightOnly', 'Both');

CREATE TABLE queue_rows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    queue_id UUID NOT NULL REFERENCES queues(id),
    row_type row_type_enum NOT NULL,

    queue_order INT NOT NULL,
    UNIQUE (queue_id, queue_order),

    left_player_name TEXT,
    right_player_name TEXT,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    CHECK (
        (row_type = 'LeftOnly' AND left_player_name IS NOT NULL AND right_player_name IS NULL) OR
        (row_type = 'RightOnly' AND left_player_name IS NULL AND right_player_name IS NOT NULL) OR
        (row_type = 'Both' AND left_player_name IS NOT NULL AND right_player_name IS NOT NULL)
    )
);

CREATE INDEX idx_queue_rows_order ON queue_rows (queue_id, queue_order);
