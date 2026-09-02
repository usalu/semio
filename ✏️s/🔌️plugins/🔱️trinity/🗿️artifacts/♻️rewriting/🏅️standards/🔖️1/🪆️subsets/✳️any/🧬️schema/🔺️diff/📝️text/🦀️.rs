//! 🔺️ Rewriting artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::rewriting::schema::diff::RewritingDiff;
use crate::artifacts::rewriting::schema::RewritingArtifact;
use crate::artifacts::rewriting::RewritingSnapshot;
use protocol::MutationDiff;
use std::collections::BTreeMap;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
impl RewritingDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &RewritingArtifact) -> protocol::MutationApplyResult<RewritingArtifact> {
        Ok({
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
                apply_map_delta(&mut next.parameter_bindings, bindings).map_err(|error| error.under(["parameterBindings"]))?;
            }
            if let Some(layout) = &self.rule_layout {
                apply_map_delta(&mut next.rule_layout, layout).map_err(|error| error.under(["ruleLayout"]))?;
            }
            if let Some(modes) = &self.lod_mode_by_window {
                apply_map_delta(&mut next.lod_mode_by_window, modes).map_err(|error| error.under(["lodModeByWindow"]))?;
            }
            if let Some(value) = &self.before_pane_camera {
                next.before_pane_camera = value.clone();
            }
            if let Some(value) = self.reorganize_epoch {
                next.reorganize_epoch = value;
            }
            if let Some(value) = &self.locale {
                next.locale = value.clone();
            }
            next
        })
    }
}

fn apply_map_delta<V: Clone>(target: &mut BTreeMap<String, V>, delta: &BTreeMap<String, Option<V>>) -> protocol::MutationApplyResult<()> {
    for (key, value) in delta {
        if value.is_none() && !target.contains_key(key) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed map entry does not exist").at([key.as_str()]));
        }
    }
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
    Ok(())
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

impl MutationDiff<RewritingSnapshot> for RewritingDiff {
    fn apply(&self, snapshot: &RewritingSnapshot) -> protocol::MutationApplyResult<RewritingSnapshot> {
        Ok({
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
                apply_map_delta(&mut next.parameter_bindings, bindings).map_err(|error| error.under(["parameterBindings"]))?;
            }
            if let Some(layout) = &self.rule_layout {
                apply_map_delta(&mut next.rule_layout, layout).map_err(|error| error.under(["ruleLayout"]))?;
            }
            next
        })
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
        merge_map_delta(&mut self.lod_mode_by_window, other.lod_mode_by_window);
        take!(before_pane_camera);
        take!(reorganize_epoch);
        take!(locale);
    }
}
//#endregion 🔖️Apply
