//! 🧩️ Puzzle plugin — the 2d/3d/5d play apps bundled as one hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that
//! is written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, Shape V2 — see ticket
//! `26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST`), prefixed with `../../` to reach back up to the plugin
//! root. The grouping modules carry `#[path = "."]` so their own names are not spliced into that base
//! directory — without it, Rust resolves an inline module's children under
//! `<file dir>/<inline mod name>/…` and every leaf path dangles. Do not inline any component file back
//! into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint both fail on it (see
//! master ticket `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard
//! ruling).

extern crate infinite_canvas as infinite_board_port_directed_normal;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as artifact_schema;
// 🧯️ `clippy::result_large_err` — `ArtifactApp::handle` and `import_media` return
// `Result<Emit<Puzzle2dMutation, Puzzle2dConfigMutation>, Fault>`/`…, MediaError>`, the exact
// signatures the trait requires. `Fault` is a framework-owned error type; boxing it here would
// diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself (only
// on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "../../🗿️artifacts/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod flat_position {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛flat-position/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_node {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-node/🧪️tests/appends-node-c/🦀️component.rs"]
                                    mod tests_appends_node_c;
                                }
                                #[path = "."]
                                pub mod delete_node {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-node/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-node/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-node/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-node/🧪️tests/removes-node-a-and-severs-edge/🦀️component.rs"]
                                    mod tests_removes_node_a_and_severs_edge;
                                }
                                #[path = "."]
                                pub mod move_node {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-node/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-node/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-node/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-node/🧪️tests/moves-node-a/🦀️component.rs"]
                                    mod tests_moves_node_a;
                                }
                                #[path = "."]
                                pub mod replace_node_geometry {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-node-geometry/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-node-geometry/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-node-geometry/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-node-geometry/🧪️tests/circle-to-rectangle/🦀️component.rs"]
                                    mod tests_circle_to_rectangle;
                                }
                                #[path = "."]
                                pub mod change_node_kind {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗change-node-kind/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗change-node-kind/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗change-node-kind/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗change-node-kind/🧪️tests/reassigns-node-a-kind/🦀️component.rs"]
                                    mod tests_reassigns_node_a_kind;
                                }
                                #[path = "."]
                                pub mod edit_node_text {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-node-text/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-node-text/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-node-text/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-node-text/🧪️tests/retitles-node-a/🦀️component.rs"]
                                    mod tests_retitles_node_a;
                                }
                                #[path = "."]
                                pub mod change_node_icon {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-node-icon/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-node-icon/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-node-icon/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-node-icon/🧪️tests/swaps-node-a-icon/🦀️component.rs"]
                                    mod tests_swaps_node_a_icon;
                                }
                                #[path = "."]
                                pub mod scale_node {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-node/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-node/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-node/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-node/🧪️tests/doubles-node-a/🦀️component.rs"]
                                    mod tests_doubles_node_a;
                                }
                                #[path = "."]
                                pub mod change_node_visible {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁change-node-visible/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁change-node-visible/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁change-node-visible/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁change-node-visible/🧪️tests/hides-node-a/🦀️component.rs"]
                                    mod tests_hides_node_a;
                                }
                                #[path = "."]
                                pub mod change_node_locked {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-node-locked/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-node-locked/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-node-locked/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-node-locked/🧪️tests/locks-node-a/🦀️component.rs"]
                                    mod tests_locks_node_a;
                                }
                                #[path = "."]
                                pub mod change_node_root {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌟change-node-root/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌟change-node-root/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌟change-node-root/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌟change-node-root/🧪️tests/promotes-node-a-to-root/🦀️component.rs"]
                                    mod tests_promotes_node_a_to_root;
                                }
                                #[path = "."]
                                pub mod change_node_anchor {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-node-anchor/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-node-anchor/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-node-anchor/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-node-anchor/🧪️tests/fixed-to-derived/🦀️component.rs"]
                                    mod tests_fixed_to_derived;
                                }
                                #[path = "."]
                                pub mod add_node_handle {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-node-handle/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-node-handle/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-node-handle/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-node-handle/🧪️tests/appends-handle-3-to-node-b/🦀️component.rs"]
                                    mod tests_appends_handle_3_to_node_b;
                                }
                                #[path = "."]
                                pub mod remove_node_handle {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-node-handle/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-node-handle/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-node-handle/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-node-handle/🧪️tests/removes-handle-2-and-severs-edge/🦀️component.rs"]
                                    mod tests_removes_handle_2_and_severs_edge;
                                }
                                #[path = "."]
                                pub mod replace_node_handle {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-node-handle/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-node-handle/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-node-handle/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-node-handle/🧪️tests/rekind-handle-1-is-noop/🦀️component.rs"]
                                    mod tests_rekind_handle_1_is_noop;
                                }
                                #[path = "."]
                                pub mod connect_handles {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-handles/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-handles/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-handles/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-handles/🧪️tests/adds-second-edge/🦀️component.rs"]
                                    mod tests_adds_second_edge;
                                }
                                #[path = "."]
                                pub mod disconnect_handles {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-handles/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-handles/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-handles/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-handles/🧪️tests/removes-edge-1/🦀️component.rs"]
                                    mod tests_removes_edge_1;
                                }
                                #[path = "."]
                                pub mod replace_edge_geometry {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-edge-geometry/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-edge-geometry/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-edge-geometry/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-edge-geometry/🧪️tests/repositions-edge-1/🦀️component.rs"]
                                    mod tests_repositions_edge_1;
                                }
                                #[path = "."]
                                pub mod change_edge_kind {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷change-edge-kind/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷change-edge-kind/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷change-edge-kind/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷change-edge-kind/🧪️tests/rekinds-edge-1/🦀️component.rs"]
                                    mod tests_rekinds_edge_1;
                                }
                                #[path = "."]
                                pub mod change_edge_tips {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇change-edge-tips/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇change-edge-tips/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇change-edge-tips/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇change-edge-tips/🧪️tests/swaps-edge-1-tips/🦀️component.rs"]
                                    mod tests_swaps_edge_1_tips;
                                }
                                #[path = "."]
                                pub mod change_edge_visible {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👀change-edge-visible/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👀change-edge-visible/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👀change-edge-visible/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👀change-edge-visible/🧪️tests/hides-edge-1/🦀️component.rs"]
                                    mod tests_hides_edge_1;
                                }
                                #[path = "."]
                                pub mod change_edge_locked {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-edge-locked/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-edge-locked/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-edge-locked/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-edge-locked/🧪️tests/locks-edge-1/🦀️component.rs"]
                                    mod tests_locks_edge_1;
                                }
                                #[path = "."]
                                pub mod change_manifest_id {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆔change-manifest-id/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆔change-manifest-id/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆔change-manifest-id/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆔change-manifest-id/🧪️tests/repoints-manifest/🦀️component.rs"]
                                    mod tests_repoints_manifest;
                                }
                                #[path = "."]
                                pub mod connect_kind_compatibility {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🧪️tests/adds-handle-kind-pair/🦀️component.rs"]
                                    mod tests_adds_handle_kind_pair;
                                }
                                #[path = "."]
                                pub mod disconnect_kind_compatibility {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🧪️tests/removes-handle-kind-pair/🦀️component.rs"]
                                    mod tests_removes_handle_kind_pair;
                                }
                                #[path = "."]
                                pub mod replace_kind_catalogs {
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🧪️tests/installs-handle-kind-catalog/🦀️component.rs"]
                                    mod tests_installs_handle_kind_catalog;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️component.rs"]
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
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dxf {
                                            #[path = "."]
                                            pub mod v_r12 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️component.rs"]
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
            pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::diff::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::diff::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
        pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
        pub use crate::artifacts::puzzle2d::standards::v1::subsets::any::schema::snapshot::Puzzle2dSnapshot;
    }
    #[path = "."]
    pub mod puzzle5d {
        #[path = "../../🗿️artifacts/🖐️5d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod flat_position {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛flat-position/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_part {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-part/🧪️tests/appends-part-c/🦀️component.rs"]
                                    mod tests_appends_part_c;
                                }
                                #[path = "."]
                                pub mod delete_part {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-part/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-part/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-part/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-part/🧪️tests/removes-part-a-and-severs-fastener/🦀️component.rs"]
                                    mod tests_removes_part_a_and_severs_fastener;
                                }
                                #[path = "."]
                                pub mod move_part_2d {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-part2d/🧪️tests/moves-part-a/🦀️component.rs"]
                                    mod tests_moves_part_a;
                                }
                                #[path = "."]
                                pub mod replace_part_2d_geometry {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-part2d-geometry/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-part2d-geometry/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-part2d-geometry/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧊replace-part2d-geometry/🧪️tests/circle-to-rectangle/🦀️component.rs"]
                                    mod tests_circle_to_rectangle;
                                }
                                #[path = "."]
                                pub mod edit_part_2d_text {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-part2d-text/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-part2d-text/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-part2d-text/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️edit-part2d-text/🧪️tests/retitles-part-a/🦀️component.rs"]
                                    mod tests_retitles_part_a;
                                }
                                #[path = "."]
                                pub mod change_part_2d_icon {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-part2d-icon/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-part2d-icon/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-part2d-icon/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-part2d-icon/🧪️tests/swaps-icon/🦀️component.rs"]
                                    mod tests_swaps_icon;
                                }
                                #[path = "."]
                                pub mod change_part_2d_hidden {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-part2d-hidden/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-part2d-hidden/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-part2d-hidden/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-part2d-hidden/🧪️tests/hides-part-a/🦀️component.rs"]
                                    mod tests_hides_part_a;
                                }
                                #[path = "."]
                                pub mod change_part_2d_locked {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-part2d-locked/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-part2d-locked/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-part2d-locked/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-part2d-locked/🧪️tests/locks-part-a/🦀️component.rs"]
                                    mod tests_locks_part_a;
                                }
                                #[path = "."]
                                pub mod move_part_3d {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-part3d/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-part3d/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-part3d/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-part3d/🧪️tests/moves-part-a-in-world/🦀️component.rs"]
                                    mod tests_moves_part_a_in_world;
                                }
                                #[path = "."]
                                pub mod rotate_part_3d {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-part3d/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-part3d/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-part3d/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-part3d/🧪️tests/half-turn-about-z/🦀️component.rs"]
                                    mod tests_half_turn_about_z;
                                }
                                #[path = "."]
                                pub mod scale_part_3d {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-part3d/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-part3d/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-part3d/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-part3d/🧪️tests/uniform-double/🦀️component.rs"]
                                    mod tests_uniform_double;
                                }
                                #[path = "."]
                                pub mod change_part_3d_mesh {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-part3d-mesh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-part3d-mesh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-part3d-mesh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-part3d-mesh/🧪️tests/repoints-mesh/🦀️component.rs"]
                                    mod tests_repoints_mesh;
                                }
                                #[path = "."]
                                pub mod edit_part_3d_label {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-part3d-label/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-part3d-label/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-part3d-label/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-part3d-label/🧪️tests/relabels-part-a/🦀️component.rs"]
                                    mod tests_relabels_part_a;
                                }
                                #[path = "."]
                                pub mod change_part_kind {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗change-part-kind/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗change-part-kind/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗change-part-kind/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗change-part-kind/🧪️tests/reassigns-kind/🦀️component.rs"]
                                    mod tests_reassigns_kind;
                                }
                                #[path = "."]
                                pub mod change_part_anchor {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-part-anchor/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-part-anchor/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-part-anchor/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-part-anchor/🧪️tests/fixed-to-derived/🦀️component.rs"]
                                    mod tests_fixed_to_derived;
                                }
                                #[path = "."]
                                pub mod add_part_grip {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-part-grip/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-part-grip/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-part-grip/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-part-grip/🧪️tests/appends-grip-3/🦀️component.rs"]
                                    mod tests_appends_grip_3;
                                }
                                #[path = "."]
                                pub mod remove_part_grip {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-part-grip/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-part-grip/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-part-grip/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-part-grip/🧪️tests/removes-grip-1-and-severs-fastener/🦀️component.rs"]
                                    mod tests_removes_grip_1_and_severs_fastener;
                                }
                                #[path = "."]
                                pub mod replace_part_grip {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-part-grip/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-part-grip/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-part-grip/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-part-grip/🧪️tests/rekinds-grip-1/🦀️component.rs"]
                                    mod tests_rekinds_grip_1;
                                }
                                #[path = "."]
                                pub mod connect_grips {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-grips/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-grips/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-grips/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-grips/🧪️tests/adds-second-fastener/🦀️component.rs"]
                                    mod tests_adds_second_fastener;
                                }
                                #[path = "."]
                                pub mod disconnect_grips {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-grips/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-grips/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-grips/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-grips/🧪️tests/removes-fast-1/🦀️component.rs"]
                                    mod tests_removes_fast_1;
                                }
                                #[path = "."]
                                pub mod replace_fastener_geometry {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-fastener-geometry/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-fastener-geometry/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-fastener-geometry/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-fastener-geometry/🧪️tests/repositions-fast-1/🦀️component.rs"]
                                    mod tests_repositions_fast_1;
                                }
                                #[path = "."]
                                pub mod change_fastener_kind {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-fastener-kind/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-fastener-kind/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-fastener-kind/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯change-fastener-kind/🧪️tests/rekinds-fast-1/🦀️component.rs"]
                                    mod tests_rekinds_fast_1;
                                }
                                #[path = "."]
                                pub mod rename_puzzle5d {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷rename-puzzle5d/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷rename-puzzle5d/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷rename-puzzle5d/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷rename-puzzle5d/🧪️tests/relabels-document/🦀️component.rs"]
                                    mod tests_relabels_document;
                                }
                                #[path = "."]
                                pub mod change_domain {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🧪️tests/architecture-to-engineering/🦀️component.rs"]
                                    mod tests_architecture_to_engineering;
                                }
                                #[path = "."]
                                pub mod change_description {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-description/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-description/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-description/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝change-description/🧪️tests/rewrites-description/🦀️component.rs"]
                                    mod tests_rewrites_description;
                                }
                                #[path = "."]
                                pub mod connect_kind_compatibility {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🧪️tests/adds-grip-pair/🦀️component.rs"]
                                    mod tests_adds_grip_pair;
                                }
                                #[path = "."]
                                pub mod disconnect_kind_compatibility {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🧪️tests/removes-grip-pair/🦀️component.rs"]
                                    mod tests_removes_grip_pair;
                                }
                                #[path = "."]
                                pub mod replace_kind_catalogs {
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🧪️tests/null-catalogs-is-noop/🦀️component.rs"]
                                    mod tests_null_catalogs_is_noop;
                                }
                            }
                            #[path = "."]
                            pub mod transfer {
                                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✂️transfer/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️component.rs"]
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
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️component.rs"]
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
            pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::diff::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::diff::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
        pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
        pub use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;
    }
    #[path = "."]
    pub mod puzzle3d {
        #[path = "../../🗿️artifacts/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod flat_position {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛flat-position/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod flatten {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛flatten/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_object {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-object/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-object/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-object/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-object/🧪️tests/appends-object-c/🦀️component.rs"]
                                    mod tests_appends_object_c;
                                }
                                #[path = "."]
                                pub mod delete_object {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-object/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-object/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-object/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑delete-object/🧪️tests/removes-object-a-and-severs-attraction/🦀️component.rs"]
                                    mod tests_removes_object_a_and_severs_attraction;
                                }
                                #[path = "."]
                                pub mod move_object {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-object/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-object/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-object/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-object/🧪️tests/moves-object-a/🦀️component.rs"]
                                    mod tests_moves_object_a;
                                }
                                #[path = "."]
                                pub mod rotate_object {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-object/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-object/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-object/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃rotate-object/🧪️tests/half-turn-about-z/🦀️component.rs"]
                                    mod tests_half_turn_about_z;
                                }
                                #[path = "."]
                                pub mod scale_object {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-object/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-object/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-object/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏scale-object/🧪️tests/uniform-to-per-axis/🦀️component.rs"]
                                    mod tests_uniform_to_per_axis;
                                }
                                #[path = "."]
                                pub mod change_object_mesh {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-object-mesh/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-object-mesh/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-object-mesh/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱change-object-mesh/🧪️tests/repoints-object-a-mesh/🦀️component.rs"]
                                    mod tests_repoints_object_a_mesh;
                                }
                                #[path = "."]
                                pub mod edit_object_label {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-object-label/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-object-label/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-object-label/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️edit-object-label/🧪️tests/relabels-object-a/🦀️component.rs"]
                                    mod tests_relabels_object_a;
                                }
                                #[path = "."]
                                pub mod change_object_kind {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗change-object-kind/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗change-object-kind/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗change-object-kind/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏗change-object-kind/🧪️tests/reassigns-object-a-kind/🦀️component.rs"]
                                    mod tests_reassigns_object_a_kind;
                                }
                                #[path = "."]
                                pub mod change_object_anchor {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-object-anchor/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-object-anchor/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-object-anchor/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚓change-object-anchor/🧪️tests/fixed-to-derived/🦀️component.rs"]
                                    mod tests_fixed_to_derived;
                                }
                                #[path = "."]
                                pub mod change_object_hidden {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁change-object-hidden/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁change-object-hidden/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁change-object-hidden/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁change-object-hidden/🧪️tests/hides-object-a/🦀️component.rs"]
                                    mod tests_hides_object_a;
                                }
                                #[path = "."]
                                pub mod change_object_locked {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-object-locked/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-object-locked/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-object-locked/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒change-object-locked/🧪️tests/locks-object-a/🦀️component.rs"]
                                    mod tests_locks_object_a;
                                }
                                #[path = "."]
                                pub mod add_object_vortex {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-object-vortex/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-object-vortex/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-object-vortex/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-object-vortex/🧪️tests/appends-vortex-3-to-object-b/🦀️component.rs"]
                                    mod tests_appends_vortex_3_to_object_b;
                                }
                                #[path = "."]
                                pub mod remove_object_vortex {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-object-vortex/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-object-vortex/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-object-vortex/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-object-vortex/🧪️tests/removes-vortex-2-and-severs-attraction/🦀️component.rs"]
                                    mod tests_removes_vortex_2_and_severs_attraction;
                                }
                                #[path = "."]
                                pub mod replace_object_vortex {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-object-vortex/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-object-vortex/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-object-vortex/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌replace-object-vortex/🧪️tests/rekind-vortex-1-is-noop/🦀️component.rs"]
                                    mod tests_rekind_vortex_1_is_noop;
                                }
                                #[path = "."]
                                pub mod connect_vortices {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-vortices/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-vortices/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-vortices/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗connect-vortices/🧪️tests/adds-second-attraction/🦀️component.rs"]
                                    mod tests_adds_second_attraction;
                                }
                                #[path = "."]
                                pub mod disconnect_vortices {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-vortices/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-vortices/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-vortices/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-vortices/🧪️tests/removes-attraction-1/🦀️component.rs"]
                                    mod tests_removes_attraction_1;
                                }
                                #[path = "."]
                                pub mod replace_attraction_geometry {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-attraction-geometry/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-attraction-geometry/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-attraction-geometry/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧮replace-attraction-geometry/🧪️tests/repositions-attraction-1/🦀️component.rs"]
                                    mod tests_repositions_attraction_1;
                                }
                                #[path = "."]
                                pub mod create_target_volume {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌍create-target-volume/🧪️tests/appends-volume-2/🦀️component.rs"]
                                    mod tests_appends_volume_2;
                                }
                                #[path = "."]
                                pub mod delete_target_volume {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦delete-target-volume/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦delete-target-volume/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦delete-target-volume/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪦delete-target-volume/🧪️tests/removes-volume-1/🦀️component.rs"]
                                    mod tests_removes_volume_1;
                                }
                                #[path = "."]
                                pub mod move_target_volume {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-target-volume/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-target-volume/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-target-volume/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚀move-target-volume/🧪️tests/lifts-volume-1/🦀️component.rs"]
                                    mod tests_lifts_volume_1;
                                }
                                #[path = "."]
                                pub mod rotate_target_volume {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-target-volume/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-target-volume/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-target-volume/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀rotate-target-volume/🧪️tests/half-turn-about-z/🦀️component.rs"]
                                    mod tests_half_turn_about_z;
                                }
                                #[path = "."]
                                pub mod scale_target_volume {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐scale-target-volume/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐scale-target-volume/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐scale-target-volume/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐scale-target-volume/🧪️tests/per-axis-to-uniform/🦀️component.rs"]
                                    mod tests_per_axis_to_uniform;
                                }
                                #[path = "."]
                                pub mod change_target_volume_hidden {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-target-volume-hidden/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-target-volume-hidden/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-target-volume-hidden/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙈change-target-volume-hidden/🧪️tests/hides-volume-1/🦀️component.rs"]
                                    mod tests_hides_volume_1;
                                }
                                #[path = "."]
                                pub mod change_target_volume_locked {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔐change-target-volume-locked/🧪️tests/locks-volume-1/🦀️component.rs"]
                                    mod tests_locks_volume_1;
                                }
                                #[path = "."]
                                pub mod create_reference {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼create-reference/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼create-reference/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼create-reference/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼create-reference/🧪️tests/appends-reference-2/🦀️component.rs"]
                                    mod tests_appends_reference_2;
                                }
                                #[path = "."]
                                pub mod delete_reference {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-reference/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-reference/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-reference/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚮delete-reference/🧪️tests/removes-reference-1/🦀️component.rs"]
                                    mod tests_removes_reference_1;
                                }
                                #[path = "."]
                                pub mod move_reference {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯move-reference/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯move-reference/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯move-reference/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯move-reference/🧪️tests/slides-reference-1/🦀️component.rs"]
                                    mod tests_slides_reference_1;
                                }
                                #[path = "."]
                                pub mod resize_reference {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎resize-reference/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎resize-reference/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎resize-reference/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎resize-reference/🧪️tests/widens-reference-1/🦀️component.rs"]
                                    mod tests_widens_reference_1;
                                }
                                #[path = "."]
                                pub mod replace_reference_source {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇replace-reference-source/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇replace-reference-source/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇replace-reference-source/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇replace-reference-source/🧪️tests/repoints-reference-1-source/🦀️component.rs"]
                                    mod tests_repoints_reference_1_source;
                                }
                                #[path = "."]
                                pub mod change_reference_hidden {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👀change-reference-hidden/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👀change-reference-hidden/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👀change-reference-hidden/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👀change-reference-hidden/🧪️tests/hides-reference-1/🦀️component.rs"]
                                    mod tests_hides_reference_1;
                                }
                                #[path = "."]
                                pub mod change_reference_locked {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗝change-reference-locked/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗝change-reference-locked/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗝change-reference-locked/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗝change-reference-locked/🧪️tests/locks-reference-1/🦀️component.rs"]
                                    mod tests_locks_reference_1;
                                }
                                #[path = "."]
                                pub mod change_domain {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-domain/🧪️tests/architecture-to-engineering/🦀️component.rs"]
                                    mod tests_architecture_to_engineering;
                                }
                                #[path = "."]
                                pub mod connect_kind_compatibility {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝connect-kind-compatibility/🧪️tests/adds-vortex-kind-pair/🦀️component.rs"]
                                    mod tests_adds_vortex_kind_pair;
                                }
                                #[path = "."]
                                pub mod disconnect_kind_compatibility {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💔disconnect-kind-compatibility/🧪️tests/removes-vortex-kind-pair/🦀️component.rs"]
                                    mod tests_removes_vortex_kind_pair;
                                }
                                #[path = "."]
                                pub mod replace_kind_catalogs {
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📚replace-kind-catalogs/🧪️tests/installs-vortex-kind-catalog/🦀️component.rs"]
                                    mod tests_installs_vortex_kind_catalog;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️ply/🔖️1.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️component.rs"]
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
            pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::diff::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::diff::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
        pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
        pub use crate::artifacts::puzzle3d::standards::v1::subsets::any::schema::snapshot::Puzzle3dSnapshot;
    }
}
//#endregion 🗿️Artifacts

//#region ✏️Editor
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎲️board-host/🦀️component.rs"]
            pub mod board_host;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔣️icons/🦀️component.rs"]
            pub mod icons;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️layout/🦀️component.rs"]
            pub mod layout;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔗️linking/🦀️component.rs"]
            pub mod linking;
        }

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️add-node/🦀️component.rs"]
            pub mod add_node;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎲️apply-board-events/🦀️component.rs"]
            pub mod apply_board_events;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️cancel-slot/🦀️component.rs"]
            pub mod cancel_slot;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️commit-slot/🦀️component.rs"]
            pub mod commit_slot;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️cycle-candidate/🦀️component.rs"]
            pub mod cycle_candidate;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️delete-selection/🦀️component.rs"]
            pub mod delete_selection;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️duplicate-selection/🦀️component.rs"]
            pub mod duplicate_selection;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement-abort/🦀️component.rs"]
            pub mod engagement_abort;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement-control-select/🦀️component.rs"]
            pub mod engagement_control_select;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement-input/🦀️component.rs"]
            pub mod engagement_input;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement-submit/🦀️component.rs"]
            pub mod engagement_submit;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️fill-session-begin/🦀️component.rs"]
            pub mod fill_session_begin;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️fill-session-clear/🦀️component.rs"]
            pub mod fill_session_clear;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️fill-session-step/🦀️component.rs"]
            pub mod fill_session_step;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️focus-selection/🦀️component.rs"]
            pub mod focus_selection;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️force-layout/🦀️component.rs"]
            pub mod force_layout;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️lod-scale-json/🦀️component.rs"]
            pub mod lod_scale_json;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️open-slot/🦀️component.rs"]
            pub mod open_slot;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️patch-inspector/🦀️component.rs"]
            pub mod patch_inspector;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️redraw-handles/🦀️component.rs"]
            pub mod redraw_handles;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️select-same-kind/🦀️component.rs"]
            pub mod select_same_kind;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛍️set-active-example/🦀️component.rs"]
            pub mod set_active_example;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️set-active-utility/🦀️component.rs"]
            pub mod set_active_utility;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-brush-kind-weights/🦀️component.rs"]
            pub mod set_brush_kind_weights;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-brush-node-size/🦀️component.rs"]
            pub mod set_brush_node_size;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera/🦀️component.rs"]
            pub mod set_camera;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-candidate-index/🦀️component.rs"]
            pub mod set_candidate_index;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-fill-count/🦀️component.rs"]
            pub mod set_fill_count;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️set-grid-factor/🦀️component.rs"]
            pub mod set_grid_factor;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️set-grid-snap-enabled/🦀️component.rs"]
            pub mod set_grid_snap_enabled;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️component.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️set-lod-mode-for-pane/🦀️component.rs"]
            pub mod set_lod_mode_for_pane;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️set-selection-flag/🦀️component.rs"]
            pub mod set_selection_flag;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-suggestion-offset/🦀️component.rs"]
            pub mod set_suggestion_offset;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-terminology/🦀️component.rs"]
            pub mod set_terminology;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎚️options/🖌️brush/🦀️component.rs"]
                    pub mod brush;
                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎚️options/🔭️lod/🦀️component.rs"]
                    pub mod lod;
                }

                #[path = "."]
                pub mod tools {
                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️component.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod overview {
                        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                            #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️component.rs"]
                            pub mod select;
                        }
                    }

                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔍️detail/🦀️component.rs"]
                    pub mod detail;
                    #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎯️selection/🦀️component.rs"]
                    pub mod selection;
                }
            }
        }
    }

    #[path = "."]
    pub mod puzzle3d {
        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod precompute {
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs"]
            pub mod fill;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️component.rs"]
            pub mod geometry;
        }

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️accept-suggestion/🦀️component.rs"]
            pub mod accept_suggestion;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️add-brush-object/🦀️component.rs"]
            pub mod add_brush_object;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️add-object-kind/🦀️component.rs"]
            pub mod add_object_kind;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📦️add-target-volume/🦀️component.rs"]
            pub mod add_target_volume;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☀️apply-sun/🦀️component.rs"]
            pub mod apply_sun;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️close-vortex-suggestions/🦀️component.rs"]
            pub mod close_vortex_suggestions;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️create-attraction/🦀️component.rs"]
            pub mod create_attraction;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️cycle-candidate/🦀️component.rs"]
            pub mod cycle_candidate;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️delete-attraction/🦀️component.rs"]
            pub mod delete_attraction;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️delete-selection/🦀️component.rs"]
            pub mod delete_selection;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📦️delete-target-volume/🦀️component.rs"]
            pub mod delete_target_volume;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️duplicate-selection/🦀️component.rs"]
            pub mod duplicate_selection;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement-abort/🦀️component.rs"]
            pub mod engagement_abort;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️engagement-control-select/🦀️component.rs"]
            pub mod engagement_control_select;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement-input/🦀️component.rs"]
            pub mod engagement_input;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement-repeat-last/🦀️component.rs"]
            pub mod engagement_repeat_last;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement-submit/🦀️component.rs"]
            pub mod engagement_submit;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️component.rs"]
            pub mod fill_build_tick;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️focus-selection/🦀️component.rs"]
            pub mod focus_selection;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️hover-suggestion/🦀️component.rs"]
            pub mod hover_suggestion;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️open-vortex-suggestions/🦀️component.rs"]
            pub mod open_vortex_suggestions;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️patch-inspector/🦀️component.rs"]
            pub mod patch_inspector;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️register-brush-mesh/🦀️component.rs"]
            pub mod register_brush_mesh;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📦️relocate-target-volume/🦀️component.rs"]
            pub mod relocate_target_volume;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️rotate-selection/🦀️component.rs"]
            pub mod rotate_selection;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️scale-selection/🦀️component.rs"]
            pub mod scale_selection;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️select-same-kind/🦀️component.rs"]
            pub mod select_same_kind;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️set-active/🦀️component.rs"]
            pub mod set_active;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛍️set-active-example/🦀️component.rs"]
            pub mod set_active_example;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️set-automatic/🦀️component.rs"]
            pub mod set_automatic;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-brush-placement-overlap-budget/🦀️component.rs"]
            pub mod set_brush_placement_overlap_budget;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera/🦀️component.rs"]
            pub mod set_camera;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-chunk-size/🦀️component.rs"]
            pub mod set_chunk_size;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️set-depth-variable/🦀️component.rs"]
            pub mod set_depth_variable;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪣️set-fill-count/🦀️component.rs"]
            pub mod set_fill_count;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛍️set-fixture-json/🦀️component.rs"]
            pub mod set_fixture_json;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪣️set-kind-weight/🦀️component.rs"]
            pub mod set_kind_weight;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️component.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️set-manual/🦀️component.rs"]
            pub mod set_manual;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-projection/🦀️component.rs"]
            pub mod set_projection;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-proximity-radius/🦀️component.rs"]
            pub mod set_proximity_radius;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️set-selectable-kind/🦀️component.rs"]
            pub mod set_selectable_kind;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️set-selection-flag/🦀️component.rs"]
            pub mod set_selection_flag;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️set-snap-enabled/🦀️component.rs"]
            pub mod set_snap_enabled;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️set-spacing/🦀️component.rs"]
            pub mod set_spacing;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📦️set-target-volume-flag/🦀️component.rs"]
            pub mod set_target_volume_flag;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-terminology/🦀️component.rs"]
            pub mod set_terminology;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-transform-gumball-flag/🦀️component.rs"]
            pub mod set_transform_gumball_flag;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️set-visible/🦀️component.rs"]
            pub mod set_visible;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-vortex-direction/🦀️component.rs"]
            pub mod set_vortex_direction;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-vortex-show/🦀️component.rs"]
            pub mod set_vortex_show;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⚙️set-voxel-dims/🦀️component.rs"]
            pub mod set_voxel_dims;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️suggestions-tick/🦀️component.rs"]
            pub mod suggestions_tick;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️translate-selection/🦀️component.rs"]
            pub mod translate_selection;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️world-relocate/🦀️component.rs"]
            pub mod world_relocate;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/⚙️settings/🦀️component.rs"]
            pub mod settings;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎚️options/🌐️grid/🦀️component.rs"]
                    pub mod grid;
                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎚️options/🔭️lod/🦀️component.rs"]
                    pub mod lod;
                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎚️options/🎥️projection/🦀️component.rs"]
                    pub mod projection;
                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎚️options/🎯️select/🦀️component.rs"]
                    pub mod select;
                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎚️options/☀️sun/🦀️component.rs"]
                    pub mod sun;
                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎚️options/🌀️vortex/🦀️component.rs"]
                    pub mod vortex;
                }

                #[path = "."]
                pub mod tools {
                    #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️component.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🔄️transform/🦀️component.rs"]
                            pub mod transform;
                            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🧊️volume-brush/🦀️component.rs"]
                            pub mod volume_brush;
                            #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🚚️world-relocate/🦀️component.rs"]
                            pub mod world_relocate;
                        }
                    }
                }
            }
        }
    }

    #[path = "."]
    pub mod puzzle5d {
        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod precompute {
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️add-brush-part/🦀️component.rs"]
            pub mod add_brush_part;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️add-node/🦀️component.rs"]
            pub mod add_node;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️add-part-kind/🦀️component.rs"]
            pub mod add_part_kind;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎲️apply-board-events/🦀️component.rs"]
            pub mod apply_board_events;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/☀️apply-sun/🦀️component.rs"]
            pub mod apply_sun;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️create-fastener/🦀️component.rs"]
            pub mod create_fastener;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️cycle-brush-candidate/🦀️component.rs"]
            pub mod cycle_brush_candidate;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️delete-fastener/🦀️component.rs"]
            pub mod delete_fastener;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️delete-selection/🦀️component.rs"]
            pub mod delete_selection;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️duplicate-selection/🦀️component.rs"]
            pub mod duplicate_selection;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️edit-fastener/🦀️component.rs"]
            pub mod edit_fastener;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement-abort/🦀️component.rs"]
            pub mod engagement_abort;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️engagement-control-select/🦀️component.rs"]
            pub mod engagement_control_select;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement-input/🦀️component.rs"]
            pub mod engagement_input;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🤝️engagement-submit/🦀️component.rs"]
            pub mod engagement_submit;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️patch-fastener/🦀️component.rs"]
            pub mod patch_fastener;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️patch-grip/🦀️component.rs"]
            pub mod patch_grip;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️patch-part/🦀️component.rs"]
            pub mod patch_part;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️proximity-connect/🦀️component.rs"]
            pub mod proximity_connect;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️register-brush-mesh/🦀️component.rs"]
            pub mod register_brush_mesh;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️retarget-fastener/🦀️component.rs"]
            pub mod retarget_fastener;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️rotate-selection/🦀️component.rs"]
            pub mod rotate_selection;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️scale-selection/🦀️component.rs"]
            pub mod scale_selection;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️select-same-kind/🦀️component.rs"]
            pub mod select_same_kind;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️set-active/🦀️component.rs"]
            pub mod set_active;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛍️set-active-example/🦀️component.rs"]
            pub mod set_active_example;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-brush-placement-overlap-budget/🦀️component.rs"]
            pub mod set_brush_placement_overlap_budget;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera/🦀️component.rs"]
            pub mod set_camera;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera-2d/🦀️component.rs"]
            pub mod set_camera_2d;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera-3d/🦀️component.rs"]
            pub mod set_camera_3d;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪣️set-fill-count/🦀️component.rs"]
            pub mod set_fill_count;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛍️set-fixture-json/🦀️component.rs"]
            pub mod set_fixture_json;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️set-grid-factor/🦀️component.rs"]
            pub mod set_grid_factor;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌐️set-grid-snap-enabled/🦀️component.rs"]
            pub mod set_grid_snap_enabled;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-kind-weight/🦀️component.rs"]
            pub mod set_kind_weight;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️set-lod-mode/🦀️component.rs"]
            pub mod set_lod_mode;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-selection-flag/🦀️component.rs"]
            pub mod set_selection_flag;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️set-suggestion-offset/🦀️component.rs"]
            pub mod set_suggestion_offset;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️translate-selection/🦀️component.rs"]
            pub mod translate_selection;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️world-relocate/🦀️component.rs"]
            pub mod world_relocate;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️zoom-to-selection/🦀️component.rs"]
            pub mod zoom_to_selection;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️component.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎚️options/🖌️brush/🦀️component.rs"]
                    pub mod brush;
                    #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🎚️options/🪣️fill/🦀️component.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod board2d {
                        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻2d/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod actions {
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻2d/🎬️actions/🧩️apply-board-events/🦀️component.rs"]
                            pub mod apply_board_events;
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻2d/🎬️actions/📷️set-camera/🦀️component.rs"]
                            pub mod set_camera;
                        }

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻2d/🎚️options/🔭️lod/🦀️component.rs"]
                            pub mod lod;
                        }

                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻2d/🪛️utilities/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻2d/🪛️utilities/🪣️fill/🦀️component.rs"]
                            pub mod fill;
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/◻2d/🪛️utilities/🖱️select/🦀️component.rs"]
                            pub mod select;
                        }
                    }

                    #[path = "."]
                    pub mod world3d {
                        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod actions {
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/🔄️rotate-selection/🦀️component.rs"]
                            pub mod rotate_selection;
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/📐️scale-selection/🦀️component.rs"]
                            pub mod scale_selection;
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/📷️set-camera/🦀️component.rs"]
                            pub mod set_camera;
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/↔️translate-selection/🦀️component.rs"]
                            pub mod translate_selection;
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/🚚️world-relocate/🦀️component.rs"]
                            pub mod world_relocate;
                        }

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎚️options/☀️sun/🦀️component.rs"]
                            pub mod sun;
                        }

                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🔄️transform/🦀️component.rs"]
                            pub mod transform;
                            #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🚚️world-relocate/🦀️component.rs"]
                            pub mod world_relocate;
                        }
                    }
                }
            }
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod board {
                        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🎲️board/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }

    #[path = "."]
    pub mod puzzle3d {
        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️main/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }

    #[path = "."]
    pub mod puzzle5d {
        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod world3d {
                        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️3d/🦀️component.rs"]
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
#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🦀️component.rs"]
        pub mod concrete_forest;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🦀️test.rs"]
        mod concrete_forest_tests;
        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🦀️component.rs"]
        pub mod nakagin_capsule_tower;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🦀️test.rs"]
        mod nakagin_capsule_tower_tests;
    }
    #[path = "."]
    pub mod puzzle3d {
        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🦀️component.rs"]
        pub mod concrete_forest;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🦀️test.rs"]
        mod concrete_forest_tests;
        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🦀️component.rs"]
        pub mod nakagin_capsule_tower;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🦀️test.rs"]
        mod nakagin_capsule_tower_tests;
    }
    #[path = "."]
    pub mod puzzle5d {
        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/🦀️component.rs"]
        pub mod capsule_dream;
        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/🧪️tests/🦀️test.rs"]
        mod capsule_dream_tests;
        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🦀️component.rs"]
        pub mod concrete_forest;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🦀️test.rs"]
        mod concrete_forest_tests;
        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🦀️component.rs"]
        pub mod nakagin_capsule_tower;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🦀️test.rs"]
        mod nakagin_capsule_tower_tests;
    }
    #[path = "."]
    pub mod apps {
        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️component.rs"]
        pub mod demo_session_2d;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🦀️test.rs"]
        mod demo_session_2d_tests;
        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️component.rs"]
        pub mod demo_session_3d;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🦀️test.rs"]
        mod demo_session_3d_tests;
        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️component.rs"]
        pub mod demo_session_5d;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🦀️test.rs"]
        mod demo_session_5d_tests;
    }
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
