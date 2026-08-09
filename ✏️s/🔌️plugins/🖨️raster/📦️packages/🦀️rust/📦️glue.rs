//! 🖼️ Raster plugin — declarative raster board bundled as a hot-swappable WASM plugin.
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
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<RasterMutation, RasterConfigMutation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod raster {
        #[path = "../../🗿️artifacts/🖨️raster/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🖨️raster/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🖨️raster/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🖨️raster/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }
        #[path = "../../🗿️artifacts/🖨️raster/🔧️op/🦀️component.rs"]
        pub mod op;

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod add_layer {
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/➕add-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/➕add-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/➕add-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod remove_layer {
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/➖remove-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/➖remove-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/➖remove-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod patch_layer {
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/🩹patch-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/🩹patch-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/🩹patch-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod move_layer {
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/↔️move-layer/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/↔️move-layer/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/↔️move-layer/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/🖼️set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/🖼️set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/🖨️raster/🧬️mutations/🖼️set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }
        }

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/🖨️raster/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/🖨️raster/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "../../🗿️artifacts/🖨️raster/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🖨️raster/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🖨️raster/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod raster {
        #[path = "../../🎛️apps/🖨️raster/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🖨️raster/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🖨️raster/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/🖨️raster/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🖨️raster/🎮️commands/🗂️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🖨️raster/🎮️commands/🖼️layer/🦀️component.rs"]
            pub mod layer;
            #[path = "../../🎛️apps/🖨️raster/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🖨️raster/🎮️commands/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🎛️apps/🖨️raster/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/🖨️raster/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
            #[path = "../../🎛️apps/🖨️raster/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🖨️raster/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod composite {
                        #[path = "../../🎛️apps/🖨️raster/🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🖨️raster/🎭️modes/✏️edit/🪟️windows/🖼️composite/🎚️options/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                            #[path = "../../🎛️apps/🖨️raster/🎭️modes/✏️edit/🪟️windows/🖼️composite/🎚️options/🧽️eraser/🦀️component.rs"]
                            pub mod eraser;
                        }
                    }

                    #[path = "."]
                    pub mod navigator {
                        #[path = "../../🎛️apps/🖨️raster/🎭️modes/✏️edit/🪟️windows/🧭️navigator/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🖨️raster/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🖨️raster/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🖨️raster/📌️panels/🎭️masks/🦀️component.rs"]
            pub mod masks;
            #[path = "../../🎛️apps/🖨️raster/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
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
    #[path = "../../🗿️artifacts/🖨️raster/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_raster_demo;
    #[path = "../../🎛️apps/🖨️raster/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_raster_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
