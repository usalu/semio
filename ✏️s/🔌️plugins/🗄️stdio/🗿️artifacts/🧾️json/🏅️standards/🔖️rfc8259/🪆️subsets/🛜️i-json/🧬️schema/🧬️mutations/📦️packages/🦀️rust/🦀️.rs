//! 🧬️ mutations-2 of `semio-s-artifact-stdio-json` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::mutations::JsonMutation;
pub use schema::snapshot::JsonSnapshot;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub mod schema {
    pub use super::standards::v_rfc8259::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_json_schema::{STDIO_JSON_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_rfc8259 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_json_schema_2::standards::v_rfc8259::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::mutation_support;
                    pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/✏️set-member/🦀️.rs"]
                            pub mod set_member;
                            #[path = "../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/📤️remove-array-element/🦀️.rs"]
                            pub mod remove_array_element;
                            #[path = "../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/🗑️remove-member/🦀️.rs"]
                            pub mod remove_member;
                            #[path = "../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/📥️insert-array-element/🦀️.rs"]
                            pub mod insert_array_element;
                            #[path = "../../../../../🧱️base/🚪️io/💾️binary/🧬️mutations/🔢️set-scalar/🦀️.rs"]
                            pub mod set_scalar;
                        }
                    }
                    #[path = "../../../../../🧱️base/🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
            #[path = "."]
            pub mod i_json {
                pub use semio_s_artifact_stdio_json_number::standards::v_rfc8259::subsets::i_json::examples;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_json_schema_2::standards::v_rfc8259::subsets::i_json::schema::*;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_stdio_json_number::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod json_any {
        #[path = "../../../../../🧱️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "../../../../../🧱️base/✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_json_mutations::editor::json_any::modes::edit::*;
                pub use semio_s_artifact_stdio_json_schema::editor::json_any::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod json_i_json {
        #[path = "../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "../../../../✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_json_mutations::editor::json_i_json::modes::edit::*;
                pub use semio_s_artifact_stdio_json_schema::editor::json_i_json::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod json_any {
        #[path = "../../../../../🧱️base/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_json_mutations::viewer::json_any::modes::view::*;
                pub use semio_s_artifact_stdio_json_schema::viewer::json_any::modes::view::windows;
            }
        }
    }
    #[path = "."]
    pub mod json_i_json {
        #[path = "../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_json_schema_2::viewer::json_i_json::modes::view::*;
                pub use semio_s_artifact_stdio_json_schema_2::viewer::json_i_json::modes::view::windows;
            }
        }
    }
}
