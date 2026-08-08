//! 📜️ Imperative plugin — declarative imperative play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to the owner root (this file itself now lives two levels deeper, in
//! `📦️packages/🦀️rust/`, so every path carries a `../../` prefix back out to the owner root). The
//! grouping modules carry `#[path = "."]` so their own names are not spliced into that base
//! directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<ImperativeMutation, ImperativeConfigMutation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned
// error type; boxing it here would diverge from the trait it must satisfy, and the lint does not fire on
// the trait impl itself (only on the free functions the taxonomy split creates), so this is a pure
// artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod imperative {
        #[path = "../../🗿️artifacts/📜️imperative/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📜️imperative/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📜️imperative/🔧️op/🦀️component.rs"]
        pub mod op;

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📜️imperative/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod step_collection {
                #[path = "../../🗿️artifacts/📜️imperative/🧬️mutations/✂️step-collection/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📜️imperative/🧬️mutations/✂️step-collection/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/📜️imperative/🧬️mutations/✂️step-collection/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
        }

        #[path = "../../🗿️artifacts/📜️imperative/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📜️imperative/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📜️imperative/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/📜️imperative/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod imperative {
        #[path = "../../🎛️apps/📜️imperative/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/📜️imperative/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/📜️imperative/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/📜️imperative/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📜️imperative/🎮️commands/🔧️step/🦀️component.rs"]
            pub mod step;
            #[path = "../../🎛️apps/📜️imperative/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/📜️imperative/🎮️commands/🧩️contribution/🦀️component.rs"]
            pub mod contribution;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/📜️imperative/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📜️imperative/🎭️modes/✏️edit/🪟️windows/📋️main/🦀️component.rs"]
                    pub mod main;
                    #[path = "../../🎛️apps/📜️imperative/🎭️modes/✏️edit/🪟️windows/📝️script/🦀️component.rs"]
                    pub mod script;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📜️imperative/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📜️imperative/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/📜️imperative/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion 🎛️Apps

//#region 🕸️Wasm
#[cfg(target_arch = "wasm32")]
pub use apps::imperative::wasm::ImperativeSession;
//#endregion 🕸️Wasm

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/📜️imperative/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_imperative_demo;
    #[path = "../../🎛️apps/📜️imperative/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_imperative_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
