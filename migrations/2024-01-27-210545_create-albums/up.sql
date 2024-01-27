-- Your SQL goes here
CREATE TABLE albums (
  id INTEGER AUTO_INCREMENT PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  artist VARCHAR(255) NOT NULL,
  cover VARCHAR(255) NOT NULL,
  year INTEGER NOT NULL
);

INSERT INTO albums
(title, artist, cover, `year`)
VALUES('test', 'testt', 'example.com', 2020);
