pub fn load_css(config: &config::Config) {
    let style = css::load_css(&config);
    let cssProvider = gtk::CssProvider::new();
}
