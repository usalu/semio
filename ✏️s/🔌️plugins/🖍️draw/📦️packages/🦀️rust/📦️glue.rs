//! ✏️ Draw plugin — declarative draw app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, two levels inside the
//! plugin root — hence every leaf `#[path]` below is prefixed `../../`). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<DrawOperation, DrawConfigOperation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]
// 🎭️ `fsm::statechart!` (used by `apps::draw::commands::canvas`'s gesture machine) generates code
// containing `#[cfg(feature = "serde")]` gates meant for `fsm`'s OWN crate; macro hygiene splices
// that cfg check into the CALLING crate's feature list instead (a `fsm`/rustc macro-expansion
// limitation, not a real conditional-compilation bug here) — this crate declares no `serde` feature
// at all (the dependency is always-on), so rustc flags the value as unrecognized. Harmless, but a
// hard error under `-D warnings` without this crate-wide allow.
#![allow(unexpected_cfgs)]
// 🪶️ Needed by `apps::draw::semio_plugin_bundle_installer_link_shim`'s `#[linkage = "weak"]` —
// satisfies the plugin runtime when this app is linked as its own standalone WASM module. Gated to
// wasm32 only: the attribute using it is itself `#[cfg(target_arch = "wasm32")]`, and an unused
// nightly feature is a hard error under this crate's `-D warnings` gate on native targets.
#![cfg_attr(target_arch = "wasm32", feature(linkage))]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod draw {
        #[path = "../../🗿️artifacts/🖍️draw/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🖍️draw/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🖍️draw/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🖍️draw/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🖍️draw/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🖍️draw/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🖍️draw/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod draw {
        #[path = "../../🎛️apps/🖍️draw/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🖍️draw/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🖍️draw/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🖍️draw/🎮️commands/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🖍️draw/🎮️commands/🗂️layer/🦀️component.rs"]
            pub mod layer;
            #[path = "../../🎛️apps/🖍️draw/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
            #[path = "../../🎛️apps/🖍️draw/🎮️commands/🖱️canvas/🦀️component.rs"]
            pub mod canvas;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🖍️draw/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🖍️draw/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️component.rs"]
                    pub mod canvas;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🖍️draw/📌️panels/🗂️layers/🦀️component.rs"]
            pub mod layers;
            #[path = "../../🎛️apps/🖍️draw/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🖍️draw/📌️panels/🔍️properties/🦀️component.rs"]
            pub mod properties;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
semio_framework_plugin::semio_plugin! {
    id: "draw",
    label: "Draw",
    version: "0.1.0",
    setup: artifacts::draw::engine::register,
    apps: [ apps::draw::create_draw_app => apps::draw::DrawPlayApp ],
}
//#endregion 🔖️Plugin
