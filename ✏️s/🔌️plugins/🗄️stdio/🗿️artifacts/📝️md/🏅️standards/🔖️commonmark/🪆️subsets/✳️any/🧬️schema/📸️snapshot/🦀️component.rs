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

use crate::artifacts::md::STDIO_MD_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️CommonMarkModel
/// 🧩 A real CommonMark inline node. `MdInline` is a WEAK entity (recipe: weak entities are
/// whole-value replaced, never sub-diffed) -- `MdBlockDiff`'s `inlines`/`text` fields are always
/// `Option<Vec<MdInline>>`/`Option<String>` whole-value slots, never a nested inline-level triple.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    /// 🖼️ `![alt](url "title")`.
    Image {
        alt: String,
        url: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        start: Option<u32>,
        tight: bool,
        items: Vec<Vec<MdBlock>>,
    },
    /// 🔤️ Fenced OR indented source code blocks unify into this one shape (`info` is always
    /// `None` for what was originally an indented block -- indented code has no info-string
    /// position in the spec). Re-encoding always emits a fenced block (documented normal form).
    CodeBlock {
        #[serde(default, skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md")]
pub struct MdSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub blocks: Vec<MdBlock>,
}

impl Default for MdSnapshot {
    async fn default() -> Self {
        Self { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: Vec::new() }
    }
}

impl MdSnapshot {
    pub async fn from_text(text: &str) -> Self {
        let blocks = crate::artifacts::md::standards::v_commonmark::subsets::any::io::import::deserializers::parse_markdown_blocks(text);
        Self { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks }
    }

    pub async fn to_text(&self) -> String {
        crate::artifacts::md::standards::v_commonmark::subsets::any::io::export::serializers::render_markdown_blocks(&self.blocks)
    }
}
//#endregion 🔖️CommonMarkModel

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for MdSnapshot {
    const EXTENSION: &'static str = "md";
    async fn envelope_id() -> &'static str {
        "stdio.md"
    }

    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let blocks = crate::artifacts::md::standards::v_commonmark::subsets::any::io::import::deserializers::parse_markdown_blocks(body);
        Ok(Self { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks })
    }
    async fn print_dsl(&self) -> String {
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        let body = crate::artifacts::md::standards::v_commonmark::subsets::any::io::export::serializers::render_markdown_blocks(&self.blocks);
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for MdSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = crate::artifacts::md::standards::v_commonmark::subsets::any::io::export::serializers::render_markdown_blocks(&self.blocks);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, raw.as_bytes()))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        let body = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let blocks = crate::artifacts::md::standards::v_commonmark::subsets::any::io::import::deserializers::parse_markdown_blocks(&body);
        Ok(Self { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks })
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
