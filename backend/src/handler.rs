use crate::{
    model::FeedbackModel,
    schema::{CreateFeedbackSchema, FilterOptions, UpdateFeedbackSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build API with Rust, SQLX, Postgres and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[get("/feedbacks")]
pub async fn feedback_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        FeedbackModel,
        "SELECT * FROM feedbacks ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data, db)
    .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all feedback items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let feedbacks = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": feedbacks.len(),
        "feedbacks": feedbacks
    });
    HttpResponse::Ok().json(json_response)
}
