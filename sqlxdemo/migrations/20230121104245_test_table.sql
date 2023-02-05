-- Add migration script here
CREATE TABLE IF NOT EXISTS test(
    id BIGSERIAL,
    name VARCHAR(20)
)