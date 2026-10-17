//! 🚪️ Artifact representation module ownership.

pub use semio_s_artifact_stdio_json_schema_2::standards::v_rfc8259::subsets::base::io::binary::snapshot;

pub mod mutations {
    pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::io::binary::mutations::*;
    pub use semio_s_artifact_stdio_json_mutations_2::standards::v_rfc8259::subsets::base::io::binary::mutations::insert_array_element;
    pub use semio_s_artifact_stdio_json_mutations_2::standards::v_rfc8259::subsets::base::io::binary::mutations::remove_array_element;
    pub use semio_s_artifact_stdio_json_mutations_2::standards::v_rfc8259::subsets::base::io::binary::mutations::remove_member;
    pub use semio_s_artifact_stdio_json_mutations_2::standards::v_rfc8259::subsets::base::io::binary::mutations::set_member;
    pub use semio_s_artifact_stdio_json_mutations_2::standards::v_rfc8259::subsets::base::io::binary::mutations::set_scalar;
}

pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::io::binary::diff;


