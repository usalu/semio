//! 📝️ Text representation codec surface for `stdio.bmp` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_v3::subsets::any::schema::snapshot::*;
use crate::STDIO_BMP_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

impl store::ArtifactDsl for BmpSnapshot {
    const EXTENSION: &'static str = "bmp";
    fn envelope_id() -> &'static str {
        "stdio.bmp"
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if !hex.len().is_multiple_of(2) || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "BMP source requires complete ASCII hexadecimal byte pairs", semio_framework_diagnostic::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for i in (0..hex.len()).step_by(2) {
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("hex: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?);
        }
        crate::standards::v_v3::subsets::any::io::decode_bmp(&bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        crate::standards::v_v3::subsets::any::io::bmp_layout(self).expect("BmpSnapshot invariant");
        let body: String = self.bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;
