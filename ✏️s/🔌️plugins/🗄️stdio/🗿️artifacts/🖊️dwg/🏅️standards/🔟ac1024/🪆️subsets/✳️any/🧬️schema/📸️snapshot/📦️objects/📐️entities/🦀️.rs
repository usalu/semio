//! 📐️ Graphical entities: the common entity fields, every entity body and the viewport and dimension entities.

use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::{DwgComplexColor};

//#region 🔖️DrawingModel
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgEntityMode {
    ExplicitOwner,
    PaperSpace,
    #[default]
    ModelSpace,
    Reserved,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgEntityReferenceMode {
    #[default]
    ByLayer,
    ByBlock,
    Continuous,
    Explicit,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgEntityColorKind {
    #[default]
    ByLayer,
    ByBlock,
    Index,
    TrueColor,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgEntityColor {
    pub kind: DwgEntityColorKind,
    pub index: u16,
    pub rgb: u32,
    #[value(default)]
    pub transparency: Option<u32>,
    #[value(default)]
    pub name: Option<String>,
    #[value(default)]
    pub book_name: Option<String>,
    #[value(default)]
    pub color_handle: Option<u64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgEntityCommon {
    pub mode: DwgEntityMode,
    pub color: DwgEntityColor,
    pub linetype_scale: f64,
    pub linetype: DwgEntityReferenceMode,
    pub plot_style: DwgEntityReferenceMode,
    pub material: DwgEntityReferenceMode,
    pub shadow: u8,
    pub invisible: u16,
    pub lineweight: u8,
    pub layer_handle: u64,
    #[value(default)]
    pub linetype_handle: Option<u64>,
    #[value(default)]
    pub material_handle: Option<u64>,
    #[value(default)]
    pub shadow_handle: Option<u64>,
    #[value(default)]
    pub plot_style_handle: Option<u64>,
    #[value(default)]
    pub full_visual_style_handle: Option<u64>,
    #[value(default)]
    pub face_visual_style_handle: Option<u64>,
    #[value(default)]
    pub edge_visual_style_handle: Option<u64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgLineEntity {
    pub common: DwgEntityCommon,
    pub start: Vec<f64>,
    pub end: Vec<f64>,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgArcEntity {
    pub common: DwgEntityCommon,
    pub center: Vec<f64>,
    pub radius: f64,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
    pub start_angle: f64,
    pub end_angle: f64,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgLwPolylineVertex {
    pub point: Vec<f64>,
    pub bulge: f64,
    #[value(default)]
    pub vertex_id: Option<u32>,
    #[value(default)]
    pub start_width: Option<f64>,
    #[value(default)]
    pub end_width: Option<f64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgLwPolylineEntity {
    pub common: DwgEntityCommon,
    pub closed: bool,
    #[value(default)]
    pub constant_width: Option<f64>,
    pub elevation: f64,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
    #[value(default)]
    pub vertices: Vec<DwgLwPolylineVertex>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockBeginEntity {
    pub common: DwgEntityCommon,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgBlockEndEntity {
    pub common: DwgEntityCommon,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgInsertEntity {
    pub common: DwgEntityCommon,
    pub insertion: Vec<f64>,
    pub scale: Vec<f64>,
    pub rotation: f64,
    pub extrusion: Vec<f64>,
    pub block_header_handle: u64,
    #[value(default)]
    pub attribute_handles: Vec<u64>,
    #[value(default)]
    pub sequence_end_handle: Option<u64>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgDimensionTextAttachment {
    #[default]
    TopCenter,
    TopLeft,
    TopRight,
    MiddleCenter,
    MiddleLeft,
    MiddleRight,
    BottomCenter,
    BottomLeft,
    BottomRight,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgDimensionLineSpacingStyle {
    #[default]
    AtLeast,
    Exact,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgDimensionStatus {
    pub block_reference_is_exclusive: bool,
    pub user_positioned_text: bool,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgDimensionEntityCommon {
    pub common: DwgEntityCommon,
    pub extrusion: Vec<f64>,
    pub text_midpoint: Vec<f64>,
    pub elevation: f64,
    pub status: DwgDimensionStatus,
    pub user_text: String,
    pub text_rotation: f64,
    pub horizontal_direction: f64,
    pub insertion_scale: Vec<f64>,
    pub insertion_rotation: f64,
    pub attachment: DwgDimensionTextAttachment,
    pub line_spacing_style: DwgDimensionLineSpacingStyle,
    pub line_spacing_factor: f64,
    pub actual_measurement: f64,
    pub flip_arrow_1: bool,
    pub flip_arrow_2: bool,
    pub clone_insertion_point: Vec<f64>,
    pub dimension_style_handle: u64,
    #[value(default)]
    pub dimension_block_handle: Option<u64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgLinearDimensionEntity {
    pub dimension: DwgDimensionEntityCommon,
    pub extension_line_1: Vec<f64>,
    pub extension_line_2: Vec<f64>,
    pub definition_point: Vec<f64>,
    pub oblique_angle: f64,
    pub dimension_rotation: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgViewportStatusFlag {
    Perspective,
    FrontClipping,
    BackClipping,
    UcsFollow,
    FrontClipNotAtEye,
    UcsIconVisible,
    UcsIconAtOrigin,
    FastZoom,
    Snap,
    Grid,
    IsometricSnap,
    HidePlot,
    IsoPairTop,
    IsoPairRight,
    ZoomLock,
    AlwaysEnabled,
    NonRectangularClipping,
    ViewportOff,
    GridBeyondDrawingLimits,
    AdaptiveGrid,
    AdaptiveSubdivision,
    GridFollowsWorkplane,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgViewportRenderMode {
    #[default]
    Optimized2d,
    Wireframe,
    HiddenLine,
    FlatShaded,
    GouraudShaded,
    FlatShadedWithWireframe,
    GouraudShadedWithWireframe,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgOrthographicView {
    #[default]
    None,
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgShadePlotMode {
    #[default]
    AsDisplayed,
    Wireframe,
    Hidden,
    Rendered,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgDefaultLightingType {
    OneDistantLight,
    #[default]
    TwoDistantLights,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgViewportEntity {
    pub common: DwgEntityCommon,
    pub center: Vec<f64>,
    pub width: f64,
    pub height: f64,
    pub view_target: Vec<f64>,
    pub view_direction: Vec<f64>,
    pub twist_angle: f64,
    pub view_height: f64,
    pub lens_length: f64,
    pub front_clip: f64,
    pub back_clip: f64,
    pub snap_angle: f64,
    pub view_center: Vec<f64>,
    pub snap_base: Vec<f64>,
    pub snap_unit: Vec<f64>,
    pub grid_unit: Vec<f64>,
    pub circle_zoom_percent: u16,
    pub grid_major: u16,
    pub frozen_layer_handles: Vec<u64>,
    pub status: Vec<DwgViewportStatusFlag>,
    pub style_sheet: String,
    pub render_mode: DwgViewportRenderMode,
    pub ucs_at_origin: bool,
    pub ucs_per_viewport: bool,
    pub ucs_origin: Vec<f64>,
    pub ucs_x_axis: Vec<f64>,
    pub ucs_y_axis: Vec<f64>,
    pub ucs_elevation: f64,
    pub orthographic_view: DwgOrthographicView,
    pub shade_plot_mode: DwgShadePlotMode,
    pub use_default_lights: bool,
    pub default_lighting_type: DwgDefaultLightingType,
    pub brightness: f64,
    pub contrast: f64,
    pub ambient_color: DwgComplexColor,
    pub clip_boundary_handle: Option<u64>,
    pub named_ucs_handle: Option<u64>,
    pub base_ucs_handle: Option<u64>,
    pub background_handle: Option<u64>,
    pub visual_style_handle: Option<u64>,
    pub shade_plot_handle: Option<u64>,
    pub sun_handle: Option<u64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgPointEntity {
    pub common: DwgEntityCommon,
    pub point: Vec<f64>,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
    pub x_axis_angle: f64,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgCircleEntity {
    pub common: DwgEntityCommon,
    pub center: Vec<f64>,
    pub radius: f64,
    pub thickness: f64,
    pub extrusion: Vec<f64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgEllipseEntity {
    pub common: DwgEntityCommon,
    pub center: Vec<f64>,
    pub major_axis: Vec<f64>,
    pub extrusion: Vec<f64>,
    pub axis_ratio: f64,
    pub start_parameter: f64,
    pub end_parameter: f64,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgTextEntity {
    pub common: DwgEntityCommon,
    pub elevation: f64,
    pub insertion: Vec<f64>,
    #[value(default)]
    pub alignment: Option<Vec<f64>>,
    pub extrusion: Vec<f64>,
    pub thickness: f64,
    pub oblique_angle: f64,
    pub rotation: f64,
    pub height: f64,
    pub width_factor: f64,
    pub value: String,
    pub generation: u16,
    pub horizontal_alignment: u16,
    pub vertical_alignment: u16,
    pub style_handle: u64,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgSplineEntity {
    pub common: DwgEntityCommon,
    pub degree: u32,
    pub rational: bool,
    pub closed: bool,
    pub periodic: bool,
    pub knot_tolerance: f64,
    pub control_tolerance: f64,
    #[value(default)]
    pub knots: Vec<f64>,
    #[value(default)]
    pub control_points: Vec<f64>,
    #[value(default)]
    pub weights: Vec<f64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgFace3dEntity {
    pub common: DwgEntityCommon,
    pub corners: Vec<f64>,
    pub invisible_edges: u16,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgPolyline3dEntity {
    pub common: DwgEntityCommon,
    pub curve_type: u8,
    pub flags: u8,
    #[value(default)]
    pub vertex_handles: Vec<u64>,
    pub sequence_end_handle: u64,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgPolyfaceMeshEntity {
    pub common: DwgEntityCommon,
    pub vertex_count: u16,
    pub face_count: u16,
    #[value(default)]
    pub vertex_handles: Vec<u64>,
    pub sequence_end_handle: u64,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgVertexEntity {
    pub common: DwgEntityCommon,
    pub flags: u8,
    pub point: Vec<f64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgPolyfaceFaceEntity {
    pub common: DwgEntityCommon,
    pub indices: Vec<i16>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgSequenceEndEntity {
    pub common: DwgEntityCommon,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum DwgEntityBody {
    Line(DwgLineEntity),
    Arc(DwgArcEntity),
    LwPolyline(DwgLwPolylineEntity),
    BlockBegin(DwgBlockBeginEntity),
    BlockEnd(DwgBlockEndEntity),
    Insert(DwgInsertEntity),
    DimensionLinear(DwgLinearDimensionEntity),
    Viewport(DwgViewportEntity),
    Point(DwgPointEntity),
    Circle(DwgCircleEntity),
    Ellipse(DwgEllipseEntity),
    Text(DwgTextEntity),
    Spline(DwgSplineEntity),
    Face3d(DwgFace3dEntity),
    Polyline3d(DwgPolyline3dEntity),
    PolyfaceMesh(DwgPolyfaceMeshEntity),
    Vertex(DwgVertexEntity),
    PolyfaceFace(DwgPolyfaceFaceEntity),
    SequenceEnd(DwgSequenceEndEntity),
}
impl DwgEntityBody {
    /// 🧬️ The common entity data every body carries.
    pub fn common_mut(&mut self) -> &mut DwgEntityCommon {
        match self {
            Self::Line(value) => &mut value.common,
            Self::Arc(value) => &mut value.common,
            Self::LwPolyline(value) => &mut value.common,
            Self::BlockBegin(value) => &mut value.common,
            Self::BlockEnd(value) => &mut value.common,
            Self::Insert(value) => &mut value.common,
            Self::DimensionLinear(value) => &mut value.dimension.common,
            Self::Viewport(value) => &mut value.common,
            Self::Point(value) => &mut value.common,
            Self::Circle(value) => &mut value.common,
            Self::Ellipse(value) => &mut value.common,
            Self::Text(value) => &mut value.common,
            Self::Spline(value) => &mut value.common,
            Self::Face3d(value) => &mut value.common,
            Self::Polyline3d(value) => &mut value.common,
            Self::PolyfaceMesh(value) => &mut value.common,
            Self::Vertex(value) => &mut value.common,
            Self::PolyfaceFace(value) => &mut value.common,
            Self::SequenceEnd(value) => &mut value.common,
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
dwg_metadata!(dwg_entity_body_spec, dwg_entity_body_spec_controlled, dwg_entity_body_spec_producer => DwgEntityBody;
    0, "kind", false, [enum ("line", 0), ("arc", 1), ("lwPolyline", 2), ("blockBegin", 3), ("blockEnd", 4), ("insert", 5), ("dimensionLinear", 6), ("viewport", 7), ("point", 8), ("circle", 9), ("ellipse", 10), ("text", 11), ("spline", 12), ("face3d", 13), ("polyline3d", 14), ("polyfaceMesh", 15), ("vertex", 16), ("polyfaceFace", 17), ("sequenceEnd", 18)];
    1, "line", true, [typed DwgLineEntity];
    2, "arc", true, [typed DwgArcEntity];
    3, "lw_polyline", true, [typed DwgLwPolylineEntity];
    4, "block_begin", true, [typed DwgBlockBeginEntity];
    5, "block_end", true, [typed DwgBlockEndEntity];
    6, "insert", true, [typed DwgInsertEntity];
    7, "dimension_linear", true, [typed DwgLinearDimensionEntity];
    8, "viewport", true, [typed DwgViewportEntity];
    9, "point", true, [typed DwgPointEntity];
    10, "circle", true, [typed DwgCircleEntity];
    11, "ellipse", true, [typed DwgEllipseEntity];
    12, "text", true, [typed DwgTextEntity];
    13, "spline", true, [typed DwgSplineEntity];
    14, "face3d", true, [typed DwgFace3dEntity];
    15, "polyline3d", true, [typed DwgPolyline3dEntity];
    16, "polyface_mesh", true, [typed DwgPolyfaceMeshEntity];
    17, "vertex", true, [typed DwgVertexEntity];
    18, "polyface_face", true, [typed DwgPolyfaceFaceEntity];
    19, "sequence_end", true, [typed DwgSequenceEndEntity];
);
impl semio_framework_dsl_record::DslField for DwgEntityBody {
    dwg_controlled_payloads!(0; units[];
        0,1,Line,DwgLineEntity;
        1,2,Arc,DwgArcEntity;
        2,3,LwPolyline,DwgLwPolylineEntity;
        3,4,BlockBegin,DwgBlockBeginEntity;
        4,5,BlockEnd,DwgBlockEndEntity;
        5,6,Insert,DwgInsertEntity;
        6,7,DimensionLinear,DwgLinearDimensionEntity;
        7,8,Viewport,DwgViewportEntity;
        8,9,Point,DwgPointEntity;
        9,10,Circle,DwgCircleEntity;
        10,11,Ellipse,DwgEllipseEntity;
        11,12,Text,DwgTextEntity;
        12,13,Spline,DwgSplineEntity;
        13,14,Face3d,DwgFace3dEntity;
        14,15,Polyline3d,DwgPolyline3dEntity;
        15,16,PolyfaceMesh,DwgPolyfaceMeshEntity;
        16,17,Vertex,DwgVertexEntity;
        17,18,PolyfaceFace,DwgPolyfaceFaceEntity;
        18,19,SequenceEnd,DwgSequenceEndEntity;);

    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(dwg_entity_body_spec_producer())
    }

    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(dwg_entity_body_spec_producer()))
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        match self {
            Self::Line(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(0));
                record.fields.insert(1, <DwgLineEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Arc(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(1));
                record.fields.insert(2, <DwgArcEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::LwPolyline(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(2));
                record.fields.insert(3, <DwgLwPolylineEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::BlockBegin(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(3));
                record.fields.insert(4, <DwgBlockBeginEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::BlockEnd(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(4));
                record.fields.insert(5, <DwgBlockEndEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Insert(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(5));
                record.fields.insert(6, <DwgInsertEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::DimensionLinear(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(6));
                record.fields.insert(7, <DwgLinearDimensionEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Viewport(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(7));
                record.fields.insert(8, <DwgViewportEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Point(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(8));
                record.fields.insert(9, <DwgPointEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Circle(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(9));
                record.fields.insert(10, <DwgCircleEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Ellipse(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(10));
                record.fields.insert(11, <DwgEllipseEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Text(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(11));
                record.fields.insert(12, <DwgTextEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Spline(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(12));
                record.fields.insert(13, <DwgSplineEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Face3d(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(13));
                record.fields.insert(14, <DwgFace3dEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Polyline3d(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(14));
                record.fields.insert(15, <DwgPolyline3dEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::PolyfaceMesh(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(15));
                record.fields.insert(16, <DwgPolyfaceMeshEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::Vertex(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(16));
                record.fields.insert(17, <DwgVertexEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::PolyfaceFace(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(17));
                record.fields.insert(18, <DwgPolyfaceFaceEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
            Self::SequenceEnd(value) => {
                record.fields.insert(0, semio_framework_dsl_record::FieldValue::Enum(18));
                record.fields.insert(19, <DwgSequenceEndEntity as semio_framework_dsl_record::DslField>::to_value(value));
            }
        }
        semio_framework_dsl_record::FieldValue::Record(record)
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else { return Err("expected entity-body record".into()) };
        match record.get(0) {
            Some(semio_framework_dsl_record::FieldValue::Enum(0)) => Ok(Self::Line(<DwgLineEntity as semio_framework_dsl_record::DslField>::from_value(record.get(1).ok_or("LINE body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(1)) => Ok(Self::Arc(<DwgArcEntity as semio_framework_dsl_record::DslField>::from_value(record.get(2).ok_or("ARC body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(2)) => Ok(Self::LwPolyline(<DwgLwPolylineEntity as semio_framework_dsl_record::DslField>::from_value(record.get(3).ok_or("LWPOLYLINE body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(3)) => Ok(Self::BlockBegin(<DwgBlockBeginEntity as semio_framework_dsl_record::DslField>::from_value(record.get(4).ok_or("BLOCK body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(4)) => Ok(Self::BlockEnd(<DwgBlockEndEntity as semio_framework_dsl_record::DslField>::from_value(record.get(5).ok_or("ENDBLK body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(5)) => Ok(Self::Insert(<DwgInsertEntity as semio_framework_dsl_record::DslField>::from_value(record.get(6).ok_or("INSERT body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(6)) => Ok(Self::DimensionLinear(<DwgLinearDimensionEntity as semio_framework_dsl_record::DslField>::from_value(record.get(7).ok_or("DIMENSION_LINEAR body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(7)) => Ok(Self::Viewport(<DwgViewportEntity as semio_framework_dsl_record::DslField>::from_value(record.get(8).ok_or("VIEWPORT body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(8)) => Ok(Self::Point(<DwgPointEntity as semio_framework_dsl_record::DslField>::from_value(record.get(9).ok_or("POINT body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(9)) => Ok(Self::Circle(<DwgCircleEntity as semio_framework_dsl_record::DslField>::from_value(record.get(10).ok_or("CIRCLE body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(10)) => Ok(Self::Ellipse(<DwgEllipseEntity as semio_framework_dsl_record::DslField>::from_value(record.get(11).ok_or("ELLIPSE body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(11)) => Ok(Self::Text(<DwgTextEntity as semio_framework_dsl_record::DslField>::from_value(record.get(12).ok_or("TEXT body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(12)) => Ok(Self::Spline(<DwgSplineEntity as semio_framework_dsl_record::DslField>::from_value(record.get(13).ok_or("SPLINE body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(13)) => Ok(Self::Face3d(<DwgFace3dEntity as semio_framework_dsl_record::DslField>::from_value(record.get(14).ok_or("3DFACE body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(14)) => Ok(Self::Polyline3d(<DwgPolyline3dEntity as semio_framework_dsl_record::DslField>::from_value(record.get(15).ok_or("POLYLINE_3D body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(15)) => Ok(Self::PolyfaceMesh(<DwgPolyfaceMeshEntity as semio_framework_dsl_record::DslField>::from_value(record.get(16).ok_or("POLYLINE_PFACE body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(16)) => Ok(Self::Vertex(<DwgVertexEntity as semio_framework_dsl_record::DslField>::from_value(record.get(17).ok_or("VERTEX body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(17)) => Ok(Self::PolyfaceFace(<DwgPolyfaceFaceEntity as semio_framework_dsl_record::DslField>::from_value(record.get(18).ok_or("VERTEX_PFACE_FACE body missing")?)?)),
            Some(semio_framework_dsl_record::FieldValue::Enum(18)) => Ok(Self::SequenceEnd(<DwgSequenceEndEntity as semio_framework_dsl_record::DslField>::from_value(record.get(19).ok_or("SEQEND body missing")?)?)),
            other => Err(format!("unknown entity-body kind {other:?}")),
        }
    }
}
//#endregion 🔖️DrawingModel
