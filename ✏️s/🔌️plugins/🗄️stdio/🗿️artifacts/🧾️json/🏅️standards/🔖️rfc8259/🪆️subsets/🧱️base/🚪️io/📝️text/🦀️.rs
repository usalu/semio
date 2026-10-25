//! 🚪️ Artifact representation module ownership.

pub use semio_s_artifact_stdio_json_schema::standards::v_rfc8259::subsets::base::io::text::snapshot;

pub mod mutations {
    pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::io::text::mutations::*;
    pub use crate::standards::v_rfc8259::subsets::base::io::binary::mutations::set_member;
    pub use crate::standards::v_rfc8259::subsets::base::io::binary::mutations::set_scalar;
}

pub use semio_s_artifact_stdio_json_mutations::standards::v_rfc8259::subsets::base::io::text::diff;


