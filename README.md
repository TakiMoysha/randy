# Randy4

> Planned as an update to the original Randy, but due to migration on the GTK-4 i had to rewrite everything.

Conky inspired system info viewer written in Rust / GTK4

## Road Map

- [ ] Migrate origin functionality:
  - [ ] yaml config reading
  - [ ] css loading and customization
  - [ ] benches for speed testing
  - [ ] timings feature
  - [ ] Configurable modules [#configurable-modules]
  - [ ] UI settings [#ui-settings]
  - [ ] GPU sensors support
  - [ ] lm-sensors support
- [x] toml config reading
- [ ] using `gio::Settings`

### Notes

Использовать `Cell` для общего контекста в приложении, что бы добавить мутабельности в `Settings`.
Использовать стандартные конфиг и подгружать дефолтные значения из css, а не в коде.

### Migration

- xpos: 0 # deprecated
- ypos: 0 # deprecated

## Features

### Configurable Modules

- Module list:
  - system - system info
  - cpus - all cpus usage stats bar
  - cpu_consumers - top N pids using cpu and their usage
  - mem_consumers - top N pids using mem and their usage
  - filesystem - usage of a given mounted filesystem
  - net - usage recv/trans for a given network interface
  - battery - charging/discharging percentage of /sys/\*/power_supply's
- Can order the modules how you wish
- Can enable/disable modules and sub items

### UI Settings

- bar_height - the height of the bars (default: 10px)
- base_opacity - the base opacity of the Randy window. affects `window` and all sub-widgets. (default: 1.0)
- color_bar - base color of the usage bars
- color_bar_med - color of the usage bars > 50% < 80%
- color_bar_high - color of the usage bars > 80%
- color*borders - color of the GTK \_borders* (frame borders, bar borders) defaults to same as _color_text_
- color_label - color of the "labels"
- color_text - color of all other text
- decoration - hide/show window decorations
- font_family - the CSS-style font family string (font names with spaces must be wrapped in escaped quotes, eg `fo_family: "\"Terminus (TTF)\", \"Liberation Mono\", monospace"`)
- font_size
- mod_bat -modulo used to skip frames for getting battery data (default: 2)
- mod_fs - modulo used to skip frames for getting filesystem data (default: 2)
- mod_top - modulo used to skip frames for getting top data (default: 2)
- resizable - bool to make the GUI resizable
- skip_taskbar - in case you want to see a Randy item in the taskbar
- timeout - time in seconds to wait between frame updates
- xpos - starting position x
- ypos - starting position y

## Building

## Running

## Issues

gtk4 not support move function (btw wayland)

## Credits & References

1. [Origin / github.com](https://github.com/iphands/randy)
2. [GTK4 Demo App - simple css widget / github.com](https://github.com/TakiMoysha/tm-rust-book/tree/main/practice/simple-linux-widget)
3. [Rust Gtk4 css styleing / github.com](https://github.com/jbenner-radham/rust-gtk4-css-styling)
4. [Example gtk-rs (gtk4) custom widget / github.com](https://github.com/gtk-rs/examples/blob/master/src/bin/listbox_model.rs)
5. [About create custom widgets, Subclassing GTK widgets in Rust / www.figuiere.net](https://www.figuiere.net/technotes/notes/tn002/)
