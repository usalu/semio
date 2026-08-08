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

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
#![cfg_attr(target_arch = "wasm32", feature(linkage))]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<DrawMutation, DrawConfigMutation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
// 🎭️ `fsm::statechart!` (used by `apps::draw::commands::canvas`'s gesture machine) generates code
// containing `#[cfg(feature = "serde")]` gates meant for `fsm`'s OWN crate; macro hygiene splices
// that cfg check into the CALLING crate's feature list instead (a `fsm`/rustc macro-expansion
// limitation, not a real conditional-compilation bug here) — this crate declares no `serde` feature
// at all (the dependency is always-on), so rustc flags the value as unrecognized. Harmless, but a
// hard error under `-D warnings` without this crate-wide allow.
// 🪶️ Needed by `apps::draw::semio_plugin_bundle_installer_link_shim`'s `#[linkage = "weak"]` —
// satisfies the plugin runtime when this app is linked as its own standalone WASM module. Gated to
// wasm32 only: the attribute using it is itself `#[cfg(target_arch = "wasm32")]`, and an unused
// nightly feature is a hard error under this crate's `-D warnings` gate on native targets.

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
        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_layer_visible {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/👁️set-layer-visible/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/👁️set-layer-visible/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/👁️set-layer-visible/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_layer_locked {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🔒️set-layer-locked/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🔒️set-layer-locked/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🔒️set-layer-locked/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_layer_opacity {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🌫️set-layer-opacity/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🌫️set-layer-opacity/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🌫️set-layer-opacity/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_layer_blend_mode {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🖌️set-layer-blend-mode/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🖌️set-layer-blend-mode/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🖌️set-layer-blend-mode/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_layer_name {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🏷️set-layer-name/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🏷️set-layer-name/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🏷️set-layer-name/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_layer_transform {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/↔️set-layer-transform/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/↔️set-layer-transform/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/↔️set-layer-transform/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_fill {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🎨set-fill/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🎨set-fill/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🎨set-fill/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_stroke {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/✏️set-stroke/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/✏️set-stroke/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/✏️set-stroke/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_boolean_operation {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🔀set-boolean-operation/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🔀set-boolean-operation/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🔀set-boolean-operation/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_trace_params {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🖼️set-trace-params/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🖼️set-trace-params/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🖼️set-trace-params/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod add_layer {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/➕️add-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/➕️add-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/➕️add-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod duplicate_layer {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🧬️duplicate-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🧬️duplicate-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🧬️duplicate-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod remove_layer {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/➖️remove-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/➖️remove-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/➖️remove-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod reorder_layer {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🔃reorder-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🔃reorder-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/🔃reorder-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_document {
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/📄set-document/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/📄set-document/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/🖍️draw/🧬️mutations/📄set-document/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

        }

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
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🖍️draw/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_draw_demo;
    #[path = "../../🎛️apps/🖍️draw/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_draw_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
