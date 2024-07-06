-- Add up migration script here

CREATE TABLE IF NOT EXISTS user (
    user_id VARCHAR(36) BINARY NOT NULL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS team (
    team_id VARCHAR(36) BINARY NOT NULL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS team_member (
    team_id VARCHAR(36) BINARY NOT NULL,
    user_id VARCHAR(36) BINARY NOT NULL,
    member_rank SMALLINT NOT NULL
);
