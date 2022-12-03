use actix_web::{web::Form, HttpResponse, post};

#[derive(serde::Deserialize)]
pub struct UserData {
    name: String,
    email: String
}

#[post("/subscribe")]
pub async fn subscribe(_form: Form<UserData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}