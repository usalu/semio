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
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault>`, the exact signature
// `ArtifactApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split creates),
// so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
//#region 🧮️MathInternals
// 🧮️ 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d: crate-root
// aliases onto the compute-internals mounted below in `artifacts::remodeling::…::schema` — every
// `crate::algebra::`/`crate::optimize::`/`crate::lie::`/`crate::signal::`/`crate::spatial::` call
// site (the moved files' own internal references, and the app-engine files that used to say
// `math::algebra::` etc.) resolves through these, exactly as the old `math::` extern-prelude
// name used to. `semio-framework-math` is no longer a dependency of this crate.
pub(crate) use artifacts::remodeling::standards::v1::subsets::any::schema::algebra_internals as algebra;
pub(crate) use artifacts::remodeling::standards::v1::subsets::any::schema::lie_internals as lie;
pub(crate) use artifacts::remodeling::standards::v1::subsets::any::schema::optimize_internals as optimize;
pub(crate) use artifacts::remodeling::standards::v1::subsets::any::schema::signal_internals as signal;
pub(crate) use artifacts::remodeling::standards::v1::subsets::any::schema::spatial_internals as spatial;
//#endregion 🧮️MathInternals

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod remodeling {
        #[path = "../../🗿️artifacts/📸️remodeling/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            // 🧮️ 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d:
                            // Rust-only compute-internals, mirroring the `✳️table/🧬️schema/📋️tabular-internals`
                            // and `✳️brep/🧬️schema/⚙️engine` precedent — moved wholesale from `🧮️math`, sole
                            // repo-wide consumer verified to be this crate. Crate-root aliases (`crate::algebra`,
                            // `crate::optimize`, `crate::lie`, `crate::signal`, `crate::spatial`, below in this
                            // file) let the moved files' own `crate::algebra::` references and the app-engine
                            // consumer files (which used to say `math::algebra::`) resolve unchanged.
                            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/➕️algebra-internals/🦀️.rs"]
                            pub mod algebra_internals;
                            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔷️lie-internals/🦀️.rs"]
                            pub mod lie_internals;
                            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎯️optimize-internals/🦀️.rs"]
                            pub mod optimize_internals;
                            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📶️signal-internals/🦀️.rs"]
                            pub mod signal_internals;
                            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗺️spatial-internals/🦀️.rs"]
                            pub mod spatial_internals;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod relative_pose {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔄relative-pose/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_stream {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/adds-stream-c-bound-to-cam-b/🦀️.rs"]
                                    mod tests_adds_stream_c_bound_to_cam_b;
                                }
                                #[path = "."]
                                pub mod delete_stream {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/removes-stream-b-and-cascades-its-gcp-observation/🦀️.rs"]
                                    mod tests_removes_stream_b_and_cascades_its_gcp_observation;
                                }
                                #[path = "."]
                                pub mod change_stream_sync {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/shifts-stream-a-sync-offset-to-minus-seven-and-a-half/🦀️.rs"]
                                    mod tests_shifts_stream_a_sync_offset_to_minus_seven_and_a_half;
                                }
                                #[path = "."]
                                pub mod add_stream_frame {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/appends-a-third-frame-to-stream-a/🦀️.rs"]
                                    mod tests_appends_a_third_frame_to_stream_a;
                                }
                                #[path = "."]
                                pub mod remove_stream_frame {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/removes-the-last-frame-of-stream-a/🦀️.rs"]
                                    mod tests_removes_the_last_frame_of_stream_a;
                                }
                                #[path = "."]
                                pub mod replace_stream_source {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/clears-the-video-source-of-stream-a/🦀️.rs"]
                                    mod tests_clears_the_video_source_of_stream_a;
                                }
                                #[path = "."]
                                pub mod create_asset {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/stores-a-new-jpeg-frame-asset/🦀️.rs"]
                                    mod tests_stores_a_new_jpeg_frame_asset;
                                }
                                #[path = "."]
                                pub mod delete_asset {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/removes-asset-a-and-reports-its-stale-references/🦀️.rs"]
                                    mod tests_removes_asset_a_and_reports_its_stale_references;
                                }
                                #[path = "."]
                                pub mod create_camera_calibration {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🧪️tests/adds-the-cam-c-fisheye-calibration/🦀️.rs"]
                                    mod tests_adds_the_cam_c_fisheye_calibration;
                                }
                                #[path = "."]
                                pub mod update_camera_calibration {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/refines-the-cam-a-focal-length-and-rms/🦀️.rs"]
                                    mod tests_refines_the_cam_a_focal_length_and_rms;
                                }
                                #[path = "."]
                                pub mod delete_camera_calibration {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/removes-the-cam-b-calibration/🦀️.rs"]
                                    mod tests_removes_the_cam_b_calibration;
                                }
                                #[path = "."]
                                pub mod create_rig_extrinsic {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/adds-a-rig-extrinsic-for-cam-b/🦀️.rs"]
                                    mod tests_adds_a_rig_extrinsic_for_cam_b;
                                }
                                #[path = "."]
                                pub mod delete_rig_extrinsic {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/drops-the-cam-a-rig-extrinsic/🦀️.rs"]
                                    mod tests_drops_the_cam_a_rig_extrinsic;
                                }
                                #[path = "."]
                                pub mod update_rig_extrinsic {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/retunes-the-cam-a-rig-translation/🦀️.rs"]
                                    mod tests_retunes_the_cam_a_rig_translation;
                                }
                                #[path = "."]
                                pub mod create_gcp {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/adds-gcp-tower-with-one-observation/🦀️.rs"]
                                    mod tests_adds_gcp_tower_with_one_observation;
                                }
                                #[path = "."]
                                pub mod delete_gcp {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/removes-gcp-corner-and-cascades-its-observation/🦀️.rs"]
                                    mod tests_removes_gcp_corner_and_cascades_its_observation;
                                }
                                #[path = "."]
                                pub mod add_gcp_observation {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/adds-the-first-observation-to-gcp-ridge/🦀️.rs"]
                                    mod tests_adds_the_first_observation_to_gcp_ridge;
                                }
                                #[path = "."]
                                pub mod remove_gcp_observation {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/removes-the-only-observation-of-gcp-corner/🦀️.rs"]
                                    mod tests_removes_the_only_observation_of_gcp_corner;
                                }
                                #[path = "."]
                                pub mod update_ingest_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/tightens-the-ingest-sharpness-gate/🦀️.rs"]
                                    mod tests_tightens_the_ingest_sharpness_gate;
                                }
                                #[path = "."]
                                pub mod update_feature_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/switches-the-detector-to-akaze/🦀️.rs"]
                                    mod tests_switches_the_detector_to_akaze;
                                }
                                #[path = "."]
                                pub mod update_match_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/switches-the-matcher-to-a-kd-tree/🦀️.rs"]
                                    mod tests_switches_the_matcher_to_a_kd_tree;
                                }
                                #[path = "."]
                                pub mod update_sfm_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🧪️tests/switches-the-robust-loss-to-cauchy/🦀️.rs"]
                                    mod tests_switches_the_robust_loss_to_cauchy;
                                }
                                #[path = "."]
                                pub mod update_dense_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🧪️tests/raises-the-dense-resolution-and-confidence-gate/🦀️.rs"]
                                    mod tests_raises_the_dense_resolution_and_confidence_gate;
                                }
                                #[path = "."]
                                pub mod update_mesh_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🧪️tests/doubles-the-texture-size-and-drops-the-watertight-guarantee/🦀️.rs"]
                                    mod tests_doubles_the_texture_size_and_drops_the_watertight_guarantee;
                                }
                                #[path = "."]
                                pub mod update_motion_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🧪️tests/enables-motion-tracking/🦀️.rs"]
                                    mod tests_enables_motion_tracking;
                                }
                                #[path = "."]
                                pub mod update_geo_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/enables-georeferencing-with-an-origin/🦀️.rs"]
                                    mod tests_enables_georeferencing_with_an_origin;
                                }
                                #[path = "."]
                                pub mod replace_job {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/advances-the-job-to-texturing/🦀️.rs"]
                                    mod tests_advances_the_job_to_texturing;
                                }
                                #[path = "."]
                                pub mod commit_reconstruction {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod replace_sparse {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🧪️tests/swaps-in-an-uncolored-four-point-sparse-cloud/🦀️.rs"]
                                    mod tests_swaps_in_an_uncolored_four_point_sparse_cloud;
                                }
                                #[path = "."]
                                pub mod replace_dense {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🧪️tests/swaps-in-a-two-point-classified-dense-cloud/🦀️.rs"]
                                    mod tests_swaps_in_a_two_point_classified_dense_cloud;
                                }
                                #[path = "."]
                                pub mod replace_mesh_result {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/swaps-in-an-imported-untextured-mesh/🦀️.rs"]
                                    mod tests_swaps_in_an_imported_untextured_mesh;
                                }
                                #[path = "."]
                                pub mod replace_trajectory {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/clears-the-camera-trajectory/🦀️.rs"]
                                    mod tests_clears_the_camera_trajectory;
                                }
                                #[path = "."]
                                pub mod replace_tracks {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/replaces-the-moving-track-with-two-static-tracks/🦀️.rs"]
                                    mod tests_replaces_the_moving_track_with_two_static_tracks;
                                }
                                #[path = "."]
                                pub mod replace_geo_products {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/adds-the-dtm-and-ortho-rasters/🦀️.rs"]
                                    mod tests_adds_the_dtm_and_ortho_rasters;
                                }
                                #[path = "."]
                                pub mod replace_qc {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/records-a-qc-report-carrying-a-watertight-summary/🦀️.rs"]
                                    mod tests_records_a_qc_report_carrying_a_watertight_summary;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
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
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::artifacts::remodeling::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::remodeling::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::remodeling::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::remodeling::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::remodeling::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::remodeling::standards::v1::subsets::any::schema::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::remodeling::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::artifacts::remodeling::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::artifacts::remodeling::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
                mod tests;
            }
        }
    }
}
//#endregion 🗿️Artifacts

//#region ✏️Editor
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod remodeling {
        #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📷️camera/🦀️.rs"]
            pub mod camera;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌫️dense/🦀️.rs"]
            pub mod dense;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌟️feature/🦀️.rs"]
            pub mod feature;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗺️geo/🦀️.rs"]
            pub mod geo;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖼️images/🦀️.rs"]
            pub mod images;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🥽️mesh/🦀️.rs"]
            pub mod mesh;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️motion/🦀️.rs"]
            pub mod motion;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🦀️.rs"]
            pub mod reconstruction;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/🦀️.rs"]
            pub mod sfm;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️.rs"]
            pub mod video;
        }

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️add-gcp/🦀️.rs"]
            pub mod add_gcp;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️add-stream/🦀️.rs"]
            pub mod add_stream;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️advance-reconstruction/🦀️.rs"]
            pub mod advance_reconstruction;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️calibrate-cameras/🦀️.rs"]
            pub mod calibrate_cameras;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛑️cancel-reconstruction/🦀️.rs"]
            pub mod cancel_reconstruction;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-dense/🦀️.rs"]
            pub mod clear_dense;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-geo-products/🦀️.rs"]
            pub mod clear_geo_products;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-mesh-result/🦀️.rs"]
            pub mod clear_mesh_result;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-result/🦀️.rs"]
            pub mod clear_result;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-sparse/🦀️.rs"]
            pub mod clear_sparse;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-tracks/🦀️.rs"]
            pub mod clear_tracks;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️edit-calibration/🦀️.rs"]
            pub mod edit_calibration;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️export-qc-report/🦀️.rs"]
            pub mod export_qc_report;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-frame-payload/🦀️.rs"]
            pub mod import_frame_payload;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️import-frames/🦀️.rs"]
            pub mod import_frames;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️import-video/🦀️.rs"]
            pub mod import_video;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-video-bytes-payload/🦀️.rs"]
            pub mod import_video_bytes_payload;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-video-done/🦀️.rs"]
            pub mod import_video_done;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-video-frame-payload/🦀️.rs"]
            pub mod import_video_frame_payload;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️place-gcp-observation/🦀️.rs"]
            pub mod place_gcp_observation;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️remove-gcp/🦀️.rs"]
            pub mod remove_gcp;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️remove-stream/🦀️.rs"]
            pub mod remove_stream;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️reset-placeholder-mesh/🦀️.rs"]
            pub mod reset_placeholder_mesh;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️retry-stage/🦀️.rs"]
            pub mod retry_stage;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️.rs"]
            pub mod run_reconstruction;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-stage/🦀️.rs"]
            pub mod run_stage;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-active-utility/🦀️.rs"]
            pub mod set_active_utility;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-dense-params/🦀️.rs"]
            pub mod set_dense_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-feature-params/🦀️.rs"]
            pub mod set_feature_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-frame-cursor/🦀️.rs"]
            pub mod set_frame_cursor;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-geo-params/🦀️.rs"]
            pub mod set_geo_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-ingest-params/🦀️.rs"]
            pub mod set_ingest_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-layer-visibility/🦀️.rs"]
            pub mod set_layer_visibility;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-match-params/🦀️.rs"]
            pub mod set_match_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-mesh-params/🦀️.rs"]
            pub mod set_mesh_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-motion-params/🦀️.rs"]
            pub mod set_motion_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-report-table/🦀️.rs"]
            pub mod set_report_table;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-sfm-params/🦀️.rs"]
            pub mod set_sfm_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️set-stream-sync/🦀️.rs"]
            pub mod set_stream_sync;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod model {
                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod model {
                        #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🎚️options/👁️layers/🦀️.rs"]
                            pub mod layers;
                        }
                    }
                }
            }

            #[path = "."]
            pub mod capture {
                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📷️capture/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️.rs"]
                    pub mod frames;
                }
            }

            #[path = "."]
            pub mod analyze {
                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔍️analyze/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🦀️.rs"]
                    pub mod report;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🎯️calibration/🦀️.rs"]
            pub mod calibration;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗂️media/🦀️.rs"]
            pub mod media;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/⚙️parameters/🦀️.rs"]
            pub mod parameters;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/✅️quality/🦀️.rs"]
            pub mod quality;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🧵️results/🦀️.rs"]
            pub mod results;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🏃️tracks/🦀️.rs"]
            pub mod tracks;
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod remodeling {
        #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod model {
                        #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
pub use plugin::RemodelApps;
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::RemodelApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_remodeling_demo_session;
    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_remodeling_demo;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
