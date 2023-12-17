use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;

use gtk4::{
    glib, Adjustment, Button, ColorDialogButton, CompositeTemplate, MenuButton, ScrolledWindow,
    ToggleButton,
};

use crate::{RnAppWindow, RnCanvasWrapper, RnIconPicker, RnUnitEntry};
use rnote_engine::document::Format;

mod imp {
    use super::*;

    #[derive(Default, Debug, CompositeTemplate)]
    #[template(resource = "/com/github/flxzt/rnote/ui/settingswindow.ui")]
    pub(crate) struct RnSettingsWindow {
        pub(crate) temporary_format: RefCell<Format>,
        pub(crate) app_restart_toast_singleton: RefCell<Option<adw::Toast>>,

        #[template_child]
        pub(crate) general_autosave_interval_secs_row: TemplateChild<adw::SpinRow>,
        #[template_child]
        pub(crate) general_show_scrollbars_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(crate) general_inertial_scrolling_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(crate) general_regular_cursor_picker: TemplateChild<RnIconPicker>,
        #[template_child]
        pub(crate) general_regular_cursor_picker_menubutton: TemplateChild<MenuButton>,
        #[template_child]
        pub(crate) general_show_drawing_cursor_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(crate) general_drawing_cursor_picker_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(crate) general_drawing_cursor_picker: TemplateChild<RnIconPicker>,
        #[template_child]
        pub(crate) general_drawing_cursor_picker_menubutton: TemplateChild<MenuButton>,
        #[template_child]
        pub(crate) format_predefined_formats_row: TemplateChild<adw::ComboRow>,
        #[template_child]
        pub(crate) format_orientation_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(crate) format_orientation_portrait_toggle: TemplateChild<ToggleButton>,
        #[template_child]
        pub(crate) format_orientation_landscape_toggle: TemplateChild<ToggleButton>,
        #[template_child]
        pub(crate) format_width_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(crate) format_width_unitentry: TemplateChild<RnUnitEntry>,
        #[template_child]
        pub(crate) format_height_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub(crate) format_height_unitentry: TemplateChild<RnUnitEntry>,
        #[template_child]
        pub(crate) format_dpi_row: TemplateChild<adw::SpinRow>,
        #[template_child]
        pub(crate) format_dpi_adj: TemplateChild<Adjustment>,
        #[template_child]
        pub(crate) format_revert_button: TemplateChild<Button>,
        #[template_child]
        pub(crate) format_apply_button: TemplateChild<Button>,
        #[template_child]
        pub(crate) doc_format_border_color_button: TemplateChild<ColorDialogButton>,
        #[template_child]
        pub(crate) doc_background_color_button: TemplateChild<ColorDialogButton>,
        #[template_child]
        pub(crate) doc_background_patterns_row: TemplateChild<adw::ComboRow>,
        #[template_child]
        pub(crate) doc_background_pattern_color_button: TemplateChild<ColorDialogButton>,
        #[template_child]
        pub(crate) doc_background_pattern_width_unitentry: TemplateChild<RnUnitEntry>,
        #[template_child]
        pub(crate) doc_background_pattern_height_unitentry: TemplateChild<RnUnitEntry>,
        #[template_child]
        pub(crate) background_pattern_invert_color_button: TemplateChild<Button>,
    }

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
