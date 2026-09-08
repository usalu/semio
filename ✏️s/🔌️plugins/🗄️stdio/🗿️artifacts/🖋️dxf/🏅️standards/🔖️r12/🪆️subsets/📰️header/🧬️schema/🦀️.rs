//! 🧬️ DxfArtifact schema — full artifact state (mirrors `DxfSnapshot`'s persisted fields
//! one-for-one; see `📸️snapshot/🦀️.rs` module docs for the full typed-model rationale).

use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfOtherTable, DxfTables};
use crate::DxfSnapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.dxf` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf")]
pub struct DxfArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub header_vars: Vec<DxfHeaderVar>,
    #[state(artifact)]
    #[value(default)]
    pub tables: DxfTables,
    #[state(artifact)]
    #[value(default)]
    pub other_tables: Vec<DxfOtherTable>,
    #[state(artifact)]
    #[value(default)]
    pub blocks: Vec<DxfBlock>,
    #[state(artifact)]
    #[value(default)]
    pub entities: Vec<DxfEntity>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DxfArtifact {
    fn default() -> Self {
        Self::from_snapshot(DxfSnapshot::default())
    }
}

impl DxfArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> DxfSnapshot {
        DxfSnapshot { schema: self.schema.clone(), header_vars: self.header_vars.clone(), tables: self.tables.clone(), other_tables: self.other_tables.clone(), blocks: self.blocks.clone(), entities: self.entities.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: DxfSnapshot) -> Self {
        Self { schema: snapshot.schema, header_vars: snapshot.header_vars, tables: snapshot.tables, other_tables: snapshot.other_tables, blocks: snapshot.blocks, entities: snapshot.entities }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: DxfSnapshot) {
        self.schema = snapshot.schema;
        self.header_vars = snapshot.header_vars;
        self.tables = snapshot.tables;
        self.other_tables = snapshot.other_tables;
        self.blocks = snapshot.blocks;
        self.entities = snapshot.entities;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.dxf`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dxf_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.stdio.dxf",
        artifact: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
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
    use crate::{DxfDiff, DxfMutation, DxfSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.dxf` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct DxfBuilderConstruction {
        snapshot: DxfSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for DxfBuilderConstruction {
        type Snapshot = DxfSnapshot;
        type Mutation = DxfMutation;
        type Diff = DxfDiff;
        fn empty() -> Self {
            Self { snapshot: DxfSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<DxfSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DxfSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_dxf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <DxfDiff as protocol::MutationDiff<DxfSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::DxfSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.dxf` parts.
    #[derive(Clone, Debug, Default)]
    pub struct DxfParts {
        pub snapshot: Option<DxfSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.dxf` (r12/📰️header) sources.
    pub struct DxfAnalyzerAnalysis;

    impl ArtifactAnalysis for DxfAnalyzerAnalysis {
        type Parts = DxfParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };

        /// 🧭️ DXF ASCII has no fixed magic byte (unlike binary formats), so this is a structural
        /// heuristic rather than an exact match: the first non-blank line must trim to a valid
        /// integer group code, and one of the DXF section/version markers (`SECTION`, `HEADER`,
        /// `ENTITIES`, or an `AC10xx`-style version string) must appear among the first tags.
        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            let text = match source {
                AnalyzeSource::Text(text) => Some(*text),
                AnalyzeSource::Binary(_) => None,
            };
            let Some(text) = text else { return IoConfidence::Low };
            let body = match store::semio_format::split_text_preamble(text) {
                Ok((_, rest)) => rest,
                Err(_) => text,
            };
            let lines: Vec<&str> = body.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
            let Some(first) = lines.first() else { return IoConfidence::Low };
            if first.parse::<i32>().is_err() {
                return IoConfidence::Low;
            }
            let has_marker = lines.iter().take(64).any(|l| matches!(*l, "SECTION" | "HEADER" | "ENTITIES" | "EOF") || (l.len() == 6 && l.starts_with("AC") && l[2..].chars().all(|c| c.is_ascii_digit())));
            if has_marker {
                IoConfidence::High
            } else {
                IoConfidence::Medium
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = DxfParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <DxfSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <DxfSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec DxfBuilderFacets {
        construction: DxfBuilderConstruction,
        analysis: DxfAnalyzerAnalysis,
        composition: super::super::io::derived_composition::DxfComposerComposition,
    }
    builder: DxfBuilder,
    analyzer: DxfAnalyzer,
    composer: DxfComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
use crate::STDIO_DXF_DOCUMENT_SCHEMA;

/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_dxf_snapshot() -> DxfSnapshot {
    DxfSnapshot::default()
}

/// 🧬️ Genuinely 2-level-nested (a `BLOCK` with a nested entity), every-section demo snapshot —
/// the single source of truth for `fixture_honesty_law`'s shipped `🗣️.dsl.semio`/
/// `🎒️.pack.semio` fixtures AND `grammar_conformance_law`/`protocol_walk_law`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_dxf_snapshot() -> DxfSnapshot {
    use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue};
    DxfSnapshot {
        schema: STDIO_DXF_DOCUMENT_SCHEMA.into(),
        header_vars: vec![
            DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] },
            DxfHeaderVar { name: "$INSBASE".into(), group_code: 10, value: DxfValue::Point { value: [1.0, 2.0, 3.0] }, extra_group_codes: vec![] },
        ],
        tables: DxfTables {
            layers: vec![DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] }],
            styles: vec![DxfStyle { name: "STANDARD".into(), flags: 0, font_name: "txt".into(), unknown_group_codes: vec![] }],
            linetypes: vec![DxfLinetype { name: "CONTINUOUS".into(), flags: 0, description: "Solid".into(), unknown_group_codes: vec![] }],
        },
        other_tables: vec![DxfOtherTable { name: "VPORT".into(), tags: vec![DxfTag { code: 2, value: "*ACTIVE".into() }] }],
        blocks: vec![DxfBlock { name: "MYBLOCK".into(), base_point: [0.0, 0.0, 0.0], entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }], unknown_group_codes: vec![] }],
        entities: vec![
            DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] },
            DxfEntity::Circle { center: [1.0, 1.0, 0.0], radius: 2.0, layer: "0".into(), unknown_group_codes: vec![] },
            DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![(10, DxfValue::Double { value: 0.0 })] },
        ],
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
