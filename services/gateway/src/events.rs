use serde::Serialize;
use axum::Json;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::establish_connection;

// Note: timestamps are stored without timezone marker, but assumed to be EST
#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
struct DBEvent {
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
    image_url: Option<String>,
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
struct Event {
    #[serde(flatten)]
    event: DBEvent,
    _count: EventRelations
}

#[derive(Serialize)]
pub struct EventCollection {
    events: Vec<Event>,
    total: usize,
    page: usize,
    pages: usize
}

const TAKE: i64 = 12; // Events per page

pub async fn get_events() -> Json<EventCollection> {
    use crate::schema::events::dsl::*;

    let connection = &mut establish_connection();
    let items: Vec<Event> = events
        .limit(TAKE)
        .select(DBEvent::as_select())
        .load(connection)
        .expect("Error loading events")
        .into_iter().map(|e| Event { event: e, _count: EventRelations { rsvps: 0, attendance_records: 0 }})
        .collect();

    Json(EventCollection {
        total: items.len(),
        page: 1, // TODO: pagination
        pages: (items.len() as f32 / TAKE as f32).ceil() as usize,
        events: items,
    })
}
