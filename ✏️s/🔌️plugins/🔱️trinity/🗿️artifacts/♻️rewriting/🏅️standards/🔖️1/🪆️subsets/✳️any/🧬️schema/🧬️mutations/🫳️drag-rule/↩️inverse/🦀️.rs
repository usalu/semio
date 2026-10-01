//! ↩️ Inverse for `DragRuleNodes` — ONE `set-rule-layout-points` putting every moved node back: its BASE layout point when it had
//! one, cleared (back to its default slot) when it had none; nothing when the drag moves nothing.
use crate::standards::v1::subsets::any::schema::mutations::{set_rule_layout_points, RewriteRuleMutation, RuleLayoutPlacement};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DragRuleNodes, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
    let Some(layout) = super::diff::diff(payload, base).diff().rule_layout.clone() else { return Vec::new() };
    let moved: Vec<&String> = layout.entries().keys().collect();
    let points = moved.iter().filter_map(|key| base.rule_layout.get(*key).map(|point| RuleLayoutPlacement { key: (*key).clone(), x: point.x, y: point.y })).collect();
    let cleared = moved.iter().filter(|key| !base.rule_layout.contains_key(**key)).map(|key| (*key).clone()).collect();
    vec![set_rule_layout_points(points, cleared)]
}
//#endregion 🔖️Inverse
