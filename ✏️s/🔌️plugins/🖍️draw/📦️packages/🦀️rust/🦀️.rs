//! ✏️ Draw plugin — declarative draw app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, two levels inside the
//! plugin root — hence every leaf `#[path]` below is prefixed `../../`). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<DrawMutation, DrawConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
// 🎭️ `fsm::statechart!` (used by `editor::draw::commands::canvas_pointer_down`'s gesture machine) generates code
// containing `#[cfg(feature = "serde")]` gates meant for `fsm`'s OWN crate; macro hygiene splices
// that cfg check into the CALLING crate's feature list instead (a `fsm`/rustc macro-expansion
// limitation, not a real conditional-compilation bug here) — this crate declares no `serde` feature
// at all (the dependency is always-on), so rustc flags the value as unrecognized. Harmless, but a
// hard error under `-D warnings` without this crate-wide allow.

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod draw {
        #[path = "../../🗿️artifacts/🖍️draw/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🦀️.rs"]
                mod v1_component;
                pub use v1_component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod any_component;
                        pub use any_component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs"]
                            pub mod owned;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            // 🚪️ Native codec facets (design.md §1 CORRECTION: unsplit, one `impl
                            // ArtifactDsl`/`ArtifactPack` per type, sits directly under `🚪️io/<facet>/
                            // <representation>/`, relocated from `🧬️schema/<facet>/<representation>/`).
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
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
                                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️.rs"]
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
                    #[path = "."]
                    pub mod structure {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod create_layer {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🌱create-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🌱create-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🌱create-layer/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🌱create-layer/🧪️tests/appends-shape-b-at-the-root/🦀️.rs"]
                                    mod tests_appends_shape_b_at_the_root;
                                }
                                #[path = "."]
                                pub mod delete_layer {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🗑️delete-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🗑️delete-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🗑️delete-layer/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🗑️delete-layer/🧪️tests/removes-group-a-with-its-child/🦀️.rs"]
                                    mod tests_removes_group_a_with_its_child;
                                }
                                #[path = "."]
                                pub mod duplicate_layer {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🧬️duplicate-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🧬️duplicate-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🧬️duplicate-layer/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🧬️duplicate-layer/🧪️tests/rejects-a-missing-source-layer/🦀️.rs"]
                                    mod tests_rejects_a_missing_source_layer;
                                }
                                #[path = "."]
                                pub mod reorder_layer {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🔃reorder-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🔃reorder-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🔃reorder-layer/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️structure/🧬️schema/🧬️mutations/🔃reorder-layer/🧪️tests/moves-shape-a-above-shape-b/🦀️.rs"]
                                    mod tests_moves_shape_a_above_shape_b;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod style {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod replace_layer_stroke {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/♻️replace-layer-stroke/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/♻️replace-layer-stroke/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/♻️replace-layer-stroke/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/♻️replace-layer-stroke/🧪️tests/adds-a-dashed-stroke/🦀️.rs"]
                                    mod tests_adds_a_dashed_stroke;
                                }
                                #[path = "."]
                                pub mod replace_layer_fill {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/🔁replace-layer-fill/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/🔁replace-layer-fill/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/🔁replace-layer-fill/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/🔁replace-layer-fill/🧪️tests/solid-to-linear-gradient/🦀️.rs"]
                                    mod tests_solid_to_linear_gradient;
                                }
                                #[path = "."]
                                pub mod set_layer_blend_mode {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/🖌️set-layer-blend-mode/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/🖌️set-layer-blend-mode/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/🖌️set-layer-blend-mode/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/🖌️set-layer-blend-mode/🧪️tests/normal-to-multiply/🦀️.rs"]
                                    mod tests_normal_to_multiply;
                                }
                                #[path = "."]
                                pub mod set_layer_opacity {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/🌫️set-layer-opacity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/🌫️set-layer-opacity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/🌫️set-layer-opacity/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️style/🧬️schema/🧬️mutations/🌫️set-layer-opacity/🧪️tests/dims-shape-a-to-half/🦀️.rs"]
                                    mod tests_dims_shape_a_to_half;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod transform {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod update_layer_transform {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️transform/🧬️schema/🧬️mutations/🔄️update-layer-transform/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️transform/🧬️schema/🧬️mutations/🔄️update-layer-transform/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️transform/🧬️schema/🧬️mutations/🔄️update-layer-transform/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️transform/🧬️schema/🧬️mutations/🔄️update-layer-transform/🧪️tests/translates-and-scales-shape-a/🦀️.rs"]
                                    mod tests_translates_and_scales_shape_a;
                                }
                                #[path = "."]
                                pub mod set_layer_boolean_operation {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️transform/🧬️schema/🧬️mutations/🔀set-layer-boolean-operation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️transform/🧬️schema/🧬️mutations/🔀set-layer-boolean-operation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️transform/🧬️schema/🧬️mutations/🔀set-layer-boolean-operation/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️transform/🧬️schema/🧬️mutations/🔀set-layer-boolean-operation/🧪️tests/union-to-subtract/🦀️.rs"]
                                    mod tests_union_to_subtract;
                                }
                                #[path = "."]
                                pub mod update_layer_trace_params {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️transform/🧬️schema/🧬️mutations/🔧update-layer-trace-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️transform/🧬️schema/🧬️mutations/🔧update-layer-trace-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️transform/🧬️schema/🧬️mutations/🔧update-layer-trace-params/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️transform/🧬️schema/🧬️mutations/🔧update-layer-trace-params/🧪️tests/sharpens-the-trace/🦀️.rs"]
                                    mod tests_sharpens_the_trace;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod metadata {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod rename_layer {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧬️schema/🧬️mutations/✏️rename-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🧪️tests/renames-shape-a-without-touching-its-id/🦀️.rs"]
                                    mod tests_renames_shape_a_without_touching_its_id;
                                }
                                #[path = "."]
                                pub mod set_layer_visible {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧬️schema/🧬️mutations/👁️set-layer-visible/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧬️schema/🧬️mutations/👁️set-layer-visible/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧬️schema/🧬️mutations/👁️set-layer-visible/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧬️schema/🧬️mutations/👁️set-layer-visible/🧪️tests/hides-shape-a/🦀️.rs"]
                                    mod tests_hides_shape_a;
                                }
                                #[path = "."]
                                pub mod set_layer_locked {
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧬️schema/🧬️mutations/🔒️set-layer-locked/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧬️schema/🧬️mutations/🔒️set-layer-locked/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧬️schema/🧬️mutations/🔒️set-layer-locked/🦠️mutation/🦀️.rs"]
                                    pub mod mutation;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️metadata/🧬️schema/🧬️mutations/🔒️set-layer-locked/🧪️tests/locks-shape-a/🦀️.rs"]
                                    mod tests_locks_shape_a;
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
            pub use crate::artifacts::draw::standards::v1::subsets::any::io::mutations::text::*;
            pub use crate::artifacts::draw::standards::v1::subsets::any::schema::mutations::{draw_op_for_layer_field, patch_layer_field, DrawMutation};
        }
        pub mod dsl {
            pub use crate::artifacts::draw::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::draw::standards::v1::subsets::any::io::mutations::binary::*;
            pub use crate::artifacts::draw::standards::v1::subsets::any::schema::owned::*;
        }
        pub mod diff {
            pub use crate::artifacts::draw::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::draw::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::draw::standards::v1::subsets::any::io::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::draw::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::artifacts::draw::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::artifacts::draw::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}

//#endregion 🗿️Artifacts

//#region ✏️Editor
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod draw {
        #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️add-layer/🦀️.rs"]
            pub mod add_layer;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-commit-draft/🦀️.rs"]
            pub mod canvas_commit_draft;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-double-click/🦀️.rs"]
            pub mod canvas_double_click;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-escape/🦀️.rs"]
            pub mod canvas_escape;
            #[path = "../../🗿️artifacts/🖍️draw/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs"]
            pub mod canvas_pointer_down;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-move/🦀️.rs"]
            pub mod canvas_pointer_move;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-up/🦀️.rs"]
            pub mod canvas_pointer_up;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️combine-boolean/🦀️.rs"]
            pub mod combine_boolean;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️commit-document/🦀️.rs"]
            pub mod commit_document;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️delete-layer/🦀️.rs"]
            pub mod delete_layer;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️drop-layer-kind/🦀️.rs"]
            pub mod drop_layer_kind;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️duplicate-layer/🦀️.rs"]
            pub mod duplicate_layer;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️move-layer/🦀️.rs"]
            pub mod move_layer;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️patch-layer/🦀️.rs"]
            pub mod patch_layer;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️patch-layers/🦀️.rs"]
            pub mod patch_layers;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-active-utility/🦀️.rs"]
            pub mod set_active_utility;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-camera-zoom/🦀️.rs"]
            pub mod set_camera_zoom;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️set-fixture-json/🦀️.rs"]
            pub mod set_fixture_json;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️set-selected-opacity/🦀️.rs"]
            pub mod set_selected_opacity;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️set-snapshot/🦀️.rs"]
            pub mod set_snapshot;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️toggle-layer-visible/🦀️.rs"]
            pub mod toggle_layer_visible;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs"]
                    pub mod canvas;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗂️layers/🦀️.rs"]
            pub mod layers;
            #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️properties/🦀️.rs"]
            pub mod properties;
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod draw {
        #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️canvas/🦀️.rs"]
                    pub mod canvas;
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
pub use plugin::DrawApps;
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::DrawApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_draw_demo_session;
    #[path = "../../🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_draw_demo;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
