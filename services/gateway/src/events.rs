use serde::Serialize;
use axum::Json;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::establish_connection;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Event {
    id: i32,
    title: String,
    description: String,
    location: String,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    is_multi_day: bool,
    points: i32,
    volunteer_hours: f64,
    category: String,
    capacity: Option<i32>,
    organizer_id: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EventRelations {
    rsvps: usize,
    attendance_records: usize
}
#[derive(Serialize)]
pub struct EventCollection {
    events: Vec<Event>,
    total: usize,
    page: usize,
    pages: usize,
    _count: EventRelations
}

const TAKE: i64 = 12; // Events per page

pub async fn get_events() -> Json<EventCollection> {
    use crate::schema::events::dsl::*;

    let connection = &mut establish_connection();
    let items = events
        .limit(TAKE)
        .select(Event::as_select())
        .load(connection)
        .expect("Error loading events");

    Json(EventCollection {
        total: items.len(),
        page: 1, // TODO: pagination
        pages: (items.len() as f32 / TAKE as f32).ceil() as usize,
        events: items,
        _count: EventRelations { rsvps: 0, attendance_records: 0 }
    })
}
