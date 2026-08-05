//! 👁️ Trinity Jack app — config-only ephemeral view commands (viewport, text edit/select, LOD,
//! engagement inputs, pointer/selection, locale). Was ephemeral `TrinityJackRuntime` state, now emits
//! `config_operations` only — no document mutation.

use crate::apps::jack::config::{JackConfigOperation, JackEditorSelection};
use crate::artifacts::jack::op::TrinityGraphOperation;
use crate::artifacts::jack::Camera;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_viewport(viewport_json: &str) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    match serde_json::from_str::<Camera>(viewport_json) {
        Ok(camera) => Ok(Emit::config(vec![JackConfigOperation::SetCamera { camera }])),
        Err(_) => Ok(Emit::default()),
    }
}

pub(crate) fn text_edit(text: &str) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    Ok(Emit::config(vec![JackConfigOperation::SetQuery { value: text.to_string() }]))
}

pub(crate) fn text_select(start: u64, end: u64) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    Ok(Emit::config(vec![JackConfigOperation::SetEditorSelection { selection: Some(JackEditorSelection { start, end }) }]))
}

pub(crate) fn set_lod_mode(window_id: &str, value: &str) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    Ok(Emit::config(vec![JackConfigOperation::SetLodMode { window_id: window_id.to_string(), value: value.to_string() }]))
}

pub(crate) fn editor_engagement_input(value: &str) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    Ok(Emit::config(vec![JackConfigOperation::SetEditorEngagementInput { value: value.to_string() }]))
}

pub(crate) fn graph_engagement_input(value: &str) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    Ok(Emit::config(vec![JackConfigOperation::SetGraphEngagementInput { value: value.to_string() }]))
}

pub(crate) fn results_engagement_input(value: &str) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    Ok(Emit::config(vec![JackConfigOperation::SetResultsEngagementInput { value: value.to_string() }]))
}

pub(crate) fn graph_pointer_down(node_id: &Option<String>) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    Ok(Emit::config(vec![JackConfigOperation::SetSelection { node_ids: node_id.clone().map(|id| vec![id]).unwrap_or_default() }]))
}

pub(crate) fn set_selection(ids: &[String]) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    Ok(Emit::config(vec![JackConfigOperation::SetSelection { node_ids: ids.to_vec() }]))
}

pub(crate) fn set_locale(value: &str) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    Ok(Emit::config(vec![JackConfigOperation::SetLocale { value: value.to_string() }]))
}
