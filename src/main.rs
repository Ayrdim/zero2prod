// cargo watch -x check -x test -x run
// cargo tarpaulin --ignore-tests
// cargo clippy -- -D warnings
// #[allow(clippy::lint_name)] above code sections to prevent clippy from causing errors on warnings we dont care about/ cant avoid
// cargo fmt
// cargo fmt -- --check <-- checks if the format is good, will cause the CI to fail if it isn't good
// cargo audit <-- will check if any of our dependencies have any known vulnerabilities

fn main() {
    println!("Hello, world!");
}
