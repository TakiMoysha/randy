
fn get_css(conf: &ConfigString, composited: bool) -> String {
    let css: String = String::from(include_str!("styles/app.css"));
    let color_text = conf["color_text"].as_str().unwrap_or("#e1eeeb");
    let color_background = conf["color_background"]
        .as_str()
        .unwrap_or("rgba(0, 0, 0, 0.5)");
    let color_trough = match composited {
        true => conf["color_trough"].as_str().unwrap_or("rgba(0, 0, 0, 0)"),
        false => conf["color_trough"].as_str().unwrap_or(color_background),
    };

    let font_size = conf["font_size"].as_str().unwrap_or("large");
    let base_opacity = format!("{:1.4}", conf["base_opacity"].as_f64().unwrap_or(1.0));

    return css
        .replace(
            "{ bar_height }",
            conf["bar_height"].as_str().unwrap_or("10px"),
        )
        .replace("{ base_opacity }", &base_opacity)
        .replace(
            "{ color }",
            conf["color_text"].as_str().unwrap_or("#e1eeeb"),
        )
        .replace("{ color_background }", color_background)
        .replace(
            "{ color_borders }",
            conf["color_borders"].as_str().unwrap_or(color_text),
        )
        .replace(
            "{ color_bar }",
            conf["color_bar"].as_str().unwrap_or("#e1eeff"),
        )
        .replace(
            "{ color_bar_med }",
            conf["color_bar_med"].as_str().unwrap_or("#ffeeaa"),
        )
        .replace(
            "{ color_bar_high }",
            conf["color_bar_high"].as_str().unwrap_or("#ffaaaa"),
        )
        .replace(
            "{ color_label }",
            conf["color_label"].as_str().unwrap_or("#87d7ff"),
        )
        .replace("{ color_trough }", color_trough)
        .replace(
            "{ font_family }",
            conf["font_family"].as_str().unwrap_or("monospace"),
        )
        .replace(
            "{ font_size_top }",
            conf["font_size_top"].as_str().unwrap_or(font_size),
        )
        .replace("{ font_size }", font_size);
}
