//! 🔺️ Rewrite artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::rewrite::schema::diff::*;

use crate::artifacts::rewrite::schema::diff::RewriteDiff;
use crate::artifacts::rewrite::schema::RewriteArtifact;
use crate::artifacts::rewrite::RewriteSnapshot;
use protocol::MutationDiff;
use std::collections::BTreeMap;


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


//#region 🔖️Apply
impl RewriteDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &RewriteArtifact) -> RewriteArtifact {
        let mut next = artifact.clone();
        if let Some(value) = &self.before_fixture_json {
            next.before_fixture_json = value.clone();
        }
        if let Some(value) = &self.lhs_json {
            next.lhs_json = value.clone();
        }
        if let Some(value) = &self.rhs_json {
            next.rhs_json = value.clone();
        }
        if let Some(bindings) = &self.parameter_bindings {
            apply_map_delta(&mut next.parameter_bindings, bindings);
        }
        if let Some(layout) = &self.rule_layout {
            apply_map_delta(&mut next.rule_layout, layout);
        }
        if let Some(list) = &self.selected_node_ids {
            next.selected_node_ids = list.values.clone();
        }
        if let Some(value) = &self.active_hover_var {
            next.active_hover_var = value.clone();
        }
        if let Some(value) = &self.active_select_var {
            next.active_select_var = value.clone();
        }
        if let Some(modes) = &self.lod_mode_by_window {
            apply_map_delta(&mut next.lod_mode_by_window, modes);
        }
        if let Some(value) = &self.before_pane_camera {
            next.before_pane_camera = value.clone();
        }
        if let Some(value) = self.reorganize_epoch {
            next.reorganize_epoch = value;
        }
        if let Some(value) = self.hover_epoch {
            next.hover_epoch = value;
        }
        if let Some(value) = self.select_epoch {
            next.select_epoch = value;
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        next
    }
}

fn apply_map_delta<V: Clone>(target: &mut BTreeMap<String, V>, delta: &BTreeMap<String, Option<V>>) {
    for (key, value) in delta {
        match value {
            Some(v) => {
                target.insert(key.clone(), v.clone());
            }
            None => {
                target.remove(key);
            }
        }
    }
}

/// 🪢 Merges a per-key map delta into `self`'s accumulated delta (per-key upsert of the newer
/// entry) rather than replacing the whole map — two `change-*`/`remove-*` mutations touching
/// DIFFERENT keys in the same coalesced batch must both survive.
fn merge_map_delta<V>(dst: &mut Option<BTreeMap<String, Option<V>>>, src: Option<BTreeMap<String, Option<V>>>) {
    match (dst.as_mut(), src) {
        (Some(dst_map), Some(src_map)) => dst_map.extend(src_map),
        (None, Some(src_map)) => *dst = Some(src_map),
        _ => {}
    }
}

impl MutationDiff<RewriteSnapshot> for RewriteDiff {
    fn apply(&self, snapshot: &RewriteSnapshot) -> RewriteSnapshot {
        let mut next = snapshot.clone();
        if let Some(value) = &self.before_fixture_json {
            next.before_fixture_json = value.clone();
        }
        if let Some(value) = &self.lhs_json {
            next.lhs_json = value.clone();
        }
        if let Some(value) = &self.rhs_json {
            next.rhs_json = value.clone();
        }
        if let Some(bindings) = &self.parameter_bindings {
            apply_map_delta(&mut next.parameter_bindings, bindings);
        }
        if let Some(layout) = &self.rule_layout {
            apply_map_delta(&mut next.rule_layout, layout);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(before_fixture_json);
        take!(lhs_json);
        take!(rhs_json);
        merge_map_delta(&mut self.parameter_bindings, other.parameter_bindings);
        merge_map_delta(&mut self.rule_layout, other.rule_layout);
        take!(selected_node_ids);
        take!(active_hover_var);
        take!(active_select_var);
        merge_map_delta(&mut self.lod_mode_by_window, other.lod_mode_by_window);
        take!(before_pane_camera);
        take!(reorganize_epoch);
        take!(hover_epoch);
        take!(select_epoch);
        take!(locale);
    }
}
//#endregion 🔖️Apply
