use axum::response::Html;
use crate::templates::base_layout::base_layout;

pub async fn home_page() -> Html<String> {
    Html(base_layout())
}
