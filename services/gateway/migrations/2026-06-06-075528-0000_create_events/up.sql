-- Your SQL goes here
CREATE TABLE events (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL,
	description TEXT NOT NULL,
	location VARCHAR NOT NULL,
	start_time TIMESTAMP NOT NULL,
	end_time TIMESTAMP NOT NULL,
	is_multi_day BOOL NOT NULL DEFAULT FALSE,
	points INT4 NOT NULL DEFAULT 0,
	volunteer_hours FLOAT NOT NULL DEFAULT 0,
	category VARCHAR NOT NULL,
	capacity INT4,
        image_url VARCHAR,
	organizer_id VARCHAR NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

