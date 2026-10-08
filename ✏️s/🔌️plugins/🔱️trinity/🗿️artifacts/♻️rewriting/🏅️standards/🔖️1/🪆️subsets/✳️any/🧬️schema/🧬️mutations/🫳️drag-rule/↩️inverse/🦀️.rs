//! ↩️ Inverse for `DragRuleNodes` — ONE `set-rule-layout-points` putting every moved node back: its BASE layout point when it had
//! one, cleared (back to its default slot) when it had none; nothing when the drag moves nothing.
use crate::standards::v1::subsets::any::schema::mutations::{set_rule_layout_points,RewriteRuleMutation,RuleLayoutPlacement};
use crate::standards::v1::subsets::any::schema::rule_graph_position;
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DragRuleNodes, base: &RewritingSnapshot) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    if !payload.holds_invariants() || (payload.dx, payload.dy) == (0.0, 0.0) {
        return Ok(Vec::new());
    }
    let moved: std::collections::BTreeSet<&String> = payload.targets.iter().filter(|id| rule_graph_position(base, id).is_some()).collect();
    if moved.is_empty() {
        return Ok(Vec::new());
    }
    let points = moved.iter().filter_map(|key| base.rule_layout.get(key).map(|point| RuleLayoutPlacement { key: (*key).clone(), x: point.x, y: point.y })).collect();
    let cleared = moved.iter().filter(|key| !base.rule_layout.contains_key(key)).map(|key| (*key).clone()).collect();
    Ok(vec![set_rule_layout_points(points, cleared)])
}
//#endregion 🔖️Inverse
