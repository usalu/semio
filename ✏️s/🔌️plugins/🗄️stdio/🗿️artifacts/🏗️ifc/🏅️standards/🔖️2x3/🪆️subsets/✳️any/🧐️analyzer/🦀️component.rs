//! 🧐️ Ifc2x3Analyzer (2x3/✳️any) — read-only analysis; artifact/standard levels delegate here.
//! `sniff` is a REAL magic-probe (checks for the `ISO-10303-21` SPF envelope plus an `IFC2X3`
//! FILE_SCHEMA token), not a constant — `4`'s own `IfcAnalyzer::sniff(_source)` ignores its
//! argument entirely (a flagged `POLICY_SNIFF_REALITY` shape this ticket's law says never gets a
//! new allowlist entry).

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.ifc.2x3` parts.
#[derive(Clone, Debug, Default)]
pub struct Ifc2x3Parts {
    pub snapshot: Option<Ifc2x3Snapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Sniff
/// 🔍️ Real, honest confidence probe: `High` when the text/bytes look like a Part-21 envelope AND
/// declare `IFC2X3` in `FILE_SCHEMA`; `Medium` for a Part-21 envelope of an unknown schema (could
/// still decode -- IFC2X3 is layered on the same generic tokenizer); `Low` otherwise.
fn sniff_text(body: &str) -> IoConfidence {
    let trimmed = body.trim_start();
    if trimmed.starts_with("ISO-10303-21") {
        if trimmed.contains("IFC2X3") { IoConfidence::High } else { IoConfidence::Medium }
    } else {
        IoConfidence::Low
    }
}
//#endregion 🔖️Sniff

//#region 🔖️Analyzer
pub struct Ifc2x3Analyzer;

impl ArtifactAnalyzer for Ifc2x3Analyzer {
    type Parts = Ifc2x3Parts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Text(text) => {
                let body = match store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                sniff_text(body)
            }
            AnalyzeSource::Binary(bytes) => match std::str::from_utf8(bytes) {
                Ok(text) => sniff_text(text),
                Err(_) => IoConfidence::Low,
            },
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = Ifc2x3Parts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                    }
                },
                AnalyzeSource::Binary(bytes) => match <Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes) {
                    Ok(snapshot) => parts.snapshot = Some(snapshot),
                    Err(err) => {
                        confidence = IoConfidence::Low;
                        diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
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

    #[test]
    fn sniff_high_confidence_for_ifc2x3_envelope() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('IFC2X3'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
        assert_eq!(Ifc2x3Analyzer::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[test]
    fn sniff_medium_confidence_for_other_part21_schema() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
        assert_eq!(Ifc2x3Analyzer::sniff(&AnalyzeSource::Text(text)), IoConfidence::Medium);
    }

    #[test]
    fn sniff_low_confidence_for_non_part21_input() {
        assert_eq!(Ifc2x3Analyzer::sniff(&AnalyzeSource::Text("not a step file at all")), IoConfidence::Low);
        assert_eq!(Ifc2x3Analyzer::sniff(&AnalyzeSource::Binary(&[0xFF, 0xD8, 0xFF])), IoConfidence::Low);
    }
}
//#endregion 🧪️Tests
