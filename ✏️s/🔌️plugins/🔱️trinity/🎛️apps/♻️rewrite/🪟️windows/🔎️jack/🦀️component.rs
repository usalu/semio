//! 🔎️ Trinity Rewrite app — Jack window (read-only text editor showing the compiled jack query for
//! this rule, with hover/select variable occurrences).

use crate::apps::rewrite::config::RewriteConfig;
use crate::artifacts::rewrite::RewriteRuleModel;
use semio_framework_plugin::{build_text_editor_scene, text_identifier_bounds_at, TextEditorScene, UiNode};
use serde_json::json;

fn var_occurrences_json(text: &str, var: &str) -> Option<String> {
    if var.is_empty() {
        return None;
    }
    let mut ranges = Vec::new();
    let mut scan = 0usize;
    while let Some(found) = text[scan..].find(var) {
        let at = scan + found;
        let end = at + var.len();
        if text_identifier_bounds_at(text, at) == Some((at, end)) {
            ranges.push(json!({ "start": at, "end": end }));
        }
        scan = at + var.len();
    }
    if ranges.is_empty() {
        return None;
    }
    let ranges_json = serde_json::to_string(&ranges).unwrap_or_else(|_| "[]".into());
    Some(json!({ "selection": ranges_json, "hover": ranges_json }).to_string())
}

pub(crate) fn render(state: &RewriteRuleModel, cfg: &RewriteConfig) -> UiNode {
    let query = crate::apps::rewrite::compiled_jack_query(state);
    let active_var = if !cfg.active_hover_var.is_empty() { cfg.active_hover_var.as_str() } else { cfg.active_select_var.as_str() };
    build_text_editor_scene(
        crate::apps::rewrite::TRINITY_REWRITE_PLAY_SURFACE_JACK,
        crate::apps::rewrite::TRINITY_REWRITE_PLAY_CONTROLLER_ID,
        TextEditorScene { tokens_json: serde_json::to_string(&crate::core::semantic_tokens(&query)).ok(), occurrences_json: var_occurrences_json(&query, active_var), ..TextEditorScene::base(query, Some("jack".into()), None) },
    )
}
