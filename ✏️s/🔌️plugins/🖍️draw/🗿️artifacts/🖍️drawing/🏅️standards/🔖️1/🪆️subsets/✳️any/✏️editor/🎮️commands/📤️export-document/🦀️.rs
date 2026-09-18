//! 📤️ Drawing play app commands command — `export-document`: hands the document to the host as a
//! download (`Effect::DownloadMediaExport`) in one of the drawing's declared export dialects.
//! `pdf` paints the vector page (`io::export::…::pdf`), `svg` runs the stdio drawing→svg bridge.

use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::kernel::MEDIA_EXPORT_BASE64_ENCODING;
use semio_framework_plugin::{kernel::Effect, ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, NoConfig, NoConfigMutation};

/// 📄️ The default when the palette dispatches the verb without arguments.
pub const DEFAULT_EXPORT_FORMAT: &str = "pdf";

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "export-document")]
pub struct ExportDocument {
    pub format: String,
}

/// 📎️ A filename-safe stem from the document id (or title), so `semio` exports as `semio.pdf`.
fn export_stem(document: &DrawingSnapshot) -> String {
    let source = if document.id.trim().is_empty() { document.title.as_deref().unwrap_or("drawing") } else { document.id.as_str() };
    let stem: String = source.chars().map(|ch| if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') { ch } else { '-' }).collect();
    let stem = stem.trim_matches(['-', '.']).to_owned();
    if stem.is_empty() { "drawing".into() } else { stem }
}

pub fn handle(
    payload: &ExportDocument,
    doc: &ArtifactView<'_, DrawingSnapshot>,
    _cfg: &ConfigView<'_, NoConfig>,
    _session: &mut crate::editor::drawing::commands::canvas_pointer_down::DrawingSession,
) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let format = if payload.format.trim().is_empty() { DEFAULT_EXPORT_FORMAT } else { payload.format.trim() };
    let stem = export_stem(doc.snapshot);
    let effect = match format {
        "pdf" => {
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::pdf::v1_4::any::drawing_document_to_pdf(doc.snapshot)
                .map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("drawing.export.pdf"), format!("PDF export failed: {error}")))?;
            Effect::DownloadMediaExport { filename: format!("{stem}.pdf"), mime_type: "application/pdf".into(), data: base64_codec::base64_standard_encode(&bytes), encoding: Some(MEDIA_EXPORT_BASE64_ENCODING.into()) }
        }
        "svg" => {
            let (svg, _width, _height) = crate::io::drawing_document_to_svg(doc.snapshot).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("drawing.export.svg"), format!("SVG export failed: {error}")))?;
            Effect::DownloadMediaExport { filename: format!("{stem}.svg"), mime_type: "image/svg+xml".into(), data: svg, encoding: None }
        }
        other => return Err(Fault::new(FaultOrigin::App, FaultCode::new("drawing.export.format"), format!("Drawing exports to pdf or svg, not '{other}'"))),
    };
    Ok(Emit { effects: vec![effect], ..Default::default() })
}
