//! 🗂️ 🗂️ Drawing play app commands command — `duplicate-layer`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "duplicate-layer")]
pub struct DuplicateLayer {
    pub layer_id: String,
}

pub fn handle(payload: &DuplicateLayer, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    if payload.layer_id.is_empty() {
        return Ok(Emit::default());
    }
    let admission=doc.operation()?;
    let mutations=session.with_identity_control(|control|crate::editor::drawing::commands::edit_selection::plan(doc.snapshot,&[payload.layer_id.clone()],"duplicate",admission,control))?;
    Ok(Emit::mutations(mutations))
}
