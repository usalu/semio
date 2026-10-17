//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "../../../🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::model;
                    #[cfg(any(test, feature = "testkit"))]
                    pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::test_support;
                    pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::operations;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::*;
                    }
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::rule;
                }
                #[path = "../../../../../🔨️modules/🧩️registration/🦀️.rs"]
                pub mod registration;
                pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::content;
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::io::text::mutations;
                        pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::io::binary::snapshot;
                        pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::io::binary::mutations;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::io::sqlite::snapshot::*;
                            #[cfg(test)]
                            #[path = "../../../🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
                            pub mod tests;
                        }
                    }
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
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
        pub use semio_s_artifact_trinity_rewriting_layout::examples::demo::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod rewriting {
        pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::*;
        #[cfg(test)]
        #[path = "../../../🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod window_config {
            pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::window_config::*;
            #[cfg(test)]
            #[path = "../../../🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🔬️window-config-ownership/🦀️.rs"]
            pub mod tests;
        }
        pub use semio_s_artifact_trinity_rewriting_layout::editor::rewriting::ids;
        pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::ui;
        pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::document;
        pub use semio_s_artifact_trinity_rewriting_layout::editor::rewriting::terminology;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::node_graph_edit;
            pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::commands::set_lhs;
            pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::commands::add_rule_clause_command;
            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::set_active_example;
            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::add_working_node_command;
            pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::commands::set_lod_mode;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_trinity_rewriting_layout::editor::rewriting::modes::edit::*;
            }
        }
        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::panels::*;
            #[path = "."]
            pub mod document {
                pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::panels::document::*;
                pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::panels::document::*;
                #[cfg(test)]
                #[path = "../../../🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod rewriting {
        pub use semio_s_artifact_trinity_rewriting_operations::viewer::rewriting::*;
    }
}
