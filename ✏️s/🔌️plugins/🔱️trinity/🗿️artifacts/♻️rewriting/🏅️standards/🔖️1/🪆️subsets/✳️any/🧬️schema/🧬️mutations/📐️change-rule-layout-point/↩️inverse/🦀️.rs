//! ↩️ Inverse for `ChangeRuleLayoutPoint` — the OLD point looked up from BASE: `change` back to it
//! if the key existed, `remove` if it was previously absent.
use crate::standards::v1::subsets::any::schema::mutations::change_rule_layout_point::change_rule_layout_point;
use crate::standards::v1::subsets::any::schema::mutations::remove_rule_layout_point::remove_rule_layout_point;

use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ChangeRuleLayoutPoint, base: &RewritingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_rule_layout_point::ChangeRuleLayoutPoint> + From<crate::standards::v1::subsets::any::schema::mutations::remove_rule_layout_point::RemoveRuleLayoutPoint>,
{
    Ok((|| {
    match base.rule_layout.get(&payload.key) {
        Some(old) => vec![change_rule_layout_point(payload.key.clone(), *old)],
        None => vec![remove_rule_layout_point(payload.key.clone())],
    }

    })())
}
//#endregion 🔖️Inverse
