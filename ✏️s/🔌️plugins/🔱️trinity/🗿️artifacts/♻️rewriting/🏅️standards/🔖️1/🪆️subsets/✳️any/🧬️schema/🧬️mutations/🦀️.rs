//! ♻️ `trinity.rewrite.rule` semantic mutation aggregate.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf.

use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::RewritingSnapshot;

pub use super::change_parameter_binding::{change_parameter_binding, ChangeParameterBinding};
pub use super::change_rule_layout_point::{change_rule_layout_point, ChangeRuleLayoutPoint};
pub use super::edit_before_fixture::{edit_before_fixture, EditBeforeFixture};
pub use super::edit_lhs::{edit_lhs, EditLhs};
pub use super::edit_rhs::{edit_rhs, EditRhs};
pub use super::remove_parameter_binding::{remove_parameter_binding, RemoveParameterBinding};
pub use super::remove_rule_layout_point::{remove_rule_layout_point, RemoveRuleLayoutPoint};

//#region 🔖️Aggregate
/// 🧮️ Semantic rewrite-rule mutation vocabulary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslEnum, dsl::Mutations)]
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
}
//#endregion 🔖️Aggregate

//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence
