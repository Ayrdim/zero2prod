// cargo watch -x check -x test -x run
// cargo tarpaulin --ignore-tests
// cargo clippy -- -D warnings
// #[allow(clippy::lint_name)] above code sections to prevent clippy from causing errors on warnings we dont care about/ cant avoid
// cargo fmt
// cargo fmt -- --check <-- checks if the format is good, will cause the CI to fail if it isn't good
// cargo audit <-- will check if any of our dependencies have any known vulnerabilities

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
