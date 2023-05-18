// cargo watch -x check -x test -x run
// cargo tarpaulin --ignore-tests
// cargo clippy -- -D warnings
// #[allow(clippy::lint_name)] above code sections to prevent clippy from causing errors on warnings we dont care about/ cant avoid
// cargo fmt
// cargo fmt -- --check <-- checks if the format is good, will cause the CI to fail if it isn't good
// cargo audit <-- will check if any of our dependencies have any known vulnerabilities
// cargo watch -x fmt -x check -x test -x run

use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
