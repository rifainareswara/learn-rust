// Modul 06: CRUD App (Proyek Akhir)
// Cara menjalankan: cargo run -p 06_crud_app
// Lalu test dengan curl atau Postman

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use std::sync::Arc;

// --- MODEL ---
#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
struct Task {
    id: i64,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

// --- STATE ---
struct AppState {
    db: SqlitePool,
}

// --- HANDLERS ---

// GET /tasks — ambil semua task
async fn get_tasks(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks ORDER BY id")
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();
    (StatusCode::OK, Json(tasks))
}

// POST /tasks — buat task baru
async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    let result = sqlx::query("INSERT INTO tasks (title, completed) VALUES (?, false)")
        .bind(&payload.title)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// PUT /tasks/:id — toggle selesai/belum
async fn toggle_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let result = sqlx::query("UPDATE tasks SET completed = NOT completed WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

// DELETE /tasks/:id — hapus task
async fn delete_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// --- MAIN ---
#[tokio::main]
async fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Modul 06: Task Manager CRUD API     ║");
    println!("╚══════════════════════════════════════╝\n");

    // Setup database
    let opts = sqlx::sqlite::SqliteConnectOptions::new()
        .filename("tasks.db")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(opts)
        .await
        .expect("Gagal konek ke database!");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tasks (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            title     TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )",
    )
    .execute(&pool)
    .await
    .expect("Gagal membuat tabel!");

    let state = Arc::new(AppState { db: pool });

    // Router
    let app = Router::new()
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/:id", put(toggle_task).delete(delete_task))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server berjalan di http://{}", addr);
    println!();
    println!("Endpoint:");
    println!("  GET    /tasks         → Daftar semua task");
    println!("  POST   /tasks         → Buat task baru");
    println!("  PUT    /tasks/:id     → Toggle selesai/belum");
    println!("  DELETE /tasks/:id     → Hapus task");
    println!();
    println!("Contoh curl:");
    println!("  curl http://localhost:3000/tasks");
    println!("  curl -X POST http://localhost:3000/tasks -H 'Content-Type: application/json' -d '{{\"title\":\"Belajar Rust\"}}'");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
