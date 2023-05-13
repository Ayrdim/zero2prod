// cargo watch -x check -x test -x run
// cargo tarpaulin --ignore-tests
// cargo clippy -- -D warnings
// #[allow(clippy::lint_name)] above code sections to prevent clippy from causing errors on warnings we dont care about/ cant avoid
// cargo fmt
// cargo fmt -- --check <-- checks if the format is good, will cause the CI to fail if it isn't good
// cargo audit <-- will check if any of our dependencies have any known vulnerabilities
// cargo watch -x fmt -x check -x test -x run

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
