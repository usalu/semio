//! Serialize stdio.png to stdio.deflate.

use crate::PngSnapshot;
use semio_s_artifact_stdio_deflate::{DeflateSnapshot, STDIO_DEFLATE_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn serialize(from: &PngSnapshot) -> Result<DeflateSnapshot, store::PackError> {
    let bytes = crate::standards::v1_2::subsets::any::io::encode_png(from).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))?;
    Ok(DeflateSnapshot { schema: STDIO_DEFLATE_DOCUMENT_SCHEMA.into(), compression_method: 8, window_bits: 7, compression_level_hint: semio_s_artifact_stdio_deflate::schema::snapshot::DeflateLevelHint::default(), dict_id: None, payload: bytes })
}
