CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    password VARCHAR NOT NULL,
    username VARCHAR UNIQUE NOT NULL,
    name VARCHAR NOT NULL,
    summary TEXT NOT NULL
);

CREATE TABLE remote_users (
    id VARCHAR PRIMARY KEY,
    url VARCHAR UNIQUE NOT NULL,
    inbox VARCHAR UNIQUE NOT NULL,
    outbox VARCHAR UNIQUE NOT NULL,
    following VARCHAR UNIQUE NOT NULL,
    followers VARCHAR UNIQUE NOT NULL,
    username VARCHAR UNIQUE NOT NULL,
    name VARCHAR NOT NULL,
    summary TEXT NOT NULL
);

INSERT INTO users (id, password, username, name, summary) VALUES (0, 'admin', 'admin', 'Admin', 'The admin.');
