//! 📜️ Imperative plugin — declarative imperative play app bundled as a hot-swappable WASM component.
//!
//! WIRING ONLY. Every `mod` below points at exactly one taxonomy component file with a `#[path]` that is
//! written in full, relative to the owner root (this file itself now lives two levels deeper, in
//! `📦️packages/🦀️rust/`, so every path carries a `../../` prefix back out to the owner root). The
//! grouping modules carry `#[path = "."]` so their own names are not spliced into that base
//! directory — without it, Rust
//! resolves an inline module's children under `<file dir>/<inline mod name>/…` and every leaf path
//! dangles. Do not inline any component file back into this one: the taxonomy validator and the
//! `TaxonomyLibShape` policy lint both fail on it (see master ticket
//! `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`, Single-File-Repo hazard ruling).

// 🧩️ 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME D3 — the five `🧩️extensions/*/🦀️.rs`
// files mounted below each carry a `#[cfg(feature = "extension-entry")]` guard on their own
// `extension_exports!` call (this crate's `Cargo.toml` declares no such feature, so it always
// evaluates false here — see any extension's own `Cargo.toml` for the full rationale).
#![allow(unexpected_cfgs)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault>`, the exact signature
// `ArtifactApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned
// error type; boxing it here would diverge from the trait it must satisfy, and the lint does not fire on
// the trait impl itself (only on the free functions the taxonomy split creates), so this is a pure
// artefact of decomposition.
#[allow(clippy::result_large_err)]
//#region 🗿️Artifacts
#[path = "."]
pub mod artifacts {
    #[path = "."]
    pub mod procedure {
        #[path = "../../🗿️artifacts/📜️procedure/🦀️.rs"]
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
                            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                            pub mod operations;
                            #[path = "."]
                            pub mod mutations {
                                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_step {
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🧪️tests/rejects-a-duplicate-step-id-at-the-root-path/🦀️.rs"]
                                    mod tests_rejects_a_duplicate_step_id_at_the_root_path;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                                #[path = "."]
                                pub mod delete_step {
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🧪️tests/rejects-a-root-step-id-addressed-inside-a-branch-body/🦀️.rs"]
                                    mod tests_rejects_a_root_step_id_addressed_inside_a_branch_body;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                                #[path = "."]
                                pub mod reorder_steps {
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🧪️tests/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place/🦀️.rs"]
                                    mod tests_warns_that_an_over_clamped_index_leaves_the_tail_step_in_place;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                                #[path = "."]
                                pub mod edit_step_params {
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/🧪️tests/warns-that-step-1-already-carries-the-requested-params/🦀️.rs"]
                                    mod tests_warns_that_step_1_already_carries_the_requested_params;
                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
            pub use crate::artifacts::procedure::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::artifacts::procedure::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::artifacts::procedure::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::artifacts::procedure::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::artifacts::procedure::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::artifacts::procedure::standards::v1::subsets::any::schema::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::artifacts::procedure::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::artifacts::procedure::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::artifacts::procedure::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
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
    pub mod procedure {
        #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs"]
        pub mod engine;

        #[path = "."]
        pub mod config {
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️add-step/🦀️.rs"]
            pub mod add_step;
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️add-step-at/🦀️.rs"]
            pub mod add_step_at;
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️move-step/🦀️.rs"]
            pub mod move_step;
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️move-step-at/🦀️.rs"]
            pub mod move_step_at;
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️remove-step/🦀️.rs"]
            pub mod remove_step;
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️remove-step-at/🦀️.rs"]
            pub mod remove_step_at;
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️run/🦀️.rs"]
            pub mod run;
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-locale/🦀️.rs"]
            pub mod set_locale;
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️set-step-params/🦀️.rs"]
            pub mod set_step_params;
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔧️set-step-params-at/🦀️.rs"]
            pub mod set_step_params_at;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️main/🦀️.rs"]
                    pub mod main;
                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️script/🦀️.rs"]
                    pub mod script;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}
//#endregion ✏️Editor

//#region 👁️Viewer
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod procedure {
        #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📋️main/🦀️.rs"]
                    pub mod main;
                    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📝️script/🦀️.rs"]
                    pub mod script;
                }
            }
        }
    }
}
//#endregion 👁️Viewer

//#region 🧩️Extensions
#[path = "."]
pub mod extensions {
    #[path = "../../🧩️extensions/🎮️control/🦀️.rs"]
    pub mod control;
    #[path = "../../🧩️extensions/📣️effect/🦀️.rs"]
    pub mod effect;
    #[path = "../../🧩️extensions/🧠️logic/🦀️.rs"]
    pub mod logic;
    #[path = "../../🧩️extensions/🧮️math/🦀️.rs"]
    pub mod math;
    #[path = "../../🧩️extensions/📝️text/🦀️.rs"]
    pub mod text;
}
//#endregion 🧩️Extensions

//#region 🕸️Wasm
// 🌉️ The wasm-bindgen `ImperativeSession` bridge that used to be re-exported here was deleted along
// with `editor::procedure::wasm`'s content — nothing ever built it for `wasm32-unknown-unknown`.
//#endregion 🕸️Wasm

//#region 🔖️Plugin
#[path = "../../🦀️.rs"]
mod plugin;
semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::ImperativeApps);

//#region 📚️Examples
#[path = "."]
pub mod examples {
    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
    pub mod app_procedure_demo_session;
    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
    pub mod art_procedure_demo;
    #[cfg(test)]
    #[path = "../../🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🦀️.rs"]
    mod art_procedure_demo_tests;
}
//#endregion 📚️Examples

//#endregion 🔖️Plugin
