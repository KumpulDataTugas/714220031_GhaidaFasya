use actix_web::{get, App, HttpServer, HttpResponse, Responder};

#[get("/")]
async fn halo() -> impl Responder {
    HttpResponse::Ok().body("Hello, i'm Ghaida!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server berjalan di http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(halo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
