//! 📦️ Non-graphical objects: layouts and plot settings, annotation scales, draw order and the object body union.

use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::{DwgAssocNetwork, DwgAssociativeDependency, DwgAssociativeDimensionDependencyBody, DwgAssociativeGeometryDependency, DwgAssociativeValueDependency, DwgAssociativeVariable, DwgBlockGripLocationComponent, DwgDynamicBlockProxyNode, DwgEvaluationGraph};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::{DwgBlockAlignmentGrip, DwgBlockAlignmentParameter, DwgBlockBasePointParameter, DwgBlockFlipAction, DwgBlockFlipGrip, DwgBlockFlipParameter, DwgBlockLinearConstraintParameter, DwgBlockLinearGrip, DwgBlockLinearParameter, DwgBlockMoveAction, DwgBlockParameterDependencyBody, DwgBlockRepresentationData, DwgBlockScaleAction, DwgBlockStretchAction, DwgBlockVisibilityGrip, DwgBlockVisibilityParameter, DwgDynamicBlockPurgePreventer};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::{DwgAssoc2dConstraintGroup};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::{DwgEntityBody, DwgOrthographicView};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::{DwgMLeaderStyle, DwgMaterial, DwgMlineStyle, DwgTableStyle, DwgVisualStyle};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::{DwgTableControlBody, DwgTableRecordBody};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::{DwgDictionaryBody, DwgDictionaryVariable, DwgExtendedEntityData, DwgObjectCategory, DwgXRecordBody};

#[macro_use]
#[path = "🎛️metadata/🦀️.rs"]
pub mod metadata;
#[path = "🏷️xrecord/🦀️.rs"]
pub mod xrecord;
#[path = "🗃️tables/🦀️.rs"]
pub mod tables;
#[path = "📐️entities/🦀️.rs"]
pub mod entities;
#[path = "🔗️associativity/🦀️.rs"]
pub mod associativity;
#[path = "🖌️styles/🦀️.rs"]
pub mod styles;
#[path = "🧩️blocks/🦀️.rs"]
pub mod blocks;
#[path = "📏️constraints/🦀️.rs"]
pub mod constraints;

//#region 🔖️DrawingModel
#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgPlaceholder {}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgAnnotationScale {
    pub name: String,
    pub paper_units: f64,
    pub drawing_units: f64,
    pub is_unit_scale: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgDrawOrderEntry {
    pub entity_handle: u64,
    pub sort_handle: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgSortEntitiesTable {
    pub block_header_handle: u64,
    pub entries: Vec<DwgDrawOrderEntry>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgPlotOptions {
    pub use_standard_scale: bool,
    pub plot_viewport_borders: bool,
    pub plot_with_lineweights: bool,
    pub draw_viewports_first: bool,
    pub model_type: bool,
    pub update_paper: bool,
    pub initializing: bool,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgPlotPaperUnit {
    #[default]
    Inches,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgPlotRotation {
    #[default]
    QuarterTurn,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgPlotArea {
    #[default]
    Display,
    Layout,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgStandardScale {
    #[default]
    Custom,
    OneToOne,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgShadePlot {
    #[default]
    AsDisplayed,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgShadePlotResolution {
    #[default]
    Normal,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgLayoutOptions {
    pub paper_space_linetype_scaling: bool,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgLayout {
    pub page_setup_name: String,
    pub printer_configuration: String,
    pub canonical_media_name: String,
    pub stylesheet: String,
    pub name: String,
    pub plot_options: DwgPlotOptions,
    pub margins: Vec<f64>,
    pub paper_size: Vec<f64>,
    pub plot_origin: Vec<f64>,
    pub paper_unit: DwgPlotPaperUnit,
    pub rotation: DwgPlotRotation,
    pub plot_area: DwgPlotArea,
    pub plot_window_lower_left: Vec<f64>,
    pub plot_window_upper_right: Vec<f64>,
    pub paper_units: f64,
    pub drawing_units: f64,
    pub standard_scale: DwgStandardScale,
    pub standard_scale_factor: f64,
    pub paper_image_origin: Vec<f64>,
    pub shade_plot: DwgShadePlot,
    pub shade_plot_resolution: DwgShadePlotResolution,
    pub shade_plot_dpi: u16,
    pub tab_order: u16,
    pub options: DwgLayoutOptions,
    pub insertion_base: Vec<f64>,
    pub limits_minimum: Vec<f64>,
    pub limits_maximum: Vec<f64>,
    pub ucs_origin: Vec<f64>,
    pub ucs_x_axis: Vec<f64>,
    pub ucs_y_axis: Vec<f64>,
    pub ucs_elevation: f64,
    pub orthographic_view: DwgOrthographicView,
    pub extents_minimum: Vec<f64>,
    pub extents_maximum: Vec<f64>,
    pub plot_view_handle: Option<u64>,
    pub visual_style_handle: Option<u64>,
    pub block_header_handle: u64,
    pub active_viewport_handle: Option<u64>,
    pub base_ucs_handle: Option<u64>,
    pub named_ucs_handle: Option<u64>,
    pub viewport_handles: Vec<u64>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgLogicalObjectBody {
    Dictionary(DwgDictionaryBody),
    TableControl(DwgTableControlBody),
    TableRecord(DwgTableRecordBody),
    XRecord(DwgXRecordBody),
    Entity(DwgEntityBody),
    AssociativeDependency(DwgAssociativeDependency),
    AssociativeValueDependency(DwgAssociativeValueDependency),
    AssociativeGeometryDependency(DwgAssociativeGeometryDependency),
    BlockGripLocationComponent(DwgBlockGripLocationComponent),
    DynamicBlockProxyNode(DwgDynamicBlockProxyNode),
    AssociativeVariable(DwgAssociativeVariable),
    AssociativeDimensionDependencyBody(DwgAssociativeDimensionDependencyBody),
    VisualStyle(DwgVisualStyle),
    BlockParameterDependencyBody(DwgBlockParameterDependencyBody),
    BlockRepresentationData(DwgBlockRepresentationData),
    DynamicBlockPurgePreventer(DwgDynamicBlockPurgePreventer),
    EvaluationGraph(DwgEvaluationGraph),
    BlockFlipParameter(DwgBlockFlipParameter),
    BlockVisibilityParameter(DwgBlockVisibilityParameter),
    Placeholder(DwgPlaceholder),
    DictionaryVariable(DwgDictionaryVariable),
    AnnotationScale(DwgAnnotationScale),
    SortEntitiesTable(DwgSortEntitiesTable),
    TableStyle(Box<DwgTableStyle>),
    MlineStyle(DwgMlineStyle),
    MLeaderStyle(DwgMLeaderStyle),
    Material(DwgMaterial),
    BlockMoveAction(DwgBlockMoveAction),
    AssocNetwork(DwgAssocNetwork),
    Assoc2dConstraintGroup(DwgAssoc2dConstraintGroup),
    BlockLinearParameter(DwgBlockLinearParameter),
    BlockLinearGrip(DwgBlockLinearGrip),
    BlockFlipGrip(DwgBlockFlipGrip),
    BlockVisibilityGrip(DwgBlockVisibilityGrip),
    BlockAlignmentParameter(DwgBlockAlignmentParameter),
    BlockAlignmentGrip(DwgBlockAlignmentGrip),
    BlockStretchAction(DwgBlockStretchAction),
    BlockScaleAction(DwgBlockScaleAction),
    BlockFlipAction(DwgBlockFlipAction),
    BlockBasePointParameter(DwgBlockBasePointParameter),
    BlockVerticalConstraintParameter(DwgBlockLinearConstraintParameter),
    BlockHorizontalConstraintParameter(DwgBlockLinearConstraintParameter),
    Layout(DwgLayout),
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
dwg_metadata!(dwg_logical_object_body_spec, dwg_logical_object_body_spec_controlled, dwg_logical_object_body_spec_producer => DwgLogicalObjectBody;
    0, "kind", false, [enum ("dictionary", 0), ("tableControl", 1), ("tableRecord", 2), ("xrecord", 3), ("entity", 4), ("associativeDependency", 5), ("associativeValueDependency", 6), ("associativeGeometryDependency", 7), ("blockGripLocationComponent", 8), ("dynamicBlockProxyNode", 9), ("associativeVariable", 10), ("associativeDimensionDependencyBody", 11), ("visualStyle", 12), ("blockParameterDependencyBody", 13), ("blockRepresentationData", 14), ("dynamicBlockPurgePreventer", 15), ("evaluationGraph", 16), ("blockFlipParameter", 17), ("blockVisibilityParameter", 18), ("placeholder", 19), ("dictionaryVariable", 20), ("annotationScale", 21), ("sortEntitiesTable", 22), ("tableStyle", 23), ("mlineStyle", 24), ("mLeaderStyle", 25), ("material", 26), ("blockMoveAction", 27), ("assocNetwork", 28), ("assoc2dConstraintGroup", 29), ("blockLinearParameter", 30), ("blockLinearGrip", 31), ("blockFlipGrip", 32), ("blockVisibilityGrip", 33), ("blockAlignmentParameter", 34), ("blockAlignmentGrip", 35), ("blockStretchAction", 36), ("blockScaleAction", 37), ("blockFlipAction", 38), ("blockBasePointParameter", 39), ("blockVerticalConstraintParameter", 40), ("blockHorizontalConstraintParameter", 41), ("layout", 42)];
    1, "dictionary", true, [typed DwgDictionaryBody];
    2, "table_control", true, [typed DwgTableControlBody];
    3, "table_record", true, [typed DwgTableRecordBody];
    4, "xrecord", true, [typed DwgXRecordBody];
    5, "entity", true, [typed DwgEntityBody];
    6, "associative_dependency", true, [typed DwgAssociativeDependency];
    7, "associative_value_dependency", true, [typed DwgAssociativeValueDependency];
    8, "associative_geometry_dependency", true, [typed DwgAssociativeGeometryDependency];
    9, "block_grip_location_component", true, [typed DwgBlockGripLocationComponent];
    10, "dynamic_block_proxy_node", true, [typed DwgDynamicBlockProxyNode];
    11, "associative_variable", true, [typed DwgAssociativeVariable];
    12, "associative_dimension_dependency_body", true, [typed DwgAssociativeDimensionDependencyBody];
    13, "visual_style", true, [typed DwgVisualStyle];
    14, "block_parameter_dependency_body", true, [typed DwgBlockParameterDependencyBody];
    15, "block_representation_data", true, [typed DwgBlockRepresentationData];
    16, "dynamic_block_purge_preventer", true, [typed DwgDynamicBlockPurgePreventer];
    17, "evaluation_graph", true, [typed DwgEvaluationGraph];
    18, "block_flip_parameter", true, [typed DwgBlockFlipParameter];
    19, "block_visibility_parameter", true, [typed DwgBlockVisibilityParameter];
    20, "placeholder", true, [typed DwgPlaceholder];
    21, "dictionary_variable", true, [typed DwgDictionaryVariable];
    22, "annotation_scale", true, [typed DwgAnnotationScale];
    23, "sort_entities_table", true, [typed DwgSortEntitiesTable];
    24, "table_style", true, [typed DwgTableStyle];
    25, "mline_style", true, [typed DwgMlineStyle];
    26, "m_leader_style", true, [typed DwgMLeaderStyle];
    27, "material", true, [typed DwgMaterial];
    28, "block_move_action", true, [typed DwgBlockMoveAction];
    29, "assoc_network", true, [typed DwgAssocNetwork];
    30, "assoc_2d_constraint_group", true, [typed DwgAssoc2dConstraintGroup];
    31, "block_linear_parameter", true, [typed DwgBlockLinearParameter];
    32, "block_linear_grip", true, [typed DwgBlockLinearGrip];
    33, "block_flip_grip", true, [typed DwgBlockFlipGrip];
    34, "block_visibility_grip", true, [typed DwgBlockVisibilityGrip];
    35, "block_alignment_parameter", true, [typed DwgBlockAlignmentParameter];
    36, "block_alignment_grip", true, [typed DwgBlockAlignmentGrip];
    37, "block_stretch_action", true, [typed DwgBlockStretchAction];
    38, "block_scale_action", true, [typed DwgBlockScaleAction];
    39, "block_flip_action", true, [typed DwgBlockFlipAction];
    40, "block_base_point_parameter", true, [typed DwgBlockBasePointParameter];
    41, "block_vertical_constraint_parameter", true, [typed DwgBlockLinearConstraintParameter];
    42, "block_horizontal_constraint_parameter", true, [typed DwgBlockLinearConstraintParameter];
    43, "layout", true, [typed DwgLayout];
);
impl semio_framework_dsl_record::DslField for DwgLogicalObjectBody {
    dwg_controlled_payloads!(0; units[];
        0,1,Dictionary,DwgDictionaryBody;
        1,2,TableControl,DwgTableControlBody;
        2,3,TableRecord,DwgTableRecordBody;
        3,4,XRecord,DwgXRecordBody;
        4,5,Entity,DwgEntityBody;
        5,6,AssociativeDependency,DwgAssociativeDependency;
        6,7,AssociativeValueDependency,DwgAssociativeValueDependency;
        7,8,AssociativeGeometryDependency,DwgAssociativeGeometryDependency;
        8,9,BlockGripLocationComponent,DwgBlockGripLocationComponent;
        9,10,DynamicBlockProxyNode,DwgDynamicBlockProxyNode;
        10,11,AssociativeVariable,DwgAssociativeVariable;
        11,12,AssociativeDimensionDependencyBody,DwgAssociativeDimensionDependencyBody;
        12,13,VisualStyle,DwgVisualStyle;
        13,14,BlockParameterDependencyBody,DwgBlockParameterDependencyBody;
        14,15,BlockRepresentationData,DwgBlockRepresentationData;
        15,16,DynamicBlockPurgePreventer,DwgDynamicBlockPurgePreventer;
        16,17,EvaluationGraph,DwgEvaluationGraph;
        17,18,BlockFlipParameter,DwgBlockFlipParameter;
        18,19,BlockVisibilityParameter,DwgBlockVisibilityParameter;
        19,20,Placeholder,DwgPlaceholder;
        20,21,DictionaryVariable,DwgDictionaryVariable;
        21,22,AnnotationScale,DwgAnnotationScale;
        22,23,SortEntitiesTable,DwgSortEntitiesTable;
        23,24,TableStyle,Box<DwgTableStyle>;
        24,25,MlineStyle,DwgMlineStyle;
        25,26,MLeaderStyle,DwgMLeaderStyle;
        26,27,Material,DwgMaterial;
        27,28,BlockMoveAction,DwgBlockMoveAction;
        28,29,AssocNetwork,DwgAssocNetwork;
        29,30,Assoc2dConstraintGroup,DwgAssoc2dConstraintGroup;
        30,31,BlockLinearParameter,DwgBlockLinearParameter;
        31,32,BlockLinearGrip,DwgBlockLinearGrip;
        32,33,BlockFlipGrip,DwgBlockFlipGrip;
        33,34,BlockVisibilityGrip,DwgBlockVisibilityGrip;
        34,35,BlockAlignmentParameter,DwgBlockAlignmentParameter;
        35,36,BlockAlignmentGrip,DwgBlockAlignmentGrip;
        36,37,BlockStretchAction,DwgBlockStretchAction;
        37,38,BlockScaleAction,DwgBlockScaleAction;
        38,39,BlockFlipAction,DwgBlockFlipAction;
        39,40,BlockBasePointParameter,DwgBlockBasePointParameter;
        40,41,BlockVerticalConstraintParameter,DwgBlockLinearConstraintParameter;
        41,42,BlockHorizontalConstraintParameter,DwgBlockLinearConstraintParameter;
        42,43,Layout,DwgLayout;);

    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(dwg_logical_object_body_spec_producer())
    }

    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(dwg_logical_object_body_spec_producer()))
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        let (kind, payload_id, payload) = match self {
            Self::Dictionary(value) => (0, 1, <DwgDictionaryBody as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::TableControl(value) => (1, 2, <DwgTableControlBody as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::TableRecord(value) => (2, 3, <DwgTableRecordBody as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::XRecord(value) => (3, 4, <DwgXRecordBody as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::Entity(value) => (4, 5, <DwgEntityBody as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::AssociativeDependency(value) => (5, 6, <DwgAssociativeDependency as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::AssociativeValueDependency(value) => (6, 7, <DwgAssociativeValueDependency as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::AssociativeGeometryDependency(value) => (7, 8, <DwgAssociativeGeometryDependency as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockGripLocationComponent(value) => (8, 9, <DwgBlockGripLocationComponent as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::DynamicBlockProxyNode(value) => (9, 10, <DwgDynamicBlockProxyNode as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::AssociativeVariable(value) => (10, 11, <DwgAssociativeVariable as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::AssociativeDimensionDependencyBody(value) => (11, 12, <DwgAssociativeDimensionDependencyBody as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::VisualStyle(value) => (12, 13, <DwgVisualStyle as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockParameterDependencyBody(value) => (13, 14, <DwgBlockParameterDependencyBody as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockRepresentationData(value) => (14, 15, <DwgBlockRepresentationData as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::DynamicBlockPurgePreventer(value) => (15, 16, <DwgDynamicBlockPurgePreventer as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::EvaluationGraph(value) => (16, 17, <DwgEvaluationGraph as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockFlipParameter(value) => (17, 18, <DwgBlockFlipParameter as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockVisibilityParameter(value) => (18, 19, <DwgBlockVisibilityParameter as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::Placeholder(value) => (19, 20, <DwgPlaceholder as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::DictionaryVariable(value) => (20, 21, <DwgDictionaryVariable as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::AnnotationScale(value) => (21, 22, <DwgAnnotationScale as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::SortEntitiesTable(value) => (22, 23, <DwgSortEntitiesTable as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::TableStyle(value) => (23, 24, <DwgTableStyle as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::MlineStyle(value) => (24, 25, <DwgMlineStyle as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::MLeaderStyle(value) => (25, 26, <DwgMLeaderStyle as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::Material(value) => (26, 27, <DwgMaterial as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockMoveAction(value) => (27, 28, <DwgBlockMoveAction as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::AssocNetwork(value) => (28, 29, <DwgAssocNetwork as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::Assoc2dConstraintGroup(value) => (29, 30, <DwgAssoc2dConstraintGroup as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockLinearParameter(value) => (30, 31, <DwgBlockLinearParameter as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockLinearGrip(value) => (31, 32, <DwgBlockLinearGrip as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockFlipGrip(value) => (32, 33, <DwgBlockFlipGrip as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockVisibilityGrip(value) => (33, 34, <DwgBlockVisibilityGrip as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockAlignmentParameter(value) => (34, 35, <DwgBlockAlignmentParameter as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockAlignmentGrip(value) => (35, 36, <DwgBlockAlignmentGrip as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockStretchAction(value) => (36, 37, <DwgBlockStretchAction as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockScaleAction(value) => (37, 38, <DwgBlockScaleAction as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockFlipAction(value) => (38, 39, <DwgBlockFlipAction as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockBasePointParameter(value) => (39, 40, <DwgBlockBasePointParameter as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockVerticalConstraintParameter(value) => (40, 41, <DwgBlockLinearConstraintParameter as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::BlockHorizontalConstraintParameter(value) => (41, 42, <DwgBlockLinearConstraintParameter as semio_framework_dsl_record::DslField>::to_value(value)),
            Self::Layout(value) => (42, 43, <DwgLayout as semio_framework_dsl_record::DslField>::to_value(value)),
        };
        record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(kind));
        record.fields.insert(payload_id, payload);
        semio_framework_dsl_record::FieldValue::Record(record)
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else {
            return Err(format!("expected DWG object-body record, found {value:?}"));
        };
        if record.fields.values().filter(|value| !matches!(value, semio_framework_dsl_record::FieldValue::Absent)).count() != 2 {
            return Err("DWG object body must contain exactly one tagged payload".into());
        }
        match record.get(0) {
            Some(semio_framework_dsl_record::FieldValue::Enum(0)) => Ok(Self::Dictionary(<DwgDictionaryBody as semio_framework_dsl_record::DslField>::from_value(record.get(1).ok_or("dictionary body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(1)) => Ok(Self::TableControl(<DwgTableControlBody as semio_framework_dsl_record::DslField>::from_value(record.get(2).ok_or("table-control body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(2)) => Ok(Self::TableRecord(<DwgTableRecordBody as semio_framework_dsl_record::DslField>::from_value(record.get(3).ok_or("table-record body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(3)) => Ok(Self::XRecord(<DwgXRecordBody as semio_framework_dsl_record::DslField>::from_value(record.get(4).ok_or("XRECORD body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(4)) => Ok(Self::Entity(<DwgEntityBody as semio_framework_dsl_record::DslField>::from_value(record.get(5).ok_or("entity body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(5)) => Ok(Self::AssociativeDependency(<DwgAssociativeDependency as semio_framework_dsl_record::DslField>::from_value(record.get(6).ok_or("associative-dependency body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(6)) => Ok(Self::AssociativeValueDependency(<DwgAssociativeValueDependency as semio_framework_dsl_record::DslField>::from_value(record.get(7).ok_or("associative-value-dependency body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(7)) => Ok(Self::AssociativeGeometryDependency(<DwgAssociativeGeometryDependency as semio_framework_dsl_record::DslField>::from_value(record.get(8).ok_or("associative-geometry-dependency body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(8)) => Ok(Self::BlockGripLocationComponent(<DwgBlockGripLocationComponent as semio_framework_dsl_record::DslField>::from_value(record.get(9).ok_or("block-grip-location-component body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(9)) => Ok(Self::DynamicBlockProxyNode(<DwgDynamicBlockProxyNode as semio_framework_dsl_record::DslField>::from_value(record.get(10).ok_or("dynamic-block-proxy-node body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(10)) => Ok(Self::AssociativeVariable(<DwgAssociativeVariable as semio_framework_dsl_record::DslField>::from_value(record.get(11).ok_or("associative-variable body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(11)) => Ok(Self::AssociativeDimensionDependencyBody(<DwgAssociativeDimensionDependencyBody as semio_framework_dsl_record::DslField>::from_value(record.get(12).ok_or("associative-dimension-dependency body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(12)) => Ok(Self::VisualStyle(<DwgVisualStyle as semio_framework_dsl_record::DslField>::from_value(record.get(13).ok_or("visual-style body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(13)) => Ok(Self::BlockParameterDependencyBody(<DwgBlockParameterDependencyBody as semio_framework_dsl_record::DslField>::from_value(record.get(14).ok_or("block-parameter-dependency body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(14)) => Ok(Self::BlockRepresentationData(<DwgBlockRepresentationData as semio_framework_dsl_record::DslField>::from_value(record.get(15).ok_or("block-representation data missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(15)) => Ok(Self::DynamicBlockPurgePreventer(<DwgDynamicBlockPurgePreventer as semio_framework_dsl_record::DslField>::from_value(record.get(16).ok_or("dynamic-block purge-preventer body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(16)) => Ok(Self::EvaluationGraph(<DwgEvaluationGraph as semio_framework_dsl_record::DslField>::from_value(record.get(17).ok_or("evaluation-graph body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(17)) => Ok(Self::BlockFlipParameter(<DwgBlockFlipParameter as semio_framework_dsl_record::DslField>::from_value(record.get(18).ok_or("block-flip-parameter body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(18)) => Ok(Self::BlockVisibilityParameter(<DwgBlockVisibilityParameter as semio_framework_dsl_record::DslField>::from_value(record.get(19).ok_or("block-visibility-parameter body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(19)) => Ok(Self::Placeholder(<DwgPlaceholder as semio_framework_dsl_record::DslField>::from_value(record.get(20).ok_or("placeholder body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(20)) => Ok(Self::DictionaryVariable(<DwgDictionaryVariable as semio_framework_dsl_record::DslField>::from_value(record.get(21).ok_or("dictionary-variable body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(21)) => Ok(Self::AnnotationScale(<DwgAnnotationScale as semio_framework_dsl_record::DslField>::from_value(record.get(22).ok_or("annotation-scale body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(22)) => Ok(Self::SortEntitiesTable(<DwgSortEntitiesTable as semio_framework_dsl_record::DslField>::from_value(record.get(23).ok_or("sort-entities-table body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(23)) => Ok(Self::TableStyle(Box::new(<DwgTableStyle as semio_framework_dsl_record::DslField>::from_value(record.get(24).ok_or("table-style body missing")?)?))),
            Some(semio_framework_dsl_record::FieldValue::Enum(24)) => Ok(Self::MlineStyle(<DwgMlineStyle as semio_framework_dsl_record::DslField>::from_value(record.get(25).ok_or("MLINESTYLE body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(25)) => Ok(Self::MLeaderStyle(<DwgMLeaderStyle as semio_framework_dsl_record::DslField>::from_value(record.get(26).ok_or("MLEADERSTYLE body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(26)) => Ok(Self::Material(<DwgMaterial as semio_framework_dsl_record::DslField>::from_value(record.get(27).ok_or("MATERIAL body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(27)) => Ok(Self::BlockMoveAction(<DwgBlockMoveAction as semio_framework_dsl_record::DslField>::from_value(record.get(28).ok_or("BLOCKMOVEACTION body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(28)) => Ok(Self::AssocNetwork(<DwgAssocNetwork as semio_framework_dsl_record::DslField>::from_value(record.get(29).ok_or("ACDBASSOCNETWORK body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(29)) => Ok(Self::Assoc2dConstraintGroup(<DwgAssoc2dConstraintGroup as semio_framework_dsl_record::DslField>::from_value(record.get(30).ok_or("ACDBASSOC2DCONSTRAINTGROUP body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(30)) => Ok(Self::BlockLinearParameter(<DwgBlockLinearParameter as semio_framework_dsl_record::DslField>::from_value(record.get(31).ok_or("BLOCKLINEARPARAMETER body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(31)) => Ok(Self::BlockLinearGrip(<DwgBlockLinearGrip as semio_framework_dsl_record::DslField>::from_value(record.get(32).ok_or("BLOCKLINEARGRIP body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(32)) => Ok(Self::BlockFlipGrip(<DwgBlockFlipGrip as semio_framework_dsl_record::DslField>::from_value(record.get(33).ok_or("BLOCKFLIPGRIP body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(33)) => Ok(Self::BlockVisibilityGrip(<DwgBlockVisibilityGrip as semio_framework_dsl_record::DslField>::from_value(record.get(34).ok_or("BLOCKVISIBILITYGRIP body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(34)) => Ok(Self::BlockAlignmentParameter(<DwgBlockAlignmentParameter as semio_framework_dsl_record::DslField>::from_value(record.get(35).ok_or("BLOCKALIGNMENTPARAMETER body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(35)) => Ok(Self::BlockAlignmentGrip(<DwgBlockAlignmentGrip as semio_framework_dsl_record::DslField>::from_value(record.get(36).ok_or("BLOCKALIGNMENTGRIP body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(36)) => Ok(Self::BlockStretchAction(<DwgBlockStretchAction as semio_framework_dsl_record::DslField>::from_value(record.get(37).ok_or("BLOCKSTRETCHACTION body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(37)) => Ok(Self::BlockScaleAction(<DwgBlockScaleAction as semio_framework_dsl_record::DslField>::from_value(record.get(38).ok_or("BLOCKSCALEACTION body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(38)) => Ok(Self::BlockFlipAction(<DwgBlockFlipAction as semio_framework_dsl_record::DslField>::from_value(record.get(39).ok_or("BLOCKFLIPACTION body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(39)) => Ok(Self::BlockBasePointParameter(<DwgBlockBasePointParameter as semio_framework_dsl_record::DslField>::from_value(record.get(40).ok_or("BLOCKBASEPOINTPARAMETER body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(40)) => Ok(Self::BlockVerticalConstraintParameter(<DwgBlockLinearConstraintParameter as semio_framework_dsl_record::DslField>::from_value(record.get(41).ok_or("BLOCKVERTICALCONSTRAINTPARAMETER body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(41)) => Ok(Self::BlockHorizontalConstraintParameter(<DwgBlockLinearConstraintParameter as semio_framework_dsl_record::DslField>::from_value(record.get(42).ok_or("BLOCKHORIZONTALCONSTRAINTPARAMETER body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(42)) => Ok(Self::Layout(<DwgLayout as semio_framework_dsl_record::DslField>::from_value(record.get(43).ok_or("LAYOUT body missing")?)?)),
            other => Err(format!("expected DWG object-body kind, found {other:?}")),
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgLogicalObject {
    pub handle: u64,
    pub type_code: u16,
    pub class_name: String,
    pub category: DwgObjectCategory,
    #[value(default)]
    pub owner_handle: Option<u64>,
    #[value(default)]
    pub reactor_handles: Vec<u64>,
    #[value(default)]
    pub extension_dictionary_handle: Option<u64>,
    #[value(default)]
    pub referenced_handles: Vec<u64>,
    #[value(default)]
    pub extended_data: Vec<DwgExtendedEntityData>,
    #[value(default)]
    pub body: Option<DwgLogicalObjectBody>,
}
//#endregion 🔖️DrawingModel
