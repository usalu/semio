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
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<NoteMutation, NoteConfigMutation>, Fault>`, the exact signature `DocumentApp::handle`
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

        #[path = "../../🗿️artifacts/🗒️note/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/🗒️note/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🗒️note/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }
        #[path = "../../🗿️artifacts/🗒️note/🔧️op/🦀️component.rs"]
        pub mod op;

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod set_grid_visible {
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/👁️set-grid-visible/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/👁️set-grid-visible/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_grid_spacing {
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/📏set-grid-spacing/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/📏set-grid-spacing/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_grid_subdivisions {
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🔢set-grid-subdivisions/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🔢set-grid-subdivisions/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_grid_opacity {
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🌫️set-grid-opacity/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🌫️set-grid-opacity/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_snap_enabled {
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🧲set-snap-enabled/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🧲set-snap-enabled/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_snap_grid_spacing {
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/📐set-snap-grid-spacing/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/📐set-snap-grid-spacing/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_pencil_width {
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/✏️set-pencil-width/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/✏️set-pencil-width/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_eraser_radius {
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🧽set-eraser-radius/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🧽set-eraser-radius/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_blocks {
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🧱set-blocks/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🧱set-blocks/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod put_asset {
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/📥put-asset/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/📥put-asset/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod remove_asset {
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🗑️remove-asset/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/🗑️remove-asset/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/🗒️note/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/🗒️note/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/🗒️note/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "../../🗿️artifacts/🗒️note/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/🗒️note/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🗒️note/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🗒️note/🚪️io/🖊️dwg/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🗒️note/🚪️io/🖊️dwg/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod dxf {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🗒️note/🚪️io/🖊️dxf/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🗒️note/🚪️io/🖊️dxf/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🗒️note/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🗒️note/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod pdf {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🗒️note/🚪️io/📄️pdf/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🗒️note/🚪️io/📄️pdf/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🗒️note/🚪️io/📷️png/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🗒️note/🚪️io/📷️png/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/🗒️note/🚪️io/🎨️svg/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/🗒️note/🚪️io/🎨️svg/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
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

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🗒️note/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🗒️note/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🗒️note/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🗒️note/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

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

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🗒️note/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_note_demo;
    #[path = "../../🎛️apps/🗒️note/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_note_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
