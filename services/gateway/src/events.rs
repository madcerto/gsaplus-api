use serde::Serialize;
use axum::Json;
use chrono::NaiveDateTime;

#[derive(Serialize)]
pub struct Event {
    id: u32,
    title: String,
    description: String,
    location: String,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    is_multi_day: bool,
    points: u32,
    volunteer_hours: f32,
    category: String,
    capacity: u32,
    organizer_id: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct EventCollection {
    events: Vec<Event>,
    total: u32,
    page: u32,
    pages: u32
}

pub async fn get_events() -> Json<EventCollection> {
    todo!()
    //"{ \"events\": [], \"total\": 0, \"page\": 1, \"pages\": 0 }"
}
