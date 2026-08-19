//! 🧬️ DxfArtifact schema — full artifact state (mirrors `DxfSnapshot`'s persisted fields
//! one-for-one; see `📸️snapshot/🦀️component.rs` module docs for the full typed-model rationale).

use crate::artifacts::dxf::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfOtherTable, DxfTables};
use crate::artifacts::dxf::DxfSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.dxf` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf")]
pub struct DxfArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub header_vars: Vec<DxfHeaderVar>,
    #[state(artifact)]
    #[serde(default)]
    pub tables: DxfTables,
    #[state(artifact)]
    #[serde(default)]
    pub other_tables: Vec<DxfOtherTable>,
    #[state(artifact)]
    #[serde(default)]
    pub blocks: Vec<DxfBlock>,
    #[state(artifact)]
    #[serde(default)]
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
    pub async fn to_snapshot(&self) -> DxfSnapshot {
        DxfSnapshot { schema: self.schema.clone(), header_vars: self.header_vars.clone(), tables: self.tables.clone(), other_tables: self.other_tables.clone(), blocks: self.blocks.clone(), entities: self.entities.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub async fn from_snapshot(snapshot: DxfSnapshot) -> Self {
        Self { schema: snapshot.schema, header_vars: snapshot.header_vars, tables: snapshot.tables, other_tables: snapshot.other_tables, blocks: snapshot.blocks, entities: snapshot.entities }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub async fn set_snapshot(&mut self, snapshot: DxfSnapshot) {
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
pub async fn dxf_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.dxf",
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
    use crate::artifacts::dxf::{DxfDiff, DxfMutation, DxfSnapshot};
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
        async fn empty() -> Self {
            Self { snapshot: DxfSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<DxfSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DxfSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::dxf::schema::mutations::apply_dxf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <DxfDiff as protocol::MutationDiff<DxfSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
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
    use crate::artifacts::dxf::DxfSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.dxf` parts.
    #[derive(Clone, Debug, Default)]
    pub struct DxfParts {
        pub snapshot: Option<DxfSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.dxf` (r12/✳️any) sources.
    pub struct DxfAnalyzerAnalysis;

    impl ArtifactAnalysis for DxfAnalyzerAnalysis {
        type Parts = DxfParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };

        /// 🧭️ DXF ASCII has no fixed magic byte (unlike binary formats), so this is a structural
        /// heuristic rather than an exact match: the first non-blank line must trim to a valid
        /// integer group code, and one of the DXF section/version markers (`SECTION`, `HEADER`,
        /// `ENTITIES`, or an `AC10xx`-style version string) must appear among the first tags.
        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
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

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
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
use crate::artifacts::dxf::STDIO_DXF_DOCUMENT_SCHEMA;

/// 🌱 Empty persisted snapshot.
pub async fn empty_dxf_snapshot() -> DxfSnapshot {
    DxfSnapshot::default()
}

/// 🧬️ Genuinely 2-level-nested (a `BLOCK` with a nested entity), every-section demo snapshot —
/// the single source of truth for `fixture_honesty_law`'s shipped `🗣️example.dsl.semio`/
/// `🎒️example.pack.semio` fixtures AND `grammar_conformance_law`/`protocol_walk_law`.
pub async fn demo_dxf_snapshot() -> DxfSnapshot {
    use crate::artifacts::dxf::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue};
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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_matches_schema() {
        let snapshot = empty_dxf_snapshot();
        assert_eq!(snapshot.schema, STDIO_DXF_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn codec_round_trip() {
        let snap = empty_dxf_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <DxfSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <DxfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️CodecRetentionLaw
    /// 🔁️ decode→encode retains every field across every section — documented NORMAL FORM: from
    /// the SECOND generation onward decode/encode is a true fixed point.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        use crate::artifacts::dxf::schema::snapshot::{parse_dxf_document, print_dxf_document, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue};
        let snap1 = DxfSnapshot {
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
            blocks: vec![DxfBlock {
                name: "MYBLOCK".into(),
                base_point: [0.0, 0.0, 0.0],
                entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }],
                unknown_group_codes: vec![],
            }],
            entities: vec![
                DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] },
                DxfEntity::Circle { center: [1.0, 1.0, 0.0], radius: 2.0, layer: "0".into(), unknown_group_codes: vec![] },
                DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![(10, DxfValue::Double { value: 0.0 })] },
            ],
        };
        let text2 = print_dxf_document(&snap1);
        let snap2 = parse_dxf_document(&text2).expect("re-parse");
        assert_eq!(snap1, snap2, "decode(encode(snap)) must be a fixed point");

        let text3 = print_dxf_document(&snap2);
        assert_eq!(text2, text3, "from generation 2 onward, print(parse(text)) must be a true text fixed point too");
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️ConformanceLaws
    /// 🧪️ Per-artifact conformance laws — grammar/protocol parseability, `Recognizer` against
    /// real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
    /// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::dxf::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        #[semio_framework_async_macros::async_test]
        async fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_dxf_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        #[semio_framework_async_macros::async_test]
        async fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_dxf_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_dxf_snapshot();

            let parsed = <DxfSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_dxf_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_dxf_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <DxfSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_dxf_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_dxf_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
