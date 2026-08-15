//! 🧬️ PdfSnapshot schema — persistent fields + real codecs.

use crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct PageDoc {
    pub width: f64,
    pub height: f64,
    #[serde(default)]
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf")]
pub struct PdfSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    #[dsl(block)]
    pub page: PageDoc,
}

impl Default for PdfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), page: PageDoc { width: 612.0, height: 792.0, text: String::new() } }
    }
}

impl store::ArtifactDsl for PdfSnapshot {
    const EXTENSION: &'static str = "pdf";
    fn envelope_id() -> &'static str {
        "stdio.pdf"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for i in (0..hex.len()).step_by(2) {
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?);
        }
        crate::artifacts::pdf::standards::v1_4::subsets::any::io::decode_pdf(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let bytes = crate::artifacts::pdf::standards::v1_4::subsets::any::io::encode_pdf(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PdfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::pdf::standards::v1_4::subsets::any::io::encode_pdf(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema("pack envelope mismatch".into()));
        }
        let _ = options;
        crate::artifacts::pdf::standards::v1_4::subsets::any::io::decode_pdf(&inner).map_err(|e| store::PackError::Schema(e))
    }
}

//#region 🔖️SnapshotFixtures
/// 🦑 Dissolved out of the former `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-
/// STATE-MACHINES) — pure snapshot constructors, no codec/IO concern.
pub fn empty_pdf_snapshot() -> PdfSnapshot {
    PdfSnapshot::default()
}

/// 📄️ The demo `stdio.pdf` document -- the single source of truth for `📚️examples/🎬️demo/🖼️assets/
/// 🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are literally this snapshot's `print_dsl`/
/// `encode_pack` output, asserted equal by `fixture_honesty_law`). `width`/`height` are FIXED at
/// `612.0`/`792.0` -- NOT an arbitrary choice: `decode_pdf` hardcodes those two literals
/// unconditionally (never parses them back out of the encoded bytes, the documented "1.4 stays a
/// frozen stub" scope boundary) -- any OTHER width/height would make `parse_dsl(print_dsl(demo))
/// != demo`, since `parse_dsl` genuinely calls the real `decode_pdf` on the hex-decoded bytes, not
/// an identity round-trip.
pub fn demo_pdf_snapshot() -> PdfSnapshot {
    PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), page: PageDoc { width: 612.0, height: 792.0, text: "Semio Demo".into() } }
}
//#endregion 🔖️SnapshotFixtures
