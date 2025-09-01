# RustChat  

A real-time chat application built in **Rust** with [Axum](https://github.com/tokio-rs/axum), [SQLx](https://github.com/launchbadge/sqlx), and **WebSockets**, backed by a **Postgres** database.  
Fully containerized with Docker for easy setup and deployment.

![Rust](https://img.shields.io/badge/Rust-orange)
![Docker](https://img.shields.io/badge/Docker-ready-blue)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

---

## Quickstart

Clone the repo and run the containers:

```bash
git clone https://github.com/willboyle18/rustchat.git
cd rustchat

# copy environment config
cp .env.example .env

# build and run app + database
docker compose up --build
```
Once running, open [http://localhost:3000](http://localhost:3000).

## Tech Stack
- **Rust** - backend programming language
- **Axum** - web framework
- **Tokio** - async runtime
- **SQLx** - async Postgres queries + migrations
- **Postgres** - database
- **WebSockets** - real-time chat
- **HTML/CSS/JavaScript** - frontend
- **Docker** - containerized dev/production environment

## References
- [Axum Documentation](https://docs.rs/axum/latest/axum/)  
- [Tokio Documentation](https://tokio.rs/)  
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)  
- [axum-login](https://docs.rs/axum-login/latest/axum_login/)  
- [tower-sessions](https://docs.rs/tower-sessions/latest/tower_sessions/)  
- [Postgres Docs](https://www.postgresql.org/docs/current/)  
- [Docker Docs](https://docs.docker.com/)  

## License
This project is licensed under the terms of the [MIT License](./LICENSE).  
Copyright © 2025 William Boyle.
