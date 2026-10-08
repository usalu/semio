//! ↩️ Inverse for `SetRuleLayoutPoints` — ONE `set-rule-layout-points` putting every key the payload changes back: its BASE point
//! when the map held one, cleared otherwise; nothing when the payload changes nothing.
use crate::standards::v1::subsets::any::schema::mutations::{set_rule_layout_points,RewriteRuleMutation,RuleLayoutPlacement};
use crate::{LayoutPoint, RewritingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &super::SetRuleLayoutPoints, base: &RewritingSnapshot) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    if !payload.holds_invariants() {
        return Ok(Vec::new());
    }
    let placed = payload.points.iter().filter(|placement| base.rule_layout.get(&placement.key) != Some(&LayoutPoint { x: placement.x, y: placement.y })).map(|placement| &placement.key);
    let removed = payload.cleared.iter().filter(|key| base.rule_layout.contains_key(key));
    let changed: std::collections::BTreeSet<&String> = placed.chain(removed).collect();
    if changed.is_empty() {
        return Ok(Vec::new());
    }
    let points = changed.iter().filter_map(|key| base.rule_layout.get(key).map(|point| RuleLayoutPlacement { key: (*key).clone(), x: point.x, y: point.y })).collect();
    let cleared = changed.iter().filter(|key| !base.rule_layout.contains_key(key)).map(|key| (*key).clone()).collect();
    Ok(vec![set_rule_layout_points(points, cleared)])
}
//#endregion 🔖️Inverse
