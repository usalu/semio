//! 🧐️ PlyAnalyzer (1.0/✳️any) — read-only analysis, successor to the pre-migration
//! PlyDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::ply::PlySnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.ply` parts.
#[derive(Clone, Debug, Default)]
pub struct PlyParts {
    pub snapshot: Option<PlySnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.ply` (1.0/✳️any) sources.
pub struct PlyAnalyzer;

impl ArtifactAnalyzer for PlyAnalyzer {
    type Parts = PlyParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        // 🔍 PLY files (ascii or either binary variant) always start with a literal ASCII
        // "ply" magic line — `ply\n` or `ply\r\n` — per the format spec. Unlike png/las,
        // stdio.ply's text envelope embeds the raw ply bytes directly (no hex dump), so both
        // sources are checked against the same literal prefix.
        const MAGIC_LF: &[u8] = b"ply\n";
        const MAGIC_CRLF: &[u8] = b"ply\r\n";
        let starts_with_magic = |bytes: &[u8]| bytes.starts_with(MAGIC_LF) || bytes.starts_with(MAGIC_CRLF);
        match source {
            AnalyzeSource::Binary(bytes) => {
                if starts_with_magic(bytes) { IoConfidence::High } else { IoConfidence::Low }
            }
            AnalyzeSource::Text(text) => {
                let body = match store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                if starts_with_magic(body.as_bytes()) { IoConfidence::High } else { IoConfidence::Low }
            }
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = PlyParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <PlySnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <PlySnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
