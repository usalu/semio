//! 🧐️ BcfAnalyzer (2.1/✳️any) — read-only analysis, successor to the pre-migration
//! BcfDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::bcf::BcfSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.bcf` parts.
#[derive(Clone, Debug, Default)]
pub struct BcfParts {
    pub snapshot: Option<BcfSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.bcf` (2.1/✳️any) sources.
pub struct BcfAnalyzer;

impl ArtifactAnalyzer for BcfAnalyzer {
    type Parts = BcfParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        // 🕵️ Real sniff: BCF is a zip container that additionally carries a root `bcf.version`
        // entry. Reuses the zip artifact's own byte-level magic+EOCD check (never reimplemented
        // here) for the base confidence, then cheaply corroborates the `bcf.version` entry name
        // via a substring scan of the raw bytes -- filenames are stored as literal bytes in both
        // the local and central-directory headers, so this finds a real entry name without
        // paying for a full `decode_zip` (which would also inflate every snapshot PNG payload
        // just to read names -- the same cost tradeoff the zip analyzer's own sniff makes by
        // stopping at "does a well-formed EOCD exist" rather than parsing every entry).
        use crate::artifacts::zip::engine::{sniff_zip_bytes, SniffConfidence};
        match source {
            AnalyzeSource::Binary(bytes) => match sniff_zip_bytes(bytes) {
                SniffConfidence::Low => IoConfidence::Low,
                zip_confidence => {
                    let needle = b"bcf.version";
                    let has_bcf_version_name = bytes.len() >= needle.len() && bytes.windows(needle.len()).any(|w| w == needle);
                    match (zip_confidence, has_bcf_version_name) {
                        (SniffConfidence::High, true) => IoConfidence::High,
                        (SniffConfidence::High, false) => IoConfidence::Medium,
                        (SniffConfidence::Medium, _) => IoConfidence::Medium,
                        (SniffConfidence::Low, _) => unreachable!("Low was matched above"),
                    }
                }
            },
            // The DSL envelope (hex-wrapped text) preamble is what actually recognizes the text
            // form, not this byte-magic sniff.
            AnalyzeSource::Text(_) => IoConfidence::Low,
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = BcfParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <BcfSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <BcfSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::bcf::schema::snapshot::BcfEntry;

    #[test]
    fn sniff_bumps_to_high_when_bcf_version_entry_name_is_present() {
        let snap = BcfSnapshot {
            schema: "stdio.bcf".into(),
            entries: vec![BcfEntry { name: "bcf.version".into(), data: b"<Version VersionId=\"2.1\"/>".to_vec() }],
            topics: Vec::new(),
        };
        let bytes = crate::artifacts::bcf::engine::encode_bcf(&snap).expect("encode");
        assert_eq!(BcfAnalyzer::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::High);
    }

    #[test]
    fn sniff_stays_medium_for_a_real_zip_without_bcf_version() {
        let zip_snap = crate::artifacts::zip::ZipSnapshot {
            schema: crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: vec![crate::artifacts::zip::schema::snapshot::ZipEntry {
                name: "unrelated.txt".into(),
                data: b"not a bcf archive".to_vec(),
                ..Default::default()
            }],
            comment: String::new(),
        };
        let bytes = crate::artifacts::zip::engine::encode_zip(&zip_snap).expect("encode plain zip");
        assert_eq!(BcfAnalyzer::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::Medium);
    }

    #[test]
    fn sniff_rejects_non_zip_garbage() {
        assert_eq!(BcfAnalyzer::sniff(&AnalyzeSource::Binary(b"not a zip at all")), IoConfidence::Low);
    }

    #[test]
    fn sniff_treats_text_source_as_low() {
        assert_eq!(BcfAnalyzer::sniff(&AnalyzeSource::Text("deadbeef")), IoConfidence::Low);
    }
}
//#endregion 🧪️Tests
