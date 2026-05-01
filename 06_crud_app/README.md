# Modul 06 — CRUD App (Proyek Akhir)

## 🎯 Tujuan Pembelajaran

Menggabungkan semua yang dipelajari untuk membangun REST API nyata:
- Axum (web framework)
- SQLite dengan SQLx (database)
- Serde (serialisasi JSON)
- Tokio (async runtime)
- Shared State dengan `Arc<T>`

---

## 📖 Arsitektur Aplikasi

```
Request → Router → Handler → Database (SQLite)
                ↓
            Response (JSON)
```

### Endpoint API

| Method | URL | Fungsi |
|--------|-----|--------|
| `GET` | `/tasks` | Ambil semua task |
| `POST` | `/tasks` | Buat task baru |
| `PUT` | `/tasks/:id` | Toggle selesai/belum |
| `DELETE` | `/tasks/:id` | Hapus task |

---

## 📖 Konsep yang Digunakan

### 1. Model Data
```rust
#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Task {
    id: i64,
    title: String,
    completed: bool,
}
```

### 2. Shared State
```rust
struct AppState {
    db: SqlitePool,
}
// Dibungkus Arc agar bisa dibagi ke semua handler
let state = Arc::new(AppState { db: pool });
```

### 3. Router
```rust
let app = Router::new()
    .route("/tasks", get(get_tasks).post(create_task))
    .route("/tasks/:id", put(toggle_task).delete(delete_task))
    .with_state(state);
```

---

## ▶️ Cara Menjalankan

```bash
# Jalankan server
cargo run -p 06_crud_app
```

Server berjalan di: `http://localhost:3000`

---

## 🧪 Cara Tes API

```bash
# GET semua tasks
curl http://localhost:3000/tasks

# POST task baru
curl -X POST http://localhost:3000/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Belajar Rust"}'

# PUT toggle task id=1
curl -X PUT http://localhost:3000/tasks/1

# DELETE task id=1
curl -X DELETE http://localhost:3000/tasks/1
```

---

## 🎉 Selamat!

Kamu telah menyelesaikan seluruh kurikulum Rust! Langkah selanjutnya:
- Pelajari [The Rust Book](https://doc.rust-lang.org/book/) lebih dalam
- Coba buat project sendiri
- Eksplorasi ekosistem: [crates.io](https://crates.io)
