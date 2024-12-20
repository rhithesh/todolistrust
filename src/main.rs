use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use serde_json::json; // Add this to use the json! macro

// Todo item structure
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

// App state structure
struct AppState {
    todo_list: Mutex<Vec<Todo>>,
}

// Request payload for creating a todo
#[derive(Debug, Deserialize)]
struct CreateTodoRequest {
    title: String,
}

// GET /todos - List all todos
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todo_list = data.todo_list.lock().unwrap();
    HttpResponse::Ok().json(todo_list.to_vec())
}


async fn get() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Hi"
    }))
}

// POST /todos - Create a new todo
async fn create_todo(
    data: web::Data<AppState>,
    payload: web::Json<CreateTodoRequest>,
) -> impl Responder {
    let mut todo_list = data.todo_list.lock().unwrap();
    println!("Creating todo: {:?}", payload);
    
    let new_todo = Todo {
        id: todo_list.len() as u64 + 1,
        title: payload.title.clone(),
        completed: false,
    };
    
    todo_list.push(new_todo.clone());
    HttpResponse::Created().json(new_todo)
}

// PUT /todos/{id}/toggle - Toggle todo completion status
async fn toggle_todo(
    data: web::Data<AppState>,
    path: web::Path<u64>,
) -> impl Responder {
    let todo_id = path.into_inner();
    let mut todo_list = data.todo_list.lock().unwrap();
    
    if let Some(todo) = todo_list.iter_mut().find(|t| t.id == todo_id) {
        todo.completed = !todo.completed;
        HttpResponse::Ok().json(todo)
    } else {
        HttpResponse::NotFound().finish()
    }
}

// DELETE /todos/{id} - Delete a todo
async fn delete_todo(
    data: web::Data<AppState>,
    path: web::Path<u64>,
) -> impl Responder {
    let todo_id = path.into_inner();
    let mut todo_list = data.todo_list.lock().unwrap();
    
    if let Some(pos) = todo_list.iter().position(|t| t.id == todo_id) {
        todo_list.remove(pos);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        todo_list: Mutex::new(Vec::new()),
    });

    println!("Server running at http://localhost:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}/toggle", web::put().to(toggle_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
            .route("/", web::get().to(get))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}