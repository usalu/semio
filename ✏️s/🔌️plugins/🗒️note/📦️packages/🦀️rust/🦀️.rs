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
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<NoteMutation, NoteConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod note {
        #[path = "../../🗿️artifacts/🗒️note/🦀️.rs"]
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
                            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod outline {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧾outline/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🌳️pdf/🔖️1.4/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🌳️pdf/🔖️1.4/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️dxf/🔖️r12/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        #[path = "."]
                        pub mod examples {
                            #[path = "."]
                            pub mod demo {
                                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                        #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod ink {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod change_pencil_width {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/🧪️tests/🪄️thickens-pencil/🦀️.rs"]
                                    mod tests_thickens_pencil;
                                }
                                #[path = "."]
                                pub mod change_eraser_radius {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/🧪️tests/🟦️enlarges-eraser/🦀️.rs"]
                                    mod tests_enlarges_eraser;
                                }
                                #[path = "."]
                                pub mod change_block_ink_width {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/🧪️tests/🦉️thickens-the-sketch-stroke/🦀️.rs"]
                                    mod tests_thickens_the_sketch_stroke;
                                }
                                #[path = "."]
                                pub mod edit_block_ink_stroke {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/🧪️tests/🐺️redraws-the-sketch-polyline/🦀️.rs"]
                                    mod tests_redraws_the_sketch_polyline;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod edit_block_text {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/🧪️tests/🌳️replaces-the-intro-paragraphs/🦀️.rs"]
                                    mod tests_replaces_the_intro_paragraphs;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod math {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod edit_block_math {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/🧪️tests/🧿️replaces-the-tex-with-pythagoras/🦀️.rs"]
                                    mod tests_replaces_the_tex_with_pythagoras;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod table {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod insert_table_row {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬇️insert-table-row/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬇️insert-table-row/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬇️insert-table-row/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬇️insert-table-row/🧪️tests/🟤️appends-a-blank-third-row/🦀️.rs"]
                                    mod tests_appends_a_blank_third_row;
                                }
                                #[path = "."]
                                pub mod remove_table_row {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬆️remove-table-row/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬆️remove-table-row/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬆️remove-table-row/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬆️remove-table-row/🧪️tests/🌱️drops-the-trailing-blank-row/🦀️.rs"]
                                    mod tests_drops_the_trailing_blank_row;
                                }
                                #[path = "."]
                                pub mod insert_table_column {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/➡️insert-table-column/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/➡️insert-table-column/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/➡️insert-table-column/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/➡️insert-table-column/🧪️tests/🦅️appends-the-lettered-column-c/🦀️.rs"]
                                    mod tests_appends_the_lettered_column_c;
                                }
                                #[path = "."]
                                pub mod remove_table_column {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬅️remove-table-column/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬅️remove-table-column/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬅️remove-table-column/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬅️remove-table-column/🧪️tests/🔵️drops-the-trailing-column-b/🦀️.rs"]
                                    mod tests_drops_the_trailing_column_b;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod asset {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod create_asset {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🆕️create-asset/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🆕️create-asset/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🆕️create-asset/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🆕️create-asset/🧪️tests/🖼️adds-a-second-image-asset/🦀️.rs"]
                                    mod tests_adds_a_second_image_asset;
                                }
                                #[path = "."]
                                pub mod replace_asset_payload {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🔁️replace-asset-payload/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🔁️replace-asset-payload/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🔁️replace-asset-payload/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🔁️replace-asset-payload/🧪️tests/🔁️swaps-logo-payload-for-svg/🦀️.rs"]
                                    mod tests_swaps_logo_payload_for_svg;
                                }
                                #[path = "."]
                                pub mod delete_asset {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🗑️delete-asset/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🗑️delete-asset/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🗑️delete-asset/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖼️asset/🧬️schema/🧬️mutations/🗑️delete-asset/🧪️tests/🗑️removes-the-logo-asset/🦀️.rs"]
                                    mod tests_removes_the_logo_asset;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod block {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod create_block {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/➕️create-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/➕️create-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/➕️create-block/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/➕️create-block/🧪️tests/🧩️inserts-a-photo-block-at-root-index-2/🦀️.rs"]
                                    mod tests_inserts_a_photo_block_at_root_index_2;
                                }
                                #[path = "."]
                                pub mod delete_block {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/❌️delete-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/❌️delete-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/❌️delete-block/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/❌️delete-block/🧪️tests/🧩️removes-the-math-block/🦀️.rs"]
                                    mod tests_removes_the_math_block;
                                }
                                #[path = "."]
                                pub mod delete_blocks {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🧺️delete-blocks/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🧺️delete-blocks/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🧺️delete-blocks/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🧺️delete-blocks/🧪️tests/🖼️removes-the-ink-and-image-blocks/🦀️.rs"]
                                    mod tests_removes_the_ink_and_image_blocks;
                                }
                                #[path = "."]
                                pub mod duplicate_block {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🎯️duplicate-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🎯️duplicate-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🎯️duplicate-block/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🎯️duplicate-block/🧪️tests/🧩️copies-the-math-block-right-after-its-source/🦀️.rs"]
                                    mod tests_copies_the_math_block_right_after_its_source;
                                }
                                #[path = "."]
                                pub mod duplicate_blocks {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👥️duplicate-blocks/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👥️duplicate-blocks/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👥️duplicate-blocks/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👥️duplicate-blocks/🧪️tests/📌️copies-ink-and-table-with-shifting-indices/🦀️.rs"]
                                    mod tests_copies_ink_and_table_with_shifting_indices;
                                }
                                #[path = "."]
                                pub mod move_block_to_container {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/🧪️tests/🟩️reparents-ink-into-the-callout-group/🦀️.rs"]
                                    mod tests_reparents_ink_into_the_callout_group;
                                }
                                #[path = "."]
                                pub mod drag_blocks {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🧪️tests/🌳️nudges-ink-and-the-whole-group-subtree/🦀️.rs"]
                                    mod tests_nudges_ink_and_the_whole_group_subtree;
                                }
                                #[path = "."]
                                pub mod rename_block {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/🧪️tests/🧩️renames-the-table-block/🦀️.rs"]
                                    mod tests_renames_the_table_block;
                                }
                                #[path = "."]
                                pub mod change_block_visible {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/🧪️tests/🖼️hides-the-image-block/🦀️.rs"]
                                    mod tests_hides_the_image_block;
                                }
                                #[path = "."]
                                pub mod change_block_locked {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/🧪️tests/🔒️locks-the-callout-group/🦀️.rs"]
                                    mod tests_locks_the_callout_group;
                                }
                                #[path = "."]
                                pub mod move_block {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📍️move-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📍️move-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📍️move-block/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📍️move-block/🧪️tests/🧩️repositions-the-math-block/🦀️.rs"]
                                    mod tests_repositions_the_math_block;
                                }
                                #[path = "."]
                                pub mod resize_block {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/↔️resize-block/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/↔️resize-block/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/↔️resize-block/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/↔️resize-block/🧪️tests/🖼️enlarges-the-image-block/🦀️.rs"]
                                    mod tests_enlarges_the_image_block;
                                }
                                #[path = "."]
                                pub mod change_block_font_size {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔤️change-block-font-size/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔤️change-block-font-size/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔤️change-block-font-size/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔤️change-block-font-size/🧪️tests/🔤️enlarges-the-intro-font/🦀️.rs"]
                                    mod tests_enlarges_the_intro_font;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod canvas {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod change_grid_visible {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/🧪️tests/⚫️hides-the-grid/🦀️.rs"]
                                    mod tests_hides_the_grid;
                                }
                                #[path = "."]
                                pub mod change_grid_spacing {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/🧪️tests/🔵️widens-grid-spacing/🦀️.rs"]
                                    mod tests_widens_grid_spacing;
                                }
                                #[path = "."]
                                pub mod change_grid_subdivisions {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/🧪️tests/🐨️doubles-grid-subdivisions/🦀️.rs"]
                                    mod tests_doubles_grid_subdivisions;
                                }
                                #[path = "."]
                                pub mod change_grid_opacity {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/🧪️tests/🟢️raises-grid-opacity/🦀️.rs"]
                                    mod tests_raises_grid_opacity;
                                }
                                #[path = "."]
                                pub mod change_snap_enabled {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/🧪️tests/🛰️enables-snap/🦀️.rs"]
                                    mod tests_enables_snap;
                                }
                                #[path = "."]
                                pub mod change_snap_grid_spacing {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/🧪️tests/🌷️halves-snap-grid-spacing/🦀️.rs"]
                                    mod tests_halves_snap_grid_spacing;
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod document {
                        #[path = "."]
                        pub mod schema {
                            #[path = "."]
                            pub mod mutations {
                                #[path = "."]
                                pub mod rename_note {
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📜️document/🧬️schema/🧬️mutations/🏷️rename-note/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📜️document/🧬️schema/🧬️mutations/🏷️rename-note/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📜️document/🧬️schema/🧬️mutations/🏷️rename-note/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📜️document/🧬️schema/🧬️mutations/🏷️rename-note/🧪️tests/📃️retitles-the-document/🦀️.rs"]
                                    mod tests_retitles_the_document;
                                }
                            }
                        }
                    }
                }
                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;
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
            pub use crate::artifacts::note::standards::v1::subsets::any::io::mutations::text::*;
            pub use crate::artifacts::note::standards::v1::subsets::any::schema::mutations::NoteMutation;
        }
        pub mod dsl {
            pub use crate::artifacts::note::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::note::standards::v1::subsets::any::io::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::note::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::note::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::note::standards::v1::subsets::any::io::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::note::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::artifacts::note::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::artifacts::note::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }
        pub mod examples {
            pub use super::standards::v1::subsets::any::examples::*;
        }
    }
}
//#endregion 🗿️Artifacts

//#region ✏️Editor
/// ✏️ The mutation-capable surface (contract §2.1/§2.4) — every leaf `#[path]`-mounted from the real
/// subset dir under `🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`.
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod note {
        #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs"]
        pub mod retained;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-block/🦀️.rs"]
            pub mod add_block;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-block/🦀️.rs"]
            pub mod delete_block;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚫️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-block/🦀️.rs"]
            pub mod duplicate_block;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪞️duplicate-selection/🦀️.rs"]
            pub mod duplicate_selection;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💬️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖊️ink-apply-events/🦀️.rs"]
            pub mod ink_apply_events;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️load-request/🦀️.rs"]
            pub mod load_request;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-block/🦀️.rs"]
            pub mod move_block;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️navigator-engagement-input/🦀️.rs"]
            pub mod navigator_engagement_input;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕹️nudge-selection/🦀️.rs"]
            pub mod nudge_selection;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⬇️nudge-selection-down/🦀️.rs"]
            pub mod nudge_selection_down;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏬️nudge-selection-down-fast/🦀️.rs"]
            pub mod nudge_selection_down_fast;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⬅️nudge-selection-left/🦀️.rs"]
            pub mod nudge_selection_left;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏪️nudge-selection-left-fast/🦀️.rs"]
            pub mod nudge_selection_left_fast;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➡️nudge-selection-right/🦀️.rs"]
            pub mod nudge_selection_right;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏩️nudge-selection-right-fast/🦀️.rs"]
            pub mod nudge_selection_right_fast;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⬆️nudge-selection-up/🦀️.rs"]
            pub mod nudge_selection_up;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏫️nudge-selection-up-fast/🦀️.rs"]
            pub mod nudge_selection_up_fast;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-blocks/🦀️.rs"]
            pub mod patch_blocks;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💾️save-download/🦀️.rs"]
            pub mod save_download;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️set-active-utility/🦀️.rs"]
            pub mod set_active_utility;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️set-camera-zoom/🦀️.rs"]
            pub mod set_camera_zoom;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧽️set-eraser-radius/🦀️.rs"]
            pub mod set_eraser_radius;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧫️set-fixture-json/🦀️.rs"]
            pub mod set_fixture_json;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔲️set-grid-opacity/🦀️.rs"]
            pub mod set_grid_opacity;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️set-grid-spacing/🦀️.rs"]
            pub mod set_grid_spacing;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔢️set-grid-subdivisions/🦀️.rs"]
            pub mod set_grid_subdivisions;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-grid-visible/🦀️.rs"]
            pub mod set_grid_visible;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️set-pencil-width/🦀️.rs"]
            pub mod set_pencil_width;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️set-snap-enabled/🦀️.rs"]
            pub mod set_snap_enabled;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️set-snap-grid-spacing/🦀️.rs"]
            pub mod set_snap_grid_spacing;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod composite {
                        #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🎥️camera/🦀️.rs"]
                            pub mod camera;
                            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧹️eraser-point/🦀️.rs"]
                            pub mod eraser_point;
                            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧽️eraser-stroke/🦀️.rs"]
                            pub mod eraser_stroke;
                            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🔲️grid/🦀️.rs"]
                            pub mod grid;
                            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/✏️pencil/🦀️.rs"]
                            pub mod pencil;
                            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧲️snap/🦀️.rs"]
                            pub mod snap;
                        }
                    }

                    #[path = "."]
                    pub mod navigator {
                        #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️navigator/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️navigator/☑️options/🔲️grid-visible/🦀️.rs"]
                            pub mod grid_visible;
                            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️navigator/☑️options/🔍️zoom/🦀️.rs"]
                            pub mod zoom;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}
//#endregion ✏️Editor
//#region 👁️Viewer
/// 👁️ The read-only surface (contract §2.2/§2.6) — a genuinely independent module tree from
/// `editor` above, never `#[path]`-mounting anything under `✏️editor/`: that would let
/// `policyViewerPurityBreaches`' substring check on the sibling editor module catch a real
/// dependency, but the deeper reason is architectural — the viewer must stay constructible without
/// ever touching the editor's mutation-capable types.
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod note {
        #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️composite/🦀️.rs"]
                    pub mod composite;
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
pub use plugin::NoteApps;
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::NoteApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_note_demo_session;
    #[path = "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_note_demo;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
