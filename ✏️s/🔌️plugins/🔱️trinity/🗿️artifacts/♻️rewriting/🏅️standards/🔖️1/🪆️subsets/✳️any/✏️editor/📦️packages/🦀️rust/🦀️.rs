//! ✏️ editor of `semio-s-artifact-trinity-rewriting` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::unnecessary_wraps)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

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
                    pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::model;
                    pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::snapshot;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::*;
                    }
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::rule;
                }
                pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::content;
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
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::delete_rule_clause;
        #[path = "../../🪟️window/🎚️config/🦀️.rs"]
        pub mod window_config;
        pub use semio_s_artifact_trinity_rewriting_layout::editor::rewriting::ids;
        #[path = "../../🖌️ui/🦀️.rs"]
        pub mod ui;
        pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::document;
        pub use semio_s_artifact_trinity_rewriting_layout::editor::rewriting::terminology;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::node_graph_edit;
            pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::commands::set_lhs;
            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::set_rhs;
            pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::commands::set_parameter;
            pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::commands::add_rule_clause_command;
            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::reset_rule;
            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::set_active_example;
            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::patch_nodes;
            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::add_working_node_command;
            #[path = "."]
            pub mod set_viewport {
                #[path = "../../🎮️commands/🖥️set-viewport/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::commands::reorganize;
            #[path = "."]
            pub mod set_lod_mode {
                #[path = "../../🎮️commands/🔬️set-lod/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_trinity_rewriting_layout::editor::rewriting::modes::edit::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod before {
                        #[path = "../../🎭️modes/✏️edit/🪟️windows/⬅️before/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod after {
                        #[path = "../../🎭️modes/✏️edit/🪟️windows/⏭️after/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod lhs {
                        #[path = "../../🎭️modes/✏️edit/🪟️windows/👈️lhs/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod rhs {
                        #[path = "../../🎭️modes/✏️edit/🪟️windows/➡️rhs/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::modes::edit::windows::jack;
                    #[path = "."]
                    pub mod parameters {
                        #[path = "../../🎭️modes/✏️edit/🪟️windows/🎛️parameters/🦀️.rs"]
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
                #[path = "../../📌️panels/🗿️artifact/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod catalogue {
                #[path = "../../📌️panels/📚️catalogue/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod inspection {
                #[path = "../../📌️panels/🔍️inspection/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
