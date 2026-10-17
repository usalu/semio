//! 🚪️ Artifact representation module ownership.

pub use semio_s_artifact_stdio_xml_snapshot::standards::v1_0::subsets::base::io::text::snapshot;

pub mod mutations {
    pub use semio_s_artifact_stdio_xml_mutations::standards::v1_0::subsets::base::io::text::mutations::*;
    pub use semio_s_artifact_stdio_xml_tests::standards::v1_0::subsets::base::io::text::mutations::insert_element;
    pub use semio_s_artifact_stdio_xml_tests::standards::v1_0::subsets::base::io::text::mutations::remove_element;
    pub use semio_s_artifact_stdio_xml_tests::standards::v1_0::subsets::base::io::text::mutations::set_attribute;
    pub use semio_s_artifact_stdio_xml_tests::standards::v1_0::subsets::base::io::text::mutations::set_declaration;
    pub use semio_s_artifact_stdio_xml_tests::standards::v1_0::subsets::base::io::text::mutations::set_doctype;
    pub use semio_s_artifact_stdio_xml_tests::standards::v1_0::subsets::base::io::text::mutations::set_text;
}

pub use semio_s_artifact_stdio_xml_schema::standards::v1_0::subsets::base::io::text::diff;


