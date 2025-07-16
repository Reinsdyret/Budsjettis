use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Expense {
    pub id: Option<i64>,
    pub description: String,
    pub amount: f64,
    pub category: String,
    pub date: String, // We'll use simple string dates for now
}

#[derive(Debug, Deserialize)]
pub struct NewExpense {
    pub description: String,
    pub amount: f64,
    pub category: String,
}
