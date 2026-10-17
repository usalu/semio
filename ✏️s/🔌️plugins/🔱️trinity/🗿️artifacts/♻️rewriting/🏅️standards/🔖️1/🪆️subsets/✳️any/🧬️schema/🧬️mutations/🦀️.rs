//! ♻️ `trinity.rewrite.rule` semantic mutation aggregate — the parent lane edits the rule's own members.
//!
//! Every variant wraps the payload owned by its direct `<mutation>/🦀️.rs` leaf. The working graph lives in the composed
//! `workingGraph` child (`s.stdio.semio@v1/graph`); every relative working-graph edit is a child-lane leaf of that shared
//! vocabulary (design §20.15), so no parent leaf reads the child — `edit-working-graph` replaces the whole child handle.

use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;

pub use crate::standards::v1::subsets::any::schema::mutations::change_parameter_binding::{change_parameter_binding, ChangeParameterBinding};
pub use crate::standards::v1::subsets::any::schema::mutations::change_rule_layout_point::{change_rule_layout_point, ChangeRuleLayoutPoint};
pub use crate::standards::v1::subsets::any::schema::mutations::edit_working_graph::{edit_working_graph, EditWorkingGraph};
pub use crate::standards::v1::subsets::any::schema::mutations::edit_lhs::EditLhs;
pub use crate::standards::v1::subsets::any::schema::mutations::edit_rhs::EditRhs;
pub use crate::standards::v1::subsets::any::schema::mutations::remove_parameter_binding::RemoveParameterBinding;
pub use crate::standards::v1::subsets::any::schema::mutations::remove_rule_layout_point::RemoveRuleLayoutPoint;
pub use crate::standards::v1::subsets::any::schema::mutations::drag_rule_nodes::DragRuleNodes;
pub use crate::standards::v1::subsets::any::schema::mutations::set_rule_layout_points::{RuleLayoutPlacement, SetRuleLayoutPoints};
pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::mutations::support;
pub(crate) use support::{offset_text};


//#region 🔖️Aggregate
/// 🧮️ Semantic rewrite-rule mutation vocabulary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = RewritingSnapshot, diff = RewritingDiff, schema = "s.trinity.rewriting")]
pub enum RewriteRuleMutation {
    EditWorkingGraph(EditWorkingGraph),
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


//#region 🧪️StructuralCorrespondence
#[cfg(test)]
#[path = "🧪️tests/🔬️structural-correspondence/🦀️.rs"]
mod structural_correspondence_tests;
//#endregion 🧪️StructuralCorrespondence

//#region 🔖️ConcreteBuilders
/// 🏗️ Builder of the `drag_rule_nodes` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn drag_rule_nodes(targets: Vec<String>, dx: f64, dy: f64) -> RewriteRuleMutation {
    crate::standards::v1::subsets::any::schema::mutations::drag_rule_nodes::drag_rule_nodes::<RewriteRuleMutation>(targets, dx, dy)
}

/// 🏗️ Builder of the `edit_lhs` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn edit_lhs(new_lhs: crate::standards::v1::subsets::any::schema::rule::Lhs) -> RewriteRuleMutation {
    crate::standards::v1::subsets::any::schema::mutations::edit_lhs::edit_lhs::<RewriteRuleMutation>(new_lhs)
}

/// 🏗️ Builder of the `edit_rhs` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn edit_rhs(new_rhs: crate::standards::v1::subsets::any::schema::rule::Rhs) -> RewriteRuleMutation {
    crate::standards::v1::subsets::any::schema::mutations::edit_rhs::edit_rhs::<RewriteRuleMutation>(new_rhs)
}

/// 🏗️ Builder of the `remove_parameter_binding` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn remove_parameter_binding(key: String) -> RewriteRuleMutation {
    crate::standards::v1::subsets::any::schema::mutations::remove_parameter_binding::remove_parameter_binding::<RewriteRuleMutation>(key)
}

/// 🏗️ Builder of the `remove_rule_layout_point` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn remove_rule_layout_point(key: String) -> RewriteRuleMutation {
    crate::standards::v1::subsets::any::schema::mutations::remove_rule_layout_point::remove_rule_layout_point::<RewriteRuleMutation>(key)
}

/// 🏗️ Builder of the `set_rule_layout_points` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn set_rule_layout_points(points: Vec<RuleLayoutPlacement>, cleared: Vec<String>) -> RewriteRuleMutation {
    crate::standards::v1::subsets::any::schema::mutations::set_rule_layout_points::set_rule_layout_points::<RewriteRuleMutation>(points, cleared)
}
//#endregion 🔖️ConcreteBuilders
