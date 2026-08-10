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

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "."]
                pub mod engine {
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🦀️component.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🌟️feature/🦀️component.rs"]
                    pub mod feature;
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🌫️dense/🦀️component.rs"]
                    pub mod dense;
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️component.rs"]
                    pub mod video;
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🏃️motion/🦀️component.rs"]
                    pub mod motion;
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🏭️reconstruction/🦀️component.rs"]
                    pub mod reconstruction;
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/📷️camera/🦀️component.rs"]
                    pub mod camera;
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/📸️sfm/🦀️component.rs"]
                    pub mod sfm;
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🖼️images/🦀️component.rs"]
                    pub mod images;
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🗺️geo/🦀️component.rs"]
                    pub mod geo;
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/⚙️engine/🥽️mesh/🦀️component.rs"]
                    pub mod mesh;
                }
                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🏗️builder/🦀️component.rs"]
                pub mod builder;
                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🧐️analyzer/🦀️component.rs"]
                pub mod analyzer;
                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🎹️composer/🦀️component.rs"]
                pub mod composer;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod set_qc {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️set-qc/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️set-qc/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✅️set-qc/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_sparse {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✨️set-sparse/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✨️set-sparse/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✨️set-sparse/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_geo_products {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️set-geo-products/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️set-geo-products/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍️set-geo-products/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_feature_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌟️set-feature-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌟️set-feature-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌟️set-feature-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_dense {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌧️set-dense/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌧️set-dense/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌧️set-dense/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_dense_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️set-dense-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️set-dense-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️set-dense-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_mesh_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-mesh-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-mesh-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛set-mesh-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_streams {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-streams/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-streams/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎞️set-streams/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_motion_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️set-motion-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️set-motion-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏃️set-motion-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_job {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️set-job/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️set-job/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏭️set-job/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_tracks {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-tracks/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-tracks/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📈️set-tracks/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_gcps {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️set-gcps/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️set-gcps/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️set-gcps/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_calibration {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-calibration/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-calibration/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-calibration/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_ingest_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️set-ingest-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️set-ingest-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️set-ingest-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_mesh_result {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-mesh-result/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-mesh-result/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️set-mesh-result/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_match_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️set-match-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️set-match-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️set-match-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_asset {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-asset/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-asset/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️set-asset/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_geo_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-geo-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-geo-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗺️set-geo-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_trajectory {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️set-trajectory/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️set-trajectory/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛤️set-trajectory/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod set_sfm_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-sfm-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-sfm-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️set-sfm-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🏗️builder/🦀️component.rs"]
                        pub mod builder;
                        #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧐️analyzer/🦀️component.rs"]
                        pub mod analyzer;
                        #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🎹️composer/🦀️component.rs"]
                        pub mod composer;
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod glb {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️glb/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod glb {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️glb/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod engine {
            pub use super::standards::v1::engine::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::diff::*; pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::diff::text::*; pub mod schema { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::snapshot::binary::*; } }

        #[path = "../../🗿️artifacts/📸️remodel/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/📸️remodel/🧐️analyzer/🦀️component.rs"]
        pub mod analyzer;
        #[path = "../../🗿️artifacts/📸️remodel/🎹️composer/🦀️component.rs"]
        pub mod composer;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📸️remodel/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
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
#[path = "../../🦀️component.rs"]
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
