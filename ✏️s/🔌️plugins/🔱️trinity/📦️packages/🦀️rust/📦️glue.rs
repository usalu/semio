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
// 📌️ Command-group handler functions (`🎮️commands/<group>/component.rs`) are decomposed out of a
// single `DocumentApp::handle` match, one function per command — the uniform `Result<Emit<_, _>,
// Fault>` signature is dictated by the dispatch call site (some commands in the same group DO fail;
// others never do), so per-function `Ok(...)`-only bodies are intentional, not a mistake to unwrap.
#[allow(clippy::unnecessary_wraps)]

//#region 🔤️Jack kernel
#[path = "../../🌳️ast/🦀️component.rs"]
pub mod ast;
#[path = "../../🔤️lexer/🦀️component.rs"]
pub mod lexer;
#[path = "../../🧮️executor/🦀️component.rs"]
pub mod executor;
#[path = "../../🗣️language-service/🦀️component.rs"]
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
        pub use crate::artifacts::rewrite::schema::snapshot::RewriteSnapshot;
        pub use crate::artifacts::rewrite::schema::mutations::RewriteRuleMutation;
        pub use crate::artifacts::rewrite::schema::diff::RewriteDiff;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod set_state {
                    #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/🧬️mutations/🎛set-state/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/🧬️mutations/🎛set-state/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/♻️rewrite/🧬️schema/🧬️mutations/🎛set-state/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::rewrite::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::rewrite::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::rewrite::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::rewrite::schema::diff::*; pub use crate::artifacts::rewrite::schema::diff::text::*; pub mod schema { pub use crate::artifacts::rewrite::schema::diff::*; } pub mod text { pub use crate::artifacts::rewrite::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::rewrite::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::rewrite::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::rewrite::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/♻️rewrite/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/♻️rewrite/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/♻️rewrite/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod docx {
                            #[path = "../../🗿️artifacts/♻️rewrite/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📜️docx/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/♻️rewrite/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod md {
                            #[path = "../../🗿️artifacts/♻️rewrite/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod pdf {
                            #[path = "../../🗿️artifacts/♻️rewrite/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod txt {
                            #[path = "../../🗿️artifacts/♻️rewrite/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🦀️component.rs"]
                            mod component;
                            pub use component::*;
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
                        pub mod docx {
                            #[path = "../../🗿️artifacts/♻️rewrite/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📜️docx/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/♻️rewrite/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod md {
                            #[path = "../../🗿️artifacts/♻️rewrite/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod pdf {
                            #[path = "../../🗿️artifacts/♻️rewrite/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod txt {
                            #[path = "../../🗿️artifacts/♻️rewrite/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod docx {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::rewrite::io::export::serializers::artifacts::docx::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::rewrite::io::import::deserializers::artifacts::docx::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::rewrite::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::rewrite::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod md {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::rewrite::io::export::serializers::artifacts::md::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::rewrite::io::import::deserializers::artifacts::md::*;
                }
            }
            #[path = "."]
            pub mod pdf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::rewrite::io::export::serializers::artifacts::pdf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::rewrite::io::import::deserializers::artifacts::pdf::*;
                }
            }
            #[path = "."]
            pub mod txt {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::rewrite::io::export::serializers::artifacts::txt::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::rewrite::io::import::deserializers::artifacts::txt::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/♻️rewrite/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod jack {
        #[path = "../../🗿️artifacts/🔌️jack/🦀️component.rs"]
        mod component;
        pub use component::*;
        pub use crate::artifacts::jack::schema::snapshot::JackSnapshot;
        pub use crate::artifacts::jack::schema::mutations::TrinityGraphMutation;
        pub use crate::artifacts::jack::schema::diff::JackDiff;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod set_data_property {
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/🎛set-data-property/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/🎛set-data-property/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/🎛set-data-property/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_fixture {
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/🎛set-fixture/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/🎛set-fixture/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/🎛set-fixture/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod clear_data_property {
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌clear-data-property/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌clear-data-property/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌clear-data-property/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod create_edge {
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌create-edge/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌create-edge/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌create-edge/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod create_node {
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌create-node/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌create-node/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌create-node/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod delete_edge {
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌delete-edge/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌delete-edge/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌delete-edge/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod delete_node {
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌delete-node/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌delete-node/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌delete-node/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod rename {
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌rename/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌rename/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌rename/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod reposition {
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌reposition/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌reposition/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🔌️jack/🧬️schema/🧬️mutations/📌reposition/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::jack::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::jack::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::jack::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::jack::schema::diff::*; pub use crate::artifacts::jack::schema::diff::text::*; pub mod schema { pub use crate::artifacts::jack::schema::diff::*; } pub mod text { pub use crate::artifacts::jack::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::jack::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::jack::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::jack::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/🔌️jack/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🔌️jack/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🔌️jack/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod csv {
                            #[path = "../../🗿️artifacts/🔌️jack/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🔌️jack/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod md {
                            #[path = "../../🗿️artifacts/🔌️jack/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🔌️jack/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../🗿️artifacts/🔌️jack/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
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
                        pub mod csv {
                            #[path = "../../🗿️artifacts/🔌️jack/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🔌️jack/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod md {
                            #[path = "../../🗿️artifacts/🔌️jack/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🔌️jack/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../🗿️artifacts/🔌️jack/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod csv {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::jack::io::export::serializers::artifacts::csv::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::jack::io::import::deserializers::artifacts::csv::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::jack::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::jack::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod md {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::jack::io::export::serializers::artifacts::md::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::jack::io::import::deserializers::artifacts::md::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::jack::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::jack::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::jack::io::export::serializers::artifacts::svg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::jack::io::import::deserializers::artifacts::svg::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🔌️jack/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
    }
}
//#endregion 🔖️Artifacts

//#region 🔖️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod jack {
        #[path = "../../🎛️apps/🔌️jack/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🔌️jack/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🔌️jack/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🔌️jack/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🔌️jack/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod terminology {
            #[path = "../../🎛️apps/🔌️jack/🗣️terminology/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[cfg(target_arch = "wasm32")]
        #[path = "../../🎛️apps/🔌️jack/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub(crate) mod fixture {
                #[path = "../../🎛️apps/🔌️jack/🎮️commands/🗺️fixture/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod query {
                #[path = "../../🎛️apps/🔌️jack/🎮️commands/🔎️query/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod view {
                #[path = "../../🎛️apps/🔌️jack/🎮️commands/👁️view/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }

        #[path = "."]
        pub mod windows {
            #[path = "."]
            pub(crate) mod graph {
                #[path = "../../🎛️apps/🔌️jack/🪟️windows/🌐️graph/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod editor {
                #[path = "../../🎛️apps/🔌️jack/🪟️windows/📝️editor/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod results {
                #[path = "../../🎛️apps/🔌️jack/🪟️windows/📊️results/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub(crate) mod document {
                #[path = "../../🎛️apps/🔌️jack/📌️panels/📄️document/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod catalogue {
                #[path = "../../🎛️apps/🔌️jack/📌️panels/📚️catalogue/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod inspection {
                #[path = "../../🎛️apps/🔌️jack/📌️panels/🔍️inspection/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }
    }

    #[path = "."]
    pub mod rewrite {
        #[path = "../../🎛️apps/♻️rewrite/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/♻️rewrite/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/♻️rewrite/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/♻️rewrite/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/♻️rewrite/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod terminology {
            #[path = "../../🎛️apps/♻️rewrite/🗣️terminology/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod world {
            #[path = "../../🎛️apps/♻️rewrite/🌍️world/🦀️component.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub(crate) mod rule {
                #[path = "../../🎛️apps/♻️rewrite/🎮️commands/📜️rule/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod view {
                #[path = "../../🎛️apps/♻️rewrite/🎮️commands/👁️view/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }

        #[path = "."]
        pub mod windows {
            #[path = "."]
            pub(crate) mod before {
                #[path = "../../🎛️apps/♻️rewrite/🪟️windows/⬅️before/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod after {
                #[path = "../../🎛️apps/♻️rewrite/🪟️windows/➡️after/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod lhs {
                #[path = "../../🎛️apps/♻️rewrite/🪟️windows/⬅️lhs/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod rhs {
                #[path = "../../🎛️apps/♻️rewrite/🪟️windows/➡️rhs/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod jack {
                #[path = "../../🎛️apps/♻️rewrite/🪟️windows/🔎️jack/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod parameters {
                #[path = "../../🎛️apps/♻️rewrite/🪟️windows/🎛️parameters/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub(crate) mod document {
                #[path = "../../🎛️apps/♻️rewrite/📌️panels/📄️document/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod catalogue {
                #[path = "../../🎛️apps/♻️rewrite/📌️panels/📚️catalogue/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }

            #[path = "."]
            pub(crate) mod inspection {
                #[path = "../../🎛️apps/♻️rewrite/📌️panels/🔍️inspection/🦀️component.rs"]
                mod component;
                pub(crate) use component::*;
            }
        }
    }
}
//#endregion 🔖️Apps

//#region 🔖️Bundle
/// 🗂️ Registers this crate's two document kinds' pack↔dsl codecs so `framework/sync`'s
/// `FolderEndpoint::Pack` (and any other schema-string-keyed caller) can print/parse them without
/// depending on the artifacts' concrete `Projection`/`Mutation` types.
fn register_trinity_exports() {
    artifacts::jack::engine::register();
    artifacts::rewrite::engine::register();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::jack::TrinityJackPlayApp>(artifacts::jack::TRINITY_GRAPH_SCHEMA);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<apps::rewrite::TrinityRewritePlayApp>(artifacts::rewrite::REWRITE_RULE_SCHEMA);
}

//#region 🔖️Plugin
#[path = "../../🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/♻️rewrite/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_rewrite_demo;
    #[path = "../../🗿️artifacts/🔌️jack/📚️examples/🎬️demo/🦀️component.rs"]
    pub mod art_jack_demo;
    #[path = "../../🎛️apps/♻️rewrite/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_rewrite_demo_session;
    #[path = "../../🎛️apps/🔌️jack/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_jack_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
//#endregion 🔖️Bundle
