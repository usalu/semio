//! ↩️ Inverse for `RemoveRuleLayoutPoint` — the OLD point looked up from BASE, restored via
//! `change-rule-layout-point`. Missing key ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::change_rule_layout_point::change_rule_layout_point;

use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::RemoveRuleLayoutPoint, base: &RewritingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_rule_layout_point::ChangeRuleLayoutPoint>,
{
    Ok((|| {
    base.rule_layout.get(&payload.key).map(|old| vec![change_rule_layout_point(payload.key.clone(), *old)]).unwrap_or_default()

    })())
}
//#endregion 🔖️Inverse
