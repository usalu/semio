//! 🧱️ Block plugin — 2D, 3D, and 5D single-kind-definition editors in one hot-swappable WASM plugin.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to THIS file's directory (the plugin root). The grouping modules carry
//! `#[path = "."]` so their own names are not spliced into that base directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

extern crate semio_framework_schema as schema;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as vcs;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<...Mutation, ...ConfigMutation>, Fault>`, the exact signature `DocumentApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]

#[path = "../../🦀️component.rs"]
mod block_shared;
pub use block_shared::*;

//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod block2d {
        #[path = "../../🗿️artifacts/◻2d/🦀️component.rs"]
        mod component;
        pub use component::*;
        pub use crate::artifacts::block2d::schema::snapshot::Block2dSnapshot;
        pub use crate::artifacts::block2d::schema::mutations::Block2dMutation;
        pub use crate::artifacts::block2d::schema::diff::Block2dDiff;
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
                pub mod remove_attribute {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-attribute/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-attribute/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-attribute/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_compatibility_rule {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-compatibility-rule/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-compatibility-rule/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-compatibility-rule/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_handle {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-handle/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-handle/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-handle/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_handle_kind {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-handle-kind/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-handle-kind/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/➖remove-handle-kind/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_attribute {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-attribute/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-attribute/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-attribute/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_authors {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-authors/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-authors/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-authors/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_camera2d {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-camera2d/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-camera2d/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-camera2d/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_compatibility_rule {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-compatibility-rule/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-compatibility-rule/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-compatibility-rule/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_handle {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-handle/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-handle/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-handle/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_handle_kind {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-handle-kind/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-handle-kind/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-handle-kind/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_node_kind {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-node-kind/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-node-kind/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-node-kind/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_presentation {
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-presentation/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-presentation/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/◻2d/🧬️schema/🧬️mutations/🎛set-presentation/↩️inverse/🦀️component.rs"]
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
            }
        }
        pub mod op { pub use crate::artifacts::block2d::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::block2d::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::block2d::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::block2d::schema::diff::*; pub use crate::artifacts::block2d::schema::diff::text::*; pub mod schema { pub use crate::artifacts::block2d::schema::diff::*; } pub mod text { pub use crate::artifacts::block2d::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::block2d::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::block2d::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::block2d::schema::snapshot::binary::*; } }
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
                        pub mod glb {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
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
                        pub mod obj {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
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
                        pub mod stl {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
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
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
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
                        pub mod obj {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
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
                        pub mod stl {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🦀️component.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/◻2d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
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
                    pub use crate::artifacts::block2d::io::export::serializers::artifacts::glb::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block2d::io::import::deserializers::artifacts::glb::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block2d::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block2d::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod obj {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block2d::io::export::serializers::artifacts::obj::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block2d::io::import::deserializers::artifacts::obj::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block2d::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block2d::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod stl {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block2d::io::export::serializers::artifacts::stl::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block2d::io::import::deserializers::artifacts::stl::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block2d::io::export::serializers::artifacts::zip::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block2d::io::import::deserializers::artifacts::zip::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/◻2d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod block5d {
        #[path = "../../🗿️artifacts/🖐️5d/🦀️component.rs"]
        mod component;
        pub use component::*;
        pub use crate::artifacts::block5d::schema::snapshot::Block5dSnapshot;
        pub use crate::artifacts::block5d::schema::mutations::Block5dMutation;
        pub use crate::artifacts::block5d::schema::diff::Block5dDiff;
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
                pub mod remove_attribute {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-attribute/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-attribute/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-attribute/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_compatibility_rule {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-compatibility-rule/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-compatibility-rule/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-compatibility-rule/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_grip {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-grip/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-grip/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-grip/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_grip_kind {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-grip-kind/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-grip-kind/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-grip-kind/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_representation {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-representation/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-representation/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/➖remove-representation/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_attribute {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-attribute/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-attribute/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-attribute/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_authors {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-authors/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-authors/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-authors/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_camera2d {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-camera2d/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-camera2d/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-camera2d/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_camera3d {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-camera3d/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-camera3d/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-camera3d/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_compatibility_rule {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-compatibility-rule/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-compatibility-rule/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-compatibility-rule/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_grip {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-grip/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-grip/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-grip/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_grip_kind {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-grip-kind/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-grip-kind/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-grip-kind/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_part_kind {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-part-kind/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-part-kind/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-part-kind/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_part2d {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-part2d/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-part2d/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-part2d/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_part3d {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-part3d/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-part3d/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-part3d/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_representation {
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-representation/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-representation/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🎛set-representation/↩️inverse/🦀️component.rs"]
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
        pub mod op { pub use crate::artifacts::block5d::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::block5d::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::block5d::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::block5d::schema::diff::*; pub use crate::artifacts::block5d::schema::diff::text::*; pub mod schema { pub use crate::artifacts::block5d::schema::diff::*; } pub mod text { pub use crate::artifacts::block5d::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::block5d::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::block5d::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::block5d::schema::snapshot::binary::*; } }
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
                    pub use crate::artifacts::block5d::io::export::serializers::artifacts::glb::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block5d::io::import::deserializers::artifacts::glb::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block5d::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block5d::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod obj {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block5d::io::export::serializers::artifacts::obj::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block5d::io::import::deserializers::artifacts::obj::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block5d::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block5d::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod stl {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block5d::io::export::serializers::artifacts::stl::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block5d::io::import::deserializers::artifacts::stl::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block5d::io::export::serializers::artifacts::zip::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block5d::io::import::deserializers::artifacts::zip::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🖐️5d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod block3d {
        #[path = "../../🗿️artifacts/🧊️3d/🦀️component.rs"]
        mod component;
        pub use component::*;
        pub use crate::artifacts::block3d::schema::snapshot::Block3dSnapshot;
        pub use crate::artifacts::block3d::schema::mutations::Block3dMutation;
        pub use crate::artifacts::block3d::schema::diff::Block3dDiff;
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
                pub mod remove_attribute {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-attribute/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-attribute/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-attribute/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_compatibility_rule {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-compatibility-rule/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-compatibility-rule/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-compatibility-rule/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_representation {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-representation/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-representation/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-representation/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_vortex {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-vortex/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-vortex/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-vortex/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod remove_vortex_kind {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-vortex-kind/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-vortex-kind/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/➖remove-vortex-kind/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_attribute {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-attribute/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-attribute/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-attribute/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_authors {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-authors/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-authors/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-authors/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_camera3d {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-camera3d/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-camera3d/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-camera3d/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_compatibility_rule {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-compatibility-rule/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-compatibility-rule/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-compatibility-rule/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_object_kind {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-object-kind/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-object-kind/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-object-kind/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_representation {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-representation/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-representation/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-representation/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_vortex {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-vortex/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-vortex/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-vortex/↩️inverse/🦀️component.rs"]
                    pub mod inverse;
                }
                #[path = "."]
                pub mod set_vortex_kind {
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-vortex-kind/🦠️mutation/🦀️component.rs"]
                    pub mod mutation;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-vortex-kind/🔺️diff/🦀️component.rs"]
                    pub mod diff;
                    #[path = "../../🗿️artifacts/🧊️3d/🧬️schema/🧬️mutations/🎛set-vortex-kind/↩️inverse/🦀️component.rs"]
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
        pub mod op { pub use crate::artifacts::block3d::schema::mutations::text::*; }
        pub mod dsl { pub use crate::artifacts::block3d::schema::snapshot::text::*; }
        pub mod spr { pub use crate::artifacts::block3d::schema::mutations::binary::*; }
        pub mod diff { pub use crate::artifacts::block3d::schema::diff::*; pub use crate::artifacts::block3d::schema::diff::text::*; pub mod schema { pub use crate::artifacts::block3d::schema::diff::*; } pub mod text { pub use crate::artifacts::block3d::schema::diff::text::*; } }
        pub mod mutations { pub use crate::artifacts::block3d::schema::mutations::*; }
        pub mod snapshot { pub mod schema { pub use crate::artifacts::block3d::schema::snapshot::*; } pub mod pack { pub use crate::artifacts::block3d::schema::snapshot::binary::*; } }
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
                        pub mod glb {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
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
                        pub mod obj {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
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
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
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
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️glb/🦀️component.rs"]
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
                        pub mod obj {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🦀️component.rs"]
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
                        #[path = "."]
                        pub mod zip {
                            #[path = "../../🗿️artifacts/🧊️3d/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🦀️component.rs"]
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
                    pub use crate::artifacts::block3d::io::export::serializers::artifacts::glb::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block3d::io::import::deserializers::artifacts::glb::*;
                }
            }
            #[path = "."]
            pub mod json {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block3d::io::export::serializers::artifacts::json::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block3d::io::import::deserializers::artifacts::json::*;
                }
            }
            #[path = "."]
            pub mod obj {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block3d::io::export::serializers::artifacts::obj::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block3d::io::import::deserializers::artifacts::obj::*;
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block3d::io::export::serializers::artifacts::png::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block3d::io::import::deserializers::artifacts::png::*;
                }
            }
            #[path = "."]
            pub mod stl {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block3d::io::export::serializers::artifacts::stl::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block3d::io::import::deserializers::artifacts::stl::*;
                }
            }
            #[path = "."]
            pub mod zip {
                #[path = "."]
                pub mod export {
                    pub use crate::artifacts::block3d::io::export::serializers::artifacts::zip::*;
                }
                #[path = "."]
                pub mod import {
                    pub use crate::artifacts::block3d::io::import::deserializers::artifacts::zip::*;
                }
            }
        }
        #[path = "."]
        pub mod engine {
            #[path = "../../🗿️artifacts/🧊️3d/⚙️engine/🦀️component.rs"]
            mod component;
            pub use component::*;
        }
    }
}
//#endregion 🗿️Artifacts

//#region 🎛️Apps
#[path = "."]
pub mod apps {
    #[path = "."]
    pub mod block2d {
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
            #[path = "../../🎛️apps/◻2d/🎮️commands/🏷️kind/🦀️component.rs"]
            pub mod kind;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🔘️handle-kind/🦀️component.rs"]
            pub mod handle_kind;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🌱️handle/🦀️component.rs"]
            pub mod handle;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🔗️compatibility/🦀️component.rs"]
            pub mod compatibility;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/◻2d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️component.rs"]
                    pub mod board;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/◻2d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/◻2d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod block3d {
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

        #[path = "../../🎛️apps/🧊️3d/🌍️world/🦀️component.rs"]
        pub mod world;
        #[path = "../../🎛️apps/🧊️3d/🗣️terminology/🦀️component.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🏷️kind/🦀️component.rs"]
            pub mod kind;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🧱️representation/🦀️component.rs"]
            pub mod representation;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🔘️vortex-kind/🦀️component.rs"]
            pub mod vortex_kind;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🌀️vortex/🦀️component.rs"]
            pub mod vortex;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🎨️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🪟️window/🦀️component.rs"]
            pub mod window;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🖌️brush/🦀️component.rs"]
            pub mod brush;
            #[path = "../../🎛️apps/🧊️3d/🎮️commands/🎥️camera/🦀️component.rs"]
            pub mod camera;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod world {
                        #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️component.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🎚️options/🧱️representations/🦀️component.rs"]
                            pub mod representations;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🎚️options/🔀️quick-representation/🦀️component.rs"]
                            pub mod quick_representation;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🎚️options/↔️arrangement/🦀️component.rs"]
                            pub mod arrangement;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🎚️options/📏️spacing/🦀️component.rs"]
                            pub mod spacing;
                            #[path = "../../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🌐️world/🎚️options/🖌️brush/🦀️component.rs"]
                            pub mod brush;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🧊️3d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🧊️3d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }

    #[path = "."]
    pub mod block5d {
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

        #[path = "."]
        pub mod commands {
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🏷️kind/🦀️component.rs"]
            pub mod kind;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🔘️grip-kind/🦀️component.rs"]
            pub mod grip_kind;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🌱️grip/🦀️component.rs"]
            pub mod grip;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🎨️example/🦀️component.rs"]
            pub mod example;
            #[path = "../../🎛️apps/🖐️5d/🎮️commands/🗂️selection/🦀️component.rs"]
            pub mod selection;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🦀️component.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️component.rs"]
                    pub mod board;
                    #[path = "../../🎛️apps/🖐️5d/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️component.rs"]
                    pub mod world;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🎛️apps/🖐️5d/📌️panels/📄️document/🦀️component.rs"]
            pub mod document;
            #[path = "../../🎛️apps/🖐️5d/📌️panels/🔍️inspection/🦀️component.rs"]
            pub mod inspection;
        }
    }
}
//#endregion 🎛️Apps

//#region 🔖️Plugin
#[path = "../../🔌️plugin/🦀️component.rs"]
mod plugin;
/// 🔌️ Registers block artifact codecs and pilot languages.
pub fn register_block_exports() {
    crate::artifacts::block2d::engine::register();
    crate::artifacts::block3d::engine::register();
    crate::artifacts::block5d::engine::register();
}

semio_framework_plugin::plugin_exports!(plugin::plugin);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/◻2d/📚️examples/🎬️hexagonal-cut-concrete-forest-left/🦀️component.rs"]
    pub mod art_2d_hexagonal_cut_concrete_forest_left;
    #[path = "../../🗿️artifacts/◻2d/📚️examples/🎬️hexagonal-cut-concrete-forest-right/🦀️component.rs"]
    pub mod art_2d_hexagonal_cut_concrete_forest_right;
    #[path = "../../🗿️artifacts/🖐️5d/📚️examples/🎬️hexagonal-cut-concrete-forest-left/🦀️component.rs"]
    pub mod art_5d_hexagonal_cut_concrete_forest_left;
    #[path = "../../🗿️artifacts/🖐️5d/📚️examples/🎬️nakagin-capsule/🦀️component.rs"]
    pub mod art_5d_nakagin_capsule;
    #[path = "../../🗿️artifacts/🧊️3d/📚️examples/🎬️hexagonal-cut-concrete-forest-left/🦀️component.rs"]
    pub mod art_3d_hexagonal_cut_concrete_forest_left;
    #[path = "../../🗿️artifacts/🧊️3d/📚️examples/🎬️nakagin-capsule/🦀️component.rs"]
    pub mod art_3d_nakagin_capsule;
    #[path = "../../🎛️apps/◻2d/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_2d_demo_session;
    #[path = "../../🎛️apps/🖐️5d/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_5d_demo_session;
    #[path = "../../🎛️apps/🧊️3d/📚️examples/🎬️demo-session/🦀️component.rs"]
    pub mod app_3d_demo_session;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
