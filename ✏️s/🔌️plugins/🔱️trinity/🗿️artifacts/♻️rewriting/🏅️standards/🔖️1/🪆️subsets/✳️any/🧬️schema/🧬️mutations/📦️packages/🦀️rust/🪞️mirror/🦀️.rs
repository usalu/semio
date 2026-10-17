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
                    pub use semio_s_artifact_trinity_rewriting_layout::standards::v1::subsets::any::schema::layout;
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::model;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::snapshot::*;
                        pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::snapshot::json;
                    }
                    #[path = "."]
                    pub mod inferences {
                        pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::inferences::*;
                        pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::inferences::*;
                        #[path = "."]
                        pub mod bounds {
                            pub use semio_s_artifact_trinity_rewriting_inferences::standards::v1::subsets::any::schema::inferences::bounds::*;
                            #[cfg(test)]
                            //#region 🧪️Tests
                            #[path = "../../../../💡️inferences/📦bounds/🧪️tests/🔬️unit/🦀️.rs"]
                            pub mod tests;
                        }
                        #[cfg(test)]
                        //#region 🧪️Tests
                        #[path = "../../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
                        pub mod tests;
                    }
                    pub use semio_s_artifact_trinity_rewriting_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod edit_working_graph {
                            #[path = "../../../🖼️edit-working-graph/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🖼️edit-working-graph/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🖼️edit-working-graph/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod edit_lhs {
                            #[path = "../../../👈️edit-lhs/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../👈️edit-lhs/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../👈️edit-lhs/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod edit_rhs {
                            #[path = "../../../👉️edit-rhs/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../👉️edit-rhs/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../👉️edit-rhs/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_parameter_binding {
                            #[path = "../../../🔧️change-parameter/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔧️change-parameter/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔧️change-parameter/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_parameter_binding {
                            #[path = "../../../🧹️remove-parameter-binding/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧹️remove-parameter-binding/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧹️remove-parameter-binding/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_rule_layout_point {
                            #[path = "../../../📐️change-rule-layout/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../📐️change-rule-layout/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../📐️change-rule-layout/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_rule_layout_point {
                            #[path = "../../../🗑️remove-rule-layout/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🗑️remove-rule-layout/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🗑️remove-rule-layout/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::rule;
                }
                #[path = "."]
                pub mod io {
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
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
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
                                            #[path = "../../../../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
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
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod rewriting {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/📜️rule/🦀️.rs"]
                    pub mod rule;
                }
            }
        }
    }
}
