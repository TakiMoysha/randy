dev:
	echo "dev"
	cargo run -- -c config/randy.toml


cliff:
	git cliff 424c8.. -o CHANGELOG.md

