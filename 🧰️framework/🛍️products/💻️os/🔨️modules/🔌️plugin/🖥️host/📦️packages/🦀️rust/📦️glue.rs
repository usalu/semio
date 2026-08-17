//! 🖥️ Plugin host — Shape V2 glue.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
#[path = "../../🦀️component.rs"]
mod component;
pub use component::*;

/// 🎚️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §4/§3: `OpeningResolver`
/// (above, in `component.rs`) consumes `OpeningPreferences` by type, so this facet — authored by
/// lane 0-C, left unwired by design ("out of this lease's scope", see its own module doc) — is
/// mounted here, the one crate that actually needs it. WIRING ONLY, mirrors the `✏️s/🔌️plugins/📕️norm`
/// plugin's `config`/`mutations` nesting idiom exactly: `#[path = "."]` on every inline grouping mod
/// so its own name is not spliced into the base directory Rust would otherwise derive from the mod
/// identifier (every real dir here is emoji-named, never matching the plain-ASCII mod identifier).
#[path = "."]
pub mod opening_config {
    #[path = "../../../../../🎚️config/🧬️schema/🦀️component.rs"]
    mod component;
    pub use component::*;

    #[path = "."]
    pub mod mutations {
        #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod set_default_app {
            #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🔺️diff/🦀️component.rs"]
            pub mod diff;
            #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/↩️inverse/🦀️component.rs"]
            pub mod inverse;
            #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🦠️mutation/🦀️component.rs"]
            pub mod mutation;
        }

        #[path = "."]
        pub mod clear_default_app {
            #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🔺️diff/🦀️component.rs"]
            pub mod diff;
            #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/↩️inverse/🦀️component.rs"]
            pub mod inverse;
            #[path = "../../../../../🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🦠️mutation/🦀️component.rs"]
            pub mod mutation;
        }
    }
}
