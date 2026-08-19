//! 🔺️ Trinity plugin — Jack and Rewrite apps in one hot-swappable WASM plugin.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]`
//! written in full, relative to THIS file's directory (`📦️packages/🦀️rust/`, per SHAPE V2 —
//! `26/08/05/SHAPE-V2-TREE-PURITY-BROADCAST` — so every leaf path carries a `../../` prefix to reach
//! back out to the owner-root tree). The grouping modules carry `#[path = "."]` so their own names
//! are not spliced into that base directory.

extern crate infinite_canvas as infinite_board_port_directed_normal;
extern crate infinite_canvas as infinite_board_port_directed;
#[allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
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
#[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌳️ast/🦀️component.rs"]
pub mod ast;
#[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔤️lexer/🦀️component.rs"]
pub mod lexer;
#[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🦀️component.rs"]
pub mod executor;
#[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗣️language-service/🦀️component.rs"]
pub mod language_service;
pub use language_service as core;
//#endregion 🔤️Jack kernel

//#region 🔖️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod rewrite {
        #[path = "../../🗿️artifacts/♻️rewrite/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod edit_before_fixture {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod edit_lhs {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod edit_rhs {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_parameter_binding {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod remove_parameter_binding {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_rule_layout_point {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod remove_rule_layout_point {
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
        pub mod op { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::RewriteSnapshot;
        pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
        pub use crate::artifacts::rewrite::standards::v1::subsets::any::schema::diff::RewriteDiff;


        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
    #[path = "."]
    pub mod jack {
        #[path = "../../🗿️artifacts/🔌️jack/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                                #[path = "."]
                                pub mod flat_position {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛flat-position/🦀️component.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                                pub mod text;
                                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                                pub mod binary;
                                #[path = "."]
                                pub mod create_node {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-node/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod delete_node {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-node/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod create_edge {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️create-edge/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod delete_edge {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️delete-edge/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod rename_node {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod move_node {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-node/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod change_data_property {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-data-property/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                                #[path = "."]
                                pub mod remove_data_property {
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🦠️mutation/🦀️component.rs"]
                                    pub mod mutation;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/🔺️diff/🦀️component.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-data-property/↩️inverse/🦀️component.rs"]
                                    pub mod inverse;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs"]
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
                                                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs"]
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
        pub mod op { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::*; pub mod schema { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::*; } pub mod text { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::text::*; } pub mod pack { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::binary::*; } pub mod binary { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::binary::*; } }
        pub mod mutations { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::*; pub mod schema { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::*; } pub mod text { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::text::*; } pub mod pack { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::binary::*; } pub mod binary { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::binary::*; } }
        pub mod snapshot { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::*; pub mod schema { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::*; } pub mod text { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::text::*; } pub mod pack { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::binary::*; } pub mod binary { pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::binary::*; } }
        pub use crate::artifacts::jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
        pub use crate::artifacts::jack::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
        pub use crate::artifacts::jack::standards::v1::subsets::any::schema::diff::JackDiff;


        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
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
        #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod terminology {
            #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[cfg(target_arch = "wasm32")]
        #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            // 🕹️ `query` stays a named submodule (rather than flattened) — `seeded_jack_config` and
            // the catalogue panel both reach into it via `commands::query::{run_jack_query,preset_query}`.
            #[path = "."]
            pub(crate) mod query {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️run-query/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use query::run_query;

            // 🕹️ Every other command file is self-contained (its own private copy of any shared
            // helpers) and exposes exactly one `pub(crate) fn` matching its directory's verb —
            // re-exported here by name, flat, matching how `TrinityJackCommand::handle` calls them.
            #[path = "."]
            mod set_fixture_json_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️set-fixture-json/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_fixture_json_leaf::set_fixture_json;

            #[path = "."]
            mod delete_selection_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️delete-selection/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use delete_selection_leaf::delete_selection;

            #[path = "."]
            mod patch_nodes_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️patch-nodes/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use patch_nodes_leaf::patch_nodes;

            #[path = "."]
            mod reorganize_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️reorganize/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use reorganize_leaf::reorganize;

            #[path = "."]
            mod load_example_query_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️load-example-query/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use load_example_query_leaf::load_example_query;

            #[path = "."]
            mod set_active_example_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️set-active-example/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_active_example_leaf::set_active_example;

            #[path = "."]
            mod format_document_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️format-document/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use format_document_leaf::format_document;

            #[path = "."]
            mod request_completions_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️request-completions/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use request_completions_leaf::request_completions;

            #[path = "."]
            mod set_viewport_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-viewport/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_viewport_leaf::set_viewport;

            #[path = "."]
            mod text_edit_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️text-edit/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use text_edit_leaf::text_edit;

            #[path = "."]
            mod text_select_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️text-select/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use text_select_leaf::text_select;

            #[path = "."]
            mod set_lod_mode_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-lod-mode/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_lod_mode_leaf::set_lod_mode;

            #[path = "."]
            mod editor_engagement_input_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️editor-engagement-input/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use editor_engagement_input_leaf::editor_engagement_input;

            #[path = "."]
            mod graph_engagement_input_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️graph-engagement-input/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use graph_engagement_input_leaf::graph_engagement_input;

            #[path = "."]
            mod results_engagement_input_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️results-engagement-input/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use results_engagement_input_leaf::results_engagement_input;

            #[path = "."]
            mod set_locale_leaf {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-locale/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_locale_leaf::set_locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
            #[path = "."]
            pub(crate) mod graph {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod editor {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️editor/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod results {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️component.rs"]
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
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod catalogue {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📚️catalogue/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod inspection {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }
    }

    #[path = "."]
    pub mod rewrite {
        #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod terminology {
            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod world {
            #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌍️world/🦀️component.rs"]
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
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️node-graph-edit/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use node_graph_edit_leaf::node_graph_edit;

            #[path = "."]
            mod set_lhs_json_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️set-lhs-json/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_lhs_json_leaf::set_lhs_json;

            #[path = "."]
            mod set_rhs_json_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️set-rhs-json/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_rhs_json_leaf::set_rhs_json;

            #[path = "."]
            mod set_parameter_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️set-parameter/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_parameter_leaf::set_parameter;

            #[path = "."]
            mod add_rule_clause_command_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️add-rule-clause-command/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use add_rule_clause_command_leaf::add_rule_clause_command;

            #[path = "."]
            mod reset_rule_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️reset-rule/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use reset_rule_leaf::reset_rule;

            #[path = "."]
            mod patch_nodes_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📜️patch-nodes/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use patch_nodes_leaf::patch_nodes;

            #[path = "."]
            mod set_viewport_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-viewport/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_viewport_leaf::set_viewport;

            #[path = "."]
            mod reorganize_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️reorganize/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use reorganize_leaf::reorganize;

            #[path = "."]
            mod set_lod_mode_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-lod-mode/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_lod_mode_leaf::set_lod_mode;

            #[path = "."]
            mod set_locale_leaf {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-locale/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
            pub(crate) use set_locale_leaf::set_locale;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
            #[path = "."]
            pub(crate) mod before {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/⬅️before/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod after {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/⏭️after/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod lhs {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👈️lhs/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod rhs {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/➡️rhs/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod jack {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔎️jack/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod parameters {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎛️parameters/🦀️component.rs"]
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
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod catalogue {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📚️catalogue/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod inspection {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️component.rs"]
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
        #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️graph/🦀️component.rs"]
                    pub mod graph;
                }
            }
        }
    }

    #[path = "."]
    pub mod rewrite {
        #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📜️rule/🦀️component.rs"]
                    pub mod rule;
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Bundle
//#region 🔖️Plugin
#[path = "../../🦀️component.rs"]
mod plugin;
#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_rewrite_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
    mod art_rewrite_demo_tests;
    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_jack_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️test.rs"]
    mod art_jack_demo_tests;
    #[path = "../../🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_rewrite_demo_session;
    #[path = "../../🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_jack_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
//#endregion 🔖️Bundle
