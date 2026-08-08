//! 👁️ Trinity Jack app — config-only ephemeral view commands (viewport, text edit/select, LOD,
//! engagement inputs, pointer/selection, locale). Was ephemeral `TrinityJackRuntime` state, now emits
//! `config_mutations` only — no document mutation.

use crate::apps::jack::config::{JackConfigMutation, JackEditorSelection};
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::Camera;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_viewport(viewport_json: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    match serde_json::from_str::<Camera>(viewport_json) {
        Ok(camera) => Ok(Emit::config(vec![JackConfigMutation::SetCamera { camera }])),
        Err(_) => Ok(Emit::default()),
    }
}

pub(crate) fn text_edit(text: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetQuery { value: text.to_string() }]))
}

pub(crate) fn text_select(start: u64, end: u64) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetEditorSelection { selection: Some(JackEditorSelection { start, end }) }]))
}

pub(crate) fn set_lod_mode(window_id: &str, value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetLodMode { window_id: window_id.to_string(), value: value.to_string() }]))
}

pub(crate) fn editor_engagement_input(value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetEditorEngagementInput { value: value.to_string() }]))
}

pub(crate) fn graph_engagement_input(value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetGraphEngagementInput { value: value.to_string() }]))
}

pub(crate) fn results_engagement_input(value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetResultsEngagementInput { value: value.to_string() }]))
}

pub(crate) fn graph_pointer_down(node_id: &Option<String>) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetSelection { node_ids: node_id.clone().map(|id| vec![id]).unwrap_or_default() }]))
}

pub(crate) fn set_selection(ids: &[String]) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetSelection { node_ids: ids.to_vec() }]))
}

pub(crate) fn set_locale(value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetLocale { value: value.to_string() }]))
}
