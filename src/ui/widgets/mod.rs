pub mod battery {
    use gtk::glib::{self, subclass::types::ObjectSubclassIsExt};

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
        pub struct BatteryWidget {}

        #[glib::object_subclass]
        impl ObjectSubclass for BatteryWidget {
            const NAME: &'static str = "BatteryWidget";
            type Type = super::BatteryWidget;
            type ParentType = gtk::Box;
        }

        impl ObjectImpl for BatteryWidget {
            fn constructed(&self) {
                self.parent_constructed();
                self.obj().setup();
            }
        }

        impl WidgetImpl for BatteryWidget {}

        impl BoxImpl for BatteryWidget {}
    }

    glib::wrapper! {
        pub struct BatteryWidget(ObjectSubclass<imp::BatteryWidget>) @extends gtk::Widget, gtk::Box;
    }

    impl BatteryWidget {
        pub fn new() -> Self {
            glib::Object::builder().build()
        }

        pub(crate) fn setup(&self) {
            let imp = self.imp();
        }
    }
}
