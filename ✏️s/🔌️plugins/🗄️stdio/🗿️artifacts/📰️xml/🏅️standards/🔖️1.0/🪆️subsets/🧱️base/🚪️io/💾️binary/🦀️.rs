//! 🚪️ Artifact representation module ownership.

pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::io::binary::snapshot;

pub mod mutations {
    pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::io::binary::mutations::*;
    pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::base::io::binary::mutations::insert_element;
    pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::base::io::binary::mutations::remove_element;
    pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::base::io::binary::mutations::set_attribute;
    pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::base::io::binary::mutations::set_declaration;
    pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::base::io::binary::mutations::set_doctype;
    pub use semio_s_artifact_stdio_xml_schema_2::standards::v1_0::subsets::base::io::binary::mutations::set_text;
}

pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::io::binary::diff;


