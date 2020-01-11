use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer, Responder, HttpResponse};


//#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .service(
                // static files
                fs::Files::new("/dashboard", "./dashboard/dist").show_files_listing().index_file("index.html"),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
