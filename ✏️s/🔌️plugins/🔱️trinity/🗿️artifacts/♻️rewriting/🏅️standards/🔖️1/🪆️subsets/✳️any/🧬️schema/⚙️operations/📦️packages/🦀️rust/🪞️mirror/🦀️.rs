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
                    pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_trinity_rewriting_layout::standards::v1::subsets::any::schema::layout;
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::errors;
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::model;
                    pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::application;
                    #[cfg(any(test, feature = "testkit"))]
                    pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::test_support;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::snapshot::*;
                        pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::snapshot::*;
                        pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::snapshot::json;
                        #[cfg(test)]
                        #[path = "../../../../♻️retirement/🧪️tests/🔬️document-retirement/🦀️.rs"]
                        pub mod document_retirement_tests;
                    }
                    pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::inferences;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::schema::diff::*;
                        pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../🔺️diff/🧪️tests/🗂️map-ownership/🦀️.rs"]
                        pub mod map_ownership_tests;
                    }
                    #[path = "."]
                    pub mod operations {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::mutations::support;
                        #[path = "."]
                        pub mod edit_working_graph {
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_working_graph::*;
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_working_graph::diff;
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_working_graph::inverse;
                            #[cfg(test)]
                            #[path = "../../../../🧬️mutations/🖼️edit-working-graph/🧪️tests/🕸️swaps/🦀️.rs"]
                            pub mod tests_swaps_in_a_two_node_before_graph;
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
                            #[path = "../../../../🧬️mutations/🫳️drag-rule/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/🫳️drag-rule/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/🫳️drag-rule/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod set_rule_layout_points {
                            #[path = "../../../../🧬️mutations/📍️set-rule-layout/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../../🧬️mutations/📍️set-rule-layout/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../../🧬️mutations/📍️set-rule-layout/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::rule;
                }
                #[path = "../../../../../🪆️content/🦀️.rs"]
                pub mod content;
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::io::text::diff;
                        #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
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
        #[cfg(test)]
        #[path = "../../../../../📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod rewriting {
        #[path = "../../../../../✏️editor/🎮️commands/🗑️delete-rule-clause/🦀️.rs"]
        pub mod delete_rule_clause;
        pub use semio_s_artifact_trinity_rewriting_layout::editor::rewriting::ids;
        #[path = "../../../../../✏️editor/📄️document/🦀️.rs"]
        pub mod document;
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod set_lhs {
                #[path = "../../../../../✏️editor/🎮️commands/👈️set-lhs-json/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_parameter {
                #[path = "../../../../../✏️editor/🎮️commands/🎛️set-parameter/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod add_rule_clause_command {
                #[path = "../../../../../✏️editor/🎮️commands/➕️add-rule-clause-command/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod reorganize {
                #[path = "../../../../../✏️editor/🎮️commands/🧹️reorganize/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod rewriting {
        #[path = "../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_trinity_rewriting_mut_any::viewer::rewriting::modes::view::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_trinity_rewriting_mut_any::viewer::rewriting::modes::view::windows::*;
                    #[path = "."]
                    pub mod rule {
                        pub use semio_s_artifact_trinity_rewriting_mut_any::viewer::rewriting::modes::view::windows::rule::*;
                        #[cfg(test)]
                        #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/📜️rule/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
