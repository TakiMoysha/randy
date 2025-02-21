pub mod battery {
    use gtk::{
        glib::{self, subclass::types::ObjectSubclassIsExt},
        prelude::{BoxExt, OrientableExt, WidgetExt},
    };

    pub mod imp {
        use gtk::{
            glib::subclass::{
                object::{ObjectImpl, ObjectImplExt},
                types::{ObjectSubclass, ObjectSubclassExt},
            },
            subclass::{box_::BoxImpl, widget::WidgetImpl},
        };

        use super::*;

        #[derive(Default, Debug)]
        pub struct BatteryWidget {
            pub layout: gtk::Box,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for BatteryWidget {
            const NAME: &'static str = "BatteryWidget";
            type Type = super::BatteryWidget;
            type ParentType = gtk::Box;
        }

        impl ObjectImpl for BatteryWidget {
            fn constructed(&self) {
                self.parent_constructed();
                self.obj().check();
            }
        }

        impl WidgetImpl for BatteryWidget {}

        impl BoxImpl for BatteryWidget {}
    }

    glib::wrapper! {
        pub struct BatteryWidget(ObjectSubclass<imp::BatteryWidget>)
            @extends gtk::Widget, gtk::Box,
            @implements gtk::Buildable, gtk::Orientable;
    }

    impl BatteryWidget {
        pub fn new() -> Self {
            let obj: Self = glib::Object::builder().build();
            let imp = obj.imp();
            imp.layout.set_orientation(gtk::Orientation::Horizontal);
            imp.layout.set_css_classes(&["battery-widget"]);

            obj
        }

        pub(crate) fn check(&self) {
            let imp = self.imp();
            println!("DEBUG: check");
        }
    }
}
