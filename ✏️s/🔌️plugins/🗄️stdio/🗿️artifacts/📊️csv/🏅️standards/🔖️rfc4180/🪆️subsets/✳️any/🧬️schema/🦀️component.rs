//! 🧬️ CsvArtifact schema — full artifact state.

use crate::artifacts::csv::schema::snapshot::CsvRecord;
use crate::artifacts::csv::CsvSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.csv` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv")]
pub struct CsvArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub has_header: bool,
    #[state(artifact)]
    #[serde(default)]
    pub records: Vec<CsvRecord>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for CsvArtifact {
    fn default() -> Self {
        Self::from_snapshot(CsvSnapshot::default())
    }
}

impl CsvArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> CsvSnapshot {
        CsvSnapshot {
            schema: self.schema.clone(),
            has_header: self.has_header,
            records: self.records.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: CsvSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            has_header: snapshot.has_header,
            records: snapshot.records,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: CsvSnapshot) {
        self.schema = snapshot.schema;
        self.has_header = snapshot.has_header;
        self.records = snapshot.records;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.csv`.
pub fn csv_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.csv",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::csv::{CsvDiff, CsvMutation, CsvSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.csv` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct CsvBuilderConstruction {
        snapshot: CsvSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for CsvBuilderConstruction {
        type Snapshot = CsvSnapshot;
        type Mutation = CsvMutation;
        type Diff = CsvDiff;
        fn empty() -> Self {
            Self { snapshot: CsvSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<CsvSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<CsvSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::csv::schema::mutations::apply_csv_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <CsvDiff as protocol::MutationDiff<CsvSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
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
    pub struct CsvAnalyzerAnalysis;

    /// 🔍 CSV has no magic bytes — sniff by checking that a real RFC4180 parse of the
    /// first few lines yields a consistent field count across records (a strong tabular
    /// signal) and that at least one delimiter/quote is actually present.
    fn looks_like_csv(text: &str) -> IoConfidence {
        let sample: String = text.lines().take(20).collect::<Vec<_>>().join("\n");
        if sample.trim().is_empty() {
            return IoConfidence::Low;
        }
        let snapshot = crate::artifacts::csv::schema::snapshot::decode_csv_with(&sample, false);
        if snapshot.records.is_empty() {
            return IoConfidence::Low;
        }
        let width = snapshot.records[0].fields.len();
        if width == 0 {
            return IoConfidence::Low;
        }
        let consistent = snapshot.records.iter().all(|r| r.fields.len() == width);
        let has_delimiter = sample.contains(',');
        match (consistent, width > 1, has_delimiter) {
            (true, true, true) => IoConfidence::High,
            (true, _, true) => IoConfidence::Medium,
            _ => IoConfidence::Low,
        }
    }

    impl ArtifactAnalysis for CsvAnalyzerAnalysis {
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
            assert_eq!(CsvAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
        }

        #[test]
        fn sniff_unrelated_text_is_low() {
            assert_eq!(CsvAnalyzerAnalysis::sniff(&AnalyzeSource::Text("just a plain sentence.")), IoConfidence::Low);
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec CsvBuilderFacets {
        construction: derived_construction::CsvBuilderConstruction,
        analysis: derived_analysis::CsvAnalyzerAnalysis,
        composition: super::super::io::derived_composition::CsvComposerComposition,
    }
    builder: CsvBuilder,
    analyzer: CsvAnalyzer,
    composer: CsvComposer,
);
//#endregion 🧬️DerivedArtifactFacets
