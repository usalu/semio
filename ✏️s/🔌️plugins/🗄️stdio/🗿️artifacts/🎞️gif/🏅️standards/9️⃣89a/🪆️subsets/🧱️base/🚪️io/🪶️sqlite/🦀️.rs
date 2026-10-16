//! 🚪️ Artifact representation module ownership.



//#region 🧩️ArtifactItems
pub mod artifact_items {
/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_GIF_DOCUMENT_SCHEMA: &str = "stdio.gif";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
/// 🧬️ Artifact schema descriptor id.
pub const GIF_ARTIFACT_SCHEMA_ID: &str = "s.stdio.gif";
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    Vec::new()
}
}
//#endregion 🧩️ArtifactItems6
