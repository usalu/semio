//! 🔬️ tests-2 of `semio-s-artifact-stdio-xml` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::mutations::XmlMutation;
pub use schema::snapshot::XmlSnapshot;
pub use standards::v1_0::subsets::base::examples;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub mod schema {
    pub use super::standards::v1_0::subsets::base::schema::*;
}
pub use semio_s_artifact_stdio_xml_snapshot::{STDIO_XML_DOCUMENT_SCHEMA};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::base::examples;
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::schema::*;
                    pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::io::text::snapshot;
                    }
                }
            }
            #[path = "."]
            pub mod valid {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::valid::schema::*;
                    #[path = "."]
                    pub mod valid_mutations {
                        pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::valid::schema::valid_mutations::*;
                        #[cfg(test)]
                        #[path = "../../🦀️.rs"]
                        mod tests;
                    }
                }
                #[path = "."]
                pub mod examples {
                    pub use semio_s_artifact_stdio_xml_inferences::standards::v1_0::subsets::valid::examples::demo;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod xml_any {
        #[path = "../../../../../../../🧱️base/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_xml_schema_2::editor::xml_any::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_xml_mutations::editor::xml_any::modes::edit::*;
                pub use semio_s_artifact_stdio_xml_mutations::editor::xml_any::modes::edit::windows;
            }
        }
    }
    #[path = "."]
    pub mod xml_valid {
        #[path = "../../../../../../✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_xml_inferences::editor::xml_valid::edit_rules;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_stdio_xml_mutations::editor::xml_valid::modes::edit::*;
                pub use semio_s_artifact_stdio_xml_mutations::editor::xml_valid::modes::edit::windows;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod xml_valid {
        #[path = "../../../../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_stdio_xml_mutations::viewer::xml_valid::modes::view::*;
                pub use semio_s_artifact_stdio_xml_mutations::viewer::xml_valid::modes::view::windows;
            }
        }
    }
}
