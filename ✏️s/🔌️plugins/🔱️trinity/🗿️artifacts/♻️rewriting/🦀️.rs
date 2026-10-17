//! ♻️ `trinity.rewrite.rule` artifact — document entities (constitutional: general).

#![allow(clippy::unnecessary_wraps)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate protocol as replication;
extern crate semio_framework_value_derive as value_derive;

// 📜️ `RewritingSnapshot`/`RewriteRuleMutation` derive their `pack::snapshot::ArtifactDsl`/`protocol::OpText`
// impls directly (see `#[derive(dsl::DslRecord)]` above and `#[derive(dsl::DslEnum)]` in `🔧️op`) —
// every field already binds through the `dsl::` engine with no foreign types, so no hand-written
// parser/printer or twin type is needed anywhere in this artifact (unlike `jack`'s `JackSnapshot`).

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        pub use semio_s_artifact_trinity_rewriting_v1::standards::v1::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                pub use semio_s_artifact_trinity_rewriting_v1::standards::v1::subsets::any::*;

                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::errors;
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::model;
                    pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::application;
                    #[cfg(any(test, feature = "testkit"))]
pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::test_support;
                    pub use semio_s_artifact_trinity_rewriting_layout::standards::v1::subsets::any::schema::graph_manifest;
                    pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::operations;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod edit_working_graph {
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_working_graph::*;

                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_working_graph::diff;
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_working_graph::inverse;

                        }
                        #[path = "."]
                        pub mod edit_lhs {
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_lhs::*;

                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_lhs::diff;
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_lhs::inverse;

                        }
                        #[path = "."]
                        pub mod edit_rhs {
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_rhs::*;

                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_rhs::diff;
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_rhs::inverse;

                        }
                        #[path = "."]
                        pub mod change_parameter_binding {
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::change_parameter_binding::*;

                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::change_parameter_binding::diff;
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::change_parameter_binding::inverse;

                        }
                        #[path = "."]
                        pub mod remove_parameter_binding {
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::remove_parameter_binding::*;

                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::remove_parameter_binding::diff;
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::remove_parameter_binding::inverse;

                        }
                        #[path = "."]
                        pub mod change_rule_layout_point {
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::change_rule_layout_point::*;

                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::change_rule_layout_point::diff;
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::change_rule_layout_point::inverse;

                        }
                        #[path = "."]
                        pub mod remove_rule_layout_point {
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::remove_rule_layout_point::*;

                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::remove_rule_layout_point::diff;
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::remove_rule_layout_point::inverse;

                        }
                        #[path = "."]
                        pub mod drag_rule_nodes {
                            pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::drag_rule_nodes::*;

                            pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::drag_rule_nodes::diff;
                            pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::drag_rule_nodes::inverse;

                        }
                        #[path = "."]
                        pub mod set_rule_layout_points {
                            pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::set_rule_layout_points::*;

                            pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::set_rule_layout_points::diff;
                            pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::set_rule_layout_points::inverse;

                        }
                    }
                }
                pub use semio_s_artifact_trinity_rewriting_v1::standards::v1::subsets::any::registration;
                pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::content;
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::io::binary::inferences;
}
pub mod sqlite {
    pub use semio_s_artifact_trinity_rewriting_layout::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::standards::v1::subsets::any::io::text::*;
    pub use semio_s_artifact_trinity_rewriting_layout::standards::v1::subsets::any::io::text::inferences;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
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
                                pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
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

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                pub use semio_s_artifact_trinity_rewriting_layout::editor::rewriting::examples::demo_session::*;

            }
        }

        pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::window_config;

        pub use semio_s_artifact_trinity_rewriting_layout::editor::rewriting::ids;
        pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::ui;
        pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::document;

        pub use semio_s_artifact_trinity_rewriting_layout::editor::rewriting::terminology;

        #[path = "."]
        pub mod commands {
            // 🕹️ Every command file is self-contained (its own private copy of any shared
            // helpers) and exposes exactly one `pub fn` matching its directory's verb —
            // re-exported here by name, flat, matching how `TrinityRewritingCommand::handle` calls them.
            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::node_graph_edit;

            pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::commands::set_lhs;

            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::set_rhs;

            pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::commands::set_parameter;

            pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::commands::add_rule_clause_command;

            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::reset_rule;

            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::set_active_example;

            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::patch_nodes;

            pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::commands::add_working_node_command;

            pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::commands::set_viewport;

            pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::commands::reorganize;

            pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::commands::set_lod_mode;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_trinity_rewriting_layout::editor::rewriting::modes::edit::*;

                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::modes::edit::windows::before;

                    pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::modes::edit::windows::after;

                    pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::modes::edit::windows::lhs;

                    pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::modes::edit::windows::rhs;

                    pub use semio_s_artifact_trinity_rewriting_tests_narrows_the_lhs_pattern_to_a_shaft_neighbour::editor::rewriting::modes::edit::windows::jack;

                    pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::modes::edit::windows::parameters;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::panels::document;

            pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::panels::catalogue;

            pub use semio_s_artifact_trinity_rewriting_editor::editor::rewriting::panels::inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod rewriting {
        pub use semio_s_artifact_trinity_rewriting_operations::viewer::rewriting::*;

        pub use semio_s_artifact_trinity_rewriting_layout::viewer::rewriting::presence;
        pub use semio_s_artifact_trinity_rewriting_layout::viewer::rewriting::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_trinity_rewriting_mut_any::viewer::rewriting::modes::view::*;

                pub use semio_s_artifact_trinity_rewriting_mut_any::viewer::rewriting::modes::view::windows;
            }
        }
    }
}
