//! 👁️ 👁️ Trinity Rewrite app command — `text-hover`.

use crate::apps::rewrite::config::RewriteConfigMutation;
use crate::artifacts::jack::Camera;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
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

pub(crate) fn text_hover(state: &RewriteSnapshot, var: &Option<String>, offset: &Option<u64>, hover_epoch: u64) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
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
