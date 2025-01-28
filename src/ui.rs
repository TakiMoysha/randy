use gtk::prelude::*;
use gtk::ApplicationWindow;

pub fn build_ui(app: &gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Randy")
        .build();

    window.present();
}
