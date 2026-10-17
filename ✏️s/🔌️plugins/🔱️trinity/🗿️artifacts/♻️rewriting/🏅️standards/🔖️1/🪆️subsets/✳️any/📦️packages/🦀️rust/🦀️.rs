//! ✳️ model of `semio-s-artifact-trinity-rewriting` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::unnecessary_wraps)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as vcs;
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
                    #[path = "."]
                    pub mod graph_manifest {
                        #[path = "../../../../../../🤖️generated/🫲️rewrite-lhs/🦀️.rs"]
                        pub mod rewrite_lhs;
                    }
                    #[path = "../../../../../../🔨️modules/⚠️errors/🦀️.rs"]
                    pub mod errors;
                    #[path = "../../../../../../🔨️modules/🔺️model/🦀️.rs"]
                    pub mod model;
                    #[path = "."]
                    pub mod snapshot {
                        #[path="../../🧬️schema/📸️snapshot/🔣️json/🦀️.rs"]
                        pub mod json;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🧬️schema/🧬️mutations/🧰️support/🦀️.rs"]
                        pub mod support;
                    }
                    #[path="../../🧬️schema/🌳️typed/📜️rule/🦀️.rs"]
                    pub mod rule;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../🚪️io/💾️binary/🧬️mutations/📐️change-rule-layout/🦀️.rs"]
                            pub mod change_rule_layout_point;
                            #[path = "../../🚪️io/💾️binary/🧬️mutations/🫳️drag-rule/🦀️.rs"]
                            pub mod drag_rule_nodes;
                            #[path = "../../🚪️io/💾️binary/🧬️mutations/👉️edit-rhs/🦀️.rs"]
                            pub mod edit_rhs;
                            #[path = "../../🚪️io/💾️binary/🧬️mutations/🖼️edit-working-graph/🦀️.rs"]
                            pub mod edit_working_graph;
                            #[path = "../../🚪️io/💾️binary/🧬️mutations/🗑️remove-rule-layout/🦀️.rs"]
                            pub mod remove_rule_layout_point;
                            #[path = "../../🚪️io/💾️binary/🧬️mutations/🔧️change-parameter/🦀️.rs"]
                            pub mod change_parameter_binding;
                            #[path = "../../🚪️io/💾️binary/🧬️mutations/👈️edit-lhs/🦀️.rs"]
                            pub mod edit_lhs;
                            #[path = "../../🚪️io/💾️binary/🧬️mutations/🧹️remove-parameter-binding/🦀️.rs"]
                            pub mod remove_parameter_binding;
                            #[path = "../../🚪️io/💾️binary/🧬️mutations/📍️set-rule-layout/🦀️.rs"]
                            pub mod set_rule_layout_points;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../🚪️io/📝️text/🧬️mutations/📐️change-rule-layout/🦀️.rs"]
                            pub mod change_rule_layout_point;
                            #[path = "../../🚪️io/📝️text/🧬️mutations/🫳️drag-rule/🦀️.rs"]
                            pub mod drag_rule_nodes;
                            #[path = "../../🚪️io/📝️text/🧬️mutations/👉️edit-rhs/🦀️.rs"]
                            pub mod edit_rhs;
                            #[path = "../../🚪️io/📝️text/🧬️mutations/🖼️edit-working-graph/🦀️.rs"]
                            pub mod edit_working_graph;
                            #[path = "../../🚪️io/📝️text/🧬️mutations/🗑️remove-rule-layout/🦀️.rs"]
                            pub mod remove_rule_layout_point;
                            #[path = "../../🚪️io/📝️text/🧬️mutations/🔧️change-parameter/🦀️.rs"]
                            pub mod change_parameter_binding;
                            #[path = "../../🚪️io/📝️text/🧬️mutations/👈️edit-lhs/🦀️.rs"]
                            pub mod edit_lhs;
                            #[path = "../../🚪️io/📝️text/🧬️mutations/🧹️remove-parameter-binding/🦀️.rs"]
                            pub mod remove_parameter_binding;
                            #[path = "../../🚪️io/📝️text/🧬️mutations/📍️set-rule-layout/🦀️.rs"]
                            pub mod set_rule_layout_points;
                        }
                    }
                }
            }
        }
    }
}
