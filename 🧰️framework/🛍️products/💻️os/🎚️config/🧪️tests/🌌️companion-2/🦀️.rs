//! 🌌️ Companion tests of os-config (2 of 2): configuration schema, mutation and text IO tests.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

pub use semio_framework_os_config::*;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod opening_config {
    pub use semio_framework_os_config::opening_config::*;

    #[path = "."]
    pub mod mutations {
        pub use semio_framework_os_config::opening_config::mutations::*;

        #[path = "."]
        pub mod ui_preferences {
            pub use semio_framework_os_config::opening_config::mutations::ui_preferences::*;

            #[path = "../../🧬️schema/🧬️mutations/🎨️ui-preferences/🧪️tests/🎨️updates-every-os-ui-preference/🦀️.rs"]
            mod tests_updates_every_os_ui_preference;

            #[path = "."]
            pub mod set_appearance {
                pub use semio_framework_os_config::opening_config::mutations::ui_preferences::set_appearance::*;
                use semio_framework_os_config::opening_config::{UiPreferences, UiPreferencesDiff};
                use semio_framework_os_config::opening_config::mutations::UiPreferencesConfigMutation;
                use semio_framework_value_derive::{FromValue, ToValue};

                #[path = "../../🧬️schema/🧬️mutations/🌗️set-appearance/🧪️tests/🟰️keeps-appearance/🦀️.rs"]
                mod tests_keeps_appearance;
            }

            #[path = "."]
            pub mod set_custom_driver {
                pub use semio_framework_os_config::opening_config::mutations::ui_preferences::set_custom_driver::*;
                use semio_framework_os_config::opening_config::{UiPreferences, UiPreferencesDiff};
                use semio_framework_os_config::opening_config::mutations::UiPreferencesConfigMutation;
                use semio_framework_value_derive::{FromValue, ToValue};

                #[path = "../../🧬️schema/🧬️mutations/🚗️set-custom-driver/🧪️tests/✏️sets-custom-driver/🦀️.rs"]
                mod tests_sets_custom_driver;

                #[path = "../../🧬️schema/🧬️mutations/🚗️set-custom-driver/🧪️tests/🟰️keeps-custom-driver/🦀️.rs"]
                mod tests_keeps_custom_driver;
            }

            #[path = "."]
            pub mod set_custom_theme {
                pub use semio_framework_os_config::opening_config::mutations::ui_preferences::set_custom_theme::*;
                use semio_framework_os_config::opening_config::{UiPreferences, UiPreferencesDiff};
                use semio_framework_os_config::opening_config::mutations::UiPreferencesConfigMutation;
                use semio_framework_value_derive::{FromValue, ToValue};

                #[path = "../../🧬️schema/🧬️mutations/🎨️set-custom-theme/🧪️tests/✏️sets-custom-theme/🦀️.rs"]
                mod tests_sets_custom_theme;

                #[path = "../../🧬️schema/🧬️mutations/🎨️set-custom-theme/🧪️tests/🟰️keeps-custom-theme/🦀️.rs"]
                mod tests_keeps_custom_theme;
            }

            #[path = "."]
            pub mod set_driver {
                pub use semio_framework_os_config::opening_config::mutations::ui_preferences::set_driver::*;
                use semio_framework_os_config::opening_config::{UiPreferences, UiPreferencesDiff};
                use semio_framework_os_config::opening_config::mutations::UiPreferencesConfigMutation;
                use semio_framework_value_derive::{FromValue, ToValue};

                #[path = "../../🧬️schema/🧬️mutations/🕹️set-driver/🧪️tests/✏️sets-driver/🦀️.rs"]
                mod tests_sets_driver;

                #[path = "../../🧬️schema/🧬️mutations/🕹️set-driver/🧪️tests/🟰️keeps-driver/🦀️.rs"]
                mod tests_keeps_driver;
            }

            #[path = "."]
            pub mod set_keybinding_override {
                pub use semio_framework_os_config::opening_config::mutations::ui_preferences::set_keybinding_override::*;
                use semio_framework_os_config::opening_config::{UiPreferences, UiPreferencesDiff};
                use semio_framework_os_config::opening_config::mutations::UiPreferencesConfigMutation;
                use semio_framework_value_derive::{FromValue, ToValue};

                #[path = "../../🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧪️tests/✏️sets-keybinding/🦀️.rs"]
                mod tests_sets_keybinding;

                #[path = "../../🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧪️tests/🟰️keeps-keybinding/🦀️.rs"]
                mod tests_keeps_keybinding;
            }

            #[path = "."]
            pub mod set_layout {
                pub use semio_framework_os_config::opening_config::mutations::ui_preferences::set_layout::*;
                use semio_framework_os_config::opening_config::{UiPreferences, UiPreferencesDiff};
                use semio_framework_os_config::opening_config::mutations::UiPreferencesConfigMutation;
                use semio_framework_value_derive::{FromValue, ToValue};

                #[path = "../../🧬️schema/🧬️mutations/📐️set-layout/🧪️tests/✏️sets-layout/🦀️.rs"]
                mod tests_sets_layout;

                #[path = "../../🧬️schema/🧬️mutations/📐️set-layout/🧪️tests/🟰️keeps-layout/🦀️.rs"]
                mod tests_keeps_layout;
            }

            #[path = "."]
            pub mod set_locale {
                pub use semio_framework_os_config::opening_config::mutations::ui_preferences::set_locale::*;
                use semio_framework_os_config::opening_config::{UiPreferences, UiPreferencesDiff};
                use semio_framework_os_config::opening_config::mutations::UiPreferencesConfigMutation;
                use semio_framework_value_derive::{FromValue, ToValue};

                #[path = "../../🧬️schema/🧬️mutations/🗣️set-locale/🧪️tests/✏️sets-locale/🦀️.rs"]
                mod tests_sets_locale;

                #[path = "../../🧬️schema/🧬️mutations/🗣️set-locale/🧪️tests/🟰️keeps-locale/🦀️.rs"]
                mod tests_keeps_locale;
            }

            #[path = "."]
            pub mod set_named_layout {
                pub use semio_framework_os_config::opening_config::mutations::ui_preferences::set_named_layout::*;
                use semio_framework_os_config::opening_config::{UiPreferences, UiPreferencesDiff};
                use semio_framework_os_config::opening_config::mutations::UiPreferencesConfigMutation;
                use semio_framework_value_derive::{FromValue, ToValue};

                #[path = "../../🧬️schema/🧬️mutations/🗂️set-named-layout/🧪️tests/✏️sets-named-layout/🦀️.rs"]
                mod tests_sets_named_layout;

                #[path = "../../🧬️schema/🧬️mutations/🗂️set-named-layout/🧪️tests/🟰️keeps-named-layout/🦀️.rs"]
                mod tests_keeps_named_layout;
            }

            #[path = "."]
            pub mod set_terminology {
                pub use semio_framework_os_config::opening_config::mutations::ui_preferences::set_terminology::*;
                use semio_framework_os_config::opening_config::{UiPreferences, UiPreferencesDiff};
                use semio_framework_os_config::opening_config::mutations::UiPreferencesConfigMutation;
                use semio_framework_value_derive::{FromValue, ToValue};

                #[path = "../../🧬️schema/🧬️mutations/📖️set-terminology/🧪️tests/✏️sets-terminology/🦀️.rs"]
                mod tests_sets_terminology;

                #[path = "../../🧬️schema/🧬️mutations/📖️set-terminology/🧪️tests/🟰️keeps-terminology/🦀️.rs"]
                mod tests_keeps_terminology;
            }

            #[path = "."]
            pub mod set_theme {
                pub use semio_framework_os_config::opening_config::mutations::ui_preferences::set_theme::*;
                use semio_framework_os_config::opening_config::{UiPreferences, UiPreferencesDiff};
                use semio_framework_os_config::opening_config::mutations::UiPreferencesConfigMutation;
                use semio_framework_value_derive::{FromValue, ToValue};

                #[path = "../../🧬️schema/🧬️mutations/🖼️set-theme/🧪️tests/✏️sets-theme/🦀️.rs"]
                mod tests_sets_theme;

                #[path = "../../🧬️schema/🧬️mutations/🖼️set-theme/🧪️tests/🟰️keeps-theme/🦀️.rs"]
                mod tests_keeps_theme;
            }
        }
    }
}

#[path = "."]
pub mod io {
    pub use semio_framework_os_config::io::*;

    #[path = "../../🚪️io/📝️text/🧪️tests/🧬️representation"]
    pub mod text {
        pub use semio_framework_os_config::io::text::*;

        #[path = "🦀️.rs"]
        mod tests;
    }
}
