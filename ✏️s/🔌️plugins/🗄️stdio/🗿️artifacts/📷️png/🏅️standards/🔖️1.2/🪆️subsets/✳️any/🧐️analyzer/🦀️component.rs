//! 🧐️ PngAnalyzer (1.2/✳️any) — read-only analysis, successor to the pre-migration
//! PngDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::png::PngSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.png` parts.
#[derive(Clone, Debug, Default)]
pub struct PngParts {
    pub snapshot: Option<PngSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.png` (1.2/✳️any) sources.
pub struct PngAnalyzer;

impl ArtifactAnalyzer for PngAnalyzer {
    type Parts = PngParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        const SIG: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
        match source {
            AnalyzeSource::Binary(bytes) => {
                if bytes.len() >= 8 && bytes[0..8] == SIG { IoConfidence::High } else { IoConfidence::Low }
            }
            AnalyzeSource::Text(text) => {
                // 🔍 stdio.png's text envelope is a hex dump of the raw bytes after the
                // `semio ...` preamble line — decode the first 8 bytes to sniff the real signature.
                let body = match store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(16).collect();
                if hex.len() < 16 {
                    return IoConfidence::Low;
                }
                let mut decoded = [0u8; 8];
                for (i, byte) in decoded.iter_mut().enumerate() {
                    match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                        Ok(b) => *byte = b,
                        Err(_) => return IoConfidence::Low,
                    }
                }
                if decoded == SIG { IoConfidence::High } else { IoConfidence::Low }
            }
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = PngParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <PngSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <PngSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
