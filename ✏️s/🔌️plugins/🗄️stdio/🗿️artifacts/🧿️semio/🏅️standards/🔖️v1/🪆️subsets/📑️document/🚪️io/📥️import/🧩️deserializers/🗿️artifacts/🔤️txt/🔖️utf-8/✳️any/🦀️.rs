//! 📥️ Deserialize `s.stdio.semio/v1/document` from a real `s.stdio.txt` (utf-8) snapshot — the
//! simplest pair in this group: plain text has no structure beyond lines, so every line becomes
//! its own `DocBlock::Paragraph` with a single unstyled run (or zero runs for a blank line).
//! `TxtSnapshot::{trailing_newline,line_ending}` are wire-framing concerns with no document-model
//! counterpart — read but not carried into the semio snapshot (there is nothing in
//! `SemioDocumentSnapshot` for them to land in; a genuine, spec-mandated type gap, not an
//! oversight).

use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun, SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_txt::TxtSnapshot;

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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
