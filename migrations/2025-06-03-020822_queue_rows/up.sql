CREATE TABLE queue_rows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    queue_id UUID NOT NULL REFERENCES queues(id) ON DELETE CASCADE,

    -- TEXT is placeholder, will eventually be more expressive
    left_player_name TEXT,
    right_player_name TEXT,

    -- Represents ordering of rows
    queue_order INT NOT NULL,
    -- Must be unique to ensure explicit ordering
    UNIQUE (queue_id, queue_order),

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,

    -- Ensure rows are not empty
    CHECK (left_player_name IS NOT NULL OR right_player_name IS NOT NULL)
);

CREATE INDEX idx_queue_rows_order ON queue_rows (queue_id, queue_order);
