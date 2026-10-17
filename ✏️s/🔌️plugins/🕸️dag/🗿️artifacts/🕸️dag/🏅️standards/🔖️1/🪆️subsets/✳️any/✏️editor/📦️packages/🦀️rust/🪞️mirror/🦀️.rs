//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

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
                    pub use semio_s_artifact_dag_dag_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_dag_dag_schema::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_dag_dag_schema::standards::v1::subsets::any::io::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_dag_dag_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_dag_dag_inferences::snapshot;
pub use semio_s_artifact_dag_dag_inferences::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dag {
        #[path = "../../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_dag_dag_config::editor::dag::config::*;
            pub use semio_s_artifact_dag_dag_diff::editor::dag::config::schema;
        }
        #[path = "."]
        pub mod presence {
            pub use semio_s_artifact_dag_dag_diff::editor::dag::presence::*;
        }
        pub use semio_s_artifact_dag_dag_diff::editor::dag::terminology;
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod add_node {
                pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::commands::add_node::*;
                #[cfg(test)]
                #[path = "../../../🎮️commands/➕️add-node/🧪️tests/🔬️unit/🦀️.rs"]
                pub mod tests;
            }
            pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::commands::connect_media_ports;
            pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::commands::delete_selection;
            pub use semio_s_artifact_dag_dag_schema::editor::dag::commands::disconnect;
            pub use semio_s_artifact_dag_dag_schema::editor::dag::commands::graph_pointer_down;
            #[path = "../../../🎮️commands/🚚️move-media-node/🦀️.rs"]
            pub mod move_media_node;
            #[path = "../../../🎮️commands/✏️node-graph-edit/🦀️.rs"]
            pub mod node_graph_edit;
            pub use semio_s_artifact_dag_dag_schema::editor::dag::commands::node_graph_viewport;
            #[path = "../../../🎮️commands/🧬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::commands::patch_dag_nodes;
            pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::commands::remove_node;
            pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::commands::rename_dag_node;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::modes::edit::*;
                #[path = "."]
                pub mod tools {
                    pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::modes::edit::tools::*;
                    #[path = "."]
                    pub mod reorganize {
                        pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::modes::edit::tools::reorganize::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::modes::edit::windows::*;
                    #[path = "."]
                    pub mod compiled {
                        pub use semio_s_artifact_dag_dag_diff::editor::dag::modes::edit::windows::compiled::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🧬️compiled/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_dag_dag_document_behavior_tests::editor::dag::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../🎭️modes/✏️edit/🪟️windows/🕸️main/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../../📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../../📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../../📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;

        #[cfg(test)]
        #[path = "../../../📌️panels/🧪️tests/🔬️unit/🦀️.rs"]
        mod leaf_tests;
        }
    }
}
