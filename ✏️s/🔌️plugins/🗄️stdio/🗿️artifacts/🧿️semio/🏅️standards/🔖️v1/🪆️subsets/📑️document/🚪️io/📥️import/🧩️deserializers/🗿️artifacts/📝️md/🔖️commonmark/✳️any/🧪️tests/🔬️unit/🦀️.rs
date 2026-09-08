
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub(crate) fn sample_md() -> MdSnapshot {
    MdSnapshot {
        schema: semio_s_artifact_stdio_md::STDIO_MD_DOCUMENT_SCHEMA.into(),
        blocks: vec![
            MdBlock::Heading { level: 1, inlines: vec![MdInline::Text { text: "Title".into() }] },
            MdBlock::Paragraph { inlines: vec![MdInline::Strong { inlines: vec![MdInline::Text { text: "bold".into() }] }, MdInline::Text { text: " and plain".into() }] },
            MdBlock::List { ordered: false, start: None, tight: true, items: vec![vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "item one".into() }] }]] },
            MdBlock::CodeBlock { info: Some("rust".into()), literal: "fn main() {}".into() },
            MdBlock::BlockQuote { blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "quoted".into() }] }] },
        ],
    }
}

#[semio_framework_async_macros::async_test]
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

#[semio_framework_async_macros::async_test]
async fn inline_image_lifts_to_its_own_block() {
    let md = MdSnapshot {
        schema: semio_s_artifact_stdio_md::STDIO_MD_DOCUMENT_SCHEMA.into(),
        blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "see: ".into() }, MdInline::Image { alt: "a cat".into(), url: "cat.png".into(), title: None }] }],
    };
    let semio = semio_framework_plugin::resolve_ready(SemioDocumentFromMd::deserialize(&md)).expect("deserialize");
    assert_eq!(semio.blocks.len(), 2);
    assert!(matches!(&semio.blocks[1], DocBlock::Image { image_id, alt, .. } if image_id == "cat.png" && alt == "a cat"));
}
