//! 📥️ `pdf` (1.7) → `s.stdio.semio/v1/drawing` — an honestly THIN bridge, real per what
//! `PdfSnapshot` actually carries: `PdfPage.text` is already-extracted, aggregated page text
//! (per that snapshot's own module doc: "`text` doubles as the builder's authoring surface: the
//! writer regenerates a fresh content stream from it on encode") — NOT a content-stream operator
//! list. This codec never decodes `Tj`/`TJ`/path-painting operators into drawable ops (only the
//! raw, undecoded PDF object graph retains that, in `PdfSnapshot.objects`, opaque bytes this
//! bridge does not attempt to interpret — that would be re-implementing a PDF content-stream
//! interpreter, explicitly out of scope: "zero codec reimplementation"). So the real, honest
//! bridge this leaf builds is: one `DrawLayer` per PDF page, canvas from `pages[0].media_box`,
//! containing exactly one `DrawNode::Text` holding that page's whole extracted text — never
//! fabricating vector paths PDF's own typed snapshot doesn't expose.

use crate::artifacts::pdf::PdfSnapshot;
use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint2, SemioTransform};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };

//#region 🔖️Deserializer
pub struct SemioDrawingFromPdf;

impl ArtifactDeserializer for SemioDrawingFromPdf {
    type From = PdfSnapshot;
    type Into = SemioDrawingSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        if from.pages.is_empty() {
            return Err(store::PackError::Schema("pdf→semio/drawing: no pages".into()));
        }
        let first = &from.pages[0];
        let canvas = DrawCanvas { width: first.media_box[2] - first.media_box[0], height: first.media_box[3] - first.media_box[1], background: None };
        let layers = from
            .pages
            .iter()
            .enumerate()
            .map(|(i, page)| {
                let height = page.media_box[3] - page.media_box[1];
                DrawLayer {
                    id: format!("page{i}"),
                    name: format!("page{i}"),
                    visible: true,
                    root: DrawNode::Group { transform: SemioTransform::identity(), children: vec![DrawNode::Text { value: page.text.clone(), at: SemioPoint2 { x: 0.0, y: height }, style: None }] },
                }
            })
            .collect();
        Ok(SemioDrawingSnapshot { schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(), canvas, styles: Vec::new(), layers })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pdf::schema::snapshot::PdfPage;

    #[test]
    async fn maps_page_text_and_media_box() {
        let pdf = PdfSnapshot { pages: vec![PdfPage { media_box: [0.0, 0.0, 200.0, 100.0], text: "hello semio".into(), ..PdfPage::default() }], ..PdfSnapshot::default() };
        let drawing = semio_framework_plugin::resolve_ready(SemioDrawingFromPdf::deserialize(&pdf)).expect("deserialize");
        assert_eq!(drawing.canvas.width, 200.0);
        assert_eq!(drawing.canvas.height, 100.0);
        assert_eq!(drawing.layers.len(), 1);
        match &drawing.layers[0].root {
            DrawNode::Group { children, .. } => match &children[0] {
                DrawNode::Text { value, .. } => assert_eq!(value, "hello semio"),
                other => panic!("expected Text, got {other:?}"),
            },
            other => panic!("expected Group, got {other:?}"),
        }
    }

    #[test]
    async fn rejects_no_pages() {
        assert!(semio_framework_plugin::resolve_ready(SemioDrawingFromPdf::deserialize(&PdfSnapshot::default())).is_err());
    }
}
//#endregion 🔖️Tests
