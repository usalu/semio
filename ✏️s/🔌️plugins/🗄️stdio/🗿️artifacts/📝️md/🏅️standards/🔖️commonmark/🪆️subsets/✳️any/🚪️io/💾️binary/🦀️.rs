//! 🚪️ Artifact representation module ownership.

pub use semio_s_artifact_stdio_md_snapshot::standards::v_commonmark::subsets::any::io::binary::snapshot;

#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;

pub use semio_s_artifact_stdio_md_diff::standards::v_commonmark::subsets::any::io::binary::diff;



//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{STDIO_MD_DOCUMENT_SCHEMA};
pub use crate::schema::mutations::MdMutation;
pub use crate::schema::snapshot::MdSnapshot;

pub fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::bare_artifact_codec::<MdSnapshot, MdMutation>(STDIO_MD_DOCUMENT_SCHEMA);
    codec.extension = "md";
    codec
}
}
//#endregion 🧩️ArtifactItems5
