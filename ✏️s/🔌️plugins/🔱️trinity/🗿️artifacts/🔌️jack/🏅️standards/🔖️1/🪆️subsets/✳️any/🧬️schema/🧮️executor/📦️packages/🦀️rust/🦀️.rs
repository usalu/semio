//! 🧮️ executor of `semio-s-artifact-trinity-jack` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::unnecessary_wraps)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

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
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::manifest;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::graph;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::memory;
                    pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::schema::content;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::content_read;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::composition;
                    #[path = "../../../../../../../../🔨️modules/▶️query-runner/🦀️.rs"]
                    pub mod runner;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::ast;
                    #[path = "../../🦀️.rs"]
                    pub mod executor;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::language_service;
                    pub use semio_s_artifact_trinity_jack_schema::standards::v1::subsets::any::schema::host;
                    pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::schema::snapshot;
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
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/🔎️set-query/🧪️tests/🔎️replaces-the-query/🦀️.rs"]
                            pub mod tests_replaces_the_query;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;

                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod jack {
        #[path = "."]
        pub mod transient {
            #[path = "."]
            pub mod results_window {
                pub use semio_s_artifact_trinity_jack_results_window::editor::jack::transient::results_window::*;
                #[cfg(test)]
                #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧪️tests/🧩️bounded/🦀️.rs"]
                pub mod bounded_publication_tests;
            }
        }
        #[path = "."]
        pub mod window_config {
            pub use semio_s_artifact_trinity_jack_window_config::editor::jack::window_config::*;
            #[cfg(test)]
            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🧪️tests/🔬️window/🦀️.rs"]
            pub mod tests;
        }
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod delete_selection {
                #[path = "../../../../✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod patch_nodes {
                #[path = "../../../../✏️editor/🎮️commands/🩹️patch-nodes/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod format_document {
                #[path = "../../../../✏️editor/🎮️commands/✨️format-document/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_viewport {
                #[path = "../../../../✏️editor/🎮️commands/🖥️set-viewport/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod text_edit {
                #[path = "../../../../✏️editor/🎮️commands/✏️text-edit/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_lod_mode {
                #[path = "../../../../✏️editor/🎮️commands/🔬️set-lod/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod jack {
        #[path = "../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
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
