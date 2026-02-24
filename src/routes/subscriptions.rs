use actix_web::{
    HttpResponse,
    web::{Data, Form},
};
use sqlx::PgConnection;

#[derive(serde::Deserialize, Debug)]
pub struct SubscriptionForm {
    email: String,
    name: String,
}

pub async fn subscription(
    _form: Form<SubscriptionForm>,
    _connection: Data<PgConnection>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
