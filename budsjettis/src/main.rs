mod handlers;
mod models;
mod templates;
mod charts;

use axum::{
    routing::get,
    routing::delete,
    Router,
};

use handlers::{home::home_page, expenses::get_expenses, expenses::add_expense, expenses::delete_expense, expenses::get_expense_chart_by_date, expenses::get_expense_chart_by_category};
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::path::Path;

#[tokio::main]
async fn main() {
    // Create database connection
    let database_url = "sqlite:budsjettis.db";
    //
    // Ensure the database file exists
    if !Path::new("budsjettis.db").exists() {
        std::fs::File::create("budsjettis.db").unwrap();
    }

    let pool = SqlitePool::connect(database_url).await.unwrap();

    // Run migrations, create tables
    sqlx::migrate!("./migrations").run(&pool).await;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(home_page))
        .route("/expenses", get(get_expenses).post(add_expense))
        .route("/expenses/{id}", delete(delete_expense))
        .route("/expenses/chart/date", get(get_expense_chart_by_date))
        .route("/expenses/chart/category", get(get_expense_chart_by_category))
        .with_state(pool)
        .into_make_service();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Budsjettis running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
