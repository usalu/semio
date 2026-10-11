//! ♻️ Rewrite-rule document ownership retires bodies, keyed values, and semantic mutations incrementally.

use crate::standards::v1::subsets::any::schema::mutations::RuleLayoutPlacement;

use crate::{LayoutPoint, RewritingSnapshot};

semio_framework_value::artifact_retire_struct!(RewritingSnapshot { working_graph, lhs, rhs, parameter_bindings, rule_layout });
semio_framework_value::artifact_retire_struct!(LayoutPoint { x, y });
semio_framework_value::artifact_retire_struct!(RuleLayoutPlacement { key, x, y });

#[cfg(test)]
#[path = "🧪️tests/🔬️document-retirement/🦀️.rs"]
mod tests;

use super::{Pattern,Lhs,Rhs,Assignment,ParameterSpec,ParameterKind,Rule};
semio_framework_value::artifact_retire_struct!(Pattern {left_var,left_kind,edge_var,edge_kind,right_var,right_kind});
semio_framework_value::artifact_retire_struct!(Lhs {pattern,where_clause});
semio_framework_value::artifact_retire_struct!(Rhs {create,delete,set,merge,parameters});
semio_framework_value::artifact_retire_struct!(Assignment {var,prop,value});
semio_framework_value::artifact_retire_struct!(ParameterSpec {name,kind,default});
semio_framework_value::artifact_retire_struct!(Rule {name,lhs,rhs});
semio_framework_value::artifact_retire_leaf!(ParameterKind);
