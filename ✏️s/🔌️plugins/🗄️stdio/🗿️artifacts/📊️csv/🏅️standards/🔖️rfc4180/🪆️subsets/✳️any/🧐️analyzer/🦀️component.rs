//! 🧐️ CsvAnalyzer (rfc4180/✳️any) — read-only analysis, successor to the pre-migration
//! CsvDecomposer. Real logic; artifact/standard levels delegate here.

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::csv::CsvSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.csv` parts.
#[derive(Clone, Debug, Default)]
pub struct CsvParts {
    pub snapshot: Option<CsvSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.csv` (rfc4180/✳️any) sources.
pub struct CsvAnalyzer;

/// 🔍 CSV has no magic bytes — sniff by checking that a real RFC4180 parse of the
/// first few lines yields a consistent field count across records (a strong tabular
/// signal) and that at least one delimiter/quote is actually present.
fn looks_like_csv(text: &str) -> IoConfidence {
    let sample: String = text.lines().take(20).collect::<Vec<_>>().join("\n");
    if sample.trim().is_empty() {
        return IoConfidence::Low;
    }
    let records = crate::artifacts::csv::engine::decode_csv_with(&sample, false);
    if records.rows.is_empty() {
        return IoConfidence::Low;
    }
    let width = records.rows[0].len();
    if width == 0 {
        return IoConfidence::Low;
    }
    let consistent = records.rows.iter().all(|r| r.len() == width);
    let has_delimiter = sample.contains(',');
    match (consistent, width > 1, has_delimiter) {
        (true, true, true) => IoConfidence::High,
        (true, _, true) => IoConfidence::Medium,
        _ => IoConfidence::Low,
    }
}

impl ArtifactAnalyzer for CsvAnalyzer {
    type Parts = CsvParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Text(text) => {
                let body = match store::semio_format::split_text_preamble(text) {
                    Ok((_, rest)) => rest,
                    Err(_) => text,
                };
                looks_like_csv(body)
            }
            AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                Ok((_, inner)) => match String::from_utf8(inner) {
                    Ok(text) => looks_like_csv(&text),
                    Err(_) => IoConfidence::Low,
                },
                Err(_) => match std::str::from_utf8(bytes) {
                    Ok(text) => looks_like_csv(text),
                    Err(_) => IoConfidence::Low,
                },
            },
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = CsvParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => match <CsvSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                AnalyzeSource::Binary(bytes) => match <CsvSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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

    #[test]
    fn sniff_real_csv_table_is_high() {
        let text = "a,b,c\n1,2,3\n4,5,6\n";
        assert_eq!(CsvAnalyzer::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[test]
    fn sniff_unrelated_text_is_low() {
        assert_eq!(CsvAnalyzer::sniff(&AnalyzeSource::Text("just a plain sentence.")), IoConfidence::Low);
    }
}
//#endregion 🧪️Tests
