//! 🪵️ Sourcing plugin — declarative curation app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every leaf `mod` below points at exactly one taxonomy component file with a `#[path]`
//! written in full from the plugin root, prefixed with `../../` since THIS file now lives two levels
//! below the plugin root (`📦️packages/🦀️rust/`, moved here for Shape V2 tree purity — see ticket
//! `26/08/05/SOURCING-SHAPE-V2-TREE-PURITY-RETROFIT`). The grouping modules keep plain `#[path = "."]`
//! (unprefixed): `#[path]` values on nested inline `mod` blocks compose by concatenation against the
//! immediately enclosing mod's already-resolved directory, so a `../../` correction applied at every
//! nesting level would stack and over-correct — applying it exactly once, at each leaf, is both correct
//! and sufficient; the grouping modules' own names are still kept out of the base by `.` so leaf paths
//! stay writable in full from the plugin root, unchanged from before the move. Do not inline any
//! component file back into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint both
//! fail on it (see master ticket `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`,
//! Single-File-Repo hazard ruling).

// 🔓️ R7 — `SourcingModule` (schema/🦀️.rs) declares `async fn` methods and is `#[dyn_enum]`-closed
// into `SourcingModules`; Send comes structurally from that concrete enum (R3), so this lint's suggested
// `-> impl Future + Send` fix is never taken, and the method is never made sync to silence it.
#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault>`, the exact signature
// `ArtifactApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split
// creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod curation {
        #[path = "../../🗿️artifacts/🗂️curation/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod entries {
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃entries/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                            pub mod operations;
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_curated_item {
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/🧪️tests/appends-a-steel-plate-to-the-curation/🦀️.rs"]
                                    mod tests_appends_a_steel_plate_to_the_curation;
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                                #[path = "."]
                                pub mod delete_curated_item {
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/🧪️tests/removes-the-clt-panel-from-the-curation/🦀️.rs"]
                                    mod tests_removes_the_clt_panel_from_the_curation;
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                                #[path = "."]
                                pub mod change_curated_item_count {
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/🦀️.rs"]
                                    mod component;
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/🧪️tests/raises-the-glulam-beam-count-to-20/🦀️.rs"]
                                    mod tests_raises_the_glulam_beam_count_to_20;
                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
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
                                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
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
            pub use crate::artifacts::curation::standards::v1::subsets::any::io::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::curation::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::curation::standards::v1::subsets::any::io::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::curation::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::curation::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::curation::standards::v1::subsets::any::io::diff::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::curation::standards::v1::subsets::any::io::diff::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::curation::standards::v1::subsets::any::io::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::curation::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::artifacts::curation::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::artifacts::curation::standards::v1::subsets::any::io::mutations::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::curation::standards::v1::subsets::any::io::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::curation::standards::v1::subsets::any::io::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::artifacts::curation::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::artifacts::curation::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::artifacts::curation::standards::v1::subsets::any::io::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::artifacts::curation::standards::v1::subsets::any::io::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::artifacts::curation::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
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
    pub mod sourcing {
        #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧺️curation-add/🦀️.rs"]
            pub mod curation_add;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧺️curation-remove/🦀️.rs"]
            pub mod curation_remove;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧺️curation-set-count/🦀️.rs"]
            pub mod curation_set_count;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧺️drop-on-curated/🦀️.rs"]
            pub mod drop_on_curated;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧺️drop-on-pool/🦀️.rs"]
            pub mod drop_on_pool;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️set-artifact-json/🦀️.rs"]
            pub mod set_artifact_json;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔍️set-filter-min-availability/🦀️.rs"]
            pub mod set_filter_min_availability;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔍️set-filter-module/🦀️.rs"]
            pub mod set_filter_module;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔍️set-filter-query/🦀️.rs"]
            pub mod set_filter_query;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔍️set-filter-typology/🦀️.rs"]
            pub mod set_filter_typology;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗣️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔍️sort-table/🦀️.rs"]
            pub mod sort_table;
            #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️stock-from-catalogue/🦀️.rs"]
            pub mod stock_from_catalogue;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧺️curated/🦀️.rs"]
                    pub mod curated;
                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🦀️.rs"]
                    pub mod grid;
                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs"]
                    pub mod pool;
                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
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
    pub mod sourcing {
        #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🏊️pool/🦀️.rs"]
                    pub mod pool;
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
pub use plugin::SourcingApps;
#[cfg(feature = "plugin-entry")]
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::SourcingApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_curation_demo_session;
    #[path = "../../🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_curation_demo;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
