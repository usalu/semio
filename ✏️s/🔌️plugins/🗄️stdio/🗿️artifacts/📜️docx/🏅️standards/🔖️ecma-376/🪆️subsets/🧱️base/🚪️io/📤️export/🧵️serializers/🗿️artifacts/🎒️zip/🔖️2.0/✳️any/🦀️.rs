//! 📤️ Serialize stdio.docx to stdio.binary (encode ZIP bytes).

use semio_s_artifact_stdio_docx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::DocxSnapshot;
use semio_s_artifact_stdio_binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};

//#region Codec
/// 💾️ Register serializer hooks.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

/// 🎒️ Encode DocxSnapshot as ZIP container bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &DocxSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::standards::v_ecma_376::subsets::base::io::export::serializers::encode_docx(from).map_err(|e| store::PackError::from(semio_framework_value::ValueError::from(e)))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}

/// 🖨️ Encode ZIP then wrap as binary pack bytes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize_bytes(from: &DocxSnapshot) -> Result<Vec<u8>, store::PackError> {
    pack::snapshot::ArtifactPack::encode_pack_with(&serialize(from)?, &store::PackEncodeOptions::default())
}
//#endregion Codec
