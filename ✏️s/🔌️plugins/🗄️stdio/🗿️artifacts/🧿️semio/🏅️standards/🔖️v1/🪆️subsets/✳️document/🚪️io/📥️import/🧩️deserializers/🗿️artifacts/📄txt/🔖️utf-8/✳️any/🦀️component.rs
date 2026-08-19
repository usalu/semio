//! 📥️ Deserialize `s.stdio.semio/v1/document` from a real `s.stdio.txt` (utf-8) snapshot — the
//! simplest pair in this group: plain text has no structure beyond lines, so every line becomes
//! its own `DocBlock::Paragraph` with a single unstyled run (or zero runs for a blank line).
//! `TxtSnapshot::{trailing_newline,line_ending}` are wire-framing concerns with no document-model
//! counterpart — read but not carried into the semio snapshot (there is nothing in
//! `SemioDocumentSnapshot` for them to land in; a genuine, spec-mandated type gap, not an
//! oversight).

use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun, SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};
use crate::artifacts::txt::TxtSnapshot;
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

//#region 🔖️Deserializer
pub struct SemioDocumentFromTxt;

impl ArtifactDeserializer for SemioDocumentFromTxt {
    type From = TxtSnapshot;
    type Into = SemioDocumentSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let blocks = from.lines.iter().map(|line| if line.is_empty() { DocBlock::Paragraph { style_id: None, runs: Vec::new() } } else { DocBlock::Paragraph { style_id: None, runs: vec![DocRun::plain(line.clone())] } }).collect();
        Ok(SemioDocumentSnapshot { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: Vec::new(), images: Vec::new(), blocks })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::txt::schema::snapshot::LineEnding;

    pub(crate) fn sample_txt() -> TxtSnapshot {
        TxtSnapshot { schema: crate::artifacts::txt::STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: vec!["First line.".into(), String::new(), "Third line.".into()], trailing_newline: true, line_ending: LineEnding::Lf }
    }

    #[test]
    fn each_line_becomes_a_paragraph_blank_lines_become_empty_paragraphs() {
        let semio = semio_framework_plugin::resolve_ready(SemioDocumentFromTxt::deserialize(&sample_txt())).expect("deserialize");
        assert_eq!(semio.blocks.len(), 3);
        assert!(matches!(&semio.blocks[0], DocBlock::Paragraph { runs, .. } if runs[0].text == "First line."));
        assert!(matches!(&semio.blocks[1], DocBlock::Paragraph { runs, .. } if runs.is_empty()));
        assert!(matches!(&semio.blocks[2], DocBlock::Paragraph { runs, .. } if runs[0].text == "Third line."));
        assert!(semio.styles.is_empty() && semio.images.is_empty());
    }
}
//#endregion 🔖️Tests
