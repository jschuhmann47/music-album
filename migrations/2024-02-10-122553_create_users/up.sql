-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password  VARCHAR(255) NOT NULL
);

INSERT INTO users (username, password)
VALUES ('test', '123');