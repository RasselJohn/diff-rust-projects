mod forms;
mod parser;
mod views;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/day", web::get().to(views::temperature_for_day_view))
            .route("/week", web::get().to(views::temperature_for_week_view))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
