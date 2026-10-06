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
pub fn md_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.md",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
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

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_md_snapshot() -> MdSnapshot {
    MdSnapshot::default()
}

/// 📄️ The demo `stdio.md` document — genuinely exercises `Heading`/`Paragraph` (with `Strong`/
/// `Emphasis`/`Code` inline content on one physical line), a 2-level-nested `BlockQuote` (proving
/// `../../🚪️io/📝️text/📸️snapshot/📖️.grammar.semio`'s `block-quote = {GT block}+`
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

//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
