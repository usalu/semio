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
extern crate infinite_canvas as infinite_board_port_directed;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as artifact_schema;
extern crate semio_framework_os_kernel as vcs;
// 🧯️ `clippy::result_large_err` — `DocumentApp::handle` and `import_media` return
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
        pub use crate::artifacts::puzzle2d::schema::snapshot::Puzzle2dSnapshot;
        pub use crate::artifacts::puzzle2d::schema::mutations::Puzzle2dMutation;
        pub use crate::artifacts::puzzle2d::schema::diff::Puzzle2dDiff;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/◻2d/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/◻2d/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/◻2d/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/◻2d/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/◻2d/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/◻2d/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/◻2d/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod remove_edge {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/✂️remove-edge/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/✂️remove-edge/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/✂️remove-edge/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_node {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-node/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-node/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-node/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_meta {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🏷set-meta/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🏷set-meta/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🏷set-meta/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_node {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/📍set-node/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/📍set-node/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/📍set-node/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_edge {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🔗set-edge/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🔗set-edge/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🔗set-edge/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::puzzle2d::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::puzzle2d::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::puzzle2d::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::puzzle2d::schema::diff::*; pub use crate::artifacts::puzzle2d::schema::diff::text::*; pub mod schema { pub use crate::artifacts::puzzle2d::schema::diff::*; } pub mod text { pub use crate::artifacts::puzzle2d::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::puzzle2d::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::puzzle2d::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::puzzle2d::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/◻2d/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/◻2d/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/◻2d/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod dwg {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod dxf {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dxf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod pdf {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️pdf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🦀️component.rs"]
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
                        pub mod dwg {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod dxf {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dxf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod pdf {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod svg {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle2d::io::export::serializers::artifacts::dwg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle2d::io::import::deserializers::artifacts::dwg::*;
                }
            }
            #[path = "."]
            pub mod dxf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle2d::io::export::serializers::artifacts::dxf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle2d::io::import::deserializers::artifacts::dxf::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle2d::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle2d::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod pdf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle2d::io::export::serializers::artifacts::pdf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle2d::io::import::deserializers::artifacts::pdf::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle2d::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle2d::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle2d::io::export::serializers::artifacts::svg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle2d::io::import::deserializers::artifacts::svg::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🎲️board-host/🦀️component.rs"]
            pub mod board_host;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/📐️layout/🦀️component.rs"]
            pub mod layout;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🔗️linking/🦀️component.rs"]
            pub mod linking;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🔣️icons/🦀️component.rs"]
            pub mod icons;
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🖌️brush/🦀️component.rs"]
            pub mod brush;
        }
    }
    #[path = "."]
    pub mod puzzle5d {
        #[path = "../../🗿️artifacts/🖐️5d/🦀️component.rs"]
        mod component;
        pub use component::*;
        pub use crate::artifacts::puzzle5d::schema::snapshot::Puzzle5dSnapshot;
        pub use crate::artifacts::puzzle5d::schema::mutations::Puzzle5dMutation;
        pub use crate::artifacts::puzzle5d::schema::diff::Puzzle5dDiff;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod remove_fastener {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-fastener/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-fastener/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-fastener/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_part {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-part/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-part/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-part/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_fastener {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-fastener/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-fastener/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-fastener/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_part {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-part/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-part/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-part/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_meta {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🏷set-meta/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🏷set-meta/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🏷set-meta/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::puzzle5d::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::puzzle5d::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::puzzle5d::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::puzzle5d::schema::diff::*; pub use crate::artifacts::puzzle5d::schema::diff::text::*; pub mod schema { pub use crate::artifacts::puzzle5d::schema::diff::*; } pub mod text { pub use crate::artifacts::puzzle5d::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::puzzle5d::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::puzzle5d::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::puzzle5d::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/🖐️5d/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🖐️5d/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod glb {
                            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod obj {
                            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stl {
                            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
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
                        pub mod glb {
                            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod obj {
                            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stl {
                            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/🖐️5d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod glb {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle5d::io::export::serializers::artifacts::glb::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle5d::io::import::deserializers::artifacts::glb::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle5d::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle5d::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod obj {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle5d::io::export::serializers::artifacts::obj::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle5d::io::import::deserializers::artifacts::obj::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle5d::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle5d::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod stl {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle5d::io::export::serializers::artifacts::stl::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle5d::io::import::deserializers::artifacts::stl::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle5d::io::export::serializers::artifacts::zip::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle5d::io::import::deserializers::artifacts::zip::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🖐️5d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/🖐️5d/⚙️engine/✂️transfer/🦀️component.rs"]
            pub mod transfer;
            #[path = "../../🗿️artifacts/🖐️5d/⚙️engine/📐️flatten/🦀️component.rs"]
            pub mod flatten;
        }
    }
    #[path = "."]
    pub mod puzzle3d {
        #[path = "../../🗿️artifacts/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;
        pub use crate::artifacts::puzzle3d::schema::snapshot::Puzzle3dSnapshot;
        pub use crate::artifacts::puzzle3d::schema::mutations::Puzzle3dMutation;
        pub use crate::artifacts::puzzle3d::schema::diff::Puzzle3dDiff;
        #[path = "."]
        pub mod schema {
            #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod snapshot {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/📸️snapshot/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/📸️snapshot/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod diff {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🔺️diff/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🔺️diff/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🔺️diff/💾️binary/🦀️component.rs"]
                pub mod binary;
            }
            #[path = "."]
            pub mod mutations {
                #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/📝️text/🦀️component.rs"]
                pub mod text;
                #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs"]
                pub mod binary;
                #[path = "."]
                pub mod remove_attraction {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-attraction/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-attraction/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-attraction/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_object {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-object/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-object/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-object/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_reference {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-reference/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-reference/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-reference/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_target_volume {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-target-volume/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-target-volume/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-target-volume/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_attraction {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-attraction/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-attraction/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-attraction/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_object {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-object/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-object/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-object/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_reference {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-reference/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-reference/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-reference/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_target_volume {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-target-volume/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-target-volume/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-target-volume/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_meta {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🏷set-meta/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🏷set-meta/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🏷set-meta/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_snapshot {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
            }
        }
        pub mod op { pub use crate::artifacts::puzzle3d::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::puzzle3d::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::puzzle3d::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::puzzle3d::schema::diff::*; pub use crate::artifacts::puzzle3d::schema::diff::text::*; pub mod schema { pub use crate::artifacts::puzzle3d::schema::diff::*; } pub mod text { pub use crate::artifacts::puzzle3d::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::puzzle3d::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::puzzle3d::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::puzzle3d::schema::snapshot::binary::*; } }
        #[path = "../../🗿️artifacts/🧊️3d/🏗️builder/🦀️component.rs"]
        pub mod builder;
        #[path = "../../🗿️artifacts/🧊️3d/🪓️decomposer/🦀️component.rs"]
        pub mod decomposer;
        #[path = "."]
        pub mod io {
            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod import {
                #[path = "."]
                pub mod deserializers {
                    #[path = "."]
                    pub mod artifacts {
                        #[path = "."]
                        pub mod dwg {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod glb {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod gltf {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod las {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod obj {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod ply {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️ply/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stl {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🦀️component.rs"]
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
                        pub mod dwg {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod glb {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod gltf {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod json {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod las {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod obj {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod ply {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️ply/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod png {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod stl {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                }
            }
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle3d::io::export::serializers::artifacts::dwg::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle3d::io::import::deserializers::artifacts::dwg::*;
                }
            }
            #[path = "."]
            pub mod glb {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle3d::io::export::serializers::artifacts::glb::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle3d::io::import::deserializers::artifacts::glb::*;
                }
            }
            #[path = "."]
            pub mod gltf {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle3d::io::export::serializers::artifacts::gltf::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle3d::io::import::deserializers::artifacts::gltf::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle3d::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle3d::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod las {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle3d::io::export::serializers::artifacts::las::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle3d::io::import::deserializers::artifacts::las::*;
                }
            }
            #[path = "."]
            pub mod obj {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle3d::io::export::serializers::artifacts::obj::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle3d::io::import::deserializers::artifacts::obj::*;
                }
            }
            #[path = "."]
            pub mod ply {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle3d::io::export::serializers::artifacts::ply::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle3d::io::import::deserializers::artifacts::ply::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle3d::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle3d::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod stl {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::puzzle3d::io::export::serializers::artifacts::stl::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::puzzle3d::io::import::deserializers::artifacts::stl::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/⏳️session/🦀️component.rs"]
            pub mod session;
            #[path = "."]
            pub mod geometry {
                #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/📐️geometry/🦀️component.rs"]
                mod component;
                pub use component::*;
                #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/📐️geometry/🎛flatten/🦀️component.rs"]
                pub mod flatten;
            }
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🪣️fill/🦀️component.rs"]
            pub mod fill;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "../../🎛️apps/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/◻2d/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/◻2d/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/◻2d/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/◻2d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/◻2d/🎮️commands/🕸️node/🦀️component.rs"]
            pub mod node;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🌐️grid/🦀️component.rs"]
            pub mod grid;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🤝️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🔭️lod/🦀️component.rs"]
            pub mod lod;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🛍️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🎲️board/🦀️component.rs"]
            pub mod board;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/◻2d/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/◻2d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🎚️options/🔭️lod/🦀️component.rs"]
                    pub mod lod;
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🎚️options/🖌️brush/🦀️component.rs"]
                    pub mod brush;
                }

                #[path = "."]
                pub mod tools {
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️component.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod overview {
                        #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/👁️overview/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖱️select/🦀️component.rs"]
                            pub mod select;
                            #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/👁️overview/🪛️utilities/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                        }
                    }

                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🔍️detail/🦀️component.rs"]
                    pub mod detail;
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🎯️selection/🦀️component.rs"]
                    pub mod selection;
                }
            }
        }
    }

    #[path = "."]
    pub mod puzzle3d {
        #[path = "../../🎛️apps/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🧊️3d/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🧊️3d/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🧊️3d/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/🧊️3d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/🧊️3d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🛍️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/👆️hover/🦀️component.rs"]
            pub mod hover;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧊️object/🦀️component.rs"]
            pub mod object;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🔗️attraction/🦀️component.rs"]
            pub mod attraction;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧊️volume/🦀️component.rs"]
            pub mod volume;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/☀️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🔭️lod/🦀️component.rs"]
            pub mod lod;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🌐️grid/🦀️component.rs"]
            pub mod grid;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/⚙️settings/🦀️component.rs"]
            pub mod settings;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🔄️transform/🦀️component.rs"]
            pub mod transform;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🪣️fill/🦀️component.rs"]
            pub mod fill;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🤝️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗣️locale/🦀️component.rs"]
            pub mod locale;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🧊️3d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/⚙️settings/🦀️component.rs"]
            pub mod settings;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🎚️options/🎥️projection/🦀️component.rs"]
                    pub mod projection;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🎚️options/🌀️vortex/🦀️component.rs"]
                    pub mod vortex;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🎚️options/🔭️lod/🦀️component.rs"]
                    pub mod lod;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🎚️options/🌐️grid/🦀️component.rs"]
                    pub mod grid;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🎚️options/🎯️select/🦀️component.rs"]
                    pub mod select;
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🎚️options/☀️sun/🦀️component.rs"]
                    pub mod sun;
                }

                #[path = "."]
                pub mod tools {
                    #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️component.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🔄️transform/🦀️component.rs"]
                            pub mod transform;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🧊️volume-brush/🦀️component.rs"]
                            pub mod volume_brush;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧊️main/🪛️utilities/🚚️world-relocate/🦀️component.rs"]
                            pub mod world_relocate;
                        }
                    }
                }
            }
        }
    }

    #[path = "."]
    pub mod puzzle5d {
        #[path = "../../🎛️apps/🖐️5d/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🎛️apps/🖐️5d/🎚️config/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🖐️5d/🎚️config/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🎛️apps/🖐️5d/👥️presence/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🎛️apps/🖐️5d/👥️presence/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }
        #[path = "../../🎛️apps/🖐️5d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;
        #[path = "../../🎛️apps/🖐️5d/🌉️wasm/🦀️component.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🛍️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🧩️part/🦀️component.rs"]
            pub mod part;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🔗️fastener/🦀️component.rs"]
            pub mod fastener;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/✏️patch/🦀️component.rs"]
            pub mod patch;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/👆️hover/🦀️component.rs"]
            pub mod hover;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/☀️sun/🦀️component.rs"]
            pub mod sun;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🔭️lod/🦀️component.rs"]
            pub mod lod;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🌐️grid/🦀️component.rs"]
            pub mod grid;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🪣️fill/🦀️component.rs"]
            pub mod fill;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🤝️engagement/🦀️component.rs"]
            pub mod engagement;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🧰️utility/🦀️component.rs"]
            pub mod utility;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🔄️transform/🦀️component.rs"]
            pub mod transform;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🎲️board/🦀️component.rs"]
            pub mod board;
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🖐️5d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🖐️5d/📌️panels/🛍️catalogue/🦀️component.rs"]
            pub mod catalogue;
            #[path = "../../🎛️apps/🖐️5d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod options {
                    #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🎚️options/🖌️brush/🦀️component.rs"]
                    pub mod brush;
                    #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🎚️options/🪣️fill/🦀️component.rs"]
                    pub mod fill;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod board2d {
                        #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🎬️actions/🦀️component.rs"]
                        pub mod actions;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🎚️options/🔭️lod/🦀️component.rs"]
                            pub mod lod;
                        }

                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🪛️utilities/🖱️select/🦀️component.rs"]
                            pub mod select;
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🪛️utilities/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/◻2d/🪛️utilities/🪣️fill/🦀️component.rs"]
                            pub mod fill;
                        }
                    }

                    #[path = "."]
                    pub mod world3d {
                        #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🧊️3d/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎬️actions/🦀️component.rs"]
                        pub mod actions;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🧊️3d/🎚️options/☀️sun/🦀️component.rs"]
                            pub mod sun;
                        }

                        #[path = "."]
                        pub mod utilities {
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🔄️transform/🦀️component.rs"]
                            pub mod transform;
                            #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🧊️3d/🪛️utilities/🚚️world-relocate/🦀️component.rs"]
                            pub mod world_relocate;
                        }
                    }
                }
            }
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "../../🗿️artifacts/◻2d/📚️examples/🏗️nakagin-capsule-tower/🦀️component.rs"]
        pub mod nakagin_capsule_tower;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/◻2d/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🦀️test.rs"]
        mod nakagin_capsule_tower_tests;
        #[path = "../../🗿️artifacts/◻2d/📚️examples/🌲️concrete-forest/🦀️component.rs"]
        pub mod concrete_forest;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/◻2d/📚️examples/🌲️concrete-forest/🧪️tests/🦀️test.rs"]
        mod concrete_forest_tests;
    }
    #[path = "."]
    pub mod puzzle3d {
        #[path = "../../🗿️artifacts/🧊️3d/📚️examples/🏗️nakagin-capsule-tower/🦀️component.rs"]
        pub mod nakagin_capsule_tower;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/🧊️3d/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🦀️test.rs"]
        mod nakagin_capsule_tower_tests;
        #[path = "../../🗿️artifacts/🧊️3d/📚️examples/🌲️concrete-forest/🦀️component.rs"]
        pub mod concrete_forest;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/🧊️3d/📚️examples/🌲️concrete-forest/🧪️tests/🦀️test.rs"]
        mod concrete_forest_tests;
    }
    #[path = "."]
    pub mod puzzle5d {
        #[path = "../../🗿️artifacts/🖐️5d/📚️examples/🏗️nakagin-capsule-tower/🦀️component.rs"]
        pub mod nakagin_capsule_tower;
        #[path = "../../🗿️artifacts/🖐️5d/📚️examples/🌙️capsule-dream/🦀️component.rs"]
        pub mod capsule_dream;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/🖐️5d/📚️examples/🏗️nakagin-capsule-tower/🧪️tests/🦀️test.rs"]
        mod nakagin_capsule_tower_tests;
        #[path = "../../🗿️artifacts/🖐️5d/📚️examples/🌙️capsule-dream/🧪️tests/🦀️test.rs"]
        mod capsule_dream_tests;
        #[path = "../../🗿️artifacts/🖐️5d/📚️examples/🌲️concrete-forest/🦀️component.rs"]
        pub mod concrete_forest;
        #[cfg(test)]
        #[path = "../../🗿️artifacts/🖐️5d/📚️examples/🌲️concrete-forest/🧪️tests/🦀️test.rs"]
        mod concrete_forest_tests;
    }
    #[path = "."]
    pub mod apps {
        #[path = "../../🎛️apps/◻2d/📚️examples/🎬️demo-session/🦀️component.rs"]
        pub mod demo_session_2d;
        #[cfg(test)]
        #[path = "../../🎛️apps/◻2d/📚️examples/🎬️demo-session/🧪️tests/🦀️test.rs"]
        mod demo_session_2d_tests;
        #[path = "../../🎛️apps/🧊️3d/📚️examples/🎬️demo-session/🦀️component.rs"]
        pub mod demo_session_3d;
        #[cfg(test)]
        #[path = "../../🎛️apps/🧊️3d/📚️examples/🎬️demo-session/🧪️tests/🦀️test.rs"]
        mod demo_session_3d_tests;
        #[path = "../../🎛️apps/🖐️5d/📚️examples/🎬️demo-session/🦀️component.rs"]
        pub mod demo_session_5d;
        #[cfg(test)]
        #[path = "../../🎛️apps/🖐️5d/📚️examples/🎬️demo-session/🧪️tests/🦀️test.rs"]
        mod demo_session_5d_tests;
    }
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
