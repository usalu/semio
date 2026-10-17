//! 🚪️ Artifact representation module ownership.

pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::io::binary::snapshot;

pub mod mutations {
    pub use semio_s_artifact_stdio_svg_mutations::standards::v1_1::subsets::base::io::binary::mutations::*;
    pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::base::io::binary::mutations::insert_element;
    pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::base::io::binary::mutations::remove_element;
    pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::base::io::binary::mutations::set_attribute;
    pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::base::io::binary::mutations::set_declaration;
    pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::base::io::binary::mutations::set_doctype;
    pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::base::io::binary::mutations::set_element_name;
    pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::base::io::binary::mutations::set_text;
    pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::base::io::binary::mutations::set_transform;
    pub use semio_s_artifact_stdio_svg_mutations_2::standards::v1_1::subsets::base::io::binary::mutations::set_view_box;
}

pub use semio_s_artifact_stdio_svg_snapshot::standards::v1_1::subsets::base::io::binary::diff;


