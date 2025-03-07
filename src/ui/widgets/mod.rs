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

        // base class all widgets, manages the widget lifecycle, states and style
        impl WidgetImpl for BatteryWidget {}

        // Arranges child widgets into a single row or column.
        impl BoxImpl for BatteryWidget {}
    }

    glib::wrapper! {
        pub struct BatteryWidget(ObjectSubclass<imp::BatteryWidget>)
            @extends gtk::Widget, gtk::Box
            // ,
            // @implements gtk::Buildable, gtk::Orientable
        ;
    }

    impl BatteryWidget {
        pub fn new() -> Self {
            let obj: Self = glib::Object::builder().build();
            let imp = obj.imp();
            imp.layout.set_orientation(gtk::Orientation::Horizontal);
            imp.layout.set_css_classes(&["battery-widget"]);

            let test_widget = gtk::Entry::builder().build();
            imp.layout.append(&test_widget);
            obj.append(&imp.layout);
            obj
        }

        pub(crate) fn check(&self) {
            let imp = self.imp();
            println!("DEBUG: check");
        }
    }
}
