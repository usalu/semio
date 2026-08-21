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
// `Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault>`, the exact signature
// `ArtifactApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split creates),
// so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

//#region 🧮️MathInternals
// 🧮️ 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d: crate-root
// aliases onto the compute-internals mounted below in `artifacts::remodel::…::schema` — every
// `crate::algebra::`/`crate::optimize::`/`crate::lie::`/`crate::signal::`/`crate::spatial::` call
// site (the moved files' own internal references, and the app-engine files that used to say
// `math::algebra::` etc.) resolves through these, exactly as the old `math::` extern-prelude
// name used to. `semio-framework-math` is no longer a dependency of this crate.
pub(crate) use artifacts::remodel::standards::v1::subsets::any::schema::algebra_internals as algebra;
pub(crate) use artifacts::remodel::standards::v1::subsets::any::schema::optimize_internals as optimize;
pub(crate) use artifacts::remodel::standards::v1::subsets::any::schema::lie_internals as lie;
pub(crate) use artifacts::remodel::standards::v1::subsets::any::schema::signal_internals as signal;
pub(crate) use artifacts::remodel::standards::v1::subsets::any::schema::spatial_internals as spatial;
//#endregion 🧮️MathInternals

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
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            // 🧮️ 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d:
                            // Rust-only compute-internals, mirroring the `✳️table/🧬️schema/📋️tabular-internals`
                            // and `✳️brep/🧬️schema/⚙️engine` precedent — moved wholesale from `🧮️math`, sole
                            // repo-wide consumer verified to be this crate. Crate-root aliases (`crate::algebra`,
                            // `crate::optimize`, `crate::lie`, `crate::signal`, `crate::spatial`, below in this
                            // file) let the moved files' own `crate::algebra::` references and the app-engine
                            // consumer files (which used to say `math::algebra::`) resolve unchanged.
                            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/➕️algebra-internals/🦀️component.rs"]
                            pub mod algebra_internals;
                            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎯️optimize-internals/🦀️component.rs"]
                            pub mod optimize_internals;
                            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔷️lie-internals/🦀️component.rs"]
                            pub mod lie_internals;
                            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📶️signal-internals/🦀️component.rs"]
                            pub mod signal_internals;
                            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗺️spatial-internals/🦀️component.rs"]
                            pub mod spatial_internals;
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
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod relative_pose {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔄relative-pose/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
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
                                pub mod create_stream {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/adds-stream-c-bound-to-cam-b/🦀️component.rs"]
                                    mod tests_adds_stream_c_bound_to_cam_b;
                                }
                                #[path = "."]
                                pub mod delete_stream {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/removes-stream-b-and-cascades-its-gcp-observation/🦀️component.rs"]
                                    mod tests_removes_stream_b_and_cascades_its_gcp_observation;
                                }
                                #[path = "."]
                                pub mod change_stream_sync {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/shifts-stream-a-sync-offset-to-minus-seven-and-a-half/🦀️component.rs"]
                                    mod tests_shifts_stream_a_sync_offset_to_minus_seven_and_a_half;
                                }
                                #[path = "."]
                                pub mod add_stream_frame {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/appends-a-third-frame-to-stream-a/🦀️component.rs"]
                                    mod tests_appends_a_third_frame_to_stream_a;
                                }
                                #[path = "."]
                                pub mod remove_stream_frame {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/removes-the-last-frame-of-stream-a/🦀️component.rs"]
                                    mod tests_removes_the_last_frame_of_stream_a;
                                }
                                #[path = "."]
                                pub mod replace_stream_source {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/clears-the-video-source-of-stream-a/🦀️component.rs"]
                                    mod tests_clears_the_video_source_of_stream_a;
                                }
                                #[path = "."]
                                pub mod create_asset {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/stores-a-new-jpeg-frame-asset/🦀️component.rs"]
                                    mod tests_stores_a_new_jpeg_frame_asset;
                                }
                                #[path = "."]
                                pub mod delete_asset {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/removes-asset-a-and-reports-its-stale-references/🦀️component.rs"]
                                    mod tests_removes_asset_a_and_reports_its_stale_references;
                                }
                                #[path = "."]
                                pub mod create_camera_calibration {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🧪️tests/adds-the-cam-c-fisheye-calibration/🦀️component.rs"]
                                    mod tests_adds_the_cam_c_fisheye_calibration;
                                }
                                #[path = "."]
                                pub mod update_camera_calibration {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/refines-the-cam-a-focal-length-and-rms/🦀️component.rs"]
                                    mod tests_refines_the_cam_a_focal_length_and_rms;
                                }
                                #[path = "."]
                                pub mod delete_camera_calibration {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/removes-the-cam-b-calibration/🦀️component.rs"]
                                    mod tests_removes_the_cam_b_calibration;
                                }
                                #[path = "."]
                                pub mod create_rig_extrinsic {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/adds-a-rig-extrinsic-for-cam-b/🦀️component.rs"]
                                    mod tests_adds_a_rig_extrinsic_for_cam_b;
                                }
                                #[path = "."]
                                pub mod delete_rig_extrinsic {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/drops-the-cam-a-rig-extrinsic/🦀️component.rs"]
                                    mod tests_drops_the_cam_a_rig_extrinsic;
                                }
                                #[path = "."]
                                pub mod update_rig_extrinsic {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/retunes-the-cam-a-rig-translation/🦀️component.rs"]
                                    mod tests_retunes_the_cam_a_rig_translation;
                                }
                                #[path = "."]
                                pub mod create_gcp {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/adds-gcp-tower-with-one-observation/🦀️component.rs"]
                                    mod tests_adds_gcp_tower_with_one_observation;
                                }
                                #[path = "."]
                                pub mod delete_gcp {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/removes-gcp-corner-and-cascades-its-observation/🦀️component.rs"]
                                    mod tests_removes_gcp_corner_and_cascades_its_observation;
                                }
                                #[path = "."]
                                pub mod add_gcp_observation {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/adds-the-first-observation-to-gcp-ridge/🦀️component.rs"]
                                    mod tests_adds_the_first_observation_to_gcp_ridge;
                                }
                                #[path = "."]
                                pub mod remove_gcp_observation {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/removes-the-only-observation-of-gcp-corner/🦀️component.rs"]
                                    mod tests_removes_the_only_observation_of_gcp_corner;
                                }
                                #[path = "."]
                                pub mod update_ingest_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/tightens-the-ingest-sharpness-gate/🦀️component.rs"]
                                    mod tests_tightens_the_ingest_sharpness_gate;
                                }
                                #[path = "."]
                                pub mod update_feature_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/switches-the-detector-to-akaze/🦀️component.rs"]
                                    mod tests_switches_the_detector_to_akaze;
                                }
                                #[path = "."]
                                pub mod update_match_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/switches-the-matcher-to-a-kd-tree/🦀️component.rs"]
                                    mod tests_switches_the_matcher_to_a_kd_tree;
                                }
                                #[path = "."]
                                pub mod update_sfm_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🧪️tests/switches-the-robust-loss-to-cauchy/🦀️component.rs"]
                                    mod tests_switches_the_robust_loss_to_cauchy;
                                }
                                #[path = "."]
                                pub mod update_dense_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🧪️tests/raises-the-dense-resolution-and-confidence-gate/🦀️component.rs"]
                                    mod tests_raises_the_dense_resolution_and_confidence_gate;
                                }
                                #[path = "."]
                                pub mod update_mesh_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🧪️tests/doubles-the-texture-size-and-drops-the-watertight-guarantee/🦀️component.rs"]
                                    mod tests_doubles_the_texture_size_and_drops_the_watertight_guarantee;
                                }
                                #[path = "."]
                                pub mod update_motion_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🧪️tests/enables-motion-tracking/🦀️component.rs"]
                                    mod tests_enables_motion_tracking;
                                }
                                #[path = "."]
                                pub mod update_geo_params {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/enables-georeferencing-with-an-origin/🦀️component.rs"]
                                    mod tests_enables_georeferencing_with_an_origin;
                                }
                                #[path = "."]
                                pub mod replace_job {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/advances-the-job-to-texturing/🦀️component.rs"]
                                    mod tests_advances_the_job_to_texturing;
                                }
                                #[path = "."]
                                pub mod replace_sparse {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🧪️tests/swaps-in-an-uncolored-four-point-sparse-cloud/🦀️component.rs"]
                                    mod tests_swaps_in_an_uncolored_four_point_sparse_cloud;
                                }
                                #[path = "."]
                                pub mod replace_dense {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🧪️tests/swaps-in-a-two-point-classified-dense-cloud/🦀️component.rs"]
                                    mod tests_swaps_in_a_two_point_classified_dense_cloud;
                                }
                                #[path = "."]
                                pub mod replace_mesh_result {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/swaps-in-an-imported-untextured-mesh/🦀️component.rs"]
                                    mod tests_swaps_in_an_imported_untextured_mesh;
                                }
                                #[path = "."]
                                pub mod replace_trajectory {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/clears-the-camera-trajectory/🦀️component.rs"]
                                    mod tests_clears_the_camera_trajectory;
                                }
                                #[path = "."]
                                pub mod replace_tracks {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/replaces-the-moving-track-with-two-static-tracks/🦀️component.rs"]
                                    mod tests_replaces_the_moving_track_with_two_static_tracks;
                                }
                                #[path = "."]
                                pub mod replace_geo_products {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/adds-the-dtm-and-ortho-rasters/🦀️component.rs"]
                                    mod tests_adds_the_dtm_and_ortho_rasters;
                                }
                                #[path = "."]
                                pub mod replace_qc {
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/records-a-qc-report-carrying-a-watertight-summary/🦀️component.rs"]
                                    mod tests_records_a_qc_report_carrying_a_watertight_summary;
                                }

                            }
                        }
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
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::diff::*;  pub mod schema { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::remodel::standards::v1::subsets::any::schema::snapshot::binary::*; } }


        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
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
    pub mod remodel {
        #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌟️feature/🦀️component.rs"]
            pub mod feature;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌫️dense/🦀️component.rs"]
            pub mod dense;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️component.rs"]
            pub mod video;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️motion/🦀️component.rs"]
            pub mod motion;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🦀️component.rs"]
            pub mod reconstruction;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📷️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/🦀️component.rs"]
            pub mod sfm;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖼️images/🦀️component.rs"]
            pub mod images;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗺️geo/🦀️component.rs"]
            pub mod geo;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🥽️mesh/🦀️component.rs"]
            pub mod mesh;
        }

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️edit-calibration/🦀️component.rs"]
            pub mod edit_calibration;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️calibrate-cameras/🦀️component.rs"]
            pub mod calibrate_cameras;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️add-gcp/🦀️component.rs"]
            pub mod add_gcp;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️remove-gcp/🦀️component.rs"]
            pub mod remove_gcp;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️place-gcp-observation/🦀️component.rs"]
            pub mod place_gcp_observation;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-frame-payload/🦀️component.rs"]
            pub mod import_frame_payload;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-video-frame-payload/🦀️component.rs"]
            pub mod import_video_frame_payload;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-video-done/🦀️component.rs"]
            pub mod import_video_done;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-video-bytes-payload/🦀️component.rs"]
            pub mod import_video_bytes_payload;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️add-stream/🦀️component.rs"]
            pub mod add_stream;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️remove-stream/🦀️component.rs"]
            pub mod remove_stream;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️set-stream-sync/🦀️component.rs"]
            pub mod set_stream_sync;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-ingest-params/🦀️component.rs"]
            pub mod set_ingest_params;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-feature-params/🦀️component.rs"]
            pub mod set_feature_params;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-match-params/🦀️component.rs"]
            pub mod set_match_params;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-sfm-params/🦀️component.rs"]
            pub mod set_sfm_params;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-dense-params/🦀️component.rs"]
            pub mod set_dense_params;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-mesh-params/🦀️component.rs"]
            pub mod set_mesh_params;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-motion-params/🦀️component.rs"]
            pub mod set_motion_params;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-geo-params/🦀️component.rs"]
            pub mod set_geo_params;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs"]
            pub mod run_reconstruction;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️retry-stage/🦀️component.rs"]
            pub mod retry_stage;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-stage/🦀️component.rs"]
            pub mod run_stage;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️reset-placeholder-mesh/🦀️component.rs"]
            pub mod reset_placeholder_mesh;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-sparse/🦀️component.rs"]
            pub mod clear_sparse;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-dense/🦀️component.rs"]
            pub mod clear_dense;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-mesh-result/🦀️component.rs"]
            pub mod clear_mesh_result;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-tracks/🦀️component.rs"]
            pub mod clear_tracks;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-geo-products/🦀️component.rs"]
            pub mod clear_geo_products;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-result/🦀️component.rs"]
            pub mod clear_result;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️import-frames/🦀️component.rs"]
            pub mod import_frames;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️import-video/🦀️component.rs"]
            pub mod import_video;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🐚️export-qc-report/🦀️component.rs"]
            pub mod export_qc_report;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-camera/🦀️component.rs"]
            pub mod set_camera;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-layer-visibility/🦀️component.rs"]
            pub mod set_layer_visibility;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-frame-cursor/🦀️component.rs"]
            pub mod set_frame_cursor;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-report-table/🦀️component.rs"]
            pub mod set_report_table;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-active-utility/🦀️component.rs"]
            pub mod set_active_utility;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-locale/🦀️component.rs"]
            pub mod set_locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod model {
                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod model {
                        #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🎚️options/👁️layers/🦀️component.rs"]
                            pub mod layers;
                        }
                    }
                }
            }

            #[path = "."]
            pub mod capture {
                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📷️capture/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️component.rs"]
                    pub mod frames;
                }
            }

            #[path = "."]
            pub mod analyze {
                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔍️analyze/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🦀️component.rs"]
                    pub mod report;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🎯️calibration/🦀️component.rs"]
            pub mod calibration;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗂️media/🦀️component.rs"]
            pub mod media;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/⚙️parameters/🦀️component.rs"]
            pub mod parameters;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/✅️quality/🦀️component.rs"]
            pub mod quality;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🧵️results/🦀️component.rs"]
            pub mod results;
            #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🏃️tracks/🦀️component.rs"]
            pub mod tracks;
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod remodel {
        #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod model {
                        #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🦀️component.rs"]
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
#[path = "../../🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_remodel_demo;
    #[path = "../../🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_remodel_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
