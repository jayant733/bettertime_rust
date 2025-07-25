-- Your SQL goes here
CREATE TABLE regions (
    id VARCHAR PRIMARY KEY,
    name VARCHAR
);
CREATE TABLE users (
    id VARCHAR PRIMARY KEY,
    username VARCHAR,
    password VARCHAR
);
CREATE TABLE websites (
    id BIGINT PRIMARY KEY,
    url VARCHAR,
    date_time TIMESTAMP,
    user_id VARCHAR REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE website_ticks (
    id BIGINT PRIMARY KEY,
    response_time_ms INTEGER,
    status VARCHAR,
    website_id BIGINT REFERENCES websites(id) ON DELETE CASCADE,
    region_id VARCHAR REFERENCES regions(id) ON DELETE CASCADE,
    created_at TIMESTAMP
);

