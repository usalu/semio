//! 📥️ Deserialize `s.stdio.semio/v1/document` from a real `s.stdio.md` (commonmark) snapshot —
//! `MdBlock`/`MdInline` map closely onto `DocBlock`/`DocRun`, commonmark being the closest-shaped
//! informing source per the master plan. Zero codec reimplementation (`MdSnapshot` is already
//! parsed; this leaf only maps Snapshot -> Snapshot).
//!
//! Honest, documented losses (never fabricated):
//! - `underline` has no CommonMark inline construct (CommonMark models emphasis/strong only) —
//!   `RunStyle::underline` is always `false` on import.
//! - `size`/`font`/`color` have no CommonMark equivalent — always `None` on import.
//! - `MdBlock::ThematicBreak`/`HtmlBlock` and `MdInline::HtmlInline`/`Code` (inline code span) have
//!   no dedicated `DocBlock`/`DocRun` shape; `ThematicBreak`/`HtmlBlock` are dropped (no semio
//!   block for a horizontal rule or raw HTML), inline `HtmlInline`/`Code` degrade to their raw
//!   text content as a plain (unstyled) run — never silently vanish, but formatting is honestly
//!   lost.
//! - md images are INLINE (`MdInline::Image`) but semio images are BLOCK-level (`DocBlock::Image`)
//!   — an inline image nested inside running text is lifted out to its own `DocBlock::Image`
//!   (breaking the surrounding paragraph at that point is out of scope, so the image becomes its
//!   OWN paragraph-adjacent block; documented structural approximation). `bytes`/`mime` are always
//!   empty since md only carries a URL, never raw bytes.
//! - `styles` is always empty: CommonMark has no named-style concept.

use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};
use crate::artifacts::md::MdSnapshot;
use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocListItem, DocRun, RunStyle, SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

//#region 🔖️FieldMapping
/// ✍️ Flattens one inline node into zero or more runs, threading `style` down through
/// `Emphasis`/`Strong` nesting (so `**_x_**` becomes one run with both `bold` and `italic` set).
/// `Link`'s inner inlines are flattened too, carrying `style.link = Some(url)`.
async fn push_inline(inline: &MdInline, style: RunStyle, runs: &mut Vec<DocRun>, images: &mut Vec<DocBlock>) {
    match inline {
        MdInline::Text { text } => runs.push(DocRun { text: text.clone(), style }),
        MdInline::Emphasis { inlines } => {
            let s = RunStyle { italic: true, ..style };
            for i in inlines {
                push_inline(i, s.clone(), runs, images);
            }
        }
        MdInline::Strong { inlines } => {
            let s = RunStyle { bold: true, ..style };
            for i in inlines {
                push_inline(i, s.clone(), runs, images);
            }
        }
        MdInline::Code { literal } => runs.push(DocRun { text: literal.clone(), style }),
        MdInline::Link { text, url, .. } => {
            let s = RunStyle { link: Some(url.clone()), ..style };
            for i in text {
                push_inline(i, s.clone(), runs, images);
            }
        }
        MdInline::Image { alt, url, .. } => images.push(DocBlock::Image { image_id: url.clone(), alt: alt.clone(), width: None, height: None }),
        MdInline::SoftBreak => runs.push(DocRun::plain(" ")),
        MdInline::HardBreak => runs.push(DocRun::plain("\n")),
        MdInline::HtmlInline { raw } => runs.push(DocRun::plain(raw.clone())),
    }
}

/// 🧱 One inline sequence -> (runs, any image blocks pulled out of it, in trailing order).
async fn map_inlines(inlines: &[MdInline]) -> (Vec<DocRun>, Vec<DocBlock>) {
    let mut runs = Vec::new();
    let mut images = Vec::new();
    for i in inlines {
        push_inline(i, RunStyle::default(), &mut runs, &mut images);
    }
    (runs, images)
}

/// 🧩 One `MdBlock` -> zero or more `DocBlock`s (an image-bearing paragraph expands to its text
/// block plus one `Image` block per embedded image, in order).
async fn map_block(block: &MdBlock) -> Vec<DocBlock> {
    match block {
        MdBlock::Heading { level, inlines } => {
            let (runs, images) = map_inlines(inlines);
            let mut out = vec![DocBlock::Heading { level: *level, style_id: None, runs }];
            out.extend(images);
            out
        }
        MdBlock::Paragraph { inlines } => {
            let (runs, images) = map_inlines(inlines);
            let mut out = vec![DocBlock::Paragraph { style_id: None, runs }];
            out.extend(images);
            out
        }
        MdBlock::List { ordered, items, .. } => vec![DocBlock::List { ordered: *ordered, items: items.iter().map(|item_blocks| DocListItem { blocks: item_blocks.iter().flat_map(map_block).collect() }).collect() }],
        MdBlock::CodeBlock { info, literal } => vec![DocBlock::Code { language: info.clone(), text: literal.clone() }],
        MdBlock::BlockQuote { blocks } => vec![DocBlock::Quote { blocks: blocks.iter().flat_map(map_block).collect() }],
        MdBlock::ThematicBreak => Vec::new(),
        MdBlock::HtmlBlock { .. } => Vec::new(),
    }
}

//#endregion 🔖️FieldMapping

//#region 🔖️Deserializer
pub struct SemioDocumentFromMd;

impl ArtifactDeserializer for SemioDocumentFromMd {
    type From = MdSnapshot;
    type Into = SemioDocumentSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let images: Vec<DocImage> = Vec::new();
        Ok(SemioDocumentSnapshot { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: Vec::new(), images, blocks: from.blocks.iter().flat_map(map_block).collect() })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) async fn sample_md() -> MdSnapshot {
        MdSnapshot {
            schema: crate::artifacts::md::STDIO_MD_DOCUMENT_SCHEMA.into(),
            blocks: vec![
                MdBlock::Heading { level: 1, inlines: vec![MdInline::Text { text: "Title".into() }] },
                MdBlock::Paragraph { inlines: vec![MdInline::Strong { inlines: vec![MdInline::Text { text: "bold".into() }] }, MdInline::Text { text: " and plain".into() }] },
                MdBlock::List { ordered: false, start: None, tight: true, items: vec![vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item one".into() }] }]] },
                MdBlock::CodeBlock { info: Some("rust".into()), literal: "fn main() {}".into() },
                MdBlock::BlockQuote { blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "quoted".into() }] }] },
            ],
        }
    }

    #[test]
    async fn maps_headings_lists_code_and_quotes() {
        let semio = semio_framework_plugin::resolve_ready(SemioDocumentFromMd::deserialize(&sample_md())).expect("deserialize");
        assert!(semio.styles.is_empty());
        assert_eq!(semio.blocks.len(), 5);
        assert!(matches!(&semio.blocks[0], DocBlock::Heading { level: 1, runs, .. } if runs[0].text == "Title"));
        assert!(matches!(&semio.blocks[1], DocBlock::Paragraph { runs, .. } if runs[0].style.bold && runs[1].text == " and plain"));
        assert!(matches!(&semio.blocks[2], DocBlock::List { ordered: false, items } if items.len() == 1));
        assert!(matches!(&semio.blocks[3], DocBlock::Code { language: Some(l), text } if l == "rust" && text == "fn main() {}"));
        assert!(matches!(&semio.blocks[4], DocBlock::Quote { blocks } if blocks.len() == 1));
    }

    #[test]
    async fn inline_image_lifts_to_its_own_block() {
        let md = MdSnapshot {
            schema: crate::artifacts::md::STDIO_MD_DOCUMENT_SCHEMA.into(),
            blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "see: ".into() }, MdInline::Image { alt: "a cat".into(), url: "cat.png".into(), title: None }] }],
        };
        let semio = semio_framework_plugin::resolve_ready(SemioDocumentFromMd::deserialize(&md)).expect("deserialize");
        assert_eq!(semio.blocks.len(), 2);
        assert!(matches!(&semio.blocks[1], DocBlock::Image { image_id, alt, .. } if image_id == "cat.png" && alt == "a cat"));
    }
}
//#endregion 🔖️Tests
