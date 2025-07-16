use axum::{
    extract::State,
    response::Html,
    Form,
};
use sqlx::{Pool, Sqlite};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExpenseForm {
    pub description: String,
    pub amount: f64,
    pub category: String,
}

pub async fn get_expenses(State(pool): State<Pool<Sqlite>>) -> Html<String> {
    let expenses = sqlx::query!("select * from expenses order by date desc")
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut html = String::from("<ul class=\"space-y-2 w-full\">");

    for expense in expenses {
        html.push_str(&format!(
            "<li class=\"bg-white p-3 rounded shadow\">{} - {} kr ({})</li>",
            expense.description, expense.amount, expense.category
        ));
    }

    html.push_str("</ul>");
    Html(html)
}

// Add new expense
pub async fn add_expense(
    State(pool): State<Pool<Sqlite>>,
    Form(form): Form<ExpenseForm>,
) -> Html<&'static str> {
    sqlx::query!(
        "INSERT INTO expenses (description, amount, category) VALUES (?, ?, ?)",
        form.description,
        form.amount,
        form.category
    )
    .execute(&pool)
    .await
    .unwrap();

    Html("<p class=\"text-green-600\">Utgift lagt til!</p>")
}
