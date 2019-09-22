use actix_files as fs;
use actix_web::{App, HttpServer};


fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new().service(
              fs::Files::new("/", "./dist").show_files_listing().index_file("index.html")))
        .bind("127.0.0.1:8080")?
        .run()
}
