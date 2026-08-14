# Gearly - Car Parts E-Commerce 🚗⚙️

![Status](https://img.shields.io/badge/Status-In%20Progress-yellow)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)

A backend API for a Car Parts marketplace built with Rust, Axum, and Diesel.

## Overview
Gearly is a platform connecting car parts shopkeepers with users. 
- **Shopkeepers** can register, manage their inventory, and list car parts.
- **Users** can browse parts, search for specific items, and place orders.

## Features & Progress
- ✅ **Authentication:** Production-ready signup and login flows for both Users and Shopkeepers.
- ✅ **Security:** Asynchronous Argon2 password hashing and secure JWT-based stateless authorization.
- ✅ **Database:** Connection pooling with `deadpool-diesel` for optimized MySQL operations.
- 🚧 **Dashboard & Inventory Management:** (Work in Progress)

## Tech Stack
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
- **Database ORM:** [Diesel](https://diesel.rs/)
- **Database:** MySQL
- **Async Runtime:** Tokio
- **Authentication:** JWT & Argon2

## Running Locally

1. Set up your `.env` file with `DATABASE_URL=mysql://...`
2. Run migrations:
   ```bash
   diesel migration run
   ```
3. Start the server:
   ```bash
   cargo run
   ```
