//! 📜️ Canonical match pattern and ordered rewrite program, with direct owned field factories.
use semio_framework_graph_manifest::PropertyValue;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::ast::{Pattern as JackPattern, PatternEdge, PatternNode};
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all="camelCase")]
pub struct Pattern {
    pub left_var:String,
    pub left_kind:String,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub edge_var:Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub edge_kind:Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub right_var:Option<String>,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub right_kind:Option<String>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all="camelCase")]
pub struct Lhs {
    pub pattern:Pattern,
    #[value(default,skip_serializing_if="Option::is_none")]
    pub where_clause:Option<String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all="camelCase")]
pub enum ParameterKind { String,Number,Boolean }
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all="camelCase")]
pub struct ParameterSpec {pub name:String,pub kind:ParameterKind,pub default:PropertyValue}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all="camelCase")]
pub struct Assignment {pub var:String,pub prop:String,pub value:PropertyValue}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all="camelCase")]
pub struct Rhs {
    pub create:Vec<Pattern>,
    pub delete:Vec<String>,
    pub set:Vec<Assignment>,
    pub merge:Vec<Pattern>,
    pub parameters:Vec<ParameterSpec>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all="camelCase")]
pub struct Rule {pub name:String,pub lhs:Lhs,pub rhs:Rhs}

impl Pattern {
    pub fn to_jack_pattern(&self) -> JackPattern {
        let left = PatternNode { var: self.left_var.clone(), kind: self.left_kind.clone() };
        if let (Some(right_var), Some(right_kind)) = (&self.right_var, &self.right_kind) {
            JackPattern { nodes: vec![left], edge: Some(PatternEdge { var: self.edge_var.clone(), kind: self.edge_kind.clone(), directed: true, right: PatternNode { var: right_var.clone(), kind: right_kind.clone() } }) }
        } else {
            JackPattern { nodes: vec![left], edge: None }
        }
    }
}

semio_framework_value::artifact_retire_struct!(Pattern {left_var,left_kind,edge_var,edge_kind,right_var,right_kind});
semio_framework_value::artifact_retire_struct!(Lhs {pattern,where_clause});
semio_framework_value::artifact_retire_struct!(Rhs {create,delete,set,merge,parameters});
semio_framework_value::artifact_retire_struct!(Assignment {var,prop,value});
semio_framework_value::artifact_retire_struct!(ParameterSpec {name,kind,default});
semio_framework_value::artifact_retire_struct!(Rule {name,lhs,rhs});
semio_framework_value::artifact_retire_leaf!(ParameterKind);

/// 🧰️ The default value of every parameter of a rule right-hand side, keyed by parameter name.
pub fn default_parameter_bindings(rhs: &Rhs) -> semio_framework_graph_manifest::PropertyBag {
    rhs.parameters.iter().map(|param| (param.name.clone(), param.default.clone())).collect()
}
