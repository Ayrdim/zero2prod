// cargo watch -x check -x test -x run
// cargo tarpaulin --ignore-tests
// cargo clippy -- -D warnings
// #[allow(clippy::lint_name)] above code sections to prevent clippy from causing errors on warnings we dont care about/ cant avoid
// cargo fmt
// cargo fmt -- --check <-- checks if the format is good, will cause the CI to fail if it isn't good
// cargo audit <-- will check if any of our dependencies have any known vulnerabilities
// cargo watch -x fmt -x check -x test -x run

use zero2prod::startup::run;

use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    run(listener)?.await
}
