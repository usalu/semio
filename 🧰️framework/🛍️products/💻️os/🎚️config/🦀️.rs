//! 🎚️ Canonical OS configuration schema and mutation contracts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod opening_config {
    #[path = "🧬️schema/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "."]
    pub mod mutations {
        #[path = "🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "🧬️schema/🧬️mutations/🛡️change-merge-policy/🦀️.rs"]
        pub mod change_merge_policy;
        #[path = "🧬️schema/🧬️mutations/🧹clear-default-app/🦀️.rs"]
        pub mod clear_default_app;
        #[path = "🧬️schema/🧬️mutations/📌️set-default-app/🦀️.rs"]
        pub mod set_default_app;
        #[path = "🧬️schema/🧬️mutations/🪪️sign-in/🦀️.rs"]
        pub mod sign_in;
        #[path = "🧬️schema/🧬️mutations/🚪️sign-out/🦀️.rs"]
        pub mod sign_out;
        #[path = "🧬️schema/🧬️mutations/🎨️ui-preferences/🦀️.rs"]
        pub mod ui_preferences;
    }
}

pub use opening_config::*;
