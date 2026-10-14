//! 🔗️ Associativity and evaluation: dependencies, actions, variables, evaluation expressions and graphs, networks.

use semio_framework_value::{ValueError, ValueRefusalKind};

//#region 🔖️DrawingModel
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgAssociativeDependencyStatus {
    #[default]
    UpToDate,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeDependency {
    pub status: DwgAssociativeDependencyStatus,
    pub is_read_dependency: bool,
    pub is_write_dependency: bool,
    pub is_attached_to_object: bool,
    pub is_delegating_to_owning_action: bool,
    pub order: i32,
    pub dependent_on_object_handle: u64,
    #[value(default)]
    pub name: Option<String>,
    #[value(default)]
    pub read_dependency_handle: Option<u64>,
    #[value(default)]
    pub dependency_node_handle: Option<u64>,
    #[value(default)]
    pub dependency_body_handle: Option<u64>,
    pub dependency_body_id: i32,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgEvaluationVariant {
    Integer32(i32),
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
dwg_metadata!(dwg_evaluation_variant_spec, dwg_evaluation_variant_spec_controlled, dwg_evaluation_variant_spec_producer => DwgEvaluationVariant;
    0, "kind", false, [enum ("integer32", 0)];
    1, "integer32", true, [typed i32];
);
impl semio_framework_dsl_record::DslField for DwgEvaluationVariant {
    dwg_controlled_payloads!(0; units[];
        0,1,Integer32,i32;);

    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(dwg_evaluation_variant_spec_producer())
    }

    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(dwg_evaluation_variant_spec_producer()))
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        match self {
            Self::Integer32(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(0));
                record.fields.insert(1, <i32 as semio_framework_dsl_record::DslField>::to_value(value));
            }
        }
        semio_framework_dsl_record::FieldValue::Record(record)
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else { return Err("expected evaluation-variant record".into()) };
        match record.get(0) {
            Some(semio_framework_dsl_record::FieldValue::Enum(0)) => Ok(Self::Integer32(<i32 as semio_framework_dsl_record::DslField>::from_value(record.get(1).ok_or("evaluation integer32 missing")?)?)),
            other => Err(format!("unknown evaluation-variant kind {other:?}")),
        }
    }
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeValueDependency {
    pub dependency: DwgAssociativeDependency,
    pub cached_value: DwgEvaluationVariant,
    pub value_name: String,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeGeometryDependency {
    pub dependency: DwgAssociativeDependency,
    pub enabled: bool,
    pub persistent_subentity_class_name: String,
    pub dependent_on_compound_object: bool,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgEvaluationExpressionValue {
    Empty,
    Double(f64),
    PointGroup10(Vec<f64>),
    PointGroup11(Vec<f64>),
    String(String),
    Integer32(i32),
    ObjectReference(u64),
    Integer16(i16),
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
dwg_metadata!(dwg_evaluation_expression_value_spec, dwg_evaluation_expression_value_spec_controlled, dwg_evaluation_expression_value_spec_producer => DwgEvaluationExpressionValue;
    0, "kind", false, [enum ("empty", 0), ("double", 1), ("pointGroup10", 2), ("pointGroup11", 3), ("string", 4), ("integer32", 5), ("objectReference", 6), ("integer16", 7)];
    1, "double", true, [typed f64];
    2, "point_group_10", true, [typed Vec<f64>];
    3, "point_group_11", true, [typed Vec<f64>];
    4, "string", true, [typed String];
    5, "integer32", true, [typed i32];
    6, "object_reference", true, [typed u64];
    7, "integer16", true, [typed i16];
);
impl semio_framework_dsl_record::DslField for DwgEvaluationExpressionValue {
    dwg_controlled_payloads!(0; units[Empty=0];
        1,1,Double,f64;
        2,2,PointGroup10,Vec<f64>;
        3,3,PointGroup11,Vec<f64>;
        4,4,String,String;
        5,5,Integer32,i32;
        6,6,ObjectReference,u64;
        7,7,Integer16,i16;);

    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(dwg_evaluation_expression_value_spec_producer())
    }

    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(dwg_evaluation_expression_value_spec_producer()))
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        let (kind, field, value) = match self {
            Self::Empty => (0, None, None),
            Self::Double(value) => (1, Some(1), Some(<f64 as semio_framework_dsl_record::DslField>::to_value(value))),
            Self::PointGroup10(value) => (2, Some(2), Some(<Vec<f64> as semio_framework_dsl_record::DslField>::to_value(value))),
            Self::PointGroup11(value) => (3, Some(3), Some(<Vec<f64> as semio_framework_dsl_record::DslField>::to_value(value))),
            Self::String(value) => (4, Some(4), Some(<String as semio_framework_dsl_record::DslField>::to_value(value))),
            Self::Integer32(value) => (5, Some(5), Some(<i32 as semio_framework_dsl_record::DslField>::to_value(value))),
            Self::ObjectReference(value) => (6, Some(6), Some(<u64 as semio_framework_dsl_record::DslField>::to_value(value))),
            Self::Integer16(value) => (7, Some(7), Some(<i16 as semio_framework_dsl_record::DslField>::to_value(value))),
        };
        record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(kind));
        if let (Some(field), Some(value)) = (field, value) {
            record.fields.insert(field, value);
        }
        semio_framework_dsl_record::FieldValue::Record(record)
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else { return Err("expected evaluation-expression value record".into()) };
        let present = record.fields.values().filter(|value| !matches!(value, semio_framework_dsl_record::FieldValue::Absent)).count();
        let expected = if matches!(record.get(0), Some(semio_framework_dsl_record::FieldValue::Enum(0))) { 1 } else { 2 };
        if present != expected {
            return Err("evaluation-expression value must contain exactly its tagged payload".into());
        }
        match record.get(0) {
            Some(semio_framework_dsl_record::FieldValue::Enum(0)) => Ok(Self::Empty),
            Some(semio_framework_dsl_record::FieldValue::Enum(1)) => Ok(Self::Double(<f64 as semio_framework_dsl_record::DslField>::from_value(record.get(1).ok_or("evaluation-expression double missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(2)) => Ok(Self::PointGroup10(<Vec<f64> as semio_framework_dsl_record::DslField>::from_value(record.get(2).ok_or("evaluation-expression group-10 point missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(3)) => Ok(Self::PointGroup11(<Vec<f64> as semio_framework_dsl_record::DslField>::from_value(record.get(3).ok_or("evaluation-expression group-11 point missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(4)) => Ok(Self::String(<String as semio_framework_dsl_record::DslField>::from_value(record.get(4).ok_or("evaluation-expression string missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(5)) => Ok(Self::Integer32(<i32 as semio_framework_dsl_record::DslField>::from_value(record.get(5).ok_or("evaluation-expression integer32 missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(6)) => Ok(Self::ObjectReference(<u64 as semio_framework_dsl_record::DslField>::from_value(record.get(6).ok_or("evaluation-expression object reference missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(7)) => Ok(Self::Integer16(<i16 as semio_framework_dsl_record::DslField>::from_value(record.get(7).ok_or("evaluation-expression integer16 missing")?)?)),
            other => Err(format!("unknown evaluation-expression value kind {other:?}")),
        }
    }
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgEvaluationExpression {
    pub parent_id: i32,
    pub major_version: u32,
    pub minor_version: u32,
    pub value: DwgEvaluationExpressionValue,
    pub node_id: u32,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockGripLocationComponent {
    pub evaluation_expression: DwgEvaluationExpression,
    pub grip_type: u32,
    pub grip_expression: String,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgDynamicBlockProxyNode {
    pub evaluation_expression: DwgEvaluationExpression,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgAssociativeActionStatus {
    #[default]
    UpToDate,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeActionDependency {
    pub owned: bool,
    pub dependency_handle: u64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeAction {
    pub status: DwgAssociativeActionStatus,
    #[value(default)]
    pub owning_network_handle: Option<u64>,
    #[value(default)]
    pub action_body_handle: Option<u64>,
    pub action_index: i32,
    pub maximum_dependency_index: i32,
    pub dependencies: Vec<DwgAssociativeActionDependency>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeVariable {
    pub action: DwgAssociativeAction,
    pub name: String,
    pub expression: String,
    pub evaluator_id: String,
    pub description: String,
    pub evaluated_value: DwgEvaluationVariant,
    pub mergeable: bool,
    #[value(default)]
    pub mergeable_variable_name: Option<String>,
    pub must_merge: bool,
    pub referenced_value_dependency_handles: Vec<u64>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAssociativeDimensionDependencyBody {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgEvaluationGraphNode {
    pub id: u32,
    pub expression_handle: u64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgEvaluationGraphEdge {
    pub from_node_id: u32,
    pub to_node_id: u32,
    pub reference_count: u32,
    pub invertible: bool,
    pub suppressed: bool,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgEvaluationGraph {
    pub nodes: Vec<DwgEvaluationGraphNode>,
    pub edges: Vec<DwgEvaluationGraphEdge>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgNamedEvaluationNodeReference {
    pub node_id: u32,
    pub expression_name: String,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgPropertyExpressionReference {
    pub property_index: u32,
    pub node_id: u32,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgAssocNetworkMemberKind {
    Network,
    #[default]
    Action,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAssocNetworkMember {
    pub handle: u64,
    pub kind: DwgAssocNetworkMemberKind,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAssocNetwork {
    pub action: DwgAssociativeAction,
    pub network_action_index: i32,
    pub actions: Vec<DwgAssocNetworkMember>,
}
//#endregion 🔖️DrawingModel
