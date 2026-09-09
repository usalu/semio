//! 📄️ 📄️ Drawing play app commands command — `set-active-example`.

use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use crate::op::DrawingMutation;
use crate::schema::default_drawing_document;
use crate::standards::v1::subsets::any::examples;
use crate::{ArtifactDsl, DrawingSnapshot};
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🖼️ Resolves `example_id` against the subset's own registered example catalogue — the very slice
/// `SubsetDeclaration.examples` advertises to the shell's example switcher — so the id space the UI
/// offers and the id space this command accepts are the same by construction. The empty id is the
/// sanctioned "reset to a blank drawing" request; any other unregistered id is a fault, never a
/// silent no-op.
pub fn handle(
    payload: &SetActiveExample,
    _doc: &ArtifactView<'_, DrawingSnapshot>,
    _cfg: &ConfigView<'_, DrawingConfig>,
    _session: &mut crate::editor::drawing::commands::canvas_pointer_down::DrawingSession,
) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let next = if payload.example_id.is_empty() {
        default_drawing_document("empty", None)
    } else {
        let source = examples().iter().find(|source| source.id() == payload.example_id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("drawing.example.unknown"), format!("Drawing declares no example '{}'", payload.example_id)))?;
        DrawingSnapshot::parse_dsl(source.document_json()).map_err(|_| Fault::new(FaultOrigin::App, FaultCode::new("drawing.example.parse"), format!("Drawing example '{}' does not parse as a drawing document", payload.example_id)))?
    };
    Ok(Emit { effects: vec![crate::editor::drawing::drawing_reset_document_effect(&next)], ..Default::default() })
}
