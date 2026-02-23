use actix_web::{HttpResponse, web::Form};

#[derive(serde::Deserialize, Debug)]
pub struct SubscriptionForm {
    email: String,
    name: String,
}

pub async fn subscription(_form: Form<SubscriptionForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
