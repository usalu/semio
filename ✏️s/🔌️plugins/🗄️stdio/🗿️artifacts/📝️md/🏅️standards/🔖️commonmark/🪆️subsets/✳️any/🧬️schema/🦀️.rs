//! 🧬️ MdArtifact schema — full artifact state.

use crate::schema::snapshot::MdBlock;
use crate::MdSnapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.md` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md")]
pub struct MdArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> MdSnapshot {
        MdSnapshot { schema: self.schema.clone(), blocks: self.blocks.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: MdSnapshot) -> Self {
        Self { schema: snapshot.schema, blocks: snapshot.blocks }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: MdSnapshot) {
        self.schema = snapshot.schema;
        self.blocks = snapshot.blocks;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.md`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn md_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.stdio.md",
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
    use crate::{MdDiff, MdMutation, MdSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

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
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::schema::mutations::apply_md_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <MdDiff as protocol::MutationDiff<MdSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::MdSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn looks_like_markdown(text: &str) -> IoConfidence {
        if text.trim().is_empty() {
            return IoConfidence::Low;
        }
        let blocks = crate::standards::v_commonmark::subsets::any::io::import::deserializers::parse_markdown_blocks(text);
        if blocks.is_empty() {
            return IoConfidence::Low;
        }
        let has_structure = blocks.iter().any(|b| {
            !matches!(
                b,
                crate::schema::snapshot::MdBlock::Paragraph { inlines }
                    if inlines.iter().all(|n| matches!(n, crate::schema::snapshot::MdInline::Text { .. }))
            )
        });
        if has_structure {
            IoConfidence::High
        } else {
            IoConfidence::Medium
        }
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
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <MdSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_md_snapshot() -> MdSnapshot {
    MdSnapshot::default()
}

/// 📄️ The demo `stdio.md` document — genuinely exercises `Heading`/`Paragraph` (with `Strong`/
/// `Emphasis`/`Code` inline content on one physical line), a 2-level-nested `BlockQuote` (proving
/// `../../🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`'s `block-quote = {GT block}+`
/// genuine `Ref` self-recursion end-to-end), a fenced `CodeBlock` with a real info string, and
/// `ThematicBreak`. The single source of truth for `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/
/// `🎒️.pack.semio` (both are literally this snapshot's `print_dsl`/`encode_pack` output,
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
/// bookkeeping, `🔍️lexer/🦀️.rs`'s fence-scanning loop increments `byte_offset` for every
/// consumed char EXCEPT `'\n'` — so every token positioned AFTER a fence whose content spans N
/// lines gets its `byte_range` under-reported by N, which can desync `match_raw_span`'s
/// `text.get(start_byte..).find('\n')` lookup enough to make a subsequent `LINE` match
/// zero-width): `CodeBlock` is placed LAST here, with only `ThematicBreak` after it (a
/// pure-literal-token production, `"--" "-"`, untouched by raw-span corruption since it never
/// calls `LINE`/`REST`) — `BlockQuote`/`Paragraph` (both `LINE`-dependent) are placed BEFORE the
/// fence, never after it, sidestepping the corruption entirely rather than fixing the shared lexer
/// (out of this wave's ownership boundary).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_md_snapshot() -> MdSnapshot {
    use crate::schema::snapshot::{MdBlock, MdInline};
    let blocks = vec![
        MdBlock::Heading { level: 1, inlines: vec![MdInline::Text { text: "Title".into() }] },
        MdBlock::BlockQuote { blocks: vec![MdBlock::BlockQuote { blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "Deeply quoted.".into() }] }] }] },
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
    MdSnapshot { schema: crate::STDIO_MD_DOCUMENT_SCHEMA.into(), blocks }
}
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec MdBuilderFacets {
        construction: MdBuilderConstruction,
        analysis: MdAnalyzerAnalysis,
        composition: super::super::io::derived_composition::MdComposerComposition,
    }
    builder: MdBuilder,
    analyzer: MdAnalyzer,
    composer: MdComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
