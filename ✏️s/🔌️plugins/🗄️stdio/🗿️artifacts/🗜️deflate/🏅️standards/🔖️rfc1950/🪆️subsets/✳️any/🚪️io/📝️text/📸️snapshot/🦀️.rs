//! 📝️ Text representation codec surface for `stdio.deflate` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type DeflateSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_rfc1950::subsets::any::schema::snapshot::*;
use crate::STDIO_DEFLATE_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
/// 🗜️ Declares the actual cumulative bare RFC1950 producer authority for domain consumers.
use crate::standards::v_rfc1950::subsets::any::io::sqlite::snapshot::native::{compress_zlib,decompress_zlib};

impl store::ArtifactDsl for DeflateSnapshot {
    const EXTENSION: &'static str = "zz";
    fn envelope_id() -> &'static str {
        "stdio.deflate"
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if !hex.len().is_multiple_of(2) {
            return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "odd hex length", semio_framework_diagnostic::TextSpan::at(1, 1)));
        }
        let mut zlib_bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid hex: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            zlib_bytes.push(byte);
            i += 2;
        }
        crate::standards::v_rfc1950::subsets::any::io::decode_deflate_snapshot(&zlib_bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("zlib decode: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let zlib_bytes = crate::standards::v_rfc1950::subsets::any::io::encode_deflate_snapshot(self);
        let body: String = zlib_bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;
