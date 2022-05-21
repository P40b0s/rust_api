use actix_web::{HttpServer, App, web};


#[path="routes/token_routes.rs"]
mod token_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> 
{
    HttpServer::new(|| {
        App::new()
            .service(token_routes::hello)
            .service(token_routes::echo)
            .service(token_routes::json)
            .route("/hey", web::get().to(token_routes::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
