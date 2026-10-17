//! 🔬️ tests-3 of `semio-s-artifact-stdio-xml` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
pub use schema::snapshot::XmlSnapshot;
#[cfg(test)]
pub use schema::diff::XmlDiff;
#[cfg(test)]
pub use schema::mutations::XmlMutation;
#[cfg(test)]
pub use semio_s_artifact_stdio_contract::apply_mutation;
#[cfg(test)]
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
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::schema::*;
                    #[cfg(test)]
                    #[path = "../../🦀️.rs"]
                    mod tests;
                    pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::schema::snapshot;
                    pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::schema::diff;
                    pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::base::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::io::text::snapshot;
                        pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::io::text::mutations;
                        pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::io::binary::snapshot;
                        pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::io::binary::mutations;
                        pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::io::binary::diff;
                    }
                }
            }
        }
    }
}
