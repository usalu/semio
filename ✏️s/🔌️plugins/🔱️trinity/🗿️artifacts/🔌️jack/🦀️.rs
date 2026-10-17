//! 🔺️ `trinity.graph` artifact — in-memory directed property port graph with compile-time manifest.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`trinity→C:graph`, jack side — see this
//! region's own doc for why this is ONE composed child, not two: the design annotation "jack; rewriting
//! = 2 graph children" attributes the two-child shape to the SEPARATE `rewriting` app's LHS/RHS rule
//! windows, not to jack. jack's own persisted `nodes`/`edges` instance data is replaced by a single
//! composed graph-capability CHILD slot (`🔖️ContentBridge` below); the compile-time `manifest`
//! (kind/property/port DEFINITIONS, resolved from `manifestId` — see `semio_framework_graph_manifest::GraphManifest`)
//! is distinct from the instance graph capability (nodes with
//! position/ports/properties, edges with source/target) and stays an ordinary inline field, unchanged.

#![allow(clippy::unnecessary_wraps)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

#[cfg(any(test, feature = "testkit"))]
#[path="🧪️tests/🧰️support/🦀️.rs"]
pub mod test_support;
#[cfg(test)]
macro_rules! test_identity {
    ($identity:ident) => {
        let mut observer_function = |_: semio_framework_value::native_encoding::NativeEncodeProgress| true;
        let observer: &mut store::os_vcs::io::binary::entity_identity::control::Observer<'_> = &mut observer_function;
        let mut $identity = store::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1 << 20, observer).expect("finite test identity admission");
    };
}

// #region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_trinity_jack_jack::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_trinity_jack_jack::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_trinity_jack_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::errors;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::manifest;

                    pub use semio_s_artifact_trinity_jack_graph_manifest_tests::standards::v1::subsets::any::schema::graph_manifest;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::graph;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::memory;
                    pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::schema::content;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::content_read;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::composition;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::json_transport;
                    pub use semio_s_artifact_trinity_jack_executor::standards::v1::subsets::any::schema::runner;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::ast;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::lexer;
                    pub use semio_s_artifact_trinity_jack_executor::standards::v1::subsets::any::schema::executor;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::language_service;
                    pub use semio_s_artifact_trinity_jack_schema::standards::v1::subsets::any::schema::host;
                    pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::operations;

                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod set_query {
                            pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::mutations::set_query::*;

                            pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::mutations::set_query::diff;
                            pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::mutations::set_query::inverse;

                        }
                    }
                }
                pub use semio_s_artifact_trinity_jack_jack::standards::v1::subsets::any::registration;
                pub mod io {
    pub use semio_s_artifact_trinity_jack_executor::standards::v1::subsets::any::io::*;
    pub mod binary {
        pub use semio_s_artifact_trinity_jack_executor::standards::v1::subsets::any::io::binary::*;
        pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::io::binary::diff;
        pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::io::binary::inferences;
    }
    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::io::json_native;
    pub mod text {
        pub use semio_s_artifact_trinity_jack_executor::standards::v1::subsets::any::io::text::*;
        pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::io::text::inferences;
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
        pub use semio_s_artifact_trinity_jack_model::examples::demo::*;

    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod jack {
        pub use semio_s_artifact_trinity_jack_jack::editor::jack::*;
        pub use semio_s_artifact_trinity_jack_model::editor::jack::lod;

        pub use semio_s_artifact_trinity_jack_model::editor::jack::presence;
        pub use semio_s_artifact_trinity_jack_model::editor::jack::config;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                pub use semio_s_artifact_trinity_jack_model::editor::jack::examples::demo_session::*;

            }
        }

        pub use semio_s_artifact_trinity_jack_window_config::editor::jack::window_config;

        pub use semio_s_artifact_trinity_jack_transient::editor::jack::transient;

        pub use semio_s_artifact_trinity_jack_model::editor::jack::terminology;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_trinity_jack_jack::editor::jack::commands::query;

            pub use semio_s_artifact_trinity_jack_jack::editor::jack::commands::load_document_json;

            pub use semio_s_artifact_trinity_jack_executor::editor::jack::commands::delete_selection;

            pub use semio_s_artifact_trinity_jack_executor::editor::jack::commands::patch_nodes;

            pub use semio_s_artifact_trinity_jack_jack::editor::jack::commands::set_active_example;

            pub use semio_s_artifact_trinity_jack_executor::editor::jack::commands::format_document;

            pub use semio_s_artifact_trinity_jack_executor::editor::jack::commands::set_viewport;

            pub use semio_s_artifact_trinity_jack_executor::editor::jack::commands::text_edit;

            pub use semio_s_artifact_trinity_jack_executor::editor::jack::commands::set_lod_mode;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_trinity_jack_jack::editor::jack::modes::edit::*;

                pub use semio_s_artifact_trinity_jack_model::editor::jack::modes::edit::tools;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_trinity_jack_jack::editor::jack::modes::edit::windows::graph;

                    pub use semio_s_artifact_trinity_jack_jack::editor::jack::modes::edit::windows::editor;

                    pub use semio_s_artifact_trinity_jack_jack::editor::jack::modes::edit::windows::results;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_trinity_jack_jack::editor::jack::panels::document;

            pub use semio_s_artifact_trinity_jack_jack::editor::jack::panels::catalogue;

            pub use semio_s_artifact_trinity_jack_jack::editor::jack::panels::inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod jack {
        pub use semio_s_artifact_trinity_jack_executor::viewer::jack::*;

        pub use semio_s_artifact_trinity_jack_model::viewer::jack::presence;
        pub use semio_s_artifact_trinity_jack_model::viewer::jack::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_trinity_jack_language_service::viewer::jack::modes::view::*;

                pub use semio_s_artifact_trinity_jack_language_service::viewer::jack::modes::view::windows;
            }
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🪆️record-owner/🦀️.rs"]
mod canonical_record_owner_tests;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️empty-document/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️executor/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
#[cfg(test)]
#[path = "🔨️modules/🏠️host/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_2;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_3;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_4;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛️flat-position/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_5;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_6;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_7;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_8;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_9;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_10;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-query/🧵️job/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_11;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_12;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️graph/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_13;
