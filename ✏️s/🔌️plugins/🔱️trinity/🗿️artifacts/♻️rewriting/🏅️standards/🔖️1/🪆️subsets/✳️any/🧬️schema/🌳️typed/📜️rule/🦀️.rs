//! 📜️ Canonical match pattern and ordered rewrite program, with direct owned field factories.
use semio_framework_graph::manifest::PropertyValue;
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
