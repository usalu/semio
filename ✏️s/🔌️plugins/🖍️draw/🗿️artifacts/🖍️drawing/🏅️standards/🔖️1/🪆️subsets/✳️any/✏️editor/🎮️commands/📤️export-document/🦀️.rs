//! 📤️ Declares owned PNG, SVG and PDF downloads through the retained export command route.

use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin, NoConfig, NoConfigMutation};

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

/// 📎️ A fixed header copies at most 256 authored filename scalars.
pub(crate) fn export_stem(document: &DrawingSnapshot) -> String {
    let source = if document.id.chars().take(256).all(char::is_whitespace) { document.title.as_ref() } else { Some(&document.id) };
    let stem: String = source.map(|source| source.chars().take(256).map(|ch| if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') { ch } else { '-' }).collect()).unwrap_or_else(|| "drawing".into());
    let stem = stem.trim_matches(['-', '.']).to_owned();
    if stem.is_empty() { "drawing".into() } else { stem }
}

pub fn handle(
    _payload: &ExportDocument,
    _doc: &ArtifactView<'_, DrawingSnapshot>,
    _cfg: &ConfigView<'_, NoConfig>,
    _session: &mut crate::editor::drawing::commands::canvas_pointer_down::DrawingSession,
) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    Err(Fault::new(FaultOrigin::App,FaultCode::new("drawing.export.owned-route-required"),"Drawing export requires its registered owned command factory"))
}
