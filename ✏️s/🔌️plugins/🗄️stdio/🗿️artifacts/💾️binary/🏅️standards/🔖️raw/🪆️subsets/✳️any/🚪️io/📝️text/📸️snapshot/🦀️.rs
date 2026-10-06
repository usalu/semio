//! 📝️ Text representation codec surface for `stdio.binary` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type BinarySnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_raw::subsets::any::schema::snapshot::*;
use crate::STDIO_BINARY_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

impl store::ArtifactDsl for BinarySnapshot {
    const EXTENSION: &'static str = "bin";
    fn envelope_id() -> &'static str {
        "stdio.binary"
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
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("invalid hex: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        Ok(Self { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
    }
    fn print_dsl(&self) -> String {
        let body: String = self.bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;
