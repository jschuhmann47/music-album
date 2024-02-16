-- Your SQL goes here
CREATE TABLE IF NOT EXISTS albums (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME ON UPDATE CURRENT_TIMESTAMP,
    title VARCHAR(255) NOT NULL,
    artist  VARCHAR(255) NOT NULL,
    cover  VARCHAR(255) NOT NULL,
    `year` INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    
    FOREIGN KEY (user_id) REFERENCES users(id)
);

INSERT INTO albums (created_at, title, artist, cover, year)
VALUES (NOW() ,'Anchu band', 'Bring me the moon', 'https://www.slugmag.com/wp/wp-content/uploads/2013/05/bring-me-the-horizon-sempiternal.webp', 2020);
