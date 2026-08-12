//! 🧬️ Ifc2x3Artifact schema — full artifact state for the `2x3` standard (buildingSMART
//! Coordination View 2.0 era, ISO/PAS 16739:2005 schema). Sibling of `🔖️4`'s `IfcArtifact`, own
//! distinct schema id `s.stdio.ifc.2x3` so the two standards' descriptors never collide in the
//! flat `::schema::register_artifact_schema_descriptor` registry.

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.2x3")]
pub struct Ifc2x3Artifact {
    #[state(persistent)]
    pub schema: String,
    /// 📦️ The full, lossless generic Part-21 graph, wrapped in this standard's own
    /// [`Ifc2x3Snapshot`] type — the actual persisted state.
    #[state(persistent)]
    #[serde(default)]
    pub document: crate::artifacts::step::engine::part21::Part21Document,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Ifc2x3Artifact {
    fn default() -> Self {
        Self::from_snapshot(Ifc2x3Snapshot::default())
    }
}

impl Ifc2x3Artifact {
    pub fn to_snapshot(&self) -> Ifc2x3Snapshot {
        Ifc2x3Snapshot { schema: self.schema.clone(), document: self.document.clone() }
    }

    pub fn from_snapshot(snapshot: Ifc2x3Snapshot) -> Self {
        Self { schema: snapshot.schema, document: snapshot.document }
    }

    pub fn set_snapshot(&mut self, snapshot: Ifc2x3Snapshot) {
        self.schema = snapshot.schema;
        self.document = snapshot.document;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn ifc2x3_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.ifc.2x3",
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
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::diff::Ifc2x3Diff;
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::mutations::{apply_ifc2x3_mutation, Ifc2x3Mutation};
    use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct Ifc2x3BuilderConstruction {
        snapshot: Ifc2x3Snapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for Ifc2x3BuilderConstruction {
        type Snapshot = Ifc2x3Snapshot;
        type Mutation = Ifc2x3Mutation;
        type Diff = Ifc2x3Diff;
        fn empty() -> Self {
            Self { snapshot: Ifc2x3Snapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Ifc2x3Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_ifc2x3_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <Ifc2x3Diff as protocol::MutationDiff<Ifc2x3Snapshot>>::apply(&diff, &self.snapshot);
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
    pub struct Ifc2x3AnalyzerAnalysis;

    impl ArtifactAnalysis for Ifc2x3AnalyzerAnalysis {
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
            assert_eq!(Ifc2x3AnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
        }

        #[test]
        fn sniff_medium_confidence_for_other_part21_schema() {
            let text = "ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
            assert_eq!(Ifc2x3AnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::Medium);
        }

        #[test]
        fn sniff_low_confidence_for_non_part21_input() {
            assert_eq!(Ifc2x3AnalyzerAnalysis::sniff(&AnalyzeSource::Text("not a step file at all")), IoConfidence::Low);
            assert_eq!(Ifc2x3AnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&[0xFF, 0xD8, 0xFF])), IoConfidence::Low);
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec Ifc2x3BuilderFacets {
        construction: derived_construction::Ifc2x3BuilderConstruction,
        analysis: derived_analysis::Ifc2x3AnalyzerAnalysis,
        composition: super::super::io::derived_composition::Ifc2x3ComposerComposition,
    }
    builder: Ifc2x3Builder,
    analyzer: Ifc2x3Analyzer,
    composer: Ifc2x3Composer,
);
//#endregion 🧬️DerivedArtifactFacets
