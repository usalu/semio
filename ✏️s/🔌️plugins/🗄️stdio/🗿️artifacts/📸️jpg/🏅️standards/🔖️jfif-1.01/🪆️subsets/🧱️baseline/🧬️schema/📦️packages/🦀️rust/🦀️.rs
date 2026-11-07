//! 🧬️ schema-2 of `semio-s-artifact-stdio-jpg` — a path-preserving carve of the artifact root mounts.
//! 🧱️ Baseline JPEG vocabulary reuses the document schema and native conformance owner.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::JpgSnapshot;
pub use schema::diff::JpgDiff;
pub use schema::mutations::JpgMutation;
pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use schema::snapshot::JpgImage;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub use semio_s_artifact_stdio_jpg_inferences::STDIO_JPG_DOCUMENT_SCHEMA;
pub mod schema {
    pub use super::standards::v_jfif_1_01::subsets::document::schema::*;
}
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_jfif_1_01 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_jpg_schema::standards::v_jfif_1_01::subsets::document::schema::*;
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::schema::diff::*;
                        #[cfg(test)]
                        #[path = "../../../../🧾️document/🧬️schema/🔺️diff/🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
                        mod tests;
                    }
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::schema::mutations::*;
                        #[path = "."]
                        pub mod replace_image {
                            pub use semio_s_artifact_stdio_jpg_mutations::standards::v_jfif_1_01::subsets::document::schema::mutations::replace_image::*;
                            #[cfg(test)]
                            #[path = "../../../../🧾️document/🧬️schema/🧬️mutations/🖼️replace-image/🧪️tests/🦀️.rs"]
                            mod tests;
                        }
                    }
                    #[cfg(test)]
                    #[path = "../../../../🧾️document/🧪️tests/🛡️mutation-regressions/🦀️.rs"]
                    mod mutation_regressions;
                }
            }
            pub mod baseline {
                pub mod schema {
                    pub use super::super::document::schema::*;
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::baseline::schema::conformance;
                    pub use conformance::*;
                }
            }
        }
    }
}

#[path = "."]
pub mod editor {
    #[path = "../../../../🧾️document/✏️editor/🎭️modes/✏️edit/🎮️commands/🎨️paint-region/📜️declaration/🦀️.rs"]
    pub mod paint_region_declaration;
}
