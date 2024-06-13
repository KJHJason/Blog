use crate::model::index::Index;
use actix_web::{get, HttpResponse};

#[get("/api")]
async fn api_index() -> HttpResponse {
    let serialised = serde_json::to_string_pretty(&Index::new()).unwrap();
    return HttpResponse::Ok()
        .content_type("application/json")
        .body(serialised);
}