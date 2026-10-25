//! 🚪️ Artifact representation module ownership.

pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::io::binary::snapshot;

pub mod mutations {
    pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::io::binary::mutations::*;
    pub use crate::mirror::standards::v1_1::subsets::base::schema::mutations::set_attribute;
    pub use crate::mirror::standards::v1_1::subsets::base::schema::mutations::set_declaration;
    pub use crate::mirror::standards::v1_1::subsets::base::schema::mutations::set_text;
}

pub use semio_s_artifact_stdio_svg_diff::standards::v1_1::subsets::base::io::binary::diff;


