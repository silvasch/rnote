use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk4::{glib, CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Default, Debug, CompositeTemplate)]
    #[template(resource = "/com/github/flxzt/rnote/ui/settingswindow.ui")]
    pub(crate) struct RnSettingsWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for RnSettingsWindow {
        const NAME: &'static str = "RnSettingsWindow";
        type Type = super::RnSettingsWindow;
        type ParentType = adw::PreferencesWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RnSettingsWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
        }
    }

    impl WidgetImpl for RnSettingsWindow {}

    impl WindowImpl for RnSettingsWindow {}

    impl AdwWindowImpl for RnSettingsWindow {}

    impl PreferencesWindowImpl for RnSettingsWindow {}
}

glib::wrapper! {
    pub(crate) struct RnSettingsWindow(ObjectSubclass<imp::RnSettingsWindow>)
    @extends gtk4::Widget, gtk4::Window, adw::Window, adw::PreferencesWindow;
}

impl RnSettingsWindow {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for RnSettingsWindow {
    fn default() -> Self {
        glib::Object::new()
    }
}
