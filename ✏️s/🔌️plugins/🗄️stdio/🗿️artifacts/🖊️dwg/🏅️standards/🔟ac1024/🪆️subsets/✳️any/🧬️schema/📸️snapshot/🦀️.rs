//! 🧬️ DwgSnapshot schema — persistent fields + real codecs. Byte/bit-level decode logic (file
//! header decrypt, R2004+ LZ77-variant decompression, section/page directory walk) lives in
//! `⚙️engine` (ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION 🖊️dwg
//! D1-D2 wave); this module owns the typed persisted model and glues `decode_dwg`/`encode_dwg`
//! to it.

use framework_schema::ArtifactSchema;
use std::fmt;
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DWG_NEW_DOCUMENT_MAINTENANCE_VERSION;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationHistory;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationInfo;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgAuxiliaryHeader;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgClass;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgDependency;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgIndexedPreview;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgJulianDate;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgRevisionHistory;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgRgba;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgSummaryInfo;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgTemplate;
use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::DwgLogicalDrawing;
use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::new_document_handles;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgDimensionSettings;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgDrawingPolicy;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderIntegerSettings;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderModes;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderRelations;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderScalars;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderSpaceGeometry;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderStrings;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderTimeState;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderUnits;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderVariables;


#[cfg(test)]
#[path = "🧪️tests/🎛️metadata/🦀️.rs"]
mod controlled_metadata_tests;
//#region 🔖️DrawingModel
impl DwgSnapshot {
    /// 🆕️ A new AC1024 document holding `drawing`: the object graph of [`DwgLogicalDrawing::from_native`],
    /// header variables with AutoCAD 2010's defaults (their relations pointing at that graph, extents
    /// recomputed from the geometry), the classes of its three class-numbered objects, and empty
    /// file-level sections. Deterministic: the same drawing always yields the same document.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_drawing(drawing: &semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::DwgDrawing) -> Result<Self, String> {
        use new_document_handles as h;
        let mut drawing = drawing.clone();
        drawing.recompute_extents();
        let logical = DwgLogicalDrawing::from_native(&drawing)?;
        let handle_seed = logical.handle_seed();
        let epoch = DwgJulianDate { days: 2_440_588, milliseconds: 0 };
        let space = |extents_minimum: Vec<f64>, extents_maximum: Vec<f64>| DwgHeaderSpaceGeometry {
            insertion_base: vec![0.0; 3],
            extents_minimum,
            extents_maximum,
            limits_minimum: vec![0.0, 0.0],
            limits_maximum: vec![12.0, 9.0],
            elevation: 0.0,
            ucs_origin: vec![0.0; 3],
            ucs_x_axis: vec![1.0, 0.0, 0.0],
            ucs_y_axis: vec![0.0, 1.0, 0.0],
            ucs_orthographic_view: 0,
            ucs_origin_top: vec![0.0; 3],
            ucs_origin_bottom: vec![0.0; 3],
            ucs_origin_left: vec![0.0; 3],
            ucs_origin_right: vec![0.0; 3],
            ucs_origin_front: vec![0.0; 3],
            ucs_origin_back: vec![0.0; 3],
        };
        let digest = semio_framework_hash::Sha256::digest(format!("{drawing:?}").as_bytes());
        let guid = |bytes: &[u8]| format!("{{{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}", bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]);
        let class = |number: u16, cpp_class_name: &str, dxf_name: &str, object_count: u32| DwgClass {
            number,
            proxy_flags: 0,
            application_name: "ObjectDBX Classes".into(),
            cpp_class_name: cpp_class_name.into(),
            dxf_name: dxf_name.into(),
            was_zombie: false,
            item_class_id: 499,
            object_count,
            dwg_version: 22,
            maintenance_version: 42,
            reserved_values: vec![0, 0],
        };
        let header = DwgHeaderVariables {
            units: DwgHeaderUnits { unit1_conversion: 412_148_564_080.0, unit2_conversion: 1.0, unit3_conversion: 1.0, unit4_conversion: 1.0, unit1_name: "m".into(), ..Default::default() },
            modes: DwgHeaderModes { dimension_associative: true, dimension_show: true, regeneration_mode: true, fill_mode: true, paper_space_linetype_scale: true, user_timer: true, world_view: true, tile_mode: true, visual_retain: true, ..Default::default() },
            integers: DwgHeaderIntegerSettings {
                proxy_graphics: 1,
                tree_depth: 3020,
                linear_units: 2,
                linear_precision: 4,
                angular_units: 0,
                angular_precision: 0,
                attribute_mode: 1,
                spline_segments: 8,
                surface_u: 6,
                surface_v: 6,
                surface_type: 6,
                surface_tab1: 6,
                surface_tab2: 6,
                spline_type: 6,
                shade_edge: 3,
                shade_difference: 70,
                maximum_active_viewports: 64,
                isolines: 4,
                text_quality: 50,
                ..Default::default()
            },
            scalars: DwgHeaderScalars { linetype_scale: 1.0, text_size: 0.2, trace_width: 0.05, sketch_increment: 0.1, facet_resolution: 0.5, multiline_scale: 1.0, current_entity_linetype_scale: 1.0, current_entity_color_index: 256, ..Default::default() },
            time: DwgHeaderTimeState { created_at: epoch.clone(), updated_at: epoch.clone(), ..Default::default() },
            paper_space: space(vec![0.0; 3], vec![0.0; 3]),
            model_space: space(drawing.extmin.to_vec(), drawing.extmax.to_vec()),
            dimensions: DwgDimensionSettings {
                scale: 1.0,
                arrow_size: 0.18,
                extension_offset: 0.0625,
                line_increment: 0.38,
                extension: 0.18,
                fixed_extension_length: 1.0,
                jog_angle: std::f64::consts::FRAC_PI_4,
                text_inside_horizontal: true,
                text_outside_horizontal: true,
                text_height: 0.18,
                center_mark: 0.09,
                alternate_scale: 25.4,
                linear_factor: 1.0,
                text_factor: 1.0,
                gap: 0.09,
                alternate_decimal_places: 2,
                decimal_places: 4,
                tolerance_decimal_places: 4,
                alternate_units_format: 2,
                alternate_tolerance_decimal_places: 2,
                linear_unit_format: 2,
                decimal_separator: 46,
                tolerance_justification: 1,
                fit: 3,
                alternate_measurement_scale: 100.0,
                measurement_scale: 100.0,
                dimension_line_weight: -2,
                extension_line_weight: -2,
                ..Default::default()
            },
            policy: DwgDrawingPolicy {
                text_stack_alignment: 1,
                text_stack_size: 70,
                current_entity_lineweight: -1,
                external_reference_editing: true,
                extended_names: true,
                plot_style_mode: true,
                sort_entities: 127,
                hide_text: 1,
                dimension_association: 2,
                obscured_color: 257,
                intersection_color: 257,
                steps_per_second: 2.0,
                step_size: 6.0,
                dwf_3d_precision: 2.0,
                lens_length: 50.0,
                solid_history: 1,
                show_history: 1,
                polysolid_width: 0.25,
                polysolid_height: 4.0,
                loft_angle1: std::f64::consts::FRAC_PI_2,
                loft_angle2: std::f64::consts::FRAC_PI_2,
                loft_parameter: 7,
                loft_normals: 1,
                latitude: 37.795,
                longitude: -122.394,
                timezone: -8000,
                light_glyph_display: 1,
                tile_mode_light_sync: 1,
                dwf_frame: 2,
                real_world_scale: true,
                interfere_color_index: 256,
                ..Default::default()
            },
            strings: DwgHeaderStrings { menu: "acad".into(), fingerprint_guid: guid(&digest[..16]), version_guid: guid(&digest[16..]), ..Default::default() },
            relations: DwgHeaderRelations {
                handle_seed,
                current_layer: logical.layer_zero_handle()?,
                text_style: h::STANDARD_TEXT_STYLE,
                current_linetype: h::BY_LAYER,
                current_material: 0,
                dimension_style: h::STANDARD_DIMSTYLE,
                multiline_style: h::STANDARD_MLINE_STYLE,
                dimension_text_style: h::STANDARD_TEXT_STYLE,
                block_control: h::BLOCK_CONTROL,
                layer_control: h::LAYER_CONTROL,
                style_control: h::STYLE_CONTROL,
                linetype_control: h::LINETYPE_CONTROL,
                view_control: h::VIEW_CONTROL,
                ucs_control: h::UCS_CONTROL,
                viewport_control: h::VIEWPORT_CONTROL,
                appid_control: h::APPID_CONTROL,
                dimension_style_control: h::DIMSTYLE_CONTROL,
                group_dictionary: h::GROUP_DICTIONARY,
                multiline_style_dictionary: h::MLINE_STYLE_DICTIONARY,
                named_objects_dictionary: h::NAMED_OBJECTS,
                layout_dictionary: h::LAYOUT_DICTIONARY,
                plot_settings_dictionary: h::PLOT_SETTINGS_DICTIONARY,
                plot_style_name_dictionary: h::PLOT_STYLE_NAME_DICTIONARY,
                material_dictionary: h::MATERIAL_DICTIONARY,
                color_dictionary: h::COLOR_DICTIONARY,
                visual_style_dictionary: h::VISUAL_STYLE_DICTIONARY,
                paper_space_block_record: h::PAPER_SPACE,
                model_space_block_record: h::MODEL_SPACE,
                by_layer_linetype: h::BY_LAYER,
                by_block_linetype: h::BY_BLOCK,
                continuous_linetype: h::CONTINUOUS,
                ..Default::default()
            },
        };
        let zero_digest = "0".repeat(32);
        let zero_checksum = "00000000-0000-0000-0000-000000000000".to_string();
        Ok(Self {
            schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
            version: "AC1024".into(),
            maintenance_version: DWG_NEW_DOCUMENT_MAINTENANCE_VERSION,
            codepage: 30,
            drawing: logical,
            header,
            classes: vec![class(500, "AcDbDictionaryWithDefault", "ACDBDICTIONARYWDFLT", 1), class(501, "AcDbPlaceHolder", "ACDBPLACEHOLDER", 1), class(502, "AcDbLayout", "LAYOUT", 2)],
            dependencies: Vec::new(),
            summary: DwgSummaryInfo { created_at: epoch.clone(), modified_at: epoch.clone(), ..Default::default() },
            application: DwgApplicationInfo { name: "AppInfoDataList".into(), version_checksum: zero_checksum.clone(), comment_checksum: zero_checksum.clone(), product_checksum: zero_checksum, ..Default::default() },
            template: DwgTemplate::default(),
            auxiliary_header: DwgAuxiliaryHeader { created_at: epoch.clone(), updated_at: epoch, handle_seed, ..Default::default() },
            revision_history: DwgRevisionHistory::default(),
            preview: DwgIndexedPreview { palette: vec![DwgRgba::default(); 256], ..Default::default() },
            application_history: DwgApplicationHistory {
                history_identifier_one: zero_digest.clone(),
                history_identifier_two: zero_digest.clone(),
                class_version: 3,
                application_version_digest: zero_digest.clone(),
                trust_comment_digest: zero_digest.clone(),
                property_set_digest: zero_digest.clone(),
                property_format_identifier: "f29f85e0-4ff9-1068-ab91-08002b27b3d9".into(),
                product_digest: zero_digest,
                ..Default::default()
            },
        })
    }
}
//#endregion 🔖️DrawingModel
//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg")]
pub struct DwgSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub version: String,
    /// 🗓️ `maint_version` (RC, plain preamble byte 0x12) — cross-checked against LibreDWG's
    /// `header.spec` (`FIELD_RC (maint_version, 0);` right after `dwg_version` at 0x11) and
    /// verified on the real `architectural.dwg` fixture (byte 0x12 == 0x02).
    #[state(artifact)]
    #[value(default)]
    pub maintenance_version: u8,
    /// 🌐 `codepage` (RS, plain preamble bytes 0x13-0x14, little-endian) — LibreDWG's
    /// `header.spec` documents this exact offset with `//@0x13: 29/30 for ANSI_1252`; the real
    /// `architectural.dwg` fixture reads `30` there, an exact match.
    #[state(artifact)]
    #[value(default)]
    pub codepage: u16,
    #[state(artifact)]
    #[value(default)]
    pub drawing: DwgLogicalDrawing,
    #[state(artifact)]
    #[value(default)]
    pub header: DwgHeaderVariables,
    #[state(artifact)]
    #[value(default)]
    pub classes: Vec<DwgClass>,
    #[state(artifact)]
    #[value(default)]
    pub dependencies: Vec<DwgDependency>,
    #[state(artifact)]
    #[value(default)]
    pub summary: DwgSummaryInfo,
    #[state(artifact)]
    #[value(default)]
    pub application: DwgApplicationInfo,
    #[state(artifact)]
    #[value(default)]
    pub template: DwgTemplate,
    #[state(artifact)]
    #[value(default)]
    pub auxiliary_header: DwgAuxiliaryHeader,
    #[state(artifact)]
    #[value(default)]
    pub revision_history: DwgRevisionHistory,
    #[state(artifact)]
    #[value(default)]
    pub preview: DwgIndexedPreview,
    #[state(artifact)]
    #[value(default)]
    pub application_history: DwgApplicationHistory,
}
impl Default for DwgSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
            version: String::new(),
            maintenance_version: 0,
            codepage: 0,
            drawing: DwgLogicalDrawing::default(),
            header: DwgHeaderVariables::default(),
            classes: Vec::new(),
            dependencies: Vec::new(),
            summary: DwgSummaryInfo::default(),
            application: DwgApplicationInfo::default(),
            template: DwgTemplate::default(),
            auxiliary_header: DwgAuxiliaryHeader::default(),
            revision_history: DwgRevisionHistory::default(),
            preview: DwgIndexedPreview::default(),
            application_history: DwgApplicationHistory::default(),
        }
    }
}
impl DwgSnapshot {
    /// 🪞️ Clones the deterministic logical projection.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn projection(&self) -> Self {
        self.clone()
    }
}
//#endregion 🔖️Snapshot
//#region 🔖️DwgCodec
/// 📏️ The plain file-header PREAMBLE every AC1015+ DWG shares — `0x00`..=`0x15`: six ASCII version
/// characters, the application maintenance-release byte at `0x12`, the codepage `RS` at
/// `0x13`-`0x14`. A document that is EXACTLY this and nothing else is this artifact's canonical
/// EMPTY DWG, and it is not a hypothetical: `📚️examples/🎬️demo/🖼️assets/🖊️example.dwg` is committed
/// as those 22 bytes (`AC1024` followed by sixteen zeros).
pub const DWG_PREAMBLE_LEN: usize = 0x16;
/// 🚫 Typed DWG export failures distinguish invalid logical state from writer failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DwgExportError {
    InvalidLogical(String),
    InvalidVersion(String),
    HeaderMismatch(String),
    Writer(String),
}
impl fmt::Display for DwgExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLogical(message) => write!(formatter, "invalid logical DWG: {message}"),
            Self::InvalidVersion(message) => write!(formatter, "invalid DWG version: {message}"),
            Self::HeaderMismatch(message) => write!(formatter, "DWG header mismatch: {message}"),
            Self::Writer(message) => write!(formatter, "DWG writer failed: {message}"),
        }
    }
}
impl std::error::Error for DwgExportError {}
/// 🧭️ Every export refusal is an invalid logical, version, header or writer input to the DWG writer.
impl From<DwgExportError> for ValueError {
    fn from(error: DwgExportError) -> Self {
        Self::new(ValueRefusalKind::InvalidValue, error.to_string())
    }
}
/// 🏷️ The one stamp a drawing that carries content is written with: the R2004-family container [`encode_dwg`]
/// materializes lays out R2010 (AC1024) object streams only, and [`decode_dwg`] reads every other release into that
/// layout.
pub const WRITTEN_VERSION: &str = "AC1024";
//#endregion 🔖️DwgCodec
/// 🫙️ Whether `snapshot` carries the empty document — the preamble triple and nothing else. Compared
/// against a freshly defaulted snapshot wearing the same triple rather than by inspecting fields one
/// at a time, so a field added to `DwgSnapshot` later cannot quietly fall out of the question.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_preamble_only_document(snapshot: &DwgSnapshot) -> bool {
    let bare = DwgSnapshot { schema: snapshot.schema.clone(), version: snapshot.version.clone(), maintenance_version: snapshot.maintenance_version, codepage: snapshot.codepage, ..DwgSnapshot::default() };
    *snapshot == bare
}
/// 🚫️ Why this writer cannot emit `snapshot`'s stamp, as the `x-semio-invariant` id it breaks and a sentence naming the
/// value: `version-sentinel` when the stamp is not `AC` + four digits, `written-as-ac1024` when a drawing that carries
/// content is stamped anything but [`WRITTEN_VERSION`]. AC1018 (R2004) frames objects without a handle-stream size,
/// AC1027/AC1032 (R2013+) add `has_ds_data` to every object, so a foreign stamp over these object streams would be a file
/// no reader decodes as written. Only the empty preamble-only document carries no object stream a stamp could
/// contradict, so it may carry any sentinel.
///
/// @see https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf — §20 object layouts
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unwritable_version(snapshot: &DwgSnapshot) -> Option<(&'static str, String)> {
    if snapshot.version.len() != 6 || !snapshot.version.starts_with("AC") || !snapshot.version.as_bytes()[2..].iter().all(u8::is_ascii_digit) {
        return Some(("version-sentinel", format!("{:?} is not an AC10xx DWG version sentinel", snapshot.version)));
    }
    (snapshot.version != WRITTEN_VERSION && !is_preamble_only_document(snapshot)).then(|| ("written-as-ac1024", format!("a drawing that carries content is written as {WRITTEN_VERSION}; this writer cannot lay out {} object streams", snapshot.version)))
}

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_DWG_DOCUMENT_SCHEMA: &str = "stdio.dwg";

/// 🧬️ Artifact schema descriptor id.
pub const DWG_ARTIFACT_SCHEMA_ID: &str = "s.stdio.dwg";

/// 🌱 Empty persisted snapshot. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::standards::v_ac1024::subsets::any::schema::snapshot::empty_dwg_snapshot` through the `engine`
/// barrel shim, and (via the root `crate::engine` shim, ac1024-only) as
/// `crate::standards::v_ac1024::subsets::any::schema::snapshot::empty_dwg_snapshot` too.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_dwg_snapshot() -> DwgSnapshot {
    DwgSnapshot::default()
}

/// 📄️ The minimal logical `stdio.dwg` AC1024 demonstration document.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_dwg_snapshot() -> DwgSnapshot {
    DwgSnapshot { version: "AC1024".into(), maintenance_version: 2, codepage: 30, ..Default::default() }
}
