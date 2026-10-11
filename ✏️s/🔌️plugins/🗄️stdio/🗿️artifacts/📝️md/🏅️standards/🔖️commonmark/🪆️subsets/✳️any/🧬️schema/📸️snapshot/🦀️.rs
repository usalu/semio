//! 🧬️ MdSnapshot schema — complete typed CommonMark block/inline tree (not a `body: String`
//! passthrough). Real parsing/rendering lives in `⚙️engine::{parse_markdown_blocks,
//! render_markdown_blocks}`; this file only owns the persisted shape + handcrafted codecs.
//! Scope (see `⚙️engine`'s module doc for the full honest-subset list, and this artifact's
//! `f3-md-report.md` for the complete deviations list): headings, paragraphs, lists (incl.
//! nesting + tight/loose), fenced+indented code blocks (normalized to fenced on re-encode --
//! documented normal form, not the `fenced` flag the pre-migration stub carried), block quotes,
//! thematic breaks, raw HTML blocks/inlines, emphasis/strong, links/images, soft/hard breaks.
//! NOT supported (spec-real but explicitly out of scope, degrades to plain `Text`/`Paragraph`
//! rather than crashing): reference-style links/images, footnotes, setext headings, tables (GFM),
//! lazy blockquote continuation, link reference definitions.

use crate::STDIO_MD_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

//#region 🔖️CommonMarkModel
/// 🧩 A real CommonMark inline node. `MdInline` is a WEAK entity (recipe: weak entities are
/// whole-value replaced, never sub-diffed) -- `MdBlockDiff`'s `inlines`/`text` fields are always
/// `Option<Vec<MdInline>>`/`Option<String>` whole-value slots, never a nested inline-level triple.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum MdInline {
    /// 🔤️ Literal text run.
    Text { text: String },
    /// ✏️ `*em*` / `_em_`.
    Emphasis { inlines: Vec<MdInline> },
    /// 💪 `**strong**` / `__strong__`.
    Strong { inlines: Vec<MdInline> },
    /// 🔤️ `` `code span` ``.
    Code { literal: String },
    /// 🔗️ `[text](url "title")`.
    Link {
        text: Vec<MdInline>,
        url: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    /// 🖼️ `![alt](url "title")`.
    Image {
        alt: String,
        url: String,
        #[value(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    /// ↩️ A single `\n` inside a paragraph that is NOT a hard break (renders as a space/wrap
    /// point, not `<br>`).
    SoftBreak,
    /// ⏎ A line ending preceded by 2+ trailing spaces or a trailing `\` (renders as `<br>`).
    HardBreak,
    /// 🏷️ Raw inline HTML (`<tag>`, `</tag>`, `<!--comment-->`), kept verbatim per the commonmark
    /// spec's allowance for embedded HTML -- a raw-retention case, not a parse-failure case.
    HtmlInline { raw: String },
}

/// 🧱 A real CommonMark block. `MdBlock` is a STRONG-like entity: block collections (top-level
/// `MdSnapshot.blocks`, `List.items[n]`, `BlockQuote.blocks`) are all index-keyed and each gets
/// its own per-field diff (`MdBlockDiff`) rather than whole-value replacement.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum MdBlock {
    Heading {
        level: u8,
        inlines: Vec<MdInline>,
    },
    Paragraph {
        inlines: Vec<MdInline>,
    },
    /// 📃 `tight` records whether ANY blank line separated items/item-blocks in the source (loose
    /// if so) -- CommonMark's own render distinction (loose wraps item content in `<p>`, tight
    /// does not); this codec always models item content as `MdBlock::Paragraph` regardless, so
    /// `tight` is purely a round-trip/render hint, not a structural difference in `items`' shape.
    List {
        ordered: bool,
        #[value(default, skip_serializing_if = "Option::is_none")]
        start: Option<u32>,
        tight: bool,
        items: Vec<Vec<MdBlock>>,
    },
    /// 🔤️ Fenced OR indented source code blocks unify into this one shape (`info` is always
    /// `None` for what was originally an indented block -- indented code has no info-string
    /// position in the spec). Re-encoding always emits a fenced block (documented normal form).
    CodeBlock {
        #[value(default, skip_serializing_if = "Option::is_none")]
        info: Option<String>,
        literal: String,
    },
    BlockQuote {
        blocks: Vec<MdBlock>,
    },
    ThematicBreak,
    /// 🏷️ Raw HTML block, retained verbatim per the commonmark spec's embedded-HTML allowance --
    /// simplified single-rule recognition (starts with `<tag`/`</tag`/`<!--`, ends at the next
    /// blank line) rather than the full 7-condition spec grammar (documented scope cut).
    HtmlBlock {
        raw: String,
    },
}

/// 📸️ Persisted `stdio.md` snapshot: the complete top-level block sequence.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md")]
pub struct MdSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub blocks: Vec<MdBlock>,
}

impl Default for MdSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: Vec::new() }
    }
}

impl MdSnapshot {



}
//#endregion 🔖️CommonMarkModel




