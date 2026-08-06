//! 📸️ Remodel plugin — the photogrammetry/videogrammetry play app (video in → watertight mesh out)
//! bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory. Shape V2 (`26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST`)
//! puts this entry file inside `📦️packages/🦀️rust/` — two levels below the plugin root — so every leaf
//! path opens with `../../` to reach back out to the component tree. The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<RemodelOperation, RemodelConfigOperation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split creates),
// so this is a pure artefact of decomposition.
#![allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod remodel {
        #[path = "../../🗿️artifacts/📸️remodel/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📸️remodel/🔺️diff/🦀️component.rs"]
        pub mod diff;
        #[path = "../../🗿️artifacts/📸️remodel/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📸️remodel/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/📸️remodel/🎒️pack/🦀️component.rs"]
        pub mod pack;
        #[path = "../../🗿️artifacts/📸️remodel/📡️spr/🦀️component.rs"]
        pub mod spr;

        /// ⚙️ The photogrammetry stack: the app-facing translation layer (`🦀️component.rs`) plus ten
        /// sibling topic files, one per pre-merge subsystem crate. The DAG between them is unchanged —
        /// `images` → `video`/`feature`/`dense`/… → `reconstruction` — it is now expressed by `use`
        /// statements inside one crate instead of ten `Cargo.toml` path dependencies.
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/📸️remodel/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📸️remodel/⚙️engine/📷️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🗿️artifacts/📸️remodel/⚙️engine/🌫️dense/🦀️component.rs"]
            pub mod dense;
            #[path = "../../🗿️artifacts/📸️remodel/⚙️engine/🌟️feature/🦀️component.rs"]
            pub mod feature;
            #[path = "../../🗿️artifacts/📸️remodel/⚙️engine/🗺️geo/🦀️component.rs"]
            pub mod geo;
            #[path = "../../🗿️artifacts/📸️remodel/⚙️engine/🖼️images/🦀️component.rs"]
            pub mod images;
            #[path = "../../🗿️artifacts/📸️remodel/⚙️engine/🥽️mesh/🦀️component.rs"]
            pub mod mesh;
            #[path = "../../🗿️artifacts/📸️remodel/⚙️engine/🏃️motion/🦀️component.rs"]
            pub mod motion;
            #[path = "../../🗿️artifacts/📸️remodel/⚙️engine/🏭️reconstruction/🦀️component.rs"]
            pub mod reconstruction;
            #[path = "../../🗿️artifacts/📸️remodel/⚙️engine/📸️sfm/🦀️component.rs"]
            pub mod sfm;
            #[path = "../../🗿️artifacts/📸️remodel/⚙️engine/🎥️video/🦀️component.rs"]
            pub mod video;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod remodel {
        #[path = "../../🎛️apps/📸️remodel/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🎛️apps/📸️remodel/🎚️config/🦀️component.rs"]
        pub mod config;
        #[path = "../../🎛️apps/📸️remodel/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/📸️remodel/🎮️commands/🎯️calibration/🦀️component.rs"]
            pub mod calibration;
            #[path = "../../🎛️apps/📸️remodel/🎮️commands/📥️ingest/🦀️component.rs"]
            pub mod ingest;
            #[path = "../../🎛️apps/📸️remodel/🎮️commands/⚙️params/🦀️component.rs"]
            pub mod params;
            #[path = "../../🎛️apps/📸️remodel/🎮️commands/🚀️reconstruction/🦀️component.rs"]
            pub mod reconstruction;
            #[path = "../../🎛️apps/📸️remodel/🎮️commands/🧹️reset/🦀️component.rs"]
            pub mod reset;
            #[path = "../../🎛️apps/📸️remodel/🎮️commands/🐚️shell/🦀️component.rs"]
            pub mod shell;
            #[path = "../../🎛️apps/📸️remodel/🎮️commands/👁️view/🦀️component.rs"]
            pub mod view;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod model {
                #[path = "../../🎛️apps/📸️remodel/🎭️modes/🧊️model/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod model {
                        #[path = "../../🎛️apps/📸️remodel/🎭️modes/🧊️model/🪟️windows/🧊️model/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/📸️remodel/🎭️modes/🧊️model/🪟️windows/🧊️model/🎚️options/👁️layers/🦀️component.rs"]
                            pub mod layers;
                        }
                    }
                }
            }

            #[path = "."]
            pub mod capture {
                #[path = "../../🎛️apps/📸️remodel/🎭️modes/📷️capture/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📸️remodel/🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️component.rs"]
                    pub mod frames;
                }
            }

            #[path = "."]
            pub mod analyze {
                #[path = "../../🎛️apps/📸️remodel/🎭️modes/🔍️analyze/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/📸️remodel/🎭️modes/🔍️analyze/🪟️windows/📊️report/🦀️component.rs"]
                    pub mod report;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/📸️remodel/📌️panels/🎯️calibration/🦀️component.rs"]
            pub mod calibration;
            #[path = "../../🎛️apps/📸️remodel/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/📸️remodel/📌️panels/🗂️media/🦀️component.rs"]
            pub mod media;
            #[path = "../../🎛️apps/📸️remodel/📌️panels/⚙️parameters/🦀️component.rs"]
            pub mod parameters;
            #[path = "../../🎛️apps/📸️remodel/📌️panels/✅️quality/🦀️component.rs"]
            pub mod quality;
            #[path = "../../🎛️apps/📸️remodel/📌️panels/🧵️results/🦀️component.rs"]
            pub mod results;
            #[path = "../../🎛️apps/📸️remodel/📌️panels/🏃️tracks/🦀️component.rs"]
            pub mod tracks;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
semio_framework_plugin::semio_plugin! {
    id: "remodel",
    label: "Remodel",
    version: "0.1.0",
    setup: artifacts::remodel::engine::register,
    apps: [ apps::remodel::create_remodel_app => apps::remodel::RemodelPlayApp ],
}
//#endregion 🔖️Plugin
