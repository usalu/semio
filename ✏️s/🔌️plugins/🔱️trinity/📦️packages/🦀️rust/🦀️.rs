//! 🔺️ Trinity plugin — Jack and Rewrite apps in one hot-swappable WASM plugin.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]`
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, per SHAPE V2 —
//! `26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST` — so every leaf path carries a `../../` prefix to reach
//! back out to the owner-root tree). The grouping modules carry `#[path = "."]` so their own names
//! are not spliced into that base directory.

extern crate infinite_canvas as infinite_board_port_directed;
extern crate infinite_canvas as infinite_board_port_directed_normal;
#[allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;
// 📌️ Command handler functions (`🎮️commands/<verb-noun>/component.rs`) are decomposed out of a
// single `ArtifactApp::handle` match, one function per command — the uniform `Result<Emit<_, _>,
// Fault>` signature is dictated by the dispatch call site (some commands DO fail; others never do),
// so per-function `Ok(...)`-only bodies are intentional, not a mistake to unwrap.
#[allow(clippy::unnecessary_wraps)]
//#region 🔤️Jack kernel
// 🫀️ Cross-artifact query-language kernel, physically homed under `jack`'s own artifact `🧬️schema`
// (the DSL it implements is literally the "jack query language" per every file's own docstring)
// but consumed unchanged as `crate::{ast,lexer,executor,language_service}` by both the `jack` app
// and the `rewrite` artifact's `apply_rule` — see `🗿️artifacts/♻️rewrite/…/🧬️schema/🦀️component.rs`'s
// `🔖️RuleApplication` region, which names this exact sharing relationship (rehomed off the deleted
// `⚙️engine` dir by `26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES`). Moved
// `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`; module names kept stable at crate root so no call
// site elsewhere in the crate needed to change.
#[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌳️ast/🦀️.rs"]
pub mod ast;
#[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🦀️.rs"]
pub mod executor;
#[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗣️language-service/🦀️.rs"]
pub mod language_service;
#[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔤️lexer/🦀️.rs"]
pub mod lexer;
pub use language_service as core;
//#endregion 🔤️Jack kernel

//#region 🔖️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod rewrite {
        #[path = "../../🗿️artifacts/♻️rewrite/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod operations {
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod edit_before_fixture {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/🧪️tests/swaps-in-a-two-node-before-graph/🦀️.rs"]
                                    mod tests_swaps_in_a_two_node_before_graph;
                                }
                                #[path = "."]
                                pub mod edit_lhs {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/🧪️tests/narrows-the-lhs-pattern-to-a-shaft-neighbour/🦀️.rs"]
                                    mod tests_narrows_the_lhs_pattern_to_a_shaft_neighbour;
                                }
                                #[path = "."]
                                pub mod edit_rhs {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/🧪️tests/rewrites-the-rhs-to-set-a-second-property/🦀️.rs"]
                                    mod tests_rewrites_the_rhs_to_set_a_second_property;
                                }
                                #[path = "."]
                                pub mod change_parameter_binding {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/🧪️tests/retitles-the-caption-binding/🦀️.rs"]
                                    mod tests_retitles_the_caption_binding;
                                }
                                #[path = "."]
                                pub mod remove_parameter_binding {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/🧪️tests/drops-the-repeat-binding/🦀️.rs"]
                                    mod tests_drops_the_repeat_binding;
                                }
                                #[path = "."]
                                pub mod change_rule_layout_point {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/🧪️tests/nudges-the-capsule-var-off-the-shaft/🦀️.rs"]
                                    mod tests_nudges_the_capsule_var_off_the_shaft;
                                }
                                #[path = "."]
                                pub mod remove_rule_layout_point {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/🧪️tests/clears-the-shaft-layout-point/🦀️.rs"]
                                    mod tests_clears_the_shaft_layout_point;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod docx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod docx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
            pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::RewriteDiff;
        pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
        pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::operations::*;
        pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::RewriteSnapshot;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod jack {
        #[path = "../../🗿️artifacts/🔌️jack/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod flat_position {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛flat-position/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod operations {
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod wire_runtime {
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️wire-runtime/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_node {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/🧪️tests/rejects-a-node-id-the-scene-already-holds/🦀️.rs"]
                                    mod tests_rejects_a_node_id_the_scene_already_holds;
                                }
                                #[path = "."]
                                pub mod delete_node {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/rejects-deleting-a-node-the-scene-never-had/🦀️.rs"]
                                    mod tests_rejects_deleting_a_node_the_scene_never_had;
                                }
                                #[path = "."]
                                pub mod create_edge {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/🧪️tests/rejects-an-edge-whose-endpoints-are-absent/🦀️.rs"]
                                    mod tests_rejects_an_edge_whose_endpoints_are_absent;
                                }
                                #[path = "."]
                                pub mod delete_edge {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/🧪️tests/rejects-cutting-an-edge-the-scene-never-had/🦀️.rs"]
                                    mod tests_rejects_cutting_an_edge_the_scene_never_had;
                                }
                                #[path = "."]
                                pub mod rename_node {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🧪️tests/keeps-the-name-a-node-already-carries/🦀️.rs"]
                                    mod tests_keeps_the_name_a_node_already_carries;
                                }
                                #[path = "."]
                                pub mod move_node {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🧪️tests/keeps-a-node-at-the-point-it-already-occupies/🦀️.rs"]
                                    mod tests_keeps_a_node_at_the_point_it_already_occupies;
                                }
                                #[path = "."]
                                pub mod change_data_property {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🧪️tests/keeps-a-node-property-at-the-value-it-already-holds/🦀️.rs"]
                                    mod tests_keeps_a_node_property_at_the_value_it_already_holds;
                                }
                                #[path = "."]
                                pub mod remove_data_property {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/📝️text/🦀️.rs"]
                                    pub mod text;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🧪️tests/keeps-an-edge-without-the-property-it-never-had/🦀️.rs"]
                                    mod tests_keeps_an_edge_without_the_property_it_never_had;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
            pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::JackDiff;
        pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
        pub use crate::artifacts::jack::standards::v1::subsets::any::schema::operations::*;
        pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
//#endregion 🔖️Artifacts

//#region ✏️Editor
/// ✏️ The mutation-capable surface (contract §2.1/§2.4) — every leaf `#[path]`-mounted from the real
/// subset dirs under `🗿️artifacts/<jack|rewrite>/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`.
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod jack {
        #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod terminology {
            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
            mod component;
            pub use component::*;
        }

        #[cfg(target_arch = "wasm32")]
        #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            // 🕹️ `query` stays a named submodule (rather than flattened) — `seeded_jack_config` and
            // the catalogue panel both reach into it via `commands::query::{run_jack_query,preset_query}`.
            #[path = "."]
            pub(crate) mod query {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️run-query/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use query::run_query;

            // 🕹️ Every other command file is self-contained (its own private copy of any shared
            // helpers) and exposes exactly one `pub(crate) fn` matching its directory's verb —
            // re-exported here by name, flat, matching how `TrinityJackCommand::handle` calls them.
            #[path = "."]
            mod set_fixture_json_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️set-fixture-json/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_fixture_json_leaf::set_fixture_json;

            #[path = "."]
            mod delete_selection_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️delete-selection/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use delete_selection_leaf::delete_selection;

            #[path = "."]
            mod patch_nodes_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️patch-nodes/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use patch_nodes_leaf::patch_nodes;

            #[path = "."]
            mod reorganize_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️reorganize/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use reorganize_leaf::reorganize;

            #[path = "."]
            mod load_example_query_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️load-example-query/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use load_example_query_leaf::load_example_query;

            #[path = "."]
            mod set_active_example_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️set-active-example/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_active_example_leaf::set_active_example;

            #[path = "."]
            mod format_document_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️format-document/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use format_document_leaf::format_document;

            #[path = "."]
            mod request_completions_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️request-completions/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use request_completions_leaf::request_completions;

            #[path = "."]
            mod set_viewport_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-viewport/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_viewport_leaf::set_viewport;

            #[path = "."]
            mod text_edit_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️text-edit/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use text_edit_leaf::text_edit;

            #[path = "."]
            mod text_select_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️text-select/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use text_select_leaf::text_select;

            #[path = "."]
            mod set_lod_mode_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-lod-mode/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_lod_mode_leaf::set_lod_mode;

            #[path = "."]
            mod editor_engagement_input_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️editor-engagement-input/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use editor_engagement_input_leaf::editor_engagement_input;

            #[path = "."]
            mod graph_engagement_input_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️graph-engagement-input/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use graph_engagement_input_leaf::graph_engagement_input;

            #[path = "."]
            mod results_engagement_input_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️results-engagement-input/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use results_engagement_input_leaf::results_engagement_input;

            #[path = "."]
            mod set_locale_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-locale/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_locale_leaf::set_locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub(crate) mod graph {
                        #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }

                    #[path = "."]
                    pub(crate) mod editor {
                        #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }

                    #[path = "."]
                    pub(crate) mod results {
                        #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub(crate) mod document {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod catalogue {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📚️catalogue/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod inspection {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }
    }

    #[path = "."]
    pub mod rewrite {
        #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod terminology {
            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod world {
            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌍️world/🦀️.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod commands {
            // 🕹️ Every command file is self-contained (its own private copy of any shared
            // helpers) and exposes exactly one `pub(crate) fn` matching its directory's verb —
            // re-exported here by name, flat, matching how `TrinityRewriteCommand::handle` calls them.
            #[path = "."]
            mod node_graph_edit_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️node-graph-edit/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use node_graph_edit_leaf::node_graph_edit;

            #[path = "."]
            mod set_lhs_json_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️set-lhs-json/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_lhs_json_leaf::set_lhs_json;

            #[path = "."]
            mod set_rhs_json_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️set-rhs-json/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_rhs_json_leaf::set_rhs_json;

            #[path = "."]
            mod set_parameter_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️set-parameter/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_parameter_leaf::set_parameter;

            #[path = "."]
            mod add_rule_clause_command_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️add-rule-clause-command/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use add_rule_clause_command_leaf::add_rule_clause_command;

            #[path = "."]
            mod reset_rule_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️reset-rule/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use reset_rule_leaf::reset_rule;

            #[path = "."]
            mod patch_nodes_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️patch-nodes/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use patch_nodes_leaf::patch_nodes;

            #[path = "."]
            mod set_viewport_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-viewport/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_viewport_leaf::set_viewport;

            #[path = "."]
            mod reorganize_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️reorganize/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use reorganize_leaf::reorganize;

            #[path = "."]
            mod set_lod_mode_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-lod-mode/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_lod_mode_leaf::set_lod_mode;

            #[path = "."]
            mod set_locale_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-locale/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_locale_leaf::set_locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub(crate) mod before {
                        #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/⬅️before/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }

                    #[path = "."]
                    pub(crate) mod after {
                        #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/⏭️after/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }

                    #[path = "."]
                    pub(crate) mod lhs {
                        #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👈️lhs/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }

                    #[path = "."]
                    pub(crate) mod rhs {
                        #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/➡️rhs/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }

                    #[path = "."]
                    pub(crate) mod jack {
                        #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔎️jack/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }

                    #[path = "."]
                    pub(crate) mod parameters {
                        #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎛️parameters/🦀️.rs"]
                        mod component;
                        pub(crate) use component::*;
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub(crate) mod document {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod catalogue {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📚️catalogue/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod inspection {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
/// 👁️ The read-only surface (contract §2.2/§2.6) — a genuinely independent module tree from
/// `editor` above, never `#[path]`-mounting anything under `✏️editor/`: that would let
/// `policyViewerPurityBreaches`' `::editor::` substring check catch a real dependency, but the
/// deeper reason is architectural — the viewers must stay constructible without ever touching the
/// editors' mutation-capable types.
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod jack {
        #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️graph/🦀️.rs"]
                    pub mod graph;
                }
            }
        }
    }

    #[path = "."]
    pub mod rewrite {
        #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📜️rule/🦀️.rs"]
                    pub mod rule;
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Bundle
//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
pub use plugin::TrinityApps;
#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::TrinityApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_jack_demo_session;
    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_rewrite_demo_session;
    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_jack_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
    mod art_jack_demo_tests;
    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_rewrite_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
    mod art_rewrite_demo_tests;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
//#endregion 🔖️Bundle
