//! ↩️ Inverse for `SetRuleLayoutPoints` — ONE `set-rule-layout-points` putting every key the payload changes back: its BASE point
//! when the map held one, cleared otherwise; nothing when the payload changes nothing.
use crate::standards::v1::subsets::any::schema::mutations::{set_rule_layout_points, RewriteRuleMutation, RuleLayoutPlacement};
use crate::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::SetRuleLayoutPoints, base: &RewritingSnapshot) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(layout) = super::diff::diff(payload, base).diff().rule_layout.clone() else { return Vec::new() };
    let changed: Vec<&String> = layout.entries().keys().collect();
    let points = changed.iter().filter_map(|key| base.rule_layout.get(*key).map(|point| RuleLayoutPlacement { key: (*key).clone(), x: point.x, y: point.y })).collect();
    let cleared = changed.iter().filter(|key| !base.rule_layout.contains_key(**key)).map(|key| (*key).clone()).collect();
    vec![set_rule_layout_points(points, cleared)]

    })())
}
//#endregion 🔖️Inverse
