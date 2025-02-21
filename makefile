dev:
	cargo run -- -c config/randy.toml

test:
	cargo test -- --nocapture

cliff:
	git cliff 424c8.. -o CHANGELOG.md

