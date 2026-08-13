//! 🧬️ MdArtifact schema — full artifact state.

use crate::artifacts::md::schema::snapshot::MdBlock;
use crate::artifacts::md::MdSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.md` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md")]
pub struct MdArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub blocks: Vec<MdBlock>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for MdArtifact {
    fn default() -> Self {
        Self::from_snapshot(MdSnapshot::default())
    }
}

impl MdArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> MdSnapshot {
        MdSnapshot {
            schema: self.schema.clone(),
            blocks: self.blocks.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: MdSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            blocks: snapshot.blocks,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: MdSnapshot) {
        self.schema = snapshot.schema;
        self.blocks = snapshot.blocks;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.md`.
pub fn md_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.md",
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
    use crate::artifacts::md::{MdDiff, MdMutation, MdSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.md` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct MdBuilderConstruction {
        snapshot: MdSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for MdBuilderConstruction {
        type Snapshot = MdSnapshot;
        type Mutation = MdMutation;
        type Diff = MdDiff;
        fn empty() -> Self {
            Self { snapshot: MdSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<MdSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<MdSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::md::schema::mutations::apply_md_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <MdDiff as protocol::MutationDiff<MdSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::md::MdSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.md` parts.
    #[derive(Clone, Debug, Default)]
    pub struct MdParts {
        pub snapshot: Option<MdSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.md` (commonmark/✳️any) sources.
    pub struct MdAnalyzerAnalysis;

    /// 🔍 Markdown has no magic bytes — sniff by actually running the real block parser
    /// and checking for structural (non-paragraph) blocks, which plain text never produces.
    fn looks_like_markdown(text: &str) -> IoConfidence {
        if text.trim().is_empty() {
            return IoConfidence::Low;
        }
        let blocks = crate::artifacts::md::standards::v_commonmark::subsets::any::io::import::deserializers::parse_markdown_blocks(text);
        if blocks.is_empty() {
            return IoConfidence::Low;
        }
        let has_structure = blocks.iter().any(|b| {
            !matches!(
                b,
                crate::artifacts::md::schema::snapshot::MdBlock::Paragraph { inlines }
                    if inlines.iter().all(|n| matches!(n, crate::artifacts::md::schema::snapshot::MdInline::Text { .. }))
            )
        });
        if has_structure { IoConfidence::High } else { IoConfidence::Medium }
    }

    impl ArtifactAnalysis for MdAnalyzerAnalysis {
        type Parts = MdParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    looks_like_markdown(body)
                }
                AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                    Ok((_, inner)) => match String::from_utf8(inner) {
                        Ok(text) => looks_like_markdown(&text),
                        Err(_) => IoConfidence::Low,
                    },
                    Err(_) => match std::str::from_utf8(bytes) {
                        Ok(text) => looks_like_markdown(text),
                        Err(_) => IoConfidence::Low,
                    },
                },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = MdParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <MdSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                    AnalyzeSource::Binary(bytes) => match <MdSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
        fn sniff_real_markdown_structure_is_high() {
            let text = "# Title\n\n- one\n- two\n";
            assert_eq!(MdAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
        }

        #[test]
        fn sniff_plain_paragraph_text_is_medium() {
            assert_eq!(MdAnalyzerAnalysis::sniff(&AnalyzeSource::Text("just a plain sentence.")), IoConfidence::Medium);
        }

        #[test]
        fn sniff_empty_is_low() {
            assert_eq!(MdAnalyzerAnalysis::sniff(&AnalyzeSource::Text("")), IoConfidence::Low);
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_md_snapshot() -> MdSnapshot {
    MdSnapshot::default()
}

/// 📄️ The demo `stdio.md` document — genuinely exercises `Heading`/`Paragraph` (with `Strong`/
/// `Emphasis`/`Code` inline content on one physical line), a 2-level-nested `BlockQuote` (proving
/// `../../🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`'s `block-quote = {GT block}+`
/// genuine `Ref` self-recursion end-to-end), a fenced `CodeBlock` with a real info string, and
/// `ThematicBreak`. The single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/
/// `🎒️example.pack.semio` (both are literally this snapshot's `print_dsl`/`encode_pack` output,
/// asserted equal by `fixture_honesty_law` below) and for `grammar_conformance_law`'s own
/// reconstructed-body recognition test.
///
/// Deliberately does NOT use `MdBlock::List` (the grammar's own documented, architecturally
/// excluded leading-whitespace-count mechanism gap — see the snapshot grammar file's own header
/// comment), `MdBlock::HtmlBlock` (not one of the 5 kinds that grammar models), or a multi-line
/// `Paragraph`/quoted-multi-block `BlockQuote` (both would defeat the grammar's own single-`LINE`-
/// per-block-position recognition, documented on the same file).
///
/// 🐛 Block ORDER matters here for a second, independent reason (confirmed by direct
/// reproduction, filed in this wave's `mechanism_gaps` as `md-fence-byte-offset-corruption`, NOT a
/// dialect-design gap — a genuine pre-existing bug in the shared lexer's `Fence`-token byte-offset
/// bookkeeping, `🔍️lexer/🦀️component.rs`'s fence-scanning loop increments `byte_offset` for every
/// consumed char EXCEPT `'\n'` — so every token positioned AFTER a fence whose content spans N
/// lines gets its `byte_range` under-reported by N, which can desync `match_raw_span`'s
/// `text.get(start_byte..).find('\n')` lookup enough to make a subsequent `LINE` match
/// zero-width): `CodeBlock` is placed LAST here, with only `ThematicBreak` after it (a
/// pure-literal-token production, `"--" "-"`, untouched by raw-span corruption since it never
/// calls `LINE`/`REST`) — `BlockQuote`/`Paragraph` (both `LINE`-dependent) are placed BEFORE the
/// fence, never after it, sidestepping the corruption entirely rather than fixing the shared lexer
/// (out of this wave's ownership boundary).
pub fn demo_md_snapshot() -> MdSnapshot {
    use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};
    let blocks = vec![
        MdBlock::Heading { level: 1, inlines: vec![MdInline::Text { text: "Title".into() }] },
        MdBlock::BlockQuote {
            blocks: vec![MdBlock::BlockQuote {
                blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "Deeply quoted.".into() }] }],
            }],
        },
        MdBlock::Paragraph {
            inlines: vec![
                MdInline::Text { text: "Lossless ".into() },
                MdInline::Strong { inlines: vec![MdInline::Text { text: "markdown".into() }] },
                MdInline::Text { text: " body with ".into() },
                MdInline::Emphasis { inlines: vec![MdInline::Text { text: "emphasis".into() }] },
                MdInline::Text { text: " and ".into() },
                MdInline::Code { literal: "inline code".into() },
                MdInline::Text { text: ".".into() },
            ],
        },
        MdBlock::CodeBlock { info: Some("rust".into()), literal: "fn demo() -> i32 {\n    42\n}".into() },
        MdBlock::ThematicBreak,
    ];
    MdSnapshot { schema: crate::artifacts::md::STDIO_MD_DOCUMENT_SCHEMA.into(), blocks }
}
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec MdBuilderFacets {
        construction: derived_construction::MdBuilderConstruction,
        analysis: derived_analysis::MdAnalyzerAnalysis,
        composition: super::super::io::derived_composition::MdComposerComposition,
    }
    builder: MdBuilder,
    analyzer: MdAnalyzer,
    composer: MdComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::md::schema::diff::{
        diff_at_path, MdBlockAdded, MdBlockDiff, MdBlockModified, MdBlocksDiff, MdListItemAdded, MdListItemModified,
        MdListItemsDiff,
    };
    use crate::artifacts::md::schema::mutations::MdPathStep;
    use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};
    use crate::artifacts::md::standards::v_commonmark::subsets::any::io::export::serializers::render_markdown_blocks;
    use crate::artifacts::md::standards::v_commonmark::subsets::any::io::import::deserializers::{parse_inline, parse_markdown_blocks};
    use crate::artifacts::md::{MdDiff, MdMutation, STDIO_MD_DOCUMENT_SCHEMA};
    use protocol::command::DiffAlgebra;
    use protocol::{Mutation, MutationDiff};

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_md_snapshot();
        assert_eq!(snapshot.schema, STDIO_MD_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_md_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <MdSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <MdSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    #[test]
    fn demo_snapshot_round_trip() {
        let snap = demo_md_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <MdSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.blocks, snap.blocks);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <MdSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded.blocks, snap.blocks);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG1: per-artifact conformance laws (recipe §4's checklist item) — grammar/protocol
    /// parseability, `Recognizer` against real fixtures AND real `print_op`/`print_diff` output,
    /// `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff` bytes, and the
    /// fixture-honesty round-trip. Lives beside the rest of this artifact's schema tests (moved
    /// out of `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — these
    /// tests are this artifact's OWN early-warning, plus direct coverage of the mutations/diff
    /// facets that harness does not auto-discover at all.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::md::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
        /// `walk_protocol` laws below (a parse failure here fails fast with a clearer message).
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
        /// for the demo (genuinely block-quote-recursive) snapshot — same preamble-stripped body
        /// reconstruction `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses,
        /// so this is a direct proof this artifact will pass that harness once graduated.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_md_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `MdMutation` variant (`mutations::demo_mutation_cases()`), incl. the
        /// `List`-block `MdBlock` payload and both `MdPathStep` variants.
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
        /// for every representative `MdDiff` (`diff::demo_diff_cases()`), incl. both tri-states and
        /// the `Replace` kind-change fallback.
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
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff` — asserting
        /// `consumed == bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_md_snapshot());
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

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_md_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake again.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_md_snapshot();

            let parsed = <MdSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_md_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_md_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <MdSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_md_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_md_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws

    //#region 🔖️ParserUnitTests
    #[test]
    fn headings_all_levels() {
        let blocks = parse_markdown_blocks("# H1\n## H2\n###### H6\n");
        assert_eq!(blocks.len(), 3);
        assert!(matches!(&blocks[0], MdBlock::Heading { level: 1, .. }));
        assert!(matches!(&blocks[1], MdBlock::Heading { level: 2, .. }));
        assert!(matches!(&blocks[2], MdBlock::Heading { level: 6, .. }));
    }

    #[test]
    fn paragraph_and_fenced_code_block_with_info_string() {
        let text = "A paragraph of text.\n\n```rust\nfn main() {}\n```\n";
        let blocks = parse_markdown_blocks(text);
        assert_eq!(blocks.len(), 2);
        assert!(matches!(&blocks[0], MdBlock::Paragraph { .. }));
        match &blocks[1] {
            MdBlock::CodeBlock { info, literal } => {
                assert_eq!(info.as_deref(), Some("rust"));
                assert_eq!(literal, "fn main() {}");
            }
            other => panic!("expected fenced code block, got {other:?}"),
        }
    }

    #[test]
    fn indented_code_block() {
        let text = "    let x = 1;\n    let y = 2;\n";
        let blocks = parse_markdown_blocks(text);
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            MdBlock::CodeBlock { literal, .. } => assert_eq!(literal, "let x = 1;\nlet y = 2;"),
            other => panic!("expected indented code block, got {other:?}"),
        }
    }

    #[test]
    fn thematic_break_variants() {
        for text in ["---\n", "***\n", "___\n", "- - -\n"] {
            let blocks = parse_markdown_blocks(text);
            assert_eq!(blocks.len(), 1, "input {text:?}");
            assert!(matches!(&blocks[0], MdBlock::ThematicBreak), "input {text:?}");
        }
    }

    #[test]
    fn block_quote_recursive() {
        let text = "> # Quoted heading\n> a paragraph\n";
        let blocks = parse_markdown_blocks(text);
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            MdBlock::BlockQuote { blocks } => {
                assert_eq!(blocks.len(), 2);
                assert!(matches!(&blocks[0], MdBlock::Heading { level: 1, .. }));
                assert!(matches!(&blocks[1], MdBlock::Paragraph { .. }));
            }
            other => panic!("expected block quote, got {other:?}"),
        }
    }

    #[test]
    fn html_block_raw_retention() {
        let text = "<div class=\"note\">\nplain content\n</div>\n";
        let blocks = parse_markdown_blocks(text);
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            MdBlock::HtmlBlock { raw } => assert!(raw.contains("<div") && raw.contains("</div>")),
            other => panic!("expected html block, got {other:?}"),
        }
    }

    #[test]
    fn unordered_and_ordered_lists() {
        let unordered = parse_markdown_blocks("- one\n- two\n- three\n");
        assert_eq!(unordered.len(), 1);
        match &unordered[0] {
            MdBlock::List { ordered, items, tight, .. } => {
                assert!(!ordered);
                assert!(tight);
                assert_eq!(items.len(), 3);
            }
            other => panic!("expected list, got {other:?}"),
        }

        let ordered = parse_markdown_blocks("1. first\n2. second\n");
        match &ordered[0] {
            MdBlock::List { ordered, start, items, .. } => {
                assert!(*ordered);
                assert_eq!(*start, Some(1));
                assert_eq!(items.len(), 2);
            }
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn nested_list_and_loose_list() {
        let nested = parse_markdown_blocks("- outer\n  - inner a\n  - inner b\n- outer two\n");
        match &nested[0] {
            MdBlock::List { items, .. } => {
                assert_eq!(items.len(), 2);
                let has_nested = items[0].iter().any(|b| matches!(b, MdBlock::List { .. }));
                assert!(has_nested, "expected a nested list inside the first item, got {:?}", items[0]);
            }
            other => panic!("expected list, got {other:?}"),
        }

        let loose = parse_markdown_blocks("- one\n\n- two\n");
        match &loose[0] {
            MdBlock::List { tight, .. } => assert!(!tight, "blank line between items must make the list loose"),
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn emphasis_strong_links_images_and_html_in_inline() {
        let inline = parse_inline(
            "plain **strong** and *em* and [a link](https://example.com \"title\") and ![alt](img.png) and <br/>",
        );
        assert!(inline.iter().any(|n| matches!(n, MdInline::Strong { inlines } if inlines == &vec![MdInline::Text { text: "strong".into() }])));
        assert!(inline.iter().any(|n| matches!(n, MdInline::Emphasis { inlines } if inlines == &vec![MdInline::Text { text: "em".into() }])));
        let link = inline.iter().find_map(|n| match n {
            MdInline::Link { text, url, title } => Some((text.clone(), url.clone(), title.clone())),
            _ => None,
        }).expect("link present");
        assert_eq!(link.0, vec![MdInline::Text { text: "a link".into() }]);
        assert_eq!(link.1, "https://example.com");
        assert_eq!(link.2.as_deref(), Some("title"));
        let image = inline.iter().find_map(|n| match n {
            MdInline::Image { alt, url, .. } => Some((alt.clone(), url.clone())),
            _ => None,
        }).expect("image present");
        assert_eq!(image, ("alt".into(), "img.png".into()));
        assert!(inline.iter().any(|n| matches!(n, MdInline::HtmlInline { raw } if raw == "<br/>")));
    }

    #[test]
    fn inline_code_span_is_not_emphasis() {
        let inline = parse_inline("use `*not emphasis*` here");
        assert!(inline.iter().any(|n| matches!(n, MdInline::Code { literal } if literal == "*not emphasis*")));
    }

    #[test]
    fn soft_and_hard_breaks_in_paragraph() {
        let blocks = parse_markdown_blocks("line one  \nline two\\\nline three\nline four\n");
        match &blocks[0] {
            MdBlock::Paragraph { inlines } => {
                let hard_count = inlines.iter().filter(|n| matches!(n, MdInline::HardBreak)).count();
                let soft_count = inlines.iter().filter(|n| matches!(n, MdInline::SoftBreak)).count();
                assert_eq!(hard_count, 2, "trailing-2-spaces and trailing-backslash both hard-break: {inlines:?}");
                assert_eq!(soft_count, 1, "bare line ending soft-breaks: {inlines:?}");
            }
            other => panic!("expected paragraph, got {other:?}"),
        }
    }

    #[test]
    fn unrecognized_delimiter_degrades_to_plain_text() {
        // 🕳️ Exactly ONE `*` in the whole input -- genuinely unpairable (not to be confused with
        // an input containing a valid pair elsewhere, which correctly parses as emphasis).
        let inline = parse_inline("plain text with one lonely *delimiter and no partner at all");
        assert!(inline.iter().all(|n| matches!(n, MdInline::Text { .. })), "unmatched * must degrade to Text, got {inline:?}");
    }
    //#endregion 🔖️ParserUnitTests

    //#region 🔖️Fixtures
    fn sample_snapshot() -> MdSnapshot {
        MdSnapshot {
            schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
            blocks: vec![
                MdBlock::Heading { level: 1, inlines: vec![MdInline::Text { text: "Title".into() }] },
                MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "hello".into() }] },
            ],
        }
    }

    /// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field. Top-level `blocks` DIFFER IN LENGTH
    /// (a: 3, b: 2) -- required per the recipe's own documented structural limitation (naive
    /// positional `between`, `0..min(len)`, only one tail can be non-empty per call): the shared
    /// prefix (index 0 = `List`, index 1 = `CodeBlock`) is modified in every one of ITS fields
    /// (the `List`'s `items` sub-triple additionally shows removed+modified via the SAME
    /// different-length-prefix trick one level deeper: 2 items vs 1, index 0 shared+modified,
    /// index 1 dropped), while `a`'s trailing index-2 `Paragraph` is the top-level `removed` tail
    /// in `between(a, b)` and the top-level `added` tail in `between(b, a)` --
    /// `between_roundtrip_law`/`field_sweep` both check both directions, matching xml's F1
    /// precedent.
    fn sweep_a() -> MdSnapshot {
        MdSnapshot {
            schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
            blocks: vec![
                MdBlock::List {
                    ordered: false,
                    start: None,
                    tight: true,
                    items: vec![
                        vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item keep+modify".into() }] }],
                        vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item drop".into() }] }],
                    ],
                },
                MdBlock::CodeBlock { info: Some("rust".into()), literal: "fn a() {}".into() },
                MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "to remove".into() }] },
            ],
        }
    }

    fn sweep_b() -> MdSnapshot {
        MdSnapshot {
            schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
            blocks: vec![
                MdBlock::List {
                    ordered: true,
                    start: Some(3),
                    tight: false,
                    items: vec![vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item keep+modify CHANGED".into() }] }]],
                },
                MdBlock::CodeBlock { info: None, literal: "fn b() {}".into() },
            ],
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MutationDiffLaw
    fn sample_mutations() -> Vec<MdMutation> {
        vec![
            MdMutation::NoMutation,
            MdMutation::SetSnapshot { snapshot: sweep_b() },
            MdMutation::InsertBlock { path: vec![], index: 1, block: MdBlock::ThematicBreak },
            MdMutation::RemoveBlock { path: vec![], index: 0 },
            MdMutation::ReplaceBlock { path: vec![], index: 1, block: MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "replaced".into() }] } },
            MdMutation::SetInlines { path: vec![], index: 1, inlines: vec![MdInline::Text { text: "new inlines".into() }] },
        ]
    }

    #[test]
    fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = sample_snapshot();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(&diff_direct, &base);

            let mut via_apply = base.clone();
            let diff_from_apply = crate::artifacts::md::schema::mutations::apply_md_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[test]
    fn inverse_law() {
        for mutation in sample_mutations() {
            let base = sample_snapshot();

            let mut round_tripped = base.clone();
            crate::artifacts::md::schema::mutations::apply_md_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <MdMutation as Mutation<MdSnapshot>>::inverse(&mutation, &base) {
                crate::artifacts::md::schema::mutations::apply_md_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level) failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = MutationDiff::apply(&diff, &base);
            let inverse_diff = DiffAlgebra::inverse(&diff, &base);
            let restored = MutationDiff::apply(&inverse_diff, &next);
            assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    fn two_para_root(a: &str, b: &str) -> MdSnapshot {
        MdSnapshot {
            schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
            blocks: vec![
                MdBlock::Paragraph { inlines: vec![MdInline::Text { text: a.into() }] },
                MdBlock::Paragraph { inlines: vec![MdInline::Text { text: b.into() }] },
            ],
        }
    }

    fn assert_absorb_matches_sequential(base: &MdSnapshot, d1: &MdDiff, d2: &MdDiff) -> MdDiff {
        let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base));
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(MutationDiff::apply(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    fn root_blocks_diff(diff: &MdDiff) -> &MdBlocksDiff {
        diff.blocks.as_ref().expect("blocks diff present")
    }

    #[test]
    fn absorb_law() {
        // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
        {
            let base = two_para_root("a", "b");
            let d1 = Mutation::diff(&MdMutation::InsertBlock { path: vec![], index: 2, block: MdBlock::ThematicBreak }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&MdMutation::RemoveBlock { path: vec![], index: 0 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = root_blocks_diff(&absorbed);
            assert_eq!(triple.removed, vec![0]);
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].index, 1);
            assert!(matches!(triple.added[0].item, MdBlock::ThematicBreak));
        }

        // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
        {
            let base = two_para_root("a", "b");
            let d1 = Mutation::diff(&MdMutation::InsertBlock { path: vec![], index: 2, block: MdBlock::ThematicBreak }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(
                &MdMutation::InsertBlock { path: vec![], index: 2, block: MdBlock::HtmlBlock { raw: "<hr/>".into() } },
                &mid,
            );
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = root_blocks_diff(&absorbed);
            assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
        }

        // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
        {
            let base = two_para_root("a", "b");
            let d1 = Mutation::diff(
                &MdMutation::InsertBlock { path: vec![], index: 1, block: MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "f".into() }] } },
                &base,
            );
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(
                &MdMutation::SetInlines { path: vec![], index: 1, inlines: vec![MdInline::Text { text: "v".into() }] },
                &mid,
            );
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = root_blocks_diff(&absorbed);
            assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
            assert_eq!(triple.added.len(), 1);
            match &triple.added[0].item {
                MdBlock::Paragraph { inlines } => assert_eq!(inlines, &vec![MdInline::Text { text: "v".into() }]),
                other => panic!("expected paragraph, got {other:?}"),
            }
        }

        // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
        {
            let base = two_para_root("a", "b");
            let d1 = Mutation::diff(&MdMutation::SetInlines { path: vec![], index: 1, inlines: vec![MdInline::Text { text: "v".into() }] }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&MdMutation::RemoveBlock { path: vec![], index: 1 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = root_blocks_diff(&absorbed);
            assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
            assert_eq!(triple.removed, vec![1]);
        }

        // Associativity over a triple.
        {
            let base = two_para_root("a", "b");
            let d1 = Mutation::diff(&MdMutation::InsertBlock { path: vec![], index: 2, block: MdBlock::ThematicBreak }, &base);
            let mid1 = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&MdMutation::InsertBlock { path: vec![], index: 2, block: MdBlock::HtmlBlock { raw: "<hr/>".into() } }, &mid1);
            let mid2 = MutationDiff::apply(&d2, &mid1);
            let d3 = Mutation::diff(&MdMutation::RemoveBlock { path: vec![], index: 0 }, &mid2);
            let sequential = MutationDiff::apply(&d3, &mid2);

            let mut left = d1.clone();
            MutationDiff::absorb(&mut left, d2.clone());
            MutationDiff::absorb(&mut left, d3.clone());

            let mut d2_then_d3 = d2.clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.clone());
            let mut right = d1.clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(MutationDiff::apply(&left, &base), sequential, "absorb associativity (left) failed");
            assert_eq!(MutationDiff::apply(&right, &base), sequential, "absorb associativity (right) failed");
        }

        // Nested (BlockQuote) canonical: Insert-inside-quote + Remove-before-it-inside-quote.
        {
            let base = MdSnapshot {
                schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
                blocks: vec![MdBlock::BlockQuote {
                    blocks: vec![
                        MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "qa".into() }] },
                        MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "qb".into() }] },
                    ],
                }],
            };
            let path = [MdPathStep::BlockQuote { index: 0 }];
            let d1 = Mutation::diff(&MdMutation::InsertBlock { path: path.to_vec(), index: 2, block: MdBlock::ThematicBreak }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&MdMutation::RemoveBlock { path: path.to_vec(), index: 0 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let MdBlockDiff::BlockQuote { blocks: Some(inner) } = &absorbed.blocks.as_ref().unwrap().modified[0].diff else {
                panic!("expected nested block-quote diff");
            };
            assert_eq!(inner.removed, vec![0]);
            assert_eq!(inner.added.len(), 1);
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[test]
    fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(MutationDiff::apply(&<MdDiff as DiffAlgebra<MdSnapshot>>::between(&a, &b), &a), b);
        assert_eq!(MutationDiff::apply(&<MdDiff as DiffAlgebra<MdSnapshot>>::between(&b, &a), &b), a);

        let sample = sample_snapshot();
        assert_eq!(MutationDiff::apply(&<MdDiff as DiffAlgebra<MdSnapshot>>::between(&sample, &sample), &sample), sample);

        // Real fixture (the demo's `📝️example.md`) diffed against a mutated variant.
        let fixture_text = include_str!("../📚️examples/🎬️demo/🖼️assets/📝️example.md");
        let fixture_blocks = parse_markdown_blocks(fixture_text);
        let fixture = MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: fixture_blocks };
        let mut mutated = fixture.clone();
        crate::artifacts::md::schema::mutations::apply_md_mutation(
            &mut mutated,
            &MdMutation::InsertBlock { path: vec![], index: 0, block: MdBlock::ThematicBreak },
        );
        assert_ne!(fixture, mutated);
        assert_eq!(MutationDiff::apply(&<MdDiff as DiffAlgebra<MdSnapshot>>::between(&fixture, &mutated), &fixture), mutated);
        assert_eq!(MutationDiff::apply(&<MdDiff as DiffAlgebra<MdSnapshot>>::between(&mutated, &fixture), &mutated), fixture);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️CodecRetentionLaw
    #[test]
    fn codec_retention_law() {
        // Documented normal form (see `render_markdown_blocks`'s doc comment): semantic fixed
        // point at the SNAPSHOT level, not byte-identical text. Fixture is written to already be
        // a fixed point of this codec's own parse/render pair (avoids incidental normalizations --
        // e.g. indented-vs-fenced code -- that would make a byte-diff assertion meaningless here).
        let fixture_text = include_str!("../📚️examples/🎬️demo/🖼️assets/📝️example.md");
        let blocks = parse_markdown_blocks(fixture_text);
        let re_encoded_text = render_markdown_blocks(&blocks);
        let re_parsed = parse_markdown_blocks(&re_encoded_text);
        assert_eq!(re_parsed, blocks, "decode(encode(x)) must equal x at the snapshot level");

        let snap = MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks };
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <MdSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️FieldSweep
    /// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field (see the
    /// fixtures' doc comment for exactly how each collection flavor is exercised).
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <MdDiff as DiffAlgebra<MdSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&diff_ab, &a), b);
        let diff_ba = <MdDiff as DiffAlgebra<MdSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&diff_ba, &b), a);
        assert!(<MdDiff as DiffAlgebra<MdSnapshot>>::between(&a, &a).is_empty());

        // Direction a->b: top-level `removed` (a's trailing paragraph, beyond b's length) +
        // `modified` (the shared-prefix `List` in every one of its own fields, AND the shared
        // `CodeBlock`) are exercised.
        let blocks_ab = diff_ab.blocks.as_ref().expect("blocks diff present (a->b)");
        assert!(!blocks_ab.removed.is_empty(), "top-level: removed not exercised (a->b)");
        assert_eq!(blocks_ab.modified.len(), 2, "expected the List AND the CodeBlock entries modified");
        let list_entry = blocks_ab.modified.iter().find(|m| matches!(m.diff, MdBlockDiff::List { .. }))
            .expect("a List-shaped modified entry must be present");
        let MdBlockDiff::List { ordered, start, tight, items } = &list_entry.diff else { unreachable!() };
        assert!(ordered.is_some(), "List.ordered not exercised");
        assert_eq!(*start, Some(Some(3)), "List.start tri-state (None -> Some(3)) not exercised");
        assert!(tight.is_some(), "List.tight not exercised");
        let items_diff: &MdListItemsDiff = items.as_ref().expect("List.items diff present");
        assert!(!items_diff.removed.is_empty(), "List.items: removed not exercised");
        assert_eq!(items_diff.modified.len(), 1, "expected exactly one modified item");
        assert!(!items_diff.modified[0].diff.modified.is_empty(), "modified item's own content not exercised");
        let code_entry = blocks_ab.modified.iter().find(|m| matches!(m.diff, MdBlockDiff::CodeBlock { .. }))
            .expect("a CodeBlock-shaped modified entry must be present");
        let MdBlockDiff::CodeBlock { info, literal } = &code_entry.diff else { unreachable!() };
        assert_eq!(*info, Some(None), "CodeBlock.info tri-state (Some -> None) not exercised");
        assert!(literal.is_some(), "CodeBlock.literal not exercised");

        // Direction b->a: top-level `added` (a's trailing paragraph reappearing) is exercised.
        let blocks_ba = diff_ba.blocks.as_ref().expect("blocks diff present (b->a)");
        assert!(!blocks_ba.added.is_empty(), "top-level: added not exercised (b->a)");

        // Sanity: nested list-item content diff and top-level block-kind Replace both exist as
        // reachable shapes (exercised directly, not just via sweep, since the naive between()
        // can't surface every shape from one pair -- same rationale as xml's F1 precedent).
        let leaf = diff_at_path(
            &[],
            0,
            crate::artifacts::md::schema::diff::MdBlocksLeafDiff::Modified(MdBlockDiff::Replace { block: MdBlock::ThematicBreak }),
        );
        assert!(leaf.blocks.is_some());
        let nested = MdListItemsDiff {
            removed: vec![0],
            modified: vec![MdListItemModified { index: 1, diff: MdBlocksDiff { removed: vec![], modified: vec![], added: vec![MdBlockAdded { index: 0, item: MdBlock::ThematicBreak }] } }],
            added: vec![MdListItemAdded { index: 2, item: vec![MdBlock::ThematicBreak] }],
        };
        assert!(!nested.removed.is_empty() && !nested.modified.is_empty() && !nested.added.is_empty());
        let _ = MdBlockModified { index: 0, diff: MdBlockDiff::ThematicBreak };
    }
    //#endregion 🔖️FieldSweep
}
//#endregion 🧪️Tests
