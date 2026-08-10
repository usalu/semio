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

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault>`, the exact signature
// `DocumentApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split creates),
// so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod remodel {
        #[path = "../../🗿️artifacts/📸️remodel/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📸️remodel/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/📸️remodel/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📸️remodel/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
            pub use schema::*;
        }
        #[path = "../../🗿️artifacts/📸️remodel/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/📸️remodel/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_streams {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🎞️set-streams/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🎞️set-streams/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🎞️set-streams/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_asset {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🖼️set-asset/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🖼️set-asset/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🖼️set-asset/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_calibration {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📐️set-calibration/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📐️set-calibration/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📐️set-calibration/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_gcps {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📍️set-gcps/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📍️set-gcps/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📍️set-gcps/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_ingest_params {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📥️set-ingest-params/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📥️set-ingest-params/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📥️set-ingest-params/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_feature_params {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🌟️set-feature-params/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🌟️set-feature-params/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🌟️set-feature-params/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_match_params {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🔗️set-match-params/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🔗️set-match-params/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🔗️set-match-params/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_sfm_params {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🧭️set-sfm-params/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🧭️set-sfm-params/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🧭️set-sfm-params/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_dense_params {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🌫️set-dense-params/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🌫️set-dense-params/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🌫️set-dense-params/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_mesh_params {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🎛set-mesh-params/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🎛set-mesh-params/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🎛set-mesh-params/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_motion_params {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🏃️set-motion-params/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🏃️set-motion-params/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🏃️set-motion-params/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_geo_params {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🗺️set-geo-params/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🗺️set-geo-params/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🗺️set-geo-params/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_job {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🏭️set-job/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🏭️set-job/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🏭️set-job/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_sparse {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/✨️set-sparse/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/✨️set-sparse/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/✨️set-sparse/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_dense {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🌧️set-dense/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🌧️set-dense/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🌧️set-dense/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_mesh_result {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📦️set-mesh-result/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📦️set-mesh-result/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📦️set-mesh-result/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_trajectory {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🛤️set-trajectory/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🛤️set-trajectory/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🛤️set-trajectory/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_tracks {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📈️set-tracks/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📈️set-tracks/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/📈️set-tracks/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_geo_products {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🌍️set-geo-products/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🌍️set-geo-products/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/🌍️set-geo-products/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }

            #[path = "."]
            pub mod set_qc {
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/✅️set-qc/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/✅️set-qc/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/📸️remodel/🧬️mutations/✅️set-qc/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/📸️remodel/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/📸️remodel/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }
        #[path = "../../🗿️artifacts/📸️remodel/📡️spr/🦀️component.rs"]
        pub mod spr;

        /// ⚙️ The photogrammetry stack: the app-facing translation layer (`🦀️component.rs`) plus ten
        /// sibling topic files, one per pre-merge subsystem crate. The DAG between them is unchanged —
        /// `images` → `video`/`feature`/`dense`/… → `reconstruction` — it is now expressed by `use`
        /// statements inside one crate instead of ten `Cargo.toml` path dependencies.
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🖊️dwg/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🖊️dwg/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod glb {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🧊️glb/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🧊️glb/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod gltf {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🧊️gltf/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🧊️gltf/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🔣️json/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🔣️json/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod las {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/☁️las/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/☁️las/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod obj {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🧊️obj/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🧊️obj/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod ply {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/☁️ply/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/☁️ply/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/📷️png/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/📷️png/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
            #[path = "."]
            pub mod stl {
                #[path = "."]
                pub mod export {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🟪️stl/📤️export/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
                #[path = "."]
                pub mod import {
                    #[path = "../../🗿️artifacts/📸️remodel/🚪️io/🟪️stl/📥️import/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
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

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/📸️remodel/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/📸️remodel/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/📸️remodel/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/📸️remodel/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
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
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/📸️remodel/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_remodel_demo;
    #[path = "../../🎛️apps/📸️remodel/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_remodel_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
