//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../../🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::io::binary::mutations;
                    }
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_trinity_jack_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::graph;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::memory;
                    pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::schema::content;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::content_read;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::composition;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::ast;
                    pub use semio_s_artifact_trinity_jack_executor::standards::v1::subsets::any::schema::executor;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::language_service;
                    pub use semio_s_artifact_trinity_jack_schema::standards::v1::subsets::any::schema::host;
                    pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::schema::inferences;
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
                #[path = "../../../../../../../../🔨️modules/🧩️registration/🦀️.rs"]
                pub mod registration;
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
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_trinity_jack_model::editor::jack::lod;
        pub use semio_s_artifact_trinity_jack_window_config::editor::jack::window_config;
        #[path = "."]
        pub mod transient {
            pub use semio_s_artifact_trinity_jack_transient::editor::jack::transient::*;
            pub use semio_s_artifact_trinity_jack_results_window::editor::jack::transient::results_window;
            #[path = "."]
            pub mod editor_window {
                pub use semio_s_artifact_trinity_jack_transient::editor::jack::transient::editor_window::*;
                #[cfg(test)]
                #[path = "../../../🎭️modes/✏️edit/🪟️windows/📝️editor/🫧️transient/🧪️tests/🔬️selection/🦀️.rs"]
                pub mod selection_tests;
            }
        }
        pub use semio_s_artifact_trinity_jack_model::editor::jack::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod query {
                #[path = "../../../🎮️commands/▶️run-query/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod load_document_json {
                #[path = "../../../🎮️commands/📄️load-document-json/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            pub use semio_s_artifact_trinity_jack_executor::editor::jack::commands::delete_selection;
            pub use semio_s_artifact_trinity_jack_executor::editor::jack::commands::patch_nodes;
            #[path = "."]
            pub mod set_active_example {
                #[path = "../../../🎮️commands/🎯️set-active-example/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            pub use semio_s_artifact_trinity_jack_executor::editor::jack::commands::format_document;
            pub use semio_s_artifact_trinity_jack_executor::editor::jack::commands::set_viewport;
            pub use semio_s_artifact_trinity_jack_executor::editor::jack::commands::text_edit;
            pub use semio_s_artifact_trinity_jack_executor::editor::jack::commands::set_lod_mode;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                pub use semio_s_artifact_trinity_jack_model::editor::jack::modes::edit::tools;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod graph {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🌐️graph/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod editor {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/📝️editor/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod results {
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod document {
                #[path = "../../../📌️panels/🗿️artifact/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod catalogue {
                #[path = "../../../📌️panels/📚️catalogue/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod inspection {
                #[path = "../../../📌️panels/🔍️inspection/🦀️.rs"]
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
        pub use semio_s_artifact_trinity_jack_executor::viewer::jack::*;
    }
}
