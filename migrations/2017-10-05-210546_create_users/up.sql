CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    password VARCHAR NOT NULL,
    username VARCHAR UNIQUE NOT NULL,
    name VARCHAR NOT NULL,
    summary TEXT NOT NULL
);
