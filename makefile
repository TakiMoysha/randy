dev:
	echo "dev"
	cargo run -- -c config/randy.toml


cliff:
	git cliff 424c8..

# References:
# https://gtk-rs.org/gtk4-rs/stable/latest/book/hello_world.html
# https://github.com/jbenner-radham/rust-gtk4-css-styling
# gtk4 not support move function (btw wayland)
# 


