//! 🔧️ Header variables: units, modes, settings, scalars, time state, space geometry, dimension settings and relations.

use crate::standards::v_ac1024::subsets::any::schema::snapshot::document::{DwgJulianDate};

//#region 🔖️DocumentModel
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgHeaderUnits {
    pub unit1_conversion: f64,
    pub unit2_conversion: f64,
    pub unit3_conversion: f64,
    pub unit4_conversion: f64,
    pub unit1_name: String,
    pub unit2_name: String,
    pub unit3_name: String,
    pub unit4_name: String,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgHeaderModes {
    pub dimension_associative: bool,
    pub dimension_show: bool,
    pub polyline_generation: bool,
    pub orthographic_mode: bool,
    pub regeneration_mode: bool,
    pub fill_mode: bool,
    pub quick_text_mode: bool,
    pub paper_space_linetype_scale: bool,
    pub limits_check: bool,
    pub user_timer: bool,
    pub sketch_polyline: bool,
    pub angle_direction: bool,
    pub spline_frame: bool,
    pub mirror_text: bool,
    pub world_view: bool,
    pub tile_mode: bool,
    pub paper_limits_check: bool,
    pub visual_retain: bool,
    pub display_silhouette: bool,
    pub polyline_ellipse: bool,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgHeaderIntegerSettings {
    pub proxy_graphics: u16,
    pub tree_depth: i16,
    pub linear_units: u16,
    pub linear_precision: u16,
    pub angular_units: u16,
    pub angular_precision: u16,
    pub attribute_mode: u16,
    pub point_display_mode: u16,
    pub user_integer1: i16,
    pub user_integer2: i16,
    pub user_integer3: i16,
    pub user_integer4: i16,
    pub user_integer5: i16,
    pub spline_segments: u16,
    pub surface_u: u16,
    pub surface_v: u16,
    pub surface_type: u16,
    pub surface_tab1: u16,
    pub surface_tab2: u16,
    pub spline_type: u16,
    pub shade_edge: u16,
    pub shade_difference: u16,
    pub unit_mode: u16,
    pub maximum_active_viewports: u16,
    pub isolines: u16,
    pub multiline_justification: u16,
    pub text_quality: u16,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgHeaderScalars {
    pub linetype_scale: f64,
    pub text_size: f64,
    pub trace_width: f64,
    pub sketch_increment: f64,
    pub fillet_radius: f64,
    pub thickness: f64,
    pub angle_base: f64,
    pub point_display_size: f64,
    pub polyline_width: f64,
    pub user_real1: f64,
    pub user_real2: f64,
    pub user_real3: f64,
    pub user_real4: f64,
    pub user_real5: f64,
    pub chamfer_a: f64,
    pub chamfer_b: f64,
    pub chamfer_c: f64,
    pub chamfer_d: f64,
    pub facet_resolution: f64,
    pub multiline_scale: f64,
    pub current_entity_linetype_scale: f64,
    pub current_entity_color_index: u16,
    pub paper_space_viewport_scale: f64,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgHeaderTimeState {
    pub created_at: DwgJulianDate,
    pub updated_at: DwgJulianDate,
    pub editing_duration: DwgJulianDate,
    pub user_timer_duration: DwgJulianDate,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgHeaderSpaceGeometry {
    pub insertion_base: Vec<f64>,
    pub extents_minimum: Vec<f64>,
    pub extents_maximum: Vec<f64>,
    pub limits_minimum: Vec<f64>,
    pub limits_maximum: Vec<f64>,
    pub elevation: f64,
    pub ucs_origin: Vec<f64>,
    pub ucs_x_axis: Vec<f64>,
    pub ucs_y_axis: Vec<f64>,
    pub ucs_orthographic_view: u16,
    pub ucs_origin_top: Vec<f64>,
    pub ucs_origin_bottom: Vec<f64>,
    pub ucs_origin_left: Vec<f64>,
    pub ucs_origin_right: Vec<f64>,
    pub ucs_origin_front: Vec<f64>,
    pub ucs_origin_back: Vec<f64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgDimensionSettings {
    pub scale: f64,
    pub arrow_size: f64,
    pub extension_offset: f64,
    pub line_increment: f64,
    pub extension: f64,
    pub rounding: f64,
    pub line_extension: f64,
    pub tolerance_plus: f64,
    pub tolerance_minus: f64,
    pub fixed_extension_length: f64,
    pub jog_angle: f64,
    pub text_fill: u16,
    pub text_fill_color_index: u16,
    pub tolerance: bool,
    pub limits: bool,
    pub text_inside_horizontal: bool,
    pub text_outside_horizontal: bool,
    pub suppress_extension1: bool,
    pub suppress_extension2: bool,
    pub text_above: u16,
    pub zero_suppression: u16,
    pub angular_zero_suppression: u16,
    pub arc_symbol: u16,
    pub text_height: f64,
    pub center_mark: f64,
    pub tick_size: f64,
    pub alternate_scale: f64,
    pub linear_factor: f64,
    pub text_vertical_position: f64,
    pub text_factor: f64,
    pub gap: f64,
    pub alternate_rounding: f64,
    pub alternate_units: bool,
    pub alternate_decimal_places: u16,
    pub text_outside_force_line: bool,
    pub separate_arrows: bool,
    pub text_inside: bool,
    pub suppress_outside: bool,
    pub line_color_index: u16,
    pub extension_color_index: u16,
    pub text_color_index: u16,
    pub angular_decimal_places: u16,
    pub decimal_places: u16,
    pub tolerance_decimal_places: u16,
    pub alternate_units_format: u16,
    pub alternate_tolerance_decimal_places: u16,
    pub angular_unit_format: u16,
    pub fractional_format: u16,
    pub linear_unit_format: u16,
    pub decimal_separator: u16,
    pub text_movement: u16,
    pub justification: u16,
    pub suppress_dimension1: bool,
    pub suppress_dimension2: bool,
    pub tolerance_justification: u16,
    pub tolerance_zero_suppression: u16,
    pub alternate_zero_suppression: u16,
    pub alternate_tolerance_zero_suppression: u16,
    pub user_positioned_text: bool,
    pub fit: u16,
    pub fixed_extension_enabled: bool,
    pub text_direction: bool,
    pub alternate_measurement_scale: f64,
    pub measurement_scale: f64,
    pub dimension_line_weight: i16,
    pub extension_line_weight: i16,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgDrawingPolicy {
    pub text_stack_alignment: u16,
    pub text_stack_size: u16,
    pub current_entity_lineweight: i16,
    pub end_caps: u8,
    pub join_style: u8,
    pub lineweight_display: bool,
    pub external_reference_editing: bool,
    pub extended_names: bool,
    pub plot_style_mode: bool,
    pub ole_startup: bool,
    pub insertion_units: u16,
    pub current_plot_style_type: u16,
    pub sort_entities: u8,
    pub index_control: u8,
    pub hide_text: u8,
    pub xclip_frame: u8,
    pub dimension_association: u8,
    pub halo_gap: u8,
    pub obscured_color: u16,
    pub intersection_color: u16,
    pub obscured_linetype: u8,
    pub intersection_display: u8,
    pub camera_display: bool,
    pub steps_per_second: f64,
    pub step_size: f64,
    pub dwf_3d_precision: f64,
    pub lens_length: f64,
    pub camera_height: f64,
    pub solid_history: u8,
    pub show_history: u8,
    pub polysolid_width: f64,
    pub polysolid_height: f64,
    pub loft_angle1: f64,
    pub loft_angle2: f64,
    pub loft_magnitude1: f64,
    pub loft_magnitude2: f64,
    pub loft_parameter: u16,
    pub loft_normals: u8,
    pub latitude: f64,
    pub longitude: f64,
    pub north_direction: f64,
    pub timezone: i32,
    pub light_glyph_display: u8,
    pub tile_mode_light_sync: u8,
    pub dwf_frame: u8,
    pub dgn_frame: u8,
    pub real_world_scale: bool,
    pub interfere_color_index: u16,
    pub shadow_mode: u8,
    pub shadow_plane_location: f64,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgHeaderStrings {
    pub menu: String,
    pub dimension_postfix: String,
    pub dimension_alternate_postfix: String,
    pub dimension_alternate_measurement_zero_suffix: String,
    pub dimension_measurement_zero_suffix: String,
    pub hyperlink_base: String,
    pub stylesheet: String,
    pub fingerprint_guid: String,
    pub version_guid: String,
    pub project_name: String,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgHeaderRelations {
    pub handle_seed: u64,
    pub current_layer: u64,
    pub text_style: u64,
    pub current_linetype: u64,
    pub current_material: u64,
    pub dimension_style: u64,
    pub multiline_style: u64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub paper_ucs_name: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub paper_ucs_orthographic_reference: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub paper_ucs_base: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub model_ucs_name: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub model_ucs_orthographic_reference: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub model_ucs_base: Option<u64>,
    pub dimension_text_style: u64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dimension_leader_block: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dimension_block: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dimension_block1: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dimension_block2: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dimension_linetype: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dimension_extension_linetype1: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dimension_extension_linetype2: Option<u64>,
    pub block_control: u64,
    pub layer_control: u64,
    pub style_control: u64,
    pub linetype_control: u64,
    pub view_control: u64,
    pub ucs_control: u64,
    pub viewport_control: u64,
    pub appid_control: u64,
    pub dimension_style_control: u64,
    pub group_dictionary: u64,
    pub multiline_style_dictionary: u64,
    pub named_objects_dictionary: u64,
    pub layout_dictionary: u64,
    pub plot_settings_dictionary: u64,
    pub plot_style_name_dictionary: u64,
    pub material_dictionary: u64,
    pub color_dictionary: u64,
    pub visual_style_dictionary: u64,
    pub paper_space_block_record: u64,
    pub model_space_block_record: u64,
    pub by_layer_linetype: u64,
    pub by_block_linetype: u64,
    pub continuous_linetype: u64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub interfere_object_visual_style: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub interfere_viewport_visual_style: Option<u64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub drag_visual_style: Option<u64>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct DwgHeaderVariables {
    pub units: DwgHeaderUnits,
    pub modes: DwgHeaderModes,
    pub integers: DwgHeaderIntegerSettings,
    pub scalars: DwgHeaderScalars,
    pub time: DwgHeaderTimeState,
    pub paper_space: DwgHeaderSpaceGeometry,
    pub model_space: DwgHeaderSpaceGeometry,
    pub dimensions: DwgDimensionSettings,
    pub policy: DwgDrawingPolicy,
    pub strings: DwgHeaderStrings,
    pub relations: DwgHeaderRelations,
}
//#endregion 🔖️DocumentModel
