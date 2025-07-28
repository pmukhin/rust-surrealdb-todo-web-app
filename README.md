# To-Do API

This is a simple To-Do List API built with Rust and the Axum framework.

## Features

* Create a new To-Do item
* Get a list of all To-Do items
* Get a specific To-Do item by its ID

## Prerequisites

* Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
* Docker: [https://www.docker.com/get-started](https://www.docker.com/get-started)

## Getting Started

1. **Clone the repository:**

   ```bash
   git clone https://github.com/pmukhin/rust-surrealdb-todo-web-app.git
   cd rust-surrealdb-todo-web-app
   ```

2. **Start the database:**

   This project uses SurrealDB as the database. A `docker-compose.yml` file is provided for convenience.

   ```bash
   docker-compose up -d
   ```

3. **Run the application:**

   ```bash
   cargo run
   ```

   The application will be running at `http://localhost:3000`.

## API Endpoints

### Create a To-Do

* **Method:** `POST`
* **URL:** `/todos`
* **Body:**

  ```json
  {
    "title": "My first todo",
    "description": "This is a description of my first todo."
  }
  ```

### Get All To-Dos

* **Method:** `GET`
* **URL:** `/todos`

### Get a To-Do by ID

* **Method:** `GET`
* **URL:** `/todos/{id}`
