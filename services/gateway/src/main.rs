use std::{env, error::Error};
use diesel::{prelude::*, pg::Pg};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use axum::{routing::get, Router};

mod schema;
mod events;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let mut connection = establish_connection();
    run_migrations(&mut connection).unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/events", get(events::get_events));
 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
