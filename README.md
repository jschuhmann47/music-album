# Music Album API

A small API that I made to practice Rust. Used Axum and Diesel (with MySQL) plus some other stuff.

You can create users, and each user can create, upload or delete music albums. It's like a physical collection of CDs but digitally.

## Features
- User creation with hashed password in DB
- Authentication with JWT
- Create, edit and delete your music albums

## Code structure
- Entrypoint layer: parses the request, and sends response
- Usecase layer: has the application logic
- Repository layer: interacts with the database

## Demo
[output.webm](https://github.com/jschuhmann47/music-album/assets/76596375/f3fe2560-0c4f-4017-9832-6b2074543ebf)
