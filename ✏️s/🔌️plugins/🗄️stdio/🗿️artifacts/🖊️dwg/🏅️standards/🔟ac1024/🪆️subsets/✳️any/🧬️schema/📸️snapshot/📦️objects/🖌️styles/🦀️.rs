//! 🖌️ Styles: visual styles, table and cell styles, multiline styles, multileader styles and materials.

use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::{DwgComplexColor};

//#region 🔖️DrawingModel
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgVisualStylePropertyOperation {
    Inherit,
    #[default]
    Set,
    Disable,
    Enable,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgVisualStyleProperty<T> {
    pub value: T,
    pub operation: DwgVisualStylePropertyOperation,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
dwg_metadata!(dwg_visual_style_property_spec<T>, dwg_visual_style_property_spec_controlled, dwg_visual_style_property_spec_producer => DwgVisualStyleProperty;
    0, "value", false, [typed T];
    1, "operation", false, [typed DwgVisualStylePropertyOperation];
);
impl<T: semio_framework_dsl_record::DslField> semio_framework_dsl_record::DslField for DwgVisualStyleProperty<T> {
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{control.scoped_stage(|control|{
        control.begin_stage(2)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(2,control)?;
        record.insert(0,control.scoped_stage(|control|{control.begin_stage(0)?;T::to_value_controlled(&self.value,control)})?)?;control.step()?;
        record.insert(1,control.scoped_stage(|control|{control.begin_stage(0)?;<DwgVisualStylePropertyOperation as semio_framework_dsl_record::DslField>::to_value_controlled(&self.operation,control)})?)?;control.step()?;
        Ok(semio_framework_dsl_record::FieldValue::Record(record.take()))
    })}
    fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,ValueError>{let semio_framework_dsl_record::FieldValue::Record(record)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG visual-style property requires a record"))};Self::from_record_controlled(record,control)}
    fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_stage(|control|{
        control.begin_stage(record.fields.len())?;for(id,value)in &record.fields{if *id>1&&!matches!(value,semio_framework_dsl_record::FieldValue::Absent){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG visual-style property has an unowned field"))}control.step()?;}
        let value=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.scoped_stage(|control|{control.begin_stage(0)?;T::from_value_controlled(record.get(0).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DWG property value missing"))?,control)})?,T::retire_decoded);
        let operation=control.scoped_stage(|control|{control.begin_stage(0)?;<DwgVisualStylePropertyOperation as semio_framework_dsl_record::DslField>::from_value_controlled(record.get(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DWG property operation missing"))?,control)})?;
        Ok(Self{value:value.take(),operation})
    })}
    fn retire_decoded(self){T::retire_decoded(self.value);<DwgVisualStylePropertyOperation as semio_framework_dsl_record::DslField>::retire_decoded(self.operation);}
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Record(dwg_visual_style_property_spec_producer::<T>())
    }

    fn shape_controlled<C: semio_framework_dsl_record::NativeSchemaControl>(control: &mut C) -> Result<semio_framework_dsl_record::Shape, ValueError> {
        control.checkpoint()?;
        Ok(semio_framework_dsl_record::Shape::Record(dwg_visual_style_property_spec_producer::<T>()))
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        let mut record = semio_framework_dsl_record::RecordValue::default();
        record.fields.insert(0, T::to_value(&self.value));
        record.fields.insert(1, <DwgVisualStylePropertyOperation as semio_framework_dsl_record::DslField>::to_value(&self.operation));
        semio_framework_dsl_record::FieldValue::Record(record)
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Record(record) = value else { return Err("expected visual-style property record".into()) };
        Ok(Self { value: T::from_value(record.get(0).ok_or("visual-style property value missing")?)?, operation: <DwgVisualStylePropertyOperation as semio_framework_dsl_record::DslField>::from_value(record.get(1).ok_or("visual-style property operation missing")?)? })
    }
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgVisualStyleProperties {
    pub face_lighting_model: DwgVisualStyleProperty<u32>,
    pub face_lighting_quality: DwgVisualStyleProperty<u32>,
    pub face_color_mode: DwgVisualStyleProperty<u32>,
    pub face_modifiers: DwgVisualStyleProperty<u16>,
    pub face_opacity: DwgVisualStyleProperty<f64>,
    pub face_specular_amount: DwgVisualStyleProperty<f64>,
    pub face_monochrome_color: DwgVisualStyleProperty<DwgComplexColor>,
    pub edge_model: DwgVisualStyleProperty<u32>,
    pub edge_styles: DwgVisualStyleProperty<u32>,
    pub edge_intersection_color: DwgVisualStyleProperty<DwgComplexColor>,
    pub edge_obscured_color: DwgVisualStyleProperty<DwgComplexColor>,
    pub edge_obscured_line_pattern: DwgVisualStyleProperty<u32>,
    pub edge_intersection_line_pattern: DwgVisualStyleProperty<u32>,
    pub edge_crease_angle: DwgVisualStyleProperty<f64>,
    pub edge_modifiers: DwgVisualStyleProperty<u32>,
    pub edge_color: DwgVisualStyleProperty<DwgComplexColor>,
    pub edge_opacity: DwgVisualStyleProperty<f64>,
    pub edge_width: DwgVisualStyleProperty<u32>,
    pub edge_overhang: DwgVisualStyleProperty<u32>,
    pub edge_jitter: DwgVisualStyleProperty<u32>,
    pub edge_silhouette_color: DwgVisualStyleProperty<DwgComplexColor>,
    pub edge_silhouette_width: DwgVisualStyleProperty<u32>,
    pub edge_halo_gap: DwgVisualStyleProperty<u32>,
    pub edge_isolines: DwgVisualStyleProperty<u32>,
    pub hidden_edge_precision: DwgVisualStyleProperty<bool>,
    pub display_settings: DwgVisualStyleProperty<u32>,
    pub display_brightness: DwgVisualStyleProperty<f64>,
    pub display_shadow_type: DwgVisualStyleProperty<u32>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgVisualStyle {
    pub description: String,
    pub style_type: u32,
    pub extension_lighting_model: u16,
    pub internal_only: bool,
    pub properties: DwgVisualStyleProperties,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgCellContentFormat {
    pub property_override_flags: u32,
    pub property_flags: u32,
    pub value_data_type: u32,
    pub value_unit_type: u32,
    pub value_format_string: String,
    pub rotation: f64,
    pub block_scale: f64,
    pub alignment: u32,
    pub content_color: DwgComplexColor,
    pub text_style_handle: Option<u64>,
    pub text_height: f64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgCellMargins {
    pub vertical: f64,
    pub horizontal: f64,
    pub bottom: f64,
    pub right: f64,
    pub horizontal_spacing: f64,
    pub vertical_spacing: f64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgCellBorder {
    pub override_flags: u32,
    pub border_type: u32,
    pub color: DwgComplexColor,
    pub lineweight: i32,
    pub linetype_handle: Option<u64>,
    pub visible: u32,
    pub double_line_spacing: f64,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgCellBorders {
    pub top: Option<DwgCellBorder>,
    pub horizontal_inside: Option<DwgCellBorder>,
    pub bottom: Option<DwgCellBorder>,
    pub left: Option<DwgCellBorder>,
    pub vertical_inside: Option<DwgCellBorder>,
    pub right: Option<DwgCellBorder>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgCellStyle {
    pub property_override_flags: u32,
    pub merge_flags: u32,
    pub background_color: DwgComplexColor,
    pub content_layout: u32,
    pub content_format: DwgCellContentFormat,
    pub margins: DwgCellMargins,
    pub borders: DwgCellBorders,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgTableStyle {
    pub description: String,
    pub bit_flags: u32,
    pub template_style_handle: Option<u64>,
    pub table: DwgCellStyle,
    pub title: DwgCellStyle,
    pub header: DwgCellStyle,
    pub data: DwgCellStyle,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMlineLinetype {
    #[default]
    ByLayer,
    ByBlock,
    Continuous,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMlineCaps {
    pub square: bool,
    pub inner_arcs: bool,
    pub round_outer_arcs: bool,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMlineStyleElement {
    pub offset: f64,
    pub color: DwgComplexColor,
    pub linetype: DwgMlineLinetype,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMlineStyle {
    pub name: String,
    pub description: String,
    pub fill_enabled: bool,
    pub display_miters: bool,
    pub start_caps: DwgMlineCaps,
    pub end_caps: DwgMlineCaps,
    pub fill_color: DwgComplexColor,
    pub start_angle: f64,
    pub end_angle: f64,
    pub elements: Vec<DwgMlineStyleElement>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderContentType {
    None,
    Block,
    #[default]
    MText,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderDrawOrder {
    #[default]
    LeaderFirst,
    ContentFirst,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderLeaderOrder {
    #[default]
    HeadFirst,
    TailFirst,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderKind {
    Invisible,
    #[default]
    Straight,
    Spline,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderTextAttachment {
    TopOfTop,
    #[default]
    MiddleOfTop,
    Middle,
    MiddleOfBottom,
    BottomOfBottom,
    BottomLine,
    BottomOfTop,
    BottomOfTopUnderline,
    BottomOfTopNoUnderline,
    Center,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderTextAngle {
    #[default]
    Horizontal,
    Aligned,
    AlwaysRightReading,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderTextAlignment {
    #[default]
    Left,
    Center,
    Right,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderAttachmentDirection {
    #[default]
    Horizontal,
    Vertical,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMLeaderBlockConnection {
    #[default]
    Extents,
    BasePoint,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMLeaderLeaderStyle {
    pub kind: DwgMLeaderKind,
    pub color: DwgComplexColor,
    pub linetype_style_handle: u64,
    pub lineweight: i32,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMLeaderLanding {
    pub enabled: bool,
    pub gap: f64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMLeaderDogleg {
    pub enabled: bool,
    pub length: f64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMLeaderArrow {
    pub symbol_handle: Option<u64>,
    pub size: f64,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMLeaderTextStyle {
    pub default_content: String,
    pub style_handle: u64,
    pub left_attachment: DwgMLeaderTextAttachment,
    pub right_attachment: DwgMLeaderTextAttachment,
    pub angle: DwgMLeaderTextAngle,
    pub alignment: DwgMLeaderTextAlignment,
    pub color: DwgComplexColor,
    pub height: f64,
    pub frame: bool,
    pub always_left: bool,
    pub alignment_space: f64,
    pub attachment_direction: DwgMLeaderAttachmentDirection,
    pub top_attachment: DwgMLeaderTextAttachment,
    pub bottom_attachment: DwgMLeaderTextAttachment,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMLeaderBlockStyle {
    pub content_handle: Option<u64>,
    pub color: DwgComplexColor,
    pub scale: Vec<f64>,
    pub use_scale: bool,
    pub rotation: f64,
    pub use_rotation: bool,
    pub connection: DwgMLeaderBlockConnection,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMLeaderStyle {
    pub content_type: DwgMLeaderContentType,
    pub draw_order: DwgMLeaderDrawOrder,
    pub leader_order: DwgMLeaderLeaderOrder,
    pub maximum_segment_points: u32,
    pub first_segment_angle: f64,
    pub second_segment_angle: f64,
    pub leader: DwgMLeaderLeaderStyle,
    pub landing: DwgMLeaderLanding,
    pub dogleg: DwgMLeaderDogleg,
    pub description: String,
    pub arrow: DwgMLeaderArrow,
    pub text: DwgMLeaderTextStyle,
    pub block: DwgMLeaderBlockStyle,
    pub overall_scale: f64,
    pub property_overrides_changed: bool,
    pub annotative: bool,
    pub break_size: f64,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMaterialProjection {
    Inherit,
    Planar,
    #[default]
    Box,
    Cylinder,
    Sphere,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMaterialTiling {
    Inherit,
    #[default]
    Tile,
    Crop,
    Clamp,
    Mirror,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum DwgMaterialMapSource {
    #[default]
    None,
    CurrentScene,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMaterialColor {
    pub factor: f64,
    pub override_rgb: Option<u32>,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMaterialMap {
    pub blend_factor: f64,
    pub projection: DwgMaterialProjection,
    pub tiling: DwgMaterialTiling,
    pub scale_to_entity: bool,
    pub use_current_block_transform: bool,
    pub transform: Vec<f64>,
    pub source: DwgMaterialMapSource,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMaterialChannels {
    pub diffuse: bool,
    pub specular: bool,
    pub reflection: bool,
    pub opacity: bool,
    pub bump: bool,
    pub refraction: bool,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgMaterial {
    pub name: String,
    pub description: String,
    pub ambient: DwgMaterialColor,
    pub diffuse: DwgMaterialColor,
    pub diffuse_map: DwgMaterialMap,
    pub specular: DwgMaterialColor,
    pub specular_map: DwgMaterialMap,
    pub specular_gloss: f64,
    pub reflection_map: DwgMaterialMap,
    pub opacity: f64,
    pub opacity_map: DwgMaterialMap,
    pub bump_map: DwgMaterialMap,
    pub refraction_index: f64,
    pub refraction_map: DwgMaterialMap,
    pub translucence: f64,
    pub self_illumination: f64,
    pub reflectivity: f64,
    pub enabled_channels: DwgMaterialChannels,
}
//#endregion 🔖️DrawingModel
