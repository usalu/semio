//! 🧐️ ZipAnalyzer (2.0/✳️any) — read-only analysis, successor to the pre-migration
//! ZipDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::zip::ZipSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.zip` parts.
#[derive(Clone, Debug, Default)]
pub struct ZipParts {
    pub snapshot: Option<ZipSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.zip` (2.0/✳️any) sources.
pub struct ZipAnalyzer;

impl ArtifactAnalyzer for ZipAnalyzer {
    type Parts = ZipParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        // 🕵️ Real sniff: inspects the argument's bytes (magic + a well-formed EOCD), never a
        // constant. `AnalyzeSource::Text` is the hex-envelope DSL form, not raw container bytes,
        // so it can't be magic-sniffed the same way — treated as low confidence here (the DSL
        // envelope preamble, not this sniff, is what actually recognizes it).
        use crate::artifacts::zip::engine::{sniff_zip_bytes, SniffConfidence};
        match source {
            AnalyzeSource::Binary(bytes) => match sniff_zip_bytes(bytes) {
                SniffConfidence::High => IoConfidence::High,
                SniffConfidence::Medium => IoConfidence::Medium,
                SniffConfidence::Low => IoConfidence::Low,
            },
            AnalyzeSource::Text(_) => IoConfidence::Low,
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = ZipParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <ZipSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
