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
                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🦀️.rs"]
                mod standard_root;
                pub use standard_root::*;

                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod subset_root;
                        pub use subset_root::*;

                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            // 🧮️ 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d:
                            // Rust-only compute-internals, mirroring the `✳️table/🧬️schema/📋️tabular-internals`
                            // and `🧊️brep/🧬️schema/⚙️engine` precedent — moved wholesale from `🧮️math`, sole
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
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/🎥️adds-stream-c-458900/🦀️.rs"]
                                    mod tests_adds_stream_c_458900;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/🛰️adds-a-third-61fb5d/🦀️.rs"]
                                    mod tests_adds_a_third_61fb5d;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/🎞️adds-an-unbound-2b2373/🦀️.rs"]
                                    mod tests_adds_an_unbound_2b2373;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/🔂️rejects-a-6b58da/🦀️.rs"]
                                    mod tests_rejects_a_6b58da;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-stream/🧪️tests/👻️rejects-a-stream-aac5c2/🦀️.rs"]
                                    mod tests_rejects_a_stream_aac5c2;
                                }
                                #[path = "."]
                                pub mod delete_stream {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/🚫️refuses-to-3c20ff/🦀️.rs"]
                                    mod tests_refuses_to_3c20ff;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/⏮️removes-the-first-c0fc2a/🦀️.rs"]
                                    mod tests_removes_the_first_c0fc2a;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/🪓removes-the-spare-556d1d/🦀️.rs"]
                                    mod tests_removes_the_spare_556d1d;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪓delete-stream/🧪️tests/⛓️refuses-to-remove-422a37/🦀️.rs"]
                                    mod tests_refuses_to_remove_422a37;
                                }
                                #[path = "."]
                                pub mod change_stream_sync {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/⏱️shifts-stream-a-5b442c/🦀️.rs"]
                                    mod tests_shifts_stream_a_5b442c;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/🚫️refuses-to-8095d3/🦀️.rs"]
                                    mod tests_refuses_to_8095d3;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/🔁️warns-that-the-a98c13/🦀️.rs"]
                                    mod tests_warns_that_the_a98c13;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⏱️change-stream-sync/🧪️tests/⏱️retimes-the-50dd75/🦀️.rs"]
                                    mod tests_retimes_the_50dd75;
                                }
                                #[path = "."]
                                pub mod add_stream_frame {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/🎞️appends-a-third-8ac259/🦀️.rs"]
                                    mod tests_appends_a_third_8ac259;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/🚫️refuses-to-c93e98/🦀️.rs"]
                                    mod tests_refuses_to_c93e98;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/🔁️warns-that-the-1e8abe/🦀️.rs"]
                                    mod tests_warns_that_the_1e8abe;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/🎞️appends-an-0c2164/🦀️.rs"]
                                    mod tests_appends_an_0c2164;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-stream-frame/🧪️tests/🎬️refuses-a-frame-81beea/🦀️.rs"]
                                    mod tests_refuses_a_frame_81beea;
                                }
                                #[path = "."]
                                pub mod remove_stream_frame {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/🚫️removes-the-last-304bdf/🦀️.rs"]
                                    mod tests_removes_the_last_304bdf;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/⏮️drops-the-first-d98a0f/🦀️.rs"]
                                    mod tests_drops_the_first_d98a0f;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/🚫️refuses-a-frame-e7c374/🦀️.rs"]
                                    mod tests_refuses_a_frame_e7c374;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-stream-frame/🧪️tests/✂️drops-the-middle-2d6d53/🦀️.rs"]
                                    mod tests_drops_the_middle_2d6d53;
                                }
                                #[path = "."]
                                pub mod replace_stream_source {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/🧹️clears-the-video-143f2b/🦀️.rs"]
                                    mod tests_clears_the_video_143f2b;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/📼️attaches-a-607df8/🦀️.rs"]
                                    mod tests_attaches_a_607df8;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/🚫️refuses-to-f7f40d/🦀️.rs"]
                                    mod tests_refuses_to_f7f40d;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-stream-source/🧪️tests/🎥️reingests-the-311c32/🦀️.rs"]
                                    mod tests_reingests_the_311c32;
                                }
                                #[path = "."]
                                pub mod create_asset {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/🖼️stores-a-new-d56283/🦀️.rs"]
                                    mod tests_stores_a_new_d56283;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/🖼️stores-an-9f39e1/🦀️.rs"]
                                    mod tests_stores_an_9f39e1;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/🚫️refuses-an-asset-cb0d4b/🦀️.rs"]
                                    mod tests_refuses_an_asset_cb0d4b;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🧪️tests/♻️overwrites-an-a34b9d/🦀️.rs"]
                                    mod tests_overwrites_an_a34b9d;
                                }
                                #[path = "."]
                                pub mod delete_asset {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/🚫️refuses-to-c4563a/🦀️.rs"]
                                    mod tests_refuses_to_c4563a;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/🧹️drops-the-spare-c6ffb6/🦀️.rs"]
                                    mod tests_drops_the_spare_c6ffb6;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/🗺️refuses-to-5c6f74/🦀️.rs"]
                                    mod tests_refuses_to_5c6f74;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/🗑️sweeps-the-503b27/🦀️.rs"]
                                    mod tests_sweeps_the_503b27;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗞️delete-asset/🧪️tests/🖼️refuses-to-f9541f/🦀️.rs"]
                                    mod tests_refuses_to_f9541f;
                                }
                                #[path = "."]
                                pub mod create_camera_calibration {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🧪️tests/📷️adds-the-cam-c-82c8fb/🦀️.rs"]
                                    mod tests_adds_the_cam_c_82c8fb;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🧪️tests/🚫️refuses-a-camera-e92a02/🦀️.rs"]
                                    mod tests_refuses_a_camera_e92a02;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔭create-camera-calibration/🧪️tests/📷️adds-a-fourth-97e912/🦀️.rs"]
                                    mod tests_adds_a_fourth_97e912;
                                }
                                #[path = "."]
                                pub mod update_camera_calibration {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/🔍️refines-the-cam-0eaef0/🦀️.rs"]
                                    mod tests_refines_the_cam_0eaef0;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/🚫️refuses-to-b60a39/🦀️.rs"]
                                    mod tests_refuses_to_b60a39;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/🔁️warns-that-the-697b4f/🦀️.rs"]
                                    mod tests_warns_that_the_697b4f;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛠️update-camera-calibration/🧪️tests/🔍️refines-the-9fd25a/🦀️.rs"]
                                    mod tests_refines_the_9fd25a;
                                }
                                #[path = "."]
                                pub mod delete_camera_calibration {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/🚫️removes-the-cam-f90b89/🦀️.rs"]
                                    mod tests_removes_the_cam_f90b89;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/🚫️refuses-to-73655a/🦀️.rs"]
                                    mod tests_refuses_to_73655a;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/🚫️removes-the-40cba4/🦀️.rs"]
                                    mod tests_removes_the_40cba4;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫delete-camera-calibration/🧪️tests/⛓️refuses-to-remove-3c8f32/🦀️.rs"]
                                    mod tests_refuses_to_remove_3c8f32;
                                }
                                #[path = "."]
                                pub mod create_rig_extrinsic {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/🔗️adds-a-rig-2df5df/🦀️.rs"]
                                    mod tests_adds_a_rig_2df5df;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/🚫️refuses-a-second-95e04d/🦀️.rs"]
                                    mod tests_refuses_a_second_95e04d;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/🔗️places-the-0d0b8d/🦀️.rs"]
                                    mod tests_places_the_0d0b8d;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⛓️create-rig-extrinsic/🧪️tests/🚫️refuses-a-rig-cb71ba/🦀️.rs"]
                                    mod tests_refuses_a_rig_cb71ba;
                                }
                                #[path = "."]
                                pub mod delete_rig_extrinsic {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/✂️drops-the-cam-a-a1f8a2/🦀️.rs"]
                                    mod tests_drops_the_cam_a_a1f8a2;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/⏮️unplaces-the-f5b35e/🦀️.rs"]
                                    mod tests_unplaces_the_f5b35e;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/🚫️refuses-to-1805df/🦀️.rs"]
                                    mod tests_refuses_to_1805df;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-rig-extrinsic/🧪️tests/✂️unplaces-the-a39356/🦀️.rs"]
                                    mod tests_unplaces_the_a39356;
                                }
                                #[path = "."]
                                pub mod update_rig_extrinsic {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/📍️retunes-the-cam-4ca5a2/🦀️.rs"]
                                    mod tests_retunes_the_cam_4ca5a2;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/🚫️refuses-to-2cfb53/🦀️.rs"]
                                    mod tests_refuses_to_2cfb53;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/🔁️warns-that-the-89422a/🦀️.rs"]
                                    mod tests_warns_that_the_89422a;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩update-rig-extrinsic/🧪️tests/📍️retunes-the-675f52/🦀️.rs"]
                                    mod tests_retunes_the_675f52;
                                }
                                #[path = "."]
                                pub mod create_gcp {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/📍️adds-gcp-tower-d71a54/🦀️.rs"]
                                    mod tests_adds_gcp_tower_d71a54;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/🚫️refuses-a-19c1ab/🦀️.rs"]
                                    mod tests_refuses_a_19c1ab;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/📍️adds-a-quay-7569de/🦀️.rs"]
                                    mod tests_adds_a_quay_7569de;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧿create-gcp/🧪️tests/🕳️adds-a-control-298de4/🦀️.rs"]
                                    mod tests_adds_a_control_298de4;
                                }
                                #[path = "."]
                                pub mod delete_gcp {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/🚫️removes-gcp-209b7d/🦀️.rs"]
                                    mod tests_removes_gcp_209b7d;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/🚫️refuses-to-12366b/🦀️.rs"]
                                    mod tests_refuses_to_12366b;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/🚮removes-the-south-42cd9e/🦀️.rs"]
                                    mod tests_removes_the_south_42cd9e;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-gcp/🧪️tests/🕳️removes-an-8f3868/🦀️.rs"]
                                    mod tests_removes_an_8f3868;
                                }
                                #[path = "."]
                                pub mod add_gcp_observation {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/🔎️adds-the-first-05b1b5/🦀️.rs"]
                                    mod tests_adds_the_first_05b1b5;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/🚫️refuses-to-pick-3c0570/🦀️.rs"]
                                    mod tests_refuses_to_pick_3c0570;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/🔁️warns-that-this-dca661/🦀️.rs"]
                                    mod tests_warns_that_this_dca661;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎add-gcp-observation/🧪️tests/🔎️picks-the-south-eb0c4d/🦀️.rs"]
                                    mod tests_picks_the_south_eb0c4d;
                                }
                                #[path = "."]
                                pub mod remove_gcp_observation {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/🚫️removes-the-only-f82e64/🦀️.rs"]
                                    mod tests_removes_the_only_f82e64;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/⏮️drops-the-first-9ebf0b/🦀️.rs"]
                                    mod tests_drops_the_first_9ebf0b;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/🚫️refuses-an-109cf1/🦀️.rs"]
                                    mod tests_refuses_an_109cf1;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-gcp-observation/🧪️tests/🚷drops-the-middle-282fb7/🦀️.rs"]
                                    mod tests_drops_the_middle_282fb7;
                                }
                                #[path = "."]
                                pub mod update_ingest_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/🔍️tightens-the-499c47/🦀️.rs"]
                                    mod tests_tightens_the_499c47;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/🚫️refuses-an-59752a/🦀️.rs"]
                                    mod tests_refuses_an_59752a;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/🔁️warns-that-the-8eaad8/🦀️.rs"]
                                    mod tests_warns_that_the_8eaad8;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🥣update-ingest-params/🧪️tests/📥️widens-the-73f33e/🦀️.rs"]
                                    mod tests_widens_the_73f33e;
                                }
                                #[path = "."]
                                pub mod update_feature_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/🔎️switches-the-423de9/🦀️.rs"]
                                    mod tests_switches_the_423de9;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/🚫️refuses-a-d82e38/🦀️.rs"]
                                    mod tests_refuses_a_d82e38;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/🔁️warns-that-the-b6b7dc/🦀️.rs"]
                                    mod tests_warns_that_the_b6b7dc;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌠update-feature-params/🧪️tests/🌟️moves-the-3621f6/🦀️.rs"]
                                    mod tests_moves_the_3621f6;
                                }
                                #[path = "."]
                                pub mod update_match_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/🌳️switches-the-652d03/🦀️.rs"]
                                    mod tests_switches_the_652d03;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/🚫️refuses-a-ratio-65dcb9/🦀️.rs"]
                                    mod tests_refuses_a_ratio_65dcb9;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/🔁️warns-that-the-414aae/🦀️.rs"]
                                    mod tests_warns_that_the_414aae;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢update-match-params/🧪️tests/🌳️switches-to-a-kd-d6fa4b/🦀️.rs"]
                                    mod tests_switches_to_a_kd_d6fa4b;
                                }
                                #[path = "."]
                                pub mod update_sfm_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🧪️tests/🎯️switches-the-7f0371/🦀️.rs"]
                                    mod tests_switches_the_7f0371;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🧪️tests/🔁️warns-that-the-79a92a/🦀️.rs"]
                                    mod tests_warns_that_the_79a92a;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮update-sfm-params/🧪️tests/🎯️tightens-the-850036/🦀️.rs"]
                                    mod tests_tightens_the_850036;
                                }
                                #[path = "."]
                                pub mod update_dense_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🧪️tests/🔬️raises-the-dense-ddb263/🦀️.rs"]
                                    mod tests_raises_the_dense_ddb263;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🧪️tests/🔁️warns-that-the-4e65c8/🦀️.rs"]
                                    mod tests_warns_that_the_4e65c8;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌁update-dense-params/🧪️tests/🧊️sharpens-the-25044c/🦀️.rs"]
                                    mod tests_sharpens_the_25044c;
                                }
                                #[path = "."]
                                pub mod update_mesh_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🧪️tests/🔳️doubles-the-c245d5/🦀️.rs"]
                                    mod tests_doubles_the_c245d5;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🧪️tests/🔁️warns-that-the-887e9f/🦀️.rs"]
                                    mod tests_warns_that_the_887e9f;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️update-mesh-params/🧪️tests/🔳️halves-the-voxel-21b53d/🦀️.rs"]
                                    mod tests_halves_the_voxel_21b53d;
                                }
                                #[path = "."]
                                pub mod update_motion_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🧪️tests/🏃️enables-motion-2444a3/🦀️.rs"]
                                    mod tests_enables_motion_2444a3;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🧪️tests/🔁️warns-that-the-83ff67/🦀️.rs"]
                                    mod tests_warns_that_the_83ff67;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏎️update-motion-params/🧪️tests/🏃️triples-the-4bb69f/🦀️.rs"]
                                    mod tests_triples_the_4bb69f;
                                }
                                #[path = "."]
                                pub mod update_geo_params {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/🌐️enables-georefere-18a68a/🦀️.rs"]
                                    mod tests_enables_georefere_18a68a;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/🚫️refuses-a-zero-fa917f/🦀️.rs"]
                                    mod tests_refuses_a_zero_fa917f;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/🔁️warns-that-the-efc6e8/🦀️.rs"]
                                    mod tests_warns_that_the_efc6e8;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐update-geo-params/🧪️tests/🌐️halves-the-002a17/🦀️.rs"]
                                    mod tests_halves_the_002a17;
                                }
                                #[path = "."]
                                pub mod replace_job {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/🎨️advances-the-job-c1e878/🦀️.rs"]
                                    mod tests_advances_the_job_c1e878;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/🔁️warns-that-the-bdf2e9/🦀️.rs"]
                                    mod tests_warns_that_the_bdf2e9;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗️replace-job/🧪️tests/🎨️advances-the-555298/🦀️.rs"]
                                    mod tests_advances_the_555298;
                                }
                                #[path = "."]
                                pub mod commit_reconstruction {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🧪️tests/🖼️rejects-an-e9fa51/🦀️.rs"]
                                    mod tests_rejects_an_e9fa51;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🧪️tests/🕸️rejects-an-5d3a60/🦀️.rs"]
                                    mod tests_rejects_an_5d3a60;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏁commit-reconstruction/🧪️tests/⭐️rejects-an-2e5568/🦀️.rs"]
                                    mod tests_rejects_an_2e5568;
                                }
                                #[path = "."]
                                pub mod replace_sparse {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🧪️tests/✨️swaps-in-an-6d9ae4/🦀️.rs"]
                                    mod tests_swaps_in_an_6d9ae4;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🧪️tests/🔁️warns-that-the-56a3a9/🦀️.rs"]
                                    mod tests_warns_that_the_56a3a9;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⭐replace-sparse/🧪️tests/✨️swaps-in-a-re-3cfa6d/🦀️.rs"]
                                    mod tests_swaps_in_a_re_3cfa6d;
                                }
                                #[path = "."]
                                pub mod replace_dense {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🧪️tests/☁️swaps-in-a-two-c688db/🦀️.rs"]
                                    mod tests_swaps_in_a_two_c688db;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🧪️tests/🔁️warns-that-the-675b6e/🦀️.rs"]
                                    mod tests_warns_that_the_675b6e;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/☁️replace-dense/🧪️tests/☁️swaps-in-a-denser-4174e1/🦀️.rs"]
                                    mod tests_swaps_in_a_denser_4174e1;
                                }
                                #[path = "."]
                                pub mod replace_mesh_result {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/🕸️swaps-in-an-f23e71/🦀️.rs"]
                                    mod tests_swaps_in_an_f23e71;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/🔁️warns-that-the-b39bab/🦀️.rs"]
                                    mod tests_warns_that_the_b39bab;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/🕸️swaps-the-c43d9c/🦀️.rs"]
                                    mod tests_swaps_the_c43d9c;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🧪️tests/🚫️refuses-a-48f3a6/🦀️.rs"]
                                    mod tests_refuses_a_48f3a6;
                                }
                                #[path = "."]
                                pub mod replace_trajectory {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/🧹️clears-the-d2f81a/🦀️.rs"]
                                    mod tests_clears_the_d2f81a;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/🚫️refuses-to-clear-524569/🦀️.rs"]
                                    mod tests_refuses_to_clear_524569;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/🛣️swaps-in-a-three-49b17f/🦀️.rs"]
                                    mod tests_swaps_in_a_three_49b17f;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛣️replace-trajectory/🧪️tests/🕳️drops-the-6436a8/🦀️.rs"]
                                    mod tests_drops_the_6436a8;
                                }
                                #[path = "."]
                                pub mod replace_tracks {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/⏸️replaces-the-d40c68/🦀️.rs"]
                                    mod tests_replaces_the_d40c68;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/🕳️clears-every-760061/🦀️.rs"]
                                    mod tests_clears_every_760061;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/🏃️swaps-in-two-166265/🦀️.rs"]
                                    mod tests_swaps_in_two_166265;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚂replace-tracks/🧪️tests/🔁️warns-that-the-8dbf82/🦀️.rs"]
                                    mod tests_warns_that_the_8dbf82;
                                }
                                #[path = "."]
                                pub mod replace_geo_products {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/🗺️adds-the-dtm-and-64d5bb/🦀️.rs"]
                                    mod tests_adds_the_dtm_and_64d5bb;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/🚫️refuses-to-clear-b8c54a/🦀️.rs"]
                                    mod tests_refuses_to_clear_b8c54a;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/🧹️clears-the-geo-f4886e/🦀️.rs"]
                                    mod tests_clears_the_geo_f4886e;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗾replace-geo-products/🧪️tests/🗺️records-a-dtm-6e132a/🦀️.rs"]
                                    mod tests_records_a_dtm_6e132a;
                                }
                                #[path = "."]
                                pub mod replace_qc {
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/📋️records-a-qc-f5caf4/🦀️.rs"]
                                    mod tests_records_a_qc_f5caf4;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/🚫️refuses-to-clear-30cbb5/🦀️.rs"]
                                    mod tests_refuses_to_clear_30cbb5;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/🧹️clears-the-qc-1d2249/🦀️.rs"]
                                    mod tests_clears_the_qc_1d2249;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾replace-qc/🧪️tests/✅️files-a-qc-report-64d222/🦀️.rs"]
                                    mod tests_files_a_qc_report_64d222;
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
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
            pub mod text {
                pub use crate::artifacts::remodeling::standards::v1::subsets::any::schema::snapshot::text::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
                mod tests;
            }
            #[path = "."]
            pub mod synthetic_orbit {
                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🛰️synthetic-orbit/🧪️tests/🦀️.rs"]
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
        #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🦀️.rs"]
        pub mod examples;
        #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧿️add-gcp/🦀️.rs"]
            pub mod add_gcp;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱️add-stream/🦀️.rs"]
            pub mod add_stream;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏩️advance-reconstruction/🦀️.rs"]
            pub mod advance_reconstruction;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️calibrate-cameras/🦀️.rs"]
            pub mod calibrate_cameras;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛑️cancel-reconstruction/🦀️.rs"]
            pub mod cancel_reconstruction;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☁️clear-dense/🦀️.rs"]
            pub mod clear_dense;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗾️clear-geo-products/🦀️.rs"]
            pub mod clear_geo_products;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧱️clear-mesh-result/🦀️.rs"]
            pub mod clear_mesh_result;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-result/🦀️.rs"]
            pub mod clear_result;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⭐️clear-sparse/🦀️.rs"]
            pub mod clear_sparse;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚂️clear-tracks/🦀️.rs"]
            pub mod clear_tracks;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛠️edit-calibration/🦀️.rs"]
            pub mod edit_calibration;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧾️export-qc-report/🦀️.rs"]
            pub mod export_qc_report;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️import-frame-payload/🦀️.rs"]
            pub mod import_frame_payload;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎞️import-frames/🦀️.rs"]
            pub mod import_frames;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️import-video/🦀️.rs"]
            pub mod import_video;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💽️import-video-bytes-payload/🦀️.rs"]
            pub mod import_video_bytes_payload;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️import-video-done/🦀️.rs"]
            pub mod import_video_done;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📼️import-video-frame-payload/🦀️.rs"]
            pub mod import_video_frame_payload;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️place-gcp-observation/🦀️.rs"]
            pub mod place_gcp_observation;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚮️remove-gcp/🦀️.rs"]
            pub mod remove_gcp;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪓️remove-stream/🦀️.rs"]
            pub mod remove_stream;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/♻️reset-placeholder-mesh/🦀️.rs"]
            pub mod reset_placeholder_mesh;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔁️retry-stage/🦀️.rs"]
            pub mod retry_stage;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🦀️.rs"]
            pub mod run_reconstruction;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-stage/🦀️.rs"]
            pub mod run_stage;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪛️set-active-utility/🦀️.rs"]
            pub mod set_active_utility;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📷️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌁️set-dense-params/🦀️.rs"]
            pub mod set_dense_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌠️set-feature-params/🦀️.rs"]
            pub mod set_feature_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️set-frame-cursor/🦀️.rs"]
            pub mod set_frame_cursor;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️set-geo-params/🦀️.rs"]
            pub mod set_geo_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🥣️set-ingest-params/🦀️.rs"]
            pub mod set_ingest_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👓️set-layer-visibility/🦀️.rs"]
            pub mod set_layer_visibility;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪢️set-match-params/🦀️.rs"]
            pub mod set_match_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️set-mesh-params/🦀️.rs"]
            pub mod set_mesh_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏎️set-motion-params/🦀️.rs"]
            pub mod set_motion_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📊️set-report-table/🦀️.rs"]
            pub mod set_report_table;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️set-sfm-params/🦀️.rs"]
            pub mod set_sfm_params;
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️set-stream-sync/🦀️.rs"]
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
                            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/☑️options/👁️layers/🦀️.rs"]
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
            #[path = "../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
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
