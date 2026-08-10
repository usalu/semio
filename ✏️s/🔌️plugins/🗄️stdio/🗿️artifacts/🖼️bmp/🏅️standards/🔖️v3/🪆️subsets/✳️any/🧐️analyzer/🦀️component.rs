//! 🧐️ BmpAnalyzer (v3/✳️any) — read-only analysis, successor to the pre-migration
//! BmpDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::bmp::BmpSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.bmp` parts.
#[derive(Clone, Debug, Default)]
pub struct BmpParts {
    pub snapshot: Option<BmpSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.bmp` (v3/✳️any) sources.
pub struct BmpAnalyzer;

impl ArtifactAnalyzer for BmpAnalyzer {
    type Parts = BmpParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        const SIG: [u8; 2] = *b"BM";
        match source {
            AnalyzeSource::Binary(bytes) => {
                if bytes.len() >= 2 && bytes[0..2] == SIG { IoConfidence::High } else { IoConfidence::Low }
            }
            AnalyzeSource::Text(text) => {
                // 🔍 stdio.bmp's text envelope is a hex dump of the raw bytes after the
                // `semio ...` preamble line — decode the first 2 bytes to sniff the real signature.
                let body = match store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(4).collect();
                if hex.len() < 4 {
                    return IoConfidence::Low;
                }
                let mut decoded = [0u8; 2];
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
        let mut parts = BmpParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <BmpSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <BmpSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
