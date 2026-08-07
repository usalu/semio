//! 📝️ Note plugin — infinite canvas note board bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to the owner root (this file itself now lives two levels deeper, in
//! `📦️packages/🦀️rust/`, so every path carries a `../../` prefix back out to the owner root). The
//! grouping modules carry `#[path = "."]` so their own names are not spliced into that base
//! directory — without it, Rust resolves an inline module's children under `<file dir>/<inline mod
//! name>/…` and every leaf path dangles. Do not inline any component file back into this one: the
//! taxonomy validator and the `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<NoteOperation, NoteConfigOperation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod note {
        #[path = "../../🗿️artifacts/🗒️note/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🗒️note/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/🗒️note/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/🗒️note/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🗒️note/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/🗒️note/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/🗒️note/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod note {
        #[path = "../../🎛️apps/🗒️note/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/🗒️note/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/🗒️note/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🗒️note/🎮️commands/🔲️grid/🦀️component.rs"]
            pub mod grid;
            #[path = "../../🎛️apps/🗒️note/🎮️commands/🧲️snap/🦀️component.rs"]
            pub mod snap;
            #[path = "../../🎛️apps/🗒️note/🎮️commands/✏️drawing/🦀️component.rs"]
            pub mod drawing;
            #[path = "../../🎛️apps/🗒️note/🎮️commands/🧱️block/🦀️component.rs"]
            pub mod block;
            #[path = "../../🎛️apps/🗒️note/🎮️commands/🕹️nudge/🦀️component.rs"]
            pub mod nudge;
            #[path = "../../🎛️apps/🗒️note/🎮️commands/🖊️ink/🦀️component.rs"]
            pub mod ink;
            #[path = "../../🎛️apps/🗒️note/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/🗒️note/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
            #[path = "../../🎛️apps/🗒️note/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/🗒️note/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🗒️note/🎮️commands/💬️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/🗒️note/🎮️commands/🗃️fixture/🦀️component.rs"]
            pub mod fixture;
            #[path = "../../🎛️apps/🗒️note/🎮️commands/🐚️export/🦀️component.rs"]
            pub mod export;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🗒️note/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod composite {
                        #[path = "../../🎛️apps/🗒️note/🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🗒️note/🎭️modes/✏️edit/🪟️windows/🖼️composite/🎚️options/🎥️camera/🦀️component.rs"]
                            pub mod camera;
                            #[path = "../../🎛️apps/🗒️note/🎭️modes/✏️edit/🪟️windows/🖼️composite/🎚️options/🔲️grid/🦀️component.rs"]
                            pub mod grid;
                            #[path = "../../🎛️apps/🗒️note/🎭️modes/✏️edit/🪟️windows/🖼️composite/🎚️options/🧲️snap/🦀️component.rs"]
                            pub mod snap;
                            #[path = "../../🎛️apps/🗒️note/🎭️modes/✏️edit/🪟️windows/🖼️composite/🎚️options/✏️pencil/🦀️component.rs"]
                            pub mod pencil;
                            #[path = "../../🎛️apps/🗒️note/🎭️modes/✏️edit/🪟️windows/🖼️composite/🎚️options/🧽️eraser-stroke/🦀️component.rs"]
                            pub mod eraser_stroke;
                            #[path = "../../🎛️apps/🗒️note/🎭️modes/✏️edit/🪟️windows/🖼️composite/🎚️options/🧹️eraser-point/🦀️component.rs"]
                            pub mod eraser_point;
                        }
                    }

                    #[path = "."]
                    pub mod navigator {
                        #[path = "../../🎛️apps/🗒️note/🎭️modes/✏️edit/🪟️windows/🧭️navigator/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🗒️note/🎭️modes/✏️edit/🪟️windows/🧭️navigator/🎚️options/🔍️zoom/🦀️component.rs"]
                            pub mod zoom;
                            #[path = "../../🎛️apps/🗒️note/🎭️modes/✏️edit/🪟️windows/🧭️navigator/🎚️options/🔲️grid-visible/🦀️component.rs"]
                            pub mod grid_visible;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🗒️note/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🗒️note/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🗒️note/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);
//#endregion 🔖️Plugin
