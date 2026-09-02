//! 🧬️ JsonArtifact schema — full artifact state.

use crate::artifacts::json::schema::snapshot::JsonValue;
use crate::artifacts::json::JsonSnapshot;
use schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.json` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.json")]
pub struct JsonArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub value: JsonValue,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for JsonArtifact {
    fn default() -> Self {
        Self::from_snapshot(JsonSnapshot::default())
    }
}

impl JsonArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> JsonSnapshot {
        JsonSnapshot { schema: self.schema.clone(), value: self.value.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: JsonSnapshot) -> Self {
        Self { schema: snapshot.schema, value: snapshot.value }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: JsonSnapshot) {
        self.schema = snapshot.schema;
        self.value = snapshot.value;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.json`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn json_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.json",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::json::{JsonDiff, JsonMutation, JsonSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.json` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct JsonBuilderConstruction {
        snapshot: JsonSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for JsonBuilderConstruction {
        type Snapshot = JsonSnapshot;
        type Mutation = JsonMutation;
        type Diff = JsonDiff;
        fn empty() -> Self {
            Self { snapshot: JsonSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<JsonSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<JsonSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::json::schema::mutations::apply_json_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <JsonDiff as protocol::MutationDiff<JsonSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::json::JsonSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.json` parts.
    #[derive(Clone, Debug, Default)]
    pub struct JsonParts {
        pub snapshot: Option<JsonSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.json` (rfc8259/✳️any) sources.
    pub struct JsonAnalyzerAnalysis;

    /// 🔍 JSON has no magic bytes — a real parse attempt with our own rfc8259 recursive-descent
    /// parser is the strongest available signal (cheap for realistic file sizes); fall back to a
    /// first-non-whitespace-character heuristic when the bytes aren't valid UTF-8 text at all.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn looks_like_json(text: &str) -> IoConfidence {
        if crate::artifacts::json::schema::snapshot::parse_json_text(text.trim()).is_ok() {
            return IoConfidence::High;
        }
        match text.trim_start().chars().next() {
            Some('{') | Some('[') | Some('"') => IoConfidence::Medium,
            _ => IoConfidence::Low,
        }
    }

    impl ArtifactAnalysis for JsonAnalyzerAnalysis {
        type Parts = JsonParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    looks_like_json(body)
                }
                AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                    Ok((_, inner)) => match String::from_utf8(inner) {
                        Ok(text) => looks_like_json(&text),
                        Err(_) => IoConfidence::Low,
                    },
                    Err(_) => match std::str::from_utf8(bytes) {
                        Ok(text) => looks_like_json(text),
                        Err(_) => IoConfidence::Low,
                    },
                },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = JsonParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <JsonSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <JsonSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
        fn sniff_real_json_object_is_high() {
            let text = "{\"a\": 1, \"b\": [1, 2, 3]}";
            assert_eq!(JsonAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
        }

        #[test]
        fn sniff_malformed_json_is_not_high() {
            let text = "{\"a\": 1, \"b\": [1, 2, 3]";
            assert_ne!(JsonAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
        }

        #[test]
        fn sniff_unrelated_text_is_low() {
            assert_eq!(JsonAnalyzerAnalysis::sniff(&AnalyzeSource::Text("just a plain sentence.")), IoConfidence::Low);
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec JsonBuilderFacets {
        construction: JsonBuilderConstruction,
        analysis: JsonAnalyzerAnalysis,
        composition: super::super::io::derived_composition::JsonComposerComposition,
    }
    builder: JsonBuilder,
    analyzer: JsonAnalyzer,
    composer: JsonComposer,
);
//#endregion 🧬️DerivedArtifactFacets
