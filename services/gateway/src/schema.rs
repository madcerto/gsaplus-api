diesel::table! {
    events (id) {
        id -> Int4,
        title -> VarChar,
        description -> Text,
        location -> VarChar,
        start_time -> Timestamp,
        end_time -> Timestamp,
        is_multi_day -> Bool,
        points -> Int4,
        volunteer_hours -> Float8,
        category -> VarChar,
        capacity -> Nullable<Int4>,
        // TODO: status, default draft
        organizer_id -> VarChar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        // TODO: relations
        // TODO: indexes
    }
}
