//! 📤️ Declares owned PNG, SVG and PDF downloads through the retained export command route.

use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, NoConfig, NoConfigMutation};

/// 📄️ The default when the palette dispatches the verb without arguments.
pub const DEFAULT_EXPORT_FORMAT: &str = "pdf";

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "export-document")]
pub struct ExportDocument {
    pub format: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub transparent: Option<bool>,
}
impl Default for ExportDocument{fn default()->Self{Self{format:DEFAULT_EXPORT_FORMAT.into(),width:None,height:None,transparent:None}}}

pub const EXPORT_STEM_BYTES: usize = 256;

/// 📎️ Writes the bounded ASCII filename stem into the caller's original buffer.
pub fn export_stem_in<'a>(document: &DrawingSnapshot, buffer: &'a mut [u8; EXPORT_STEM_BYTES]) -> &'a str {
    let source = if document.id.chars().take(EXPORT_STEM_BYTES).all(char::is_whitespace) { document.title.as_ref() } else { Some(&document.id) };
    let mut length = 0;
    for ch in source.into_iter().flat_map(|text| text.chars().take(EXPORT_STEM_BYTES)) {
        buffer[length] = if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') { ch as u8 } else { b'-' };
        length += 1;
    }
    let stem = std::str::from_utf8(&buffer[..length]).unwrap().trim_matches(['-', '.']);
    if stem.is_empty() { "drawing" } else { stem }
}

/// 🏷️ Returns an owned filename stem for callers without an original output buffer.
pub fn export_stem(document: &DrawingSnapshot) -> String {
    export_stem_in(document, &mut [0; EXPORT_STEM_BYTES]).into()
}

pub fn handle(
    _payload: &ExportDocument,
    _doc: &ArtifactView<'_, DrawingSnapshot>,
    _cfg: &ConfigView<'_, NoConfig>,
    _session: &mut semio_s_artifact_draw_drawing_editor_tool::DrawingSession,
) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    Err(Fault::new(FaultOrigin::App,FaultCode::new("drawing.export.owned-route-required"),"Drawing export requires its registered owned command factory"))
}
