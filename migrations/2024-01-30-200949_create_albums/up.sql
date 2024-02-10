-- Your SQL goes here
CREATE TABLE IF NOT EXISTS albums (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    artist  VARCHAR(255) NOT NULL,
    cover  VARCHAR(255) NOT NULL,
    `year` INTEGER NOT NULL
);

INSERT INTO albums (title, artist, cover, year)
VALUES ('Anchu band', 'Bring me the moon', 'https://www.slugmag.com/wp/wp-content/uploads/2013/05/bring-me-the-horizon-sempiternal.webp', 2020);
