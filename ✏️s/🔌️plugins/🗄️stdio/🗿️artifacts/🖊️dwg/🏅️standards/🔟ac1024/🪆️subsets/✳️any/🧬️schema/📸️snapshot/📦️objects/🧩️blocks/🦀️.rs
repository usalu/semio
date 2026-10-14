//! 🧩️ Dynamic blocks: parameters, grips, actions, visibility states and representation data.

use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::{DwgEvaluationExpression, DwgNamedEvaluationNodeReference, DwgPropertyExpressionReference};

//#region 🔖️DrawingModel
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockParameterDependencyBody {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockRepresentationData {
    pub represented_block_header_handle: u64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgDynamicBlockPurgePreventer {
    pub protected_block_header_handle: u64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockParameterConnection {
    pub code: u32,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockParameterProperty {
    pub connections: Vec<DwgBlockParameterConnection>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgBlockParameterBaseLocation {
    #[default]
    StartPoint,
    Midpoint,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockFlipValueSet {
    pub base_label: String,
    pub flipped_label: String,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockFlipParameter {
    pub evaluation_expression: DwgEvaluationExpression,
    pub name: String,
    pub show_properties: bool,
    pub chain_actions: bool,
    pub definition_base: Vec<f64>,
    pub definition_end: Vec<f64>,
    pub properties: Vec<DwgBlockParameterProperty>,
    pub base_location: DwgBlockParameterBaseLocation,
    pub label: String,
    pub description: String,
    pub value_set: DwgBlockFlipValueSet,
    pub label_point: Vec<f64>,
    pub updated_flip: DwgNamedEvaluationNodeReference,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgVisibilityEvaluationHistory {
    #[default]
    Stateless,
    Required,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgVisibilityState {
    pub name: String,
    pub visible_entity_handles: Vec<u64>,
    pub controlled_expression_handles: Vec<u64>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockVisibilityParameter {
    pub evaluation_expression: DwgEvaluationExpression,
    pub element_name: String,
    pub show_properties: bool,
    pub chain_actions: bool,
    pub definition_point: Vec<f64>,
    pub properties: Vec<DwgBlockParameterProperty>,
    pub updated_visibility_node_id: u32,
    pub initialized: bool,
    pub name: String,
    pub description: String,
    pub evaluation_history: DwgVisibilityEvaluationHistory,
    pub eligible_entity_handles: Vec<u64>,
    pub states: Vec<DwgVisibilityState>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockElement {
    pub evaluation_expression: DwgEvaluationExpression,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockGrip {
    pub element: DwgBlockElement,
    pub location: Vec<f64>,
    pub insertion_cycling: bool,
    pub insertion_cycling_weight: i32,
    pub updated_x: DwgNamedEvaluationNodeReference,
    pub updated_y: DwgNamedEvaluationNodeReference,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockTwoPointParameter {
    pub element: DwgBlockElement,
    pub show_properties: bool,
    pub chain_actions: bool,
    pub definition_base: Vec<f64>,
    pub definition_end: Vec<f64>,
    pub properties: Vec<DwgBlockParameterProperty>,
    pub property_expression_references: Vec<DwgPropertyExpressionReference>,
    pub base_location: DwgBlockParameterBaseLocation,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockLinearParameter {
    pub parameter: DwgBlockTwoPointParameter,
    pub distance_name: String,
    pub distance_description: String,
    pub label_offset: f64,
    pub allowed_values: Vec<f64>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockLinearGrip {
    pub grip: DwgBlockGrip,
    pub orientation: Vec<f64>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockFlipGrip {
    pub grip: DwgBlockGrip,
    pub updated_flip: DwgNamedEvaluationNodeReference,
    pub orientation: Vec<f64>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockVisibilityGrip {
    pub grip: DwgBlockGrip,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockActionConnection {
    pub node_id: u32,
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockActionDependency {
    pub object_handle: u64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockAction {
    pub evaluation_expression: DwgEvaluationExpression,
    pub name: String,
    pub display_location: Vec<f64>,
    pub dependencies: Vec<DwgBlockActionDependency>,
    pub action_node_ids: Vec<u32>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgBlockMoveCoordinateMode {
    #[default]
    CartesianXy,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockMoveAction {
    pub action: DwgBlockAction,
    pub x_connection: DwgBlockActionConnection,
    pub y_connection: DwgBlockActionConnection,
    pub distance_multiplier: f64,
    pub angle_offset: f64,
    pub coordinate_mode: DwgBlockMoveCoordinateMode,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockAlignmentParameter {
    pub parameter: DwgBlockTwoPointParameter,
    pub updated_grip_node_id: u32,
    pub align_perpendicular: bool,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockAlignmentGrip {
    pub grip: DwgBlockGrip,
    pub first_location_node_id: u32,
    pub second_location_node_id: u32,
    pub orientation: Vec<f64>,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgStretchSelection {
    pub object_handle: u64,
    pub vertex_indices: Vec<u32>,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgStretchSelector {
    pub node_id: u32,
    pub point_indices: Vec<u32>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgBlockActionCoordinateMode {
    #[default]
    CartesianXy,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockStretchAction {
    pub action: DwgBlockAction,
    pub x_connection: DwgBlockActionConnection,
    pub y_connection: DwgBlockActionConnection,
    pub points: Vec<Vec<f64>>,
    pub selections: Vec<DwgStretchSelection>,
    pub selectors: Vec<DwgStretchSelector>,
    pub distance_multiplier: f64,
    pub angle_offset: f64,
    pub coordinate_mode: DwgBlockActionCoordinateMode,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockActionWithBasePoint {
    pub action: DwgBlockAction,
    pub offset: Vec<f64>,
    pub x_base_connection: DwgBlockActionConnection,
    pub y_base_connection: DwgBlockActionConnection,
    pub dependent: bool,
    pub base_point: Vec<f64>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgBlockScaleMode {
    #[default]
    Xy,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockScaleAction {
    pub base: DwgBlockActionWithBasePoint,
    pub uniform_scale_connection: DwgBlockActionConnection,
    pub x_scale_connection: DwgBlockActionConnection,
    pub y_scale_connection: DwgBlockActionConnection,
    pub mode: DwgBlockScaleMode,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockFlipAction {
    pub action: DwgBlockAction,
    pub flip_connection: DwgBlockActionConnection,
    pub updated_flip_connection: DwgBlockActionConnection,
    pub updated_base_connection: DwgBlockActionConnection,
    pub updated_end_connection: DwgBlockActionConnection,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockOnePointParameter {
    pub element: DwgBlockElement,
    pub show_properties: bool,
    pub chain_actions: bool,
    pub definition_point: Vec<f64>,
    pub properties: Vec<DwgBlockParameterProperty>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockBasePointParameter {
    pub parameter: DwgBlockOnePointParameter,
    pub point: Vec<f64>,
    pub base_point: Vec<f64>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockParameterAllowedValues {
    pub values: Vec<f64>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockLinearConstraintParameter {
    pub parameter: DwgBlockTwoPointParameter,
    pub displacement_grip_node_id: u32,
    pub dependency_handle: u64,
    pub expression_name: String,
    pub expression_description: String,
    pub value: f64,
    pub allowed_values: DwgBlockParameterAllowedValues,
}
//#endregion 🔖️DrawingModel
