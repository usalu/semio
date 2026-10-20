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
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::model;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::snapshot::*;
                        pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::snapshot::json;
                    }
                    pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::operations;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::*;
                        #[path = "."]
                        pub mod edit_lhs {
                            #[cfg(test)]
                            #[path = "../../../🦀️.rs"]
                            pub mod tests_narrows_the_lhs_pattern_to_a_shaft_neighbour;
                        }
                        #[path = "."]
                        pub mod edit_rhs {
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_rhs::*;
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_rhs::diff;
                            pub use semio_s_artifact_trinity_rewriting_mut_any::standards::v1::subsets::any::schema::mutations::edit_rhs::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../👉️edit-rhs/🧪️tests/👉️rewrites/🦀️.rs"]
                            pub mod tests_rewrites_the_rhs_to_set_a_second_property;
                        }
                        #[path = "."]
                        pub mod change_parameter_binding {
                            #[cfg(test)]
                            #[path = "../../../../../../🔧️change-parameter-binding/🧪️tests/🏷️retitles/🦀️.rs"]
                            pub mod tests_retitles_the_caption_binding;
                        }
                        #[path = "."]
                        pub mod remove_parameter_binding {
                            #[cfg(test)]
                            #[path = "../../../../../../🧹️remove-parameter-binding/🧪️tests/✂️drops/🦀️.rs"]
                            pub mod tests_drops_the_repeat_binding;
                        }
                        #[path = "."]
                        pub mod change_rule_layout_point {
                            #[cfg(test)]
                            #[path = "../../../../../../📐️change-rule-layout-point/🧪️tests/📍️nudges/🦀️.rs"]
                            pub mod tests_nudges_the_capsule_var_off_the_shaft;
                        }
                        #[path = "."]
                        pub mod remove_rule_layout_point {
                            #[cfg(test)]
                            #[path = "../../../../../../🗑️remove-rule-layout-point/🧪️tests/📐️clears/🦀️.rs"]
                            pub mod tests_clears_the_shaft_layout_point;
                        }
                        #[path = "."]
                        pub mod drag_rule_nodes {
                            pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::drag_rule_nodes::*;
                            pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::drag_rule_nodes::diff;
                            pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::drag_rule_nodes::inverse;
                            #[cfg(test)]
                            #[path = "../../../../../../🫳️drag-rule-nodes/🧪️tests/🫳️moves/🦀️.rs"]
                            pub mod tests_drags_two_rule_nodes;
                        }
                        #[path = "."]
                        pub mod set_rule_layout_points {
                            pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::schema::mutations::set_rule_layout_points::*;
                            #[cfg(test)]
                            #[path = "../../../../../../📍️set-rule-layout-points/🧪️tests/📍️places/🦀️.rs"]
                            pub mod tests_places_one_rule_node_and_clears_one;
                        }
                    }
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::rule;
                }
                #[path = "."]
                pub mod content {
                    pub use semio_s_artifact_trinity_rewriting_operations::standards::v1::subsets::any::content::*;
                    pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::content::read;
                }
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../../🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
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
        pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::delete_rule_clause;
        pub use semio_s_artifact_trinity_rewriting_layout::editor::rewriting::ids;
        pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::document;
        #[path = "."]
        pub mod commands {
            #[path = "."]
            pub mod node_graph_edit {
                #[path = "../../../../../../../../✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_rhs {
                #[path = "../../../../../../../../✏️editor/🎮️commands/👉️set-rhs-json/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod reset_rule {
                #[path = "../../../../../../../../✏️editor/🎮️commands/♻️reset-rule/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod set_active_example {
                #[path = "../../../../../../../../✏️editor/🎮️commands/🎯️set-active-example/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod patch_nodes {
                #[path = "../../../../../../../../✏️editor/🎮️commands/🩹️patch-nodes/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod add_working_node_command {
                #[path = "../../../../../../../../✏️editor/🎮️commands/➕️add-working-node/🦀️.rs"]
                mod component;
                pub use component::*;
            }
            #[path = "."]
            pub mod reorganize {
                pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::commands::reorganize::*;
                pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::commands::reorganize::*;
                #[cfg(test)]
                #[path = "../../../../../../../../✏️editor/🎮️commands/🧹️reorganize/🧪️tests/📐️document-mutation-reorganization/🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod jack {
                        #[path = "../../../../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🔎️jack/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod rewriting {
        pub use semio_s_artifact_trinity_rewriting_operations::viewer::rewriting::*;
        #[cfg(test)]
        #[path = "../../../../../../../../👁️viewer/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
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
