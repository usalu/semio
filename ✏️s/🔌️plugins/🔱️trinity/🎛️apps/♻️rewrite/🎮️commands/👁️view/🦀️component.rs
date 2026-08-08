//! 👁️ Trinity Rewrite app — config-only ephemeral view commands (selection, hover, viewport,
//! pointer, text select/hover, reorganize epoch, LOD, locale). Was ephemeral `RewritePlayRuntime`
//! state, now emits `config_mutations` only — no document mutation.

use crate::apps::rewrite::config::RewriteConfigMutation;
use crate::artifacts::jack::Camera;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteRuleModel;
use semio_framework_plugin::{Emit, Fault};

fn jack_token_at_offset(text: &str, offset: usize) -> Option<String> {
    if offset >= text.len() {
        return None;
    }
    let slice = &text[offset..];
    let token: String = slice.chars().take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_').collect();
    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

pub(crate) fn set_selection(state: &RewriteRuleModel, ids: &[String], surface_id: &Option<String>, select_epoch: u64) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    let mut config_mutations = vec![RewriteConfigMutation::SetSelection { node_ids: ids.to_vec() }];
    if let Some(node_id) = ids.first() {
        let fixture_json = crate::apps::rewrite::fixture_json_for_surface(surface_id.as_deref().unwrap_or(""), state);
        if let Some(var) = crate::apps::rewrite::sync_select_var_from_node(&fixture_json, node_id) {
            config_mutations.push(RewriteConfigMutation::SetActiveSelectVar { value: var });
        }
        config_mutations.push(RewriteConfigMutation::SetSelectEpoch { value: select_epoch + 1 });
    }
    Ok(Emit::config(config_mutations))
}

pub(crate) fn node_graph_hover(state: &RewriteRuleModel, surface_id: &Option<String>, node_id: &Option<String>, hover_epoch: u64) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    match node_id {
        Some(node_id) => {
            let fixture_json = crate::apps::rewrite::fixture_json_for_surface(surface_id.as_deref().unwrap_or(""), state);
            let mut config_mutations = vec![RewriteConfigMutation::SetHoverEpoch { value: hover_epoch + 1 }];
            if let Some(var) = crate::apps::rewrite::sync_select_var_from_node(&fixture_json, node_id) {
                config_mutations.push(RewriteConfigMutation::SetActiveHoverVar { value: var });
            }
            Ok(Emit::config(config_mutations))
        }
        None => Ok(Emit::default()),
    }
}

pub(crate) fn set_viewport(surface_id: &Option<String>, viewport_json: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    if surface_id.as_deref() == Some(crate::apps::rewrite::TRINITY_REWRITE_PLAY_SURFACE_BEFORE) {
        match serde_json::from_str::<Camera>(viewport_json) {
            Ok(camera) => Ok(Emit::config(vec![RewriteConfigMutation::SetBeforePaneCamera { camera }])),
            Err(_) => Ok(Emit::default()),
        }
    } else {
        Ok(Emit::default())
    }
}

pub(crate) fn graph_pointer_down(node_id: &Option<String>) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewriteConfigMutation::SetSelection { node_ids: node_id.clone().map(|id| vec![id]).unwrap_or_default() }]))
}

pub(crate) fn text_select(state: &RewriteRuleModel, var: &Option<String>, start: &Option<u64>, select_epoch: u64) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    let mut config_mutations = vec![RewriteConfigMutation::SetSelectEpoch { value: select_epoch + 1 }];
    if let Some(var) = var {
        config_mutations.push(RewriteConfigMutation::SetActiveSelectVar { value: var.clone() });
    } else if let Some(start) = start {
        if let Some(token) = jack_token_at_offset(&crate::apps::rewrite::compiled_jack_query(state), *start as usize) {
            config_mutations.push(RewriteConfigMutation::SetActiveSelectVar { value: token });
        }
    }
    Ok(Emit::config(config_mutations))
}

pub(crate) fn text_hover(state: &RewriteRuleModel, var: &Option<String>, offset: &Option<u64>, hover_epoch: u64) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    let mut config_mutations = vec![RewriteConfigMutation::SetHoverEpoch { value: hover_epoch + 1 }];
    if let Some(var) = var {
        config_mutations.push(RewriteConfigMutation::SetActiveHoverVar { value: var.clone() });
    } else if let Some(offset) = offset {
        if let Some(token) = jack_token_at_offset(&crate::apps::rewrite::compiled_jack_query(state), *offset as usize) {
            config_mutations.push(RewriteConfigMutation::SetActiveHoverVar { value: token });
        }
    }
    Ok(Emit::config(config_mutations))
}

pub(crate) fn reorganize(reorganize_epoch: u64) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewriteConfigMutation::SetReorganizeEpoch { value: reorganize_epoch + 1 }]))
}

pub(crate) fn set_lod_mode(window_id: &str, value: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewriteConfigMutation::SetLodMode { window_id: window_id.to_string(), value: value.to_string() }]))
}

pub(crate) fn set_locale(value: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewriteConfigMutation::SetLocale { value: value.to_string() }]))
}
