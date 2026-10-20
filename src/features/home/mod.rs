


use axum::{routing::get, Router};
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
}

pub fn router() -> Router{
    Router::new().route("/", get(index))
}

async fn index() -> Html<String> {
    let template = IndexTemplate {
    };

    Html(template.render().unwrap())
}