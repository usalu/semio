//! ♻️ `trinity.rewrite.rule` semantic mutation aggregate — the parent lane edits the rule's own members.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf. The working graph lives in the composed
//! `workingGraph` child (`s.stdio.semio@v1/graph`); every relative working-graph edit is a child-lane leaf of that shared
//! vocabulary (design §20.15), so no parent leaf reads the child — `edit-before-fixture` replaces the whole child handle.

use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

pub use super::change_parameter_binding::{change_parameter_binding, ChangeParameterBinding};
pub use super::change_rule_layout_point::{change_rule_layout_point, ChangeRuleLayoutPoint};
pub use super::edit_before_fixture::{edit_before_fixture, EditBeforeFixture};
pub use super::edit_lhs::{edit_lhs, EditLhs};
pub use super::edit_rhs::{edit_rhs, EditRhs};
pub use super::remove_parameter_binding::{remove_parameter_binding, RemoveParameterBinding};
pub use super::remove_rule_layout_point::{remove_rule_layout_point, RemoveRuleLayoutPoint};
pub use super::drag_rule_nodes::{drag_rule_nodes, DragRuleNodes};
pub use super::set_rule_layout_points::{set_rule_layout_points, RuleLayoutPlacement, SetRuleLayoutPoints};

//#region 🔖️Aggregate
/// 🧮️ Semantic rewrite-rule mutation vocabulary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = RewritingSnapshot, diff = RewritingDiff, schema = "s.trinity.rewriting")]
pub enum RewriteRuleMutation {
    EditBeforeFixture(EditBeforeFixture),
    EditLhs(EditLhs),
    EditRhs(EditRhs),
    ChangeParameterBinding(ChangeParameterBinding),
    RemoveParameterBinding(RemoveParameterBinding),
    ChangeRuleLayoutPoint(ChangeRuleLayoutPoint),
    RemoveRuleLayoutPoint(RemoveRuleLayoutPoint),
    DragRuleNodes(DragRuleNodes),
    SetRuleLayoutPoints(SetRuleLayoutPoints),
}
//#endregion 🔖️Aggregate

//#region 🔢️Offsets
/// 🔢️ A canvas offset as a label shows it: two decimals at most, trailing zeros dropped, `(en, de)`.
pub(crate) fn offset_text(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
    let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}
//#endregion 🔢️Offsets

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
