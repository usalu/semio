//! 🧐️ TiffAnalyzer (6.0/✳️any) — read-only analysis, successor to the pre-migration
//! TiffDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::tiff::TiffSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.tiff` parts.
#[derive(Clone, Debug, Default)]
pub struct TiffParts {
    pub snapshot: Option<TiffSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.tiff` (6.0/✳️any) sources.
pub struct TiffAnalyzer;

impl ArtifactAnalyzer for TiffAnalyzer {
    type Parts = TiffParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        const SIG_LE: [u8; 4] = [0x49, 0x49, 0x2A, 0x00]; // "II*\0" little-endian
        const SIG_BE: [u8; 4] = [0x4D, 0x4D, 0x00, 0x2A]; // "MM\0*" big-endian
        match source {
            AnalyzeSource::Binary(bytes) => {
                if bytes.len() >= 4 && (bytes[0..4] == SIG_LE || bytes[0..4] == SIG_BE) {
                    IoConfidence::High
                } else {
                    IoConfidence::Low
                }
            }
            AnalyzeSource::Text(text) => {
                // 🔍 stdio.tiff's text envelope is a hex dump of the raw bytes after the
                // `semio ...` preamble line — decode the first 4 bytes to sniff the real signature.
                let body = match store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(8).collect();
                if hex.len() < 8 {
                    return IoConfidence::Low;
                }
                let mut decoded = [0u8; 4];
                for (i, byte) in decoded.iter_mut().enumerate() {
                    match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                        Ok(b) => *byte = b,
                        Err(_) => return IoConfidence::Low,
                    }
                }
                if decoded == SIG_LE || decoded == SIG_BE { IoConfidence::High } else { IoConfidence::Low }
            }
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = TiffParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <TiffSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.analyze.text",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <TiffSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error(
                            "stdio.analyze.binary",
                            dsl::TextSpan::at(1, 1),
                            err.to_string(),
                        ));
                    }
                },
            }
        }
        Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
    }
}
//#endregion 🔖️Analyzer
