CREATE INDEX IF NOT EXISTS idx_messages_room_created_at
    ON messages(room_id, created_at);

CREATE INDEX IF NOT EXISTS idx_messages_user_created_at
    ON messages(user_id, created_at);

CREATE INDEX IF NOT EXISTS idx_messages_pinned_at
    ON messages(pinned_at);

CREATE INDEX IF NOT EXISTS idx_messages_reply_to_id
    ON messages(reply_to_id);

CREATE INDEX IF NOT EXISTS idx_rooms_required_role
    ON rooms(required_role);
