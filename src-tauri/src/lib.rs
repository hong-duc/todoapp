use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};
use std::fs;
use std::sync::Mutex;
use tauri::{Manager, State};

#[derive(Serialize, Deserialize)]
struct Task {
    id: i64,
    name: String,
    is_done: bool,
}

struct AppState {
    db: Mutex<Connection>,
}



#[tauri::command]
fn add_task(state: State<AppState>, name: String, is_done: bool) -> Result<i64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db.prepare("INSERT INTO tasks (name, is_done) VALUES (:name, :is_done)")
        .map_err(|e| e.to_string())?;
    let is_done_int = is_done as i32;
    stmt.execute(rusqlite::named_params! {
        ":name": &name,
        ":is_done": &is_done_int
    })
        .map_err(|e| e.to_string())?;
    Ok(db.last_insert_rowid())
}

#[tauri::command]
fn get_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db.prepare("SELECT id, name, is_done FROM tasks")
        .map_err(|e| e.to_string())?;
    let tasks = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            is_done: row.get::<_, i32>(2)? != 0,
        })
    })
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<Task>, _>>()
    .map_err(|e| e.to_string())?;
    Ok(tasks)
}

#[tauri::command]
fn update_task(state: State<AppState>, id: i32, is_done: bool, name: String) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db.prepare("UPDATE tasks SET is_done = :is_done , name=:name WHERE id = :id")
        .map_err(|e| e.to_string())?;
    let is_done_int = is_done as i32;
    let changes = stmt.execute(rusqlite::named_params! {
        ":name": &name,
        ":is_done": &is_done_int,
        ":id": &id
    })
        .map_err(|e| e.to_string())?;
    Ok(changes > 0)
}

#[tauri::command]
fn delete_task(state: State<AppState>, id: i64) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db.prepare("DELETE FROM tasks WHERE id = ?1")
        .map_err(|e| e.to_string())?;
    let changes = stmt.execute(&[&id])
        .map_err(|e| e.to_string())?;
    Ok(changes > 0)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().setup(|app|{
        let path_resolver = app.path();
        let data_dir = path_resolver.data_dir().expect("failed to get data directory").join("todoapp");
        fs::create_dir_all(&data_dir).expect("failed to create data directory");
        let db_path = data_dir.join("todo.db");
        let db = Connection::open(&db_path).expect("failed to open database");
        db.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                is_done INTEGER NOT NULL
            )",
            [],
        )
        .expect("Failed to create table");
    app.manage(AppState {db: Mutex::new(db)});
    Ok(())

    }).invoke_handler(tauri::generate_handler![
            add_task,
            get_tasks,
            update_task,
            delete_task
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}