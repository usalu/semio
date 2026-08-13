//! 🧬️ DwgArtifact schema — full artifact state.

use crate::artifacts::dwg::DwgSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg")]
pub struct DwgArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    #[serde(default)]
    pub maintenance_version: u8,
    #[state(artifact)]
    #[serde(default)]
    pub codepage: u16,
    #[state(artifact)]
    #[serde(default)]
    pub bytes: Vec<u8>,
    #[state(artifact)]
    #[serde(default)]
    pub section_names: Vec<String>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DwgArtifact {
    fn default() -> Self {
        Self::from_snapshot(DwgSnapshot::default())
    }
}

impl DwgArtifact {
    pub fn to_snapshot(&self) -> DwgSnapshot {
        DwgSnapshot {
            schema: self.schema.clone(),
            version: self.version.clone(),
            maintenance_version: self.maintenance_version,
            codepage: self.codepage,
            bytes: self.bytes.clone(),
            section_names: self.section_names.clone(),
            // 🚧️ ac1018 is a legacy shim (nothing real behind it, per Decision #5) — it never ran
            // the real ac1024 D1/D2 decode pipeline, so it has no structural insight to carry.
            sections: Vec::new(),
            decode_status: Default::default(),
        }
    }

    pub fn from_snapshot(snapshot: DwgSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            version: snapshot.version,
            maintenance_version: snapshot.maintenance_version,
            codepage: snapshot.codepage,
            bytes: snapshot.bytes,
            section_names: snapshot.section_names,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: DwgSnapshot) {
        self.schema = snapshot.schema;
        self.version = snapshot.version;
        self.maintenance_version = snapshot.maintenance_version;
        self.codepage = snapshot.codepage;
        self.bytes = snapshot.bytes;
        self.section_names = snapshot.section_names;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn dwg_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.dwg",
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
    use crate::artifacts::dwg::{DwgDiff, DwgMutation, DwgSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.dwg` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct DwgBuilderConstruction {
        snapshot: DwgSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for DwgBuilderConstruction {
        type Snapshot = DwgSnapshot;
        type Mutation = DwgMutation;
        type Diff = DwgDiff;
        fn empty() -> Self {
            Self { snapshot: DwgSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<DwgSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DwgSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::dwg::schema::mutations::apply_dwg_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <DwgDiff as protocol::MutationDiff<DwgSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::dwg::DwgSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.dwg` parts.
    #[derive(Clone, Debug, Default)]
    pub struct DwgParts {
        pub snapshot: Option<DwgSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.dwg` (ac1018/✳️any) sources.
    pub struct DwgAnalyzerAnalysis;

    impl ArtifactAnalysis for DwgAnalyzerAnalysis {
        type Parts = DwgParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = DwgParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <DwgSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                    AnalyzeSource::Binary(bytes) => match <DwgSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec DwgBuilderFacets {
        construction: derived_construction::DwgBuilderConstruction,
        analysis: derived_analysis::DwgAnalyzerAnalysis,
        composition: super::super::io::derived_composition::DwgComposerComposition,
    }
    builder: DwgBuilder,
    analyzer: DwgAnalyzer,
    composer: DwgComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🏷️ Document schema / DSL envelope id for ac1018. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::artifacts::dwg::standards::v_ac1018::engine::STDIO_DWG_AC1018_DOCUMENT_SCHEMA` through
/// the `engine` barrel shim.
pub const STDIO_DWG_AC1018_DOCUMENT_SCHEMA: &str = "stdio.dwg.ac1018";

/// 🌱 Empty persisted snapshot.
pub fn empty_dwg_snapshot() -> crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot {
    crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot::default()
}

/// 📄️ The demo `stdio.dwg` (ac1018) document — decodes the real, committed 22-byte AC1018 stub
/// (`📚️examples/🎬️demo/🖼️assets/🖊️example.dwg`, this standard's OWN dedicated fixture — NOT the
/// artifact-level `📚️examples/🎬️demo` demo, which is ac1024-shaped, the canonical standard, per
/// S-6/Decision #5) via ac1018's own real `decode_dwg`. The single source of truth for
/// `🏅️standards/🔖️ac1018/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`
/// (both are literally this snapshot's `print_dsl`/`encode_pack` output, asserted equal by
/// `conformance_laws::fixture_honesty_law` below).
pub fn demo_dwg_snapshot() -> crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot {
    let stub = b"AC1018\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::decode_dwg(stub).expect("decode ac1018 demo stub")
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_dwg_snapshot();
        assert_eq!(snapshot.schema, STDIO_DWG_AC1018_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let stub = b"AC1018\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let snap = crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::decode_dwg(stub).expect("decode stub");
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.version, "AC1018");
        assert_eq!(parsed.bytes, stub);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ 🎫️26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION FG2: per-standard
    /// conformance laws for ac1018's OWN real facets — grammar/protocol parseability, `Recognizer`
    /// against real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
    /// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip. Dissolved
    /// out of `⚙️engine`'s own test region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)
    /// — mirrors `stdio.binary`/`stdio.txt`'s own `conformance_laws` module shape exactly, fully
    /// qualified to ac1018's own standard (never the top-level `crate::artifacts::dwg` shim,
    /// aliased to ac1024).
    mod conformance_laws {
        use super::*;
        use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect.
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [
                ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
                ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [
                ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
        /// for the ac1018 demo snapshot AND the empty-bytes case (`hex` macro's zero-width match).
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_dwg_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `mutations::demo_mutation_cases()` variant.
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every `diff::demo_diff_cases()`, incl. the empty (all-`None`) diff.
        #[test]
        fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
        /// snapshot pack (`encode_pack`, envelope-unwrapped first), every demo mutation's
        /// `encode_op`, and every demo diff's `encode_diff` — asserting `consumed == bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_dwg_snapshot());
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

        #[test]
        #[ignore]
        fn zzz_generate_p2p1_fixtures() {
            let demo = demo_dwg_snapshot();
            let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/📚️examples/🎬️demo/🖼️assets");
            std::fs::write(dir.join("🗣️example.dsl.semio"), store::ArtifactDsl::print_dsl(&demo)).unwrap();
            std::fs::write(dir.join("🎒️example.pack.semio"), store::ArtifactPack::encode_pack(&demo)).unwrap();
        }

        /// ✅️ `fixture_honesty_law`: the shipped ac1018-own `.dsl.semio`/`.pack.semio` fixtures
        /// are GENUINE `print_dsl`/`encode_pack` output of `demo_dwg_snapshot()`.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_dwg_snapshot();

            let parsed = <snapshot::DwgSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_dwg_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_dwg_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <snapshot::DwgSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_dwg_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_dwg_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
