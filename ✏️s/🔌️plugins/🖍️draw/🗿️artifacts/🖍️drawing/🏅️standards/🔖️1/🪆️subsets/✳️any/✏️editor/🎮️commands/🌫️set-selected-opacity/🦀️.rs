//! 🗂️ 🗂️ Drawing play app commands command — `set-selected-opacity`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::schema::find_drawing_layer;
use crate::DrawingSnapshot;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "selected-opacity")]
pub struct SetSelectedOpacity {
    pub value: f64,
}

/// 🌫️ Sets every selected layer's opacity to `value`: the ABSOLUTE `set-layer-opacity` leaves on the committed document.
/// A slider drag is the framework scrub machine's ONE transaction of them (`T/📓️api-scrub-machine.md`); a one-shot
/// dispatch is one plain edit.
pub fn handle(payload: &SetSelectedOpacity, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let operations: Vec<DrawingMutation> = session.interaction.ids.iter().filter(|id| find_drawing_layer(document, id).is_some()).map(|id| crate::mutations::set_layer_opacity(id.clone().into(), payload.value)).collect();
    Ok(Emit::mutations(operations))
}
