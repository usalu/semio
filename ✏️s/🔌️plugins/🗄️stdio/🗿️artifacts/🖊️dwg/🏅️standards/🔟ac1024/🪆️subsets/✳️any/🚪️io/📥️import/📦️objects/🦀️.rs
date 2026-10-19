//! 📦️ The object-record walk: frame validation, dispatch per object class, layouts, scales, sort tables, dictionaries and xrecords.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DwgBitReader;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::dwg_crc16;
use semio_s_artifact_stdio_dwg_read_objects_dynamic_blocks::standards::v_ac1024::subsets::any::io::import::objects::associativity::{decode_assoc_2d_constraint_group_object, decode_assoc_dependency_object, decode_assoc_dimension_dependency_body_object, decode_assoc_geometry_dependency_object, decode_assoc_network_object, decode_assoc_value_dependency_object, decode_assoc_variable_object, decode_evaluation_graph_object};
use semio_s_artifact_stdio_dwg_read_objects_dynamic_blocks::standards::v_ac1024::subsets::any::io::import::objects::blocks::{decode_block_alignment_and_action_object, decode_block_constraint_parameter_object, decode_block_flip_parameter_object, decode_block_grip_location_component_object, decode_block_linear_parameter_object, decode_block_move_action_object, decode_block_parameter_dependency_body_object, decode_block_parameter_grip_object, decode_block_representation_data_object, decode_block_visibility_parameter_object, decode_dynamic_block_proxy_node_object, decode_dynamic_block_purge_preventer_object};
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::decode_r2004_sections;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::container::r2004_section_data;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::DWG_TYPE_BLOCK;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::DWG_TYPE_DIMENSION_LINEAR;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::DWG_TYPE_ENDBLK;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::DWG_TYPE_INSERT;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::DWG_TYPE_VIEWPORT;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::DwgObjectLayout;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::DwgObjectRecordFrame;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::decode_entity_body;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::decode_r2010_entity_common_handles;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::decode_r2010_entity_common_main;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::dimension_attachment_logical;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::dimension_spacing_logical;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::validate_entity_terminal_fill;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::viewport_status_flags;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_complex_color_value;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_object_common_relations;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_r2004_handle_map;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_r2010_eed;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_xrecord_values;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::fixed_object_name;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::object_category;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::r2010_string_content_end_bit;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::r2010_string_stream;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::read_object_handle;
use crate::standards::v_ac1024::subsets::any::io::import::objects::styles::{decode_material_object, decode_mleader_style_object, decode_mline_style_object, decode_table_style_object, decode_visual_style_object};
use crate::standards::v_ac1024::subsets::any::io::import::objects::tables::{decode_table_control_object, decode_table_record_object};

#[path = "🖌️styles/🦀️.rs"]
pub mod styles;
#[path = "🗃️tables/🦀️.rs"]
pub mod tables;

/// 🗃️ Decoded object records and their common entity mode flags.
pub type DwgObjectRecords = (Vec<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject>, Vec<(u8, u8)>);
//#region 🔖️DwgStructuralCodec
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_r2004_object_records(bytes: &[u8], classes: &[semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgClass]) -> Result<DwgObjectRecords, String> {
    let sections = decode_r2004_sections(bytes)?;
    let handles_section = sections.iter().find(|section| section.name == "AcDb:Handles").ok_or("R2004 Handles section missing")?;
    let objects_section = sections.iter().find(|section| section.name == "AcDb:AcDbObjects").ok_or("R2004 AcDbObjects section missing")?;
    let mut handle_map = decode_r2004_handle_map(&r2004_section_data(handles_section)?)?;
    handle_map.sort_by_key(|(_, address)| *address);
    let object_data = r2004_section_data(objects_section)?;
    let mut objects = Vec::with_capacity(handle_map.len());
    let mut xrecord_terminal_fills = Vec::new();
    let mut block_names = Vec::new();
    for (handle, address) in handle_map {
        let frame_bytes = object_data.get(address..).ok_or_else(|| format!("object {handle:#x} address {address} exceeds AcDbObjects"))?;
        let mut frame = DwgBitReader::new(frame_bytes);
        let payload_size = frame.read_ms().map(|value| value as usize).map_err(|error| format!("object {handle:#x} payload size: {error}"))?;
        let handle_stream_bits = frame.read_umc().map(|value| value as usize).map_err(|error| format!("object {handle:#x} handle-stream size: {error}"))?;
        frame.pad_to_byte();
        let payload_end = frame.byte_pos.checked_add(payload_size).ok_or_else(|| format!("object {handle:#x} payload end overflow"))?;
        let payload = frame_bytes.get(frame.byte_pos..payload_end).ok_or_else(|| format!("object {handle:#x} payload is truncated"))?;
        let stored_crc = frame_bytes.get(payload_end..payload_end + 2).map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]])).ok_or_else(|| format!("object {handle:#x} CRC is truncated"))?;
        let computed_crc = dwg_crc16(0xC0C1, &frame_bytes[..payload_end]);
        if stored_crc != computed_crc {
            return Err(format!("object {handle:#x} CRC mismatch: stored {stored_crc:#06x}, computed {computed_crc:#06x}"));
        }
        let data_end_bit = payload_size.checked_mul(8).and_then(|bits| bits.checked_sub(handle_stream_bits)).ok_or_else(|| format!("object {handle:#x} handle-stream size exceeds payload"))?;
        let mut data = DwgBitReader::new(payload);
        let mut handle_reader = DwgBitReader::at_bit(payload, data_end_bit).map_err(|error| format!("object {handle:#x} handle-stream start: {error}"))?;
        let type_code = data.read_bot().map_err(|error| format!("object {handle:#x} type: {error}"))?;
        let (_, object_handle) = data.read_handle().map_err(|error| format!("object {handle:#x} self handle: {error}"))?;
        if object_handle != handle {
            return Err(format!("object {handle:#x} frame self handle is {object_handle:#x}"));
        }
        let fixed = fixed_object_name(type_code);
        let class_name = if fixed == "UNKNOWN" { classes.iter().find(|class| class.number == type_code).map_or_else(|| format!("CLASS_{type_code}"), |class| class.dxf_name.clone()) } else { fixed.to_string() };
        let category = object_category(type_code);
        let extended_data = decode_r2010_eed(&mut data, handle).map_err(|error| format!("object {handle:#x} EED: {error}"))?;
        let mut object = semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject { handle, type_code, class_name, category, extended_data, ..Default::default() };
        if category == semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgObjectCategory::Entity {
            if type_code == DWG_TYPE_BLOCK || type_code == DWG_TYPE_ENDBLK {
                let (mut strings, class_main_end) = r2010_string_stream(payload, data_end_bit).map_err(|error| format!("{} {handle:#x} string stream: {error}", if type_code == DWG_TYPE_BLOCK { "BLOCK" } else { "ENDBLK" }))?;
                let mut common = decode_r2010_entity_common_main(&mut data).map_err(|error| format!("{} {handle:#x} common data: {error}", if type_code == DWG_TYPE_BLOCK { "BLOCK" } else { "ENDBLK" }))?;
                if data.bit_position() != class_main_end {
                    return Err(format!("{} {handle:#x} main stream is not exactly consumed: {} != {class_main_end}", if type_code == DWG_TYPE_BLOCK { "BLOCK" } else { "ENDBLK" }, data.bit_position()));
                }
                if type_code == DWG_TYPE_BLOCK {
                    let name = strings.read_tu().map_err(|error| format!("BLOCK {handle:#x} name: {error}"))?;
                    let string_end = r2010_string_content_end_bit(payload, data_end_bit)?;
                    if strings.bit_position() != string_end {
                        return Err(format!("BLOCK {handle:#x} string stream is not exactly consumed: {} != {string_end}", strings.bit_position()));
                    }
                    block_names.push((handle, name));
                } else if class_main_end != data_end_bit.saturating_sub(1) {
                    return Err(format!("ENDBLK {handle:#x} unexpectedly declares a string stream"));
                }
                let (owner, reactors, extension_dictionary) =
                    decode_r2010_entity_common_handles(&mut common, &mut handle_reader, handle).map_err(|error| format!("{} {handle:#x} common handles: {error}", if type_code == DWG_TYPE_BLOCK { "BLOCK" } else { "ENDBLK" }))?;
                validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, if type_code == DWG_TYPE_BLOCK { "BLOCK" } else { "ENDBLK" })?;
                object.owner_handle = owner;
                object.reactor_handles = reactors;
                object.extension_dictionary_handle = extension_dictionary;
                object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Entity(if type_code == DWG_TYPE_BLOCK {
                    semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody::BlockBegin(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgBlockBeginEntity { common: common.logical })
                } else {
                    semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody::BlockEnd(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgBlockEndEntity { common: common.logical })
                }));
            } else if type_code == DWG_TYPE_INSERT {
                let mut common = decode_r2010_entity_common_main(&mut data).map_err(|error| format!("INSERT {handle:#x} common data: {error}"))?;
                let insertion = data.read_3bd().map_err(|error| format!("INSERT {handle:#x} insertion: {error}"))?;
                let scale = match data.read_bb().map_err(|error| format!("INSERT {handle:#x} scale mode: {error}"))? {
                    0 => {
                        let x = data.read_rd()?;
                        [x, data.read_dd(x)?, data.read_dd(x)?]
                    }
                    1 => [1.0, data.read_dd(1.0)?, data.read_dd(1.0)?],
                    2 => {
                        let value = data.read_rd()?;
                        [value; 3]
                    }
                    3 => [1.0; 3],
                    _ => unreachable!(),
                };
                let rotation = data.read_bd().map_err(|error| format!("INSERT {handle:#x} rotation: {error}"))?;
                let extrusion = data.read_3bd().map_err(|error| format!("INSERT {handle:#x} extrusion: {error}"))?;
                let has_attributes = data.read_b().map_err(|error| format!("INSERT {handle:#x} attributes flag: {error}"))?;
                let attribute_count = if has_attributes { data.read_bl().map_err(|error| format!("INSERT {handle:#x} attribute count: {error}"))? as usize } else { 0 };
                if data.read_b().map_err(|error| format!("INSERT {handle:#x} string marker: {error}"))? || data.bit_position() != data_end_bit {
                    return Err(format!("INSERT {handle:#x} main/string stream is not exactly consumed"));
                }
                let (owner, reactors, extension_dictionary) = decode_r2010_entity_common_handles(&mut common, &mut handle_reader, handle)?;
                let block_header_handle = read_object_handle(&mut handle_reader, handle).map_err(|error| format!("INSERT {handle:#x} block header: {error}"))?.ok_or_else(|| format!("INSERT {handle:#x} block-header handle is null"))?;
                let mut attribute_handles = Vec::with_capacity(attribute_count);
                for index in 0..attribute_count {
                    attribute_handles.push(read_object_handle(&mut handle_reader, handle).map_err(|error| format!("INSERT {handle:#x} attribute {index}: {error}"))?.ok_or_else(|| format!("INSERT {handle:#x} attribute {index} is null"))?);
                }
                let sequence_end_handle = if has_attributes { read_object_handle(&mut handle_reader, handle).map_err(|error| format!("INSERT {handle:#x} SEQEND: {error}"))? } else { None };
                if has_attributes && sequence_end_handle.is_none() {
                    return Err(format!("INSERT {handle:#x} attributes have no SEQEND"));
                }
                validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "INSERT")?;
                object.owner_handle = owner;
                object.reactor_handles = reactors;
                object.extension_dictionary_handle = extension_dictionary;
                object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody::Insert(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgInsertEntity {
                    common: common.logical,
                    insertion: insertion.to_vec(),
                    scale: scale.to_vec(),
                    rotation,
                    extrusion: extrusion.to_vec(),
                    block_header_handle,
                    attribute_handles,
                    sequence_end_handle,
                })));
            } else if type_code == DWG_TYPE_DIMENSION_LINEAR {
                let (mut strings, class_main_end) = r2010_string_stream(payload, data_end_bit).map_err(|error| format!("DIMENSION_LINEAR {handle:#x} string stream: {error}"))?;
                let mut common = decode_r2010_entity_common_main(&mut data).map_err(|error| format!("DIMENSION_LINEAR {handle:#x} common data: {error}"))?;
                let class_version = data.read_rc()?;
                if class_version != 0 {
                    return Err(format!("DIMENSION_LINEAR {handle:#x} class version {class_version} is unsupported"));
                }
                let extrusion = data.read_3bd()?;
                let text_midpoint = data.read_2rd()?;
                let elevation = data.read_bd()?;
                let flag = data.read_rc()?;
                let block_reference_is_exclusive = flag & 0x20 != 0;
                let user_positioned_text = flag & 0x80 != 0;
                let expected_flag = 0x08 | (if block_reference_is_exclusive { 0x22 } else { 0 }) | (if user_positioned_text { 0x80 } else { 0x01 });
                if flag != expected_flag {
                    return Err(format!("DIMENSION_LINEAR {handle:#x} flag {flag:#x} disagrees with its semantic mirror bits; expected {expected_flag:#x}"));
                }
                let text_rotation = data.read_bd()?;
                let horizontal_direction = data.read_bd()?;
                let insertion_scale = data.read_3bd()?;
                let insertion_rotation = data.read_bd()?;
                let attachment = dimension_attachment_logical(data.read_bs()?)?;
                let line_spacing_style = dimension_spacing_logical(data.read_bs()?)?;
                let line_spacing_factor = data.read_bd()?;
                let actual_measurement = data.read_bd()?;
                if data.read_b()? {
                    return Err(format!("DIMENSION_LINEAR {handle:#x} reserved flag is set"));
                }
                let flip_arrow_1 = data.read_b()?;
                let flip_arrow_2 = data.read_b()?;
                let clone_insertion_point = data.read_2rd()?;
                let extension_line_1 = data.read_3bd()?;
                let extension_line_2 = data.read_3bd()?;
                let definition_point = data.read_3bd()?;
                let oblique_angle = data.read_bd()?;
                let dimension_rotation = data.read_bd()?;
                if data.bit_position() != class_main_end {
                    return Err(format!("DIMENSION_LINEAR {handle:#x} main stream is not exactly consumed: {} != {class_main_end}", data.bit_position()));
                }
                let user_text = strings.read_tu()?;
                let string_end = r2010_string_content_end_bit(payload, data_end_bit)?;
                if strings.bit_position() != string_end {
                    return Err(format!("DIMENSION_LINEAR {handle:#x} string stream is not exactly consumed: {} != {string_end}", strings.bit_position()));
                }
                let (owner, reactors, extension_dictionary) = decode_r2010_entity_common_handles(&mut common, &mut handle_reader, handle)?;
                let dimension_style_handle = read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("DIMENSION_LINEAR {handle:#x} dimension style is null"))?;
                let dimension_block_handle = read_object_handle(&mut handle_reader, handle)?;
                validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "DIMENSION_LINEAR")?;
                object.owner_handle = owner;
                object.reactor_handles = reactors;
                object.extension_dictionary_handle = extension_dictionary;
                object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody::DimensionLinear(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgLinearDimensionEntity {
                    dimension: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionEntityCommon {
                        common: common.logical,
                        extrusion: extrusion.to_vec(),
                        text_midpoint: text_midpoint.to_vec(),
                        elevation,
                        status: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionStatus { block_reference_is_exclusive, user_positioned_text },
                        user_text,
                        text_rotation,
                        horizontal_direction,
                        insertion_scale: insertion_scale.to_vec(),
                        insertion_rotation,
                        attachment,
                        line_spacing_style,
                        line_spacing_factor,
                        actual_measurement,
                        flip_arrow_1,
                        flip_arrow_2,
                        clone_insertion_point: clone_insertion_point.to_vec(),
                        dimension_style_handle,
                        dimension_block_handle,
                    },
                    extension_line_1: extension_line_1.to_vec(),
                    extension_line_2: extension_line_2.to_vec(),
                    definition_point: definition_point.to_vec(),
                    oblique_angle,
                    dimension_rotation,
                })));
            } else if type_code == DWG_TYPE_VIEWPORT {
                use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor;
                use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDefaultLightingType;
                use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody;
                use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
                use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgOrthographicView;
                use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgShadePlotMode;
                use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgViewportEntity;
                use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgViewportRenderMode;
                let (mut strings, class_main_end) = r2010_string_stream(payload, data_end_bit).map_err(|error| format!("VIEWPORT {handle:#x} string stream: {error}"))?;
                let mut common = decode_r2010_entity_common_main(&mut data).map_err(|error| format!("VIEWPORT {handle:#x} common data: {error}"))?;
                let center = data.read_3bd()?;
                let width = data.read_bd()?;
                let height = data.read_bd()?;
                let view_target = data.read_3bd()?;
                let view_direction = data.read_3bd()?;
                let twist_angle = data.read_bd()?;
                let view_height = data.read_bd()?;
                let lens_length = data.read_bd()?;
                let front_clip = data.read_bd()?;
                let back_clip = data.read_bd()?;
                let snap_angle = data.read_bd()?;
                let view_center = data.read_2rd()?;
                let snap_base = data.read_2rd()?;
                let snap_unit = data.read_2rd()?;
                let grid_unit = data.read_2rd()?;
                let circle_zoom_percent = data.read_bs()?;
                let grid_major = data.read_bs()?;
                let frozen_count = data.read_bl()? as usize;
                let status = viewport_status_flags(data.read_bl()?)?;
                let render_mode = match data.read_rc()? {
                    0 => DwgViewportRenderMode::Optimized2d,
                    1 => DwgViewportRenderMode::Wireframe,
                    2 => DwgViewportRenderMode::HiddenLine,
                    3 => DwgViewportRenderMode::FlatShaded,
                    4 => DwgViewportRenderMode::GouraudShaded,
                    5 => DwgViewportRenderMode::FlatShadedWithWireframe,
                    6 => DwgViewportRenderMode::GouraudShadedWithWireframe,
                    value => return Err(format!("VIEWPORT {handle:#x} render mode {value} is unsupported")),
                };
                let ucs_at_origin = data.read_b()?;
                let ucs_per_viewport = data.read_b()?;
                let ucs_origin = data.read_3bd()?;
                let ucs_x_axis = data.read_3bd()?;
                let ucs_y_axis = data.read_3bd()?;
                let ucs_elevation = data.read_bd()?;
                let orthographic_view = match data.read_bs()? {
                    0 => DwgOrthographicView::None,
                    1 => DwgOrthographicView::Top,
                    2 => DwgOrthographicView::Bottom,
                    3 => DwgOrthographicView::Front,
                    4 => DwgOrthographicView::Back,
                    5 => DwgOrthographicView::Left,
                    6 => DwgOrthographicView::Right,
                    value => return Err(format!("VIEWPORT {handle:#x} orthographic view {value} is unsupported")),
                };
                let shade_plot_mode = match data.read_bs()? {
                    0 => DwgShadePlotMode::AsDisplayed,
                    1 => DwgShadePlotMode::Wireframe,
                    2 => DwgShadePlotMode::Hidden,
                    3 => DwgShadePlotMode::Rendered,
                    value => return Err(format!("VIEWPORT {handle:#x} shade-plot mode {value} is unsupported")),
                };
                let use_default_lights = data.read_b()?;
                let default_lighting_type = match data.read_rc()? {
                    0 => DwgDefaultLightingType::OneDistantLight,
                    1 => DwgDefaultLightingType::TwoDistantLights,
                    value => return Err(format!("VIEWPORT {handle:#x} lighting type {value} is unsupported")),
                };
                let brightness = data.read_bd()?;
                let contrast = data.read_bd()?;
                let ambient_index = data.read_bs()?;
                let ambient_value = decode_complex_color_value(data.read_bl()?)?;
                let ambient_flags = data.read_rc()?;
                if ambient_flags & !3 != 0 {
                    return Err(format!("VIEWPORT {handle:#x} ambient color flags {ambient_flags:#x} are unsupported"));
                }
                if data.bit_position() != class_main_end {
                    return Err(format!("VIEWPORT {handle:#x} main stream is not exactly consumed: {} != {class_main_end}", data.bit_position()));
                }
                let style_sheet = strings.read_tu()?;
                let ambient_name = if ambient_flags & 1 != 0 { Some(strings.read_tu()?) } else { None };
                let ambient_book_name = if ambient_flags & 2 != 0 { Some(strings.read_tu()?) } else { None };
                let string_end = r2010_string_content_end_bit(payload, data_end_bit)?;
                if strings.bit_position() != string_end {
                    return Err(format!("VIEWPORT {handle:#x} string stream is not exactly consumed: {} != {string_end}", strings.bit_position()));
                }
                let (owner, reactors, extension_dictionary) = decode_r2010_entity_common_handles(&mut common, &mut handle_reader, handle)?;
                let mut frozen_layer_handles = Vec::with_capacity(frozen_count);
                for index in 0..frozen_count {
                    frozen_layer_handles.push(read_object_handle(&mut handle_reader, handle)?.ok_or_else(|| format!("VIEWPORT {handle:#x} frozen layer {index} is null"))?);
                }
                let clip_boundary_handle = read_object_handle(&mut handle_reader, handle)?;
                let named_ucs_handle = read_object_handle(&mut handle_reader, handle)?;
                let base_ucs_handle = read_object_handle(&mut handle_reader, handle)?;
                let background_handle = read_object_handle(&mut handle_reader, handle)?;
                let visual_style_handle = read_object_handle(&mut handle_reader, handle)?;
                let shade_plot_handle = read_object_handle(&mut handle_reader, handle)?;
                let sun_handle = read_object_handle(&mut handle_reader, handle)?;
                validate_entity_terminal_fill(&mut handle_reader, payload_size * 8, handle, "VIEWPORT")?;
                object.owner_handle = owner;
                object.reactor_handles = reactors;
                object.extension_dictionary_handle = extension_dictionary;
                object.body = Some(DwgLogicalObjectBody::Entity(DwgEntityBody::Viewport(DwgViewportEntity {
                    common: common.logical,
                    center: center.to_vec(),
                    width,
                    height,
                    view_target: view_target.to_vec(),
                    view_direction: view_direction.to_vec(),
                    twist_angle,
                    view_height,
                    lens_length,
                    front_clip,
                    back_clip,
                    snap_angle,
                    view_center: view_center.to_vec(),
                    snap_base: snap_base.to_vec(),
                    snap_unit: snap_unit.to_vec(),
                    grid_unit: grid_unit.to_vec(),
                    circle_zoom_percent,
                    grid_major,
                    frozen_layer_handles,
                    status,
                    style_sheet,
                    render_mode,
                    ucs_at_origin,
                    ucs_per_viewport,
                    ucs_origin: ucs_origin.to_vec(),
                    ucs_x_axis: ucs_x_axis.to_vec(),
                    ucs_y_axis: ucs_y_axis.to_vec(),
                    ucs_elevation,
                    orthographic_view,
                    shade_plot_mode,
                    use_default_lights,
                    default_lighting_type,
                    brightness,
                    contrast,
                    ambient_color: DwgComplexColor { index: ambient_index, value: ambient_value, name: ambient_name, book_name: ambient_book_name },
                    clip_boundary_handle,
                    named_ucs_handle,
                    base_ucs_handle,
                    background_handle,
                    visual_style_handle,
                    shade_plot_handle,
                    sun_handle,
                })));
            } else {
                let mut frame = DwgObjectRecordFrame::open(DwgObjectLayout::R2010, &object_data, handle, address)?;
                frame.data = data;
                if let Some((body, (owner, reactors, extension_dictionary))) = decode_entity_body(&mut frame, false)? {
                    validate_entity_terminal_fill(&mut frame.handles, frame.end_bit, handle, &object.class_name)?;
                    object.owner_handle = owner;
                    object.reactor_handles = reactors;
                    object.extension_dictionary_handle = extension_dictionary;
                    object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Entity(body));
                }
            }
            objects.push(object);
            continue;
        }

        let common_relations = decode_object_common_relations(&mut data, &mut handle_reader, handle);
        let (owner, reactors, extension_dictionary) = match common_relations {
            Ok(value) => value,
            Err(error)
                if type_code == 79
                    || object.class_name == "XRECORD"
                    || type_code == 506
                    || object.class_name == "VISUALSTYLE"
                    || type_code == 543
                    || object.class_name == "BLOCKPARAMDEPENDENCYBODY"
                    || type_code == 559
                    || object.class_name == "ACDB_BLOCKREPRESENTATION_DATA"
                    || type_code == 522
                    || object.class_name == "ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION"
                    || type_code == 517
                    || object.class_name == "ACAD_EVALUATION_GRAPH"
                    || type_code == 529
                    || object.class_name == "BLOCKFLIPPARAMETER"
                    || type_code == 531
                    || object.class_name == "BLOCKVISIBILITYPARAMETER"
                    || type_code == 80
                    || object.class_name == "ACDBPLACEHOLDER"
                    || type_code == 503
                    || object.class_name == "DICTIONARYVAR"
                    || type_code == 507
                    || object.class_name == "SCALE"
                    || type_code == 516
                    || object.class_name == "SORTENTSTABLE"
                    || type_code == 504
                    || object.class_name == "TABLESTYLE"
                    || type_code == 73
                    || object.class_name == "MLINESTYLE"
                    || type_code == 508
                    || object.class_name == "MLEADERSTYLE"
                    || type_code == 505
                    || object.class_name == "MATERIAL"
                    || type_code == 82
                    || object.class_name == "LAYOUT"
                    || type_code == 521
                    || object.class_name == "BLOCKMOVEACTION"
                    || type_code == 539
                    || object.class_name == "ACDBASSOCNETWORK"
                    || type_code == 540
                    || object.class_name == "ACDBASSOC2DCONSTRAINTGROUP"
                    || matches!(type_code, 527 | 528 | 530 | 532 | 533 | 534 | 535 | 536 | 537 | 538 | 546 | 548)
                    || matches!(
                        object.class_name.as_str(),
                        "BLOCKLINEARPARAMETER"
                            | "BLOCKLINEARGRIP"
                            | "BLOCKFLIPGRIP"
                            | "BLOCKVISIBILITYGRIP"
                            | "BLOCKALIGNMENTPARAMETER"
                            | "BLOCKALIGNMENTGRIP"
                            | "BLOCKSTRETCHACTION"
                            | "BLOCKSCALEACTION"
                            | "BLOCKFLIPACTION"
                            | "BLOCKBASEPOINTPARAMETER"
                            | "BLOCKVERTICALCONSTRAINTPARAMETER"
                            | "BLOCKHORIZONTALCONSTRAINTPARAMETER"
                    ) =>
            {
                return Err(format!("{} {handle:#x} common data: {error}", object.class_name));
            }
            Err(_) => {
                objects.push(object);
                continue;
            }
        };
        object.owner_handle = owner;
        object.reactor_handles = reactors;
        object.extension_dictionary_handle = extension_dictionary;
        let string_stream = r2010_string_stream(payload, data_end_bit).ok();
        let main_end_bit = string_stream.as_ref().map_or(data_end_bit, |(_, start)| *start);
        let mut strings = string_stream.map(|(reader, _)| reader);

        if type_code == 82 || object.class_name == "LAYOUT" {
            decode_layout_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 527 || object.class_name == "BLOCKLINEARPARAMETER" {
            decode_block_linear_parameter_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if matches!(type_code, 528 | 530 | 532) || matches!(object.class_name.as_str(), "BLOCKLINEARGRIP" | "BLOCKFLIPGRIP" | "BLOCKVISIBILITYGRIP") {
            decode_block_parameter_grip_object(handle, type_code, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if matches!(type_code, 533..=537) || matches!(object.class_name.as_str(), "BLOCKALIGNMENTPARAMETER" | "BLOCKALIGNMENTGRIP" | "BLOCKSTRETCHACTION" | "BLOCKSCALEACTION" | "BLOCKFLIPACTION") {
            decode_block_alignment_and_action_object(handle, type_code, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if matches!(type_code, 538 | 546 | 548) || matches!(object.class_name.as_str(), "BLOCKBASEPOINTPARAMETER" | "BLOCKVERTICALCONSTRAINTPARAMETER" | "BLOCKHORIZONTALCONSTRAINTPARAMETER") {
            decode_block_constraint_parameter_object(handle, type_code, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 539 || object.class_name == "ACDBASSOCNETWORK" {
            decode_assoc_network_object(handle, payload_size, main_end_bit, &mut data, &mut handle_reader, &mut object)?;
        } else if type_code == 540 || object.class_name == "ACDBASSOC2DCONSTRAINTGROUP" {
            decode_assoc_2d_constraint_group_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 505 || object.class_name == "MATERIAL" {
            decode_material_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 521 || object.class_name == "BLOCKMOVEACTION" {
            decode_block_move_action_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 73 || object.class_name == "MLINESTYLE" {
            decode_mline_style_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 508 || object.class_name == "MLEADERSTYLE" {
            decode_mleader_style_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 504 || object.class_name == "TABLESTYLE" {
            decode_table_style_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 516 || object.class_name == "SORTENTSTABLE" {
            decode_sort_entities_table_object(handle, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut object)?;
        } else if type_code == 507 || object.class_name == "SCALE" {
            decode_scale_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 80 || object.class_name == "ACDBPLACEHOLDER" {
            decode_placeholder_object(handle, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut object)?;
        } else if type_code == 503 || object.class_name == "DICTIONARYVAR" {
            decode_dictionary_variable_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 531 || object.class_name == "BLOCKVISIBILITYPARAMETER" {
            decode_block_visibility_parameter_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 529 || object.class_name == "BLOCKFLIPPARAMETER" {
            decode_block_flip_parameter_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 517 || object.class_name == "ACAD_EVALUATION_GRAPH" {
            decode_evaluation_graph_object(handle, payload_size, main_end_bit, &mut data, &mut handle_reader, &mut object)?;
        } else if type_code == 522 || object.class_name == "ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION" {
            decode_dynamic_block_purge_preventer_object(handle, payload_size, main_end_bit, &mut data, &mut handle_reader, &mut object)?;
        } else if type_code == 559 || object.class_name == "ACDB_BLOCKREPRESENTATION_DATA" {
            decode_block_representation_data_object(handle, payload_size, main_end_bit, &mut data, &mut handle_reader, &mut object)?;
        } else if type_code == 506 || object.class_name == "VISUALSTYLE" {
            decode_visual_style_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 543 || object.class_name == "BLOCKPARAMDEPENDENCYBODY" {
            decode_block_parameter_dependency_body_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 549 || object.class_name == "ASSOCDIMDEPENDENCYBODY" {
            decode_assoc_dimension_dependency_body_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 545 || object.class_name == "ACDBASSOCVARIABLE" {
            decode_assoc_variable_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 547 || object.class_name == "ACDB_DYNAMICBLOCKPROXYNODE" {
            decode_dynamic_block_proxy_node_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 520 || object.class_name == "ACDB_BLOCKGRIPLOCATIONCOMPONENT" {
            decode_block_grip_location_component_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 544 || object.class_name == "ACDBASSOCGEOMDEPENDENCY" {
            decode_assoc_geometry_dependency_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 541 || object.class_name == "ACDBASSOCVALUEDEPENDENCY" {
            decode_assoc_value_dependency_object(handle, payload, payload_size, data_end_bit, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 542 || object.class_name == "ACDBASSOCDEPENDENCY" {
            decode_assoc_dependency_object(handle, payload_size, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 42 || object.class_name == "ACDBDICTIONARYWDFLT" {
            decode_dictionary_with_default_object(handle, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if matches!(type_code, 48 | 50 | 52 | 56 | 60 | 62 | 64 | 66 | 68 | 70) {
            decode_table_control_object(handle, type_code, &mut data, &mut handle_reader, &mut object)?;
        } else if matches!(type_code, 49 | 51 | 53 | 57 | 65 | 67 | 69) {
            decode_table_record_object(handle, type_code, main_end_bit, &mut data, &mut handle_reader, &mut strings, &mut object)?;
        } else if type_code == 79 || object.class_name == "XRECORD" {
            decode_xrecord_object(handle, payload_size, main_end_bit, &mut data, &mut handle_reader, &mut object, &mut xrecord_terminal_fills)?;
        }
        objects.push(object);
    }
    for (block_handle, native_name) in block_names {
        let headers = objects
            .iter()
            .enumerate()
            .filter_map(|(index, object)| match object.body.as_ref() {
                Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::TableRecord(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody::BlockHeader(header))) if header.block_entity_handle == block_handle => Some(index),
                _ => None,
            })
            .collect::<Vec<_>>();
        if headers.len() != 1 {
            return Err(format!("BLOCK {block_handle:#x} has {} block-header relationships", headers.len()));
        }
        let header_index = headers[0];
        let marker_owner = objects.iter().find(|object| object.handle == block_handle).and_then(|object| object.owner_handle);
        let header_handle = objects[header_index].handle;
        let ordinary_index = objects.iter().find_map(|object| match object.body.as_ref() {
            Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::TableControl(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableControlBody::Block(control))) => control.entry_handles.iter().position(|entry| entry.handle == Some(header_handle)),
            _ => None,
        });
        let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::TableRecord(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody::BlockHeader(header))) = objects[header_index].body.as_mut() else { unreachable!() };
        if let Some(owner) = marker_owner {
            if owner != header_handle {
                return Err(format!("BLOCK {block_handle:#x} owner {owner:#x} differs from block header {header_handle:#x}"));
            }
        }
        if header.anonymous {
            let suffix = native_name.strip_prefix(&header.common.name).ok_or_else(|| format!("anonymous BLOCK {block_handle:#x} name {native_name:?} does not extend prefix {:?}", header.common.name))?;
            if suffix.is_empty() || !suffix.chars().all(|character| character.is_ascii_digit()) {
                return Err(format!("anonymous BLOCK {block_handle:#x} name {native_name:?} has a nonnumeric identity"));
            }
            let suffix_index = suffix.parse::<usize>().map_err(|error| format!("anonymous BLOCK {block_handle:#x} identity: {error}"))?;
            if ordinary_index != Some(suffix_index) {
                return Err(format!("anonymous BLOCK {block_handle:#x} identity {suffix_index} differs from ordinary block-control index {ordinary_index:?}"));
            }
            header.common.name = native_name;
        } else if header.common.name != native_name {
            return Err(format!("BLOCK {block_handle:#x} native name {native_name:?} differs from block-header name {:?}", header.common.name));
        }
    }
    Ok((objects, xrecord_terminal_fills))
}
/// 🧩️ Decodes one layout object body from its main, string and handle streams.
fn decode_layout_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLayout;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLayoutOptions;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgOrthographicView;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotArea;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotOptions;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotPaperUnit;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotRotation;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgShadePlot;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgShadePlotResolution;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgStandardScale;
    let strings = strings.as_mut().ok_or_else(|| format!("LAYOUT {handle:#x} string stream missing"))?;
    let page_setup_name = strings.read_tu()?;
    let printer_configuration = strings.read_tu()?;
    let flags = data.read_bs()?;
    if flags & !0x2eb0 != 0 {
        return Err(format!("LAYOUT {handle:#x} plot options {flags:#x} are unsupported"));
    }
    let margins = (0..4).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
    let paper_size = (0..2).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
    let canonical_media_name = strings.read_tu()?;
    let plot_origin = (0..2).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
    let paper_unit = match data.read_bs()? {
        0 => DwgPlotPaperUnit::Inches,
        value => return Err(format!("LAYOUT {handle:#x} paper unit {value} is unsupported")),
    };
    let rotation = match data.read_bs()? {
        1 => DwgPlotRotation::QuarterTurn,
        value => return Err(format!("LAYOUT {handle:#x} rotation {value} is unsupported")),
    };
    let plot_area = match data.read_bs()? {
        0 => DwgPlotArea::Display,
        5 => DwgPlotArea::Layout,
        value => return Err(format!("LAYOUT {handle:#x} plot area {value} is unsupported")),
    };
    let plot_window_lower_left = (0..2).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
    let plot_window_upper_right = (0..2).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
    let plot_view_handle = read_object_handle(handle_reader, handle)?;
    let paper_units = data.read_bd()?;
    let drawing_units = data.read_bd()?;
    let stylesheet = strings.read_tu()?;
    let standard_scale = match data.read_bs()? {
        0 => DwgStandardScale::Custom,
        16 => DwgStandardScale::OneToOne,
        value => return Err(format!("LAYOUT {handle:#x} standard scale {value} is unsupported")),
    };
    let standard_scale_factor = data.read_bd()?;
    let paper_image_origin = (0..2).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
    let shade_plot = match data.read_bs()? {
        0 => DwgShadePlot::AsDisplayed,
        value => return Err(format!("LAYOUT {handle:#x} shade plot {value} is unsupported")),
    };
    let shade_plot_resolution = match data.read_bs()? {
        2 => DwgShadePlotResolution::Normal,
        value => return Err(format!("LAYOUT {handle:#x} shade resolution {value} is unsupported")),
    };
    let shade_plot_dpi = data.read_bs()?;
    let visual_style_handle = read_object_handle(handle_reader, handle)?;
    let name = strings.read_tu()?;
    let tab_order = data.read_bs()?;
    let layout_flags = data.read_bs()?;
    if layout_flags & !1 != 0 {
        return Err(format!("LAYOUT {handle:#x} options {layout_flags:#x} are unsupported"));
    }
    let insertion_base = data.read_3bd()?.to_vec();
    let limits_minimum = data.read_2rd()?.to_vec();
    let limits_maximum = data.read_2rd()?.to_vec();
    let ucs_origin = data.read_3bd()?.to_vec();
    let ucs_x_axis = data.read_3bd()?.to_vec();
    let ucs_y_axis = data.read_3bd()?.to_vec();
    let ucs_elevation = data.read_bd()?;
    let orthographic_view = match data.read_bs()? {
        0 => DwgOrthographicView::None,
        1 => DwgOrthographicView::Top,
        2 => DwgOrthographicView::Bottom,
        3 => DwgOrthographicView::Front,
        4 => DwgOrthographicView::Back,
        5 => DwgOrthographicView::Left,
        6 => DwgOrthographicView::Right,
        value => return Err(format!("LAYOUT {handle:#x} orthographic view {value} is unsupported")),
    };
    let extents_minimum = data.read_3bd()?.to_vec();
    let extents_maximum = data.read_3bd()?.to_vec();
    let viewport_count = data.read_bl()? as usize;
    if viewport_count > 10_000 {
        return Err(format!("LAYOUT {handle:#x} viewport count is invalid"));
    }
    let block_header_handle = read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("LAYOUT {handle:#x} block header is null"))?;
    let active_viewport_handle = read_object_handle(handle_reader, handle)?;
    let base_ucs_handle = read_object_handle(handle_reader, handle)?;
    let named_ucs_handle = read_object_handle(handle_reader, handle)?;
    let viewport_handles = (0..viewport_count).map(|_| read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("LAYOUT {handle:#x} viewport is null"))).collect::<Result<Vec<_>, String>>()?;
    if data.bit_position() != main_end_bit || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
        return Err(format!("LAYOUT {handle:#x} stream boundary is invalid"));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "LAYOUT")?;
    object.body = Some(DwgLogicalObjectBody::Layout(DwgLayout {
        page_setup_name,
        printer_configuration,
        canonical_media_name,
        stylesheet,
        name,
        plot_options: DwgPlotOptions {
            use_standard_scale: flags & 16 != 0,
            plot_viewport_borders: flags & 32 != 0,
            plot_with_lineweights: flags & 128 != 0,
            draw_viewports_first: flags & 512 != 0,
            model_type: flags & 1024 != 0,
            update_paper: flags & 2048 != 0,
            initializing: flags & 8192 != 0,
        },
        margins,
        paper_size,
        plot_origin,
        paper_unit,
        rotation,
        plot_area,
        plot_window_lower_left,
        plot_window_upper_right,
        paper_units,
        drawing_units,
        standard_scale,
        standard_scale_factor,
        paper_image_origin,
        shade_plot,
        shade_plot_resolution,
        shade_plot_dpi,
        tab_order,
        options: DwgLayoutOptions { paper_space_linetype_scaling: layout_flags & 1 != 0 },
        insertion_base,
        limits_minimum,
        limits_maximum,
        ucs_origin,
        ucs_x_axis,
        ucs_y_axis,
        ucs_elevation,
        orthographic_view,
        extents_minimum,
        extents_maximum,
        plot_view_handle,
        visual_style_handle,
        block_header_handle,
        active_viewport_handle,
        base_ucs_handle,
        named_ucs_handle,
        viewport_handles,
    }));

    Ok(())
}
/// 🧩️ Decodes one sort entities table object body from its main, string and handle streams.
fn decode_sort_entities_table_object(handle: u64, payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgDrawOrderEntry;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgSortEntitiesTable;
    let count = data.read_bl()? as usize;
    if count > 50_000 {
        return Err(format!("SORTENTSTABLE {handle:#x} entry count {count} is invalid"));
    }
    let sort_handles = (0..count)
        .map(|_| {
            let (code, value) = data.read_handle()?;
            if code != 0 {
                return Err(format!("SORTENTSTABLE {handle:#x} sort key uses handle code {code}"));
            }
            Ok(value)
        })
        .collect::<Result<Vec<_>, String>>()?;
    if data.bit_position() != main_end_bit || main_end_bit + 1 != data_end_bit {
        return Err(format!("SORTENTSTABLE {handle:#x} main/string stream is not exactly consumed"));
    }
    if object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] || object.extension_dictionary_handle.is_some() || !object.extended_data.is_empty() {
        return Err(format!("SORTENTSTABLE {handle:#x} common state is invalid"));
    }
    let block_header_handle = read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("SORTENTSTABLE {handle:#x} block header is null"))?;
    let entity_handles = (0..count).map(|_| read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("SORTENTSTABLE {handle:#x} entity reference is null"))).collect::<Result<Vec<_>, String>>()?;
    let entries = entity_handles.into_iter().zip(sort_handles).map(|(entity_handle, sort_handle)| DwgDrawOrderEntry { entity_handle, sort_handle }).collect::<Vec<_>>();
    let unique_entities = entries.iter().map(|entry| entry.entity_handle).collect::<std::collections::BTreeSet<_>>();
    let unique_sorts = entries.iter().map(|entry| entry.sort_handle).collect::<std::collections::BTreeSet<_>>();
    if unique_entities.len() != entries.len() || unique_sorts.len() != entries.len() {
        return Err(format!("SORTENTSTABLE {handle:#x} contains duplicate entity or sort keys"));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "SORTENTSTABLE")?;
    object.body = Some(DwgLogicalObjectBody::SortEntitiesTable(DwgSortEntitiesTable { block_header_handle, entries }));

    Ok(())
}
/// 🧩️ Decodes one scale object body from its main, string and handle streams.
fn decode_scale_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgAnnotationScale;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    if data.read_bs()? != 0 {
        return Err(format!("SCALE {handle:#x} native format flag is unsupported"));
    }
    let paper_units = data.read_bd()?;
    let drawing_units = data.read_bd()?;
    let is_unit_scale = data.read_b()?;
    if data.bit_position() != main_end_bit {
        return Err(format!("SCALE {handle:#x} main stream is not exactly consumed"));
    }
    if object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] || object.extension_dictionary_handle.is_some() || !object.extended_data.is_empty() {
        return Err(format!("SCALE {handle:#x} common state is invalid"));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("SCALE {handle:#x} string stream missing"))?;
    let name = strings.read_tu()?;
    if strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)?
        || name.is_empty()
        || !paper_units.is_finite()
        || paper_units <= 0.0
        || !drawing_units.is_finite()
        || drawing_units <= 0.0
        || is_unit_scale && (name != "1:1" || paper_units != 1.0 || drawing_units != 1.0)
    {
        return Err(format!("SCALE {handle:#x} logical value or string boundary is invalid"));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "SCALE")?;
    object.body = Some(DwgLogicalObjectBody::AnnotationScale(DwgAnnotationScale { name, paper_units, drawing_units, is_unit_scale }));

    Ok(())
}
/// 🧩️ Decodes one placeholder object body from its main, string and handle streams.
fn decode_placeholder_object(handle: u64, payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlaceholder;
    if data.bit_position() != main_end_bit
        || main_end_bit + 1 != data_end_bit
        || object.owner_handle.is_none()
        || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()]
        || object.extension_dictionary_handle.is_some()
        || !object.extended_data.is_empty()
    {
        return Err(format!("ACDBPLACEHOLDER {handle:#x} logical stream or common state is invalid"));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "ACDBPLACEHOLDER")?;
    object.body = Some(DwgLogicalObjectBody::Placeholder(DwgPlaceholder {}));

    Ok(())
}
/// 🧩️ Decodes one dictionary variable object body from its main, string and handle streams.
fn decode_dictionary_variable_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgDictionaryVariable;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    if data.read_rc()? != 0 || data.bit_position() != main_end_bit {
        return Err(format!("DICTIONARYVAR {handle:#x} schema revision or main boundary is invalid"));
    }
    if object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] || object.extension_dictionary_handle.is_some() || !object.extended_data.is_empty() {
        return Err(format!("DICTIONARYVAR {handle:#x} common state is invalid"));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("DICTIONARYVAR {handle:#x} string stream missing"))?;
    let value = strings.read_tu()?;
    if strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
        return Err(format!("DICTIONARYVAR {handle:#x} string stream is not exactly consumed"));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "DICTIONARYVAR")?;
    object.body = Some(DwgLogicalObjectBody::DictionaryVariable(DwgDictionaryVariable { value }));

    Ok(())
}
/// 🧩️ Decodes one dictionary with default object body from its main, string and handle streams.
fn decode_dictionary_with_default_object(handle: u64, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    let item_count = data.read_bl().map_err(|error| format!("dictionary {handle:#x} item count: {error}"))? as usize;
    let cloning_flag = data.read_bs().map_err(|error| format!("dictionary {handle:#x} cloning flag: {error}"))?;
    let hard_owner = data.read_rc().map_err(|error| format!("dictionary {handle:#x} hard-owner flag: {error}"))? != 0;
    let names = (0..item_count).map(|_| strings.as_mut().ok_or("dictionary string stream missing")?.read_tu().map_err(|error| format!("dictionary {handle:#x} entry name: {error}"))).collect::<Result<Vec<_>, String>>()?;
    let mut entries = Vec::with_capacity(item_count);
    for name in names {
        let reference = read_object_handle(handle_reader, handle).map_err(|error| format!("dictionary {handle:#x} entry {name} handle: {error}"))?.ok_or_else(|| format!("dictionary {handle:#x} entry {name} has a null handle"))?;
        entries.push(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgNamedReference { name, handle: reference });
    }
    let default_entry_handle = if object.class_name == "ACDBDICTIONARYWDFLT" { read_object_handle(handle_reader, handle).map_err(|error| format!("dictionary {handle:#x} default entry handle: {error}"))? } else { None };
    object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Dictionary(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgDictionaryBody { entries, cloning_flag, hard_owner, default_entry_handle }));

    Ok(())
}
/// 🧩️ Decodes one xrecord object body from its main, string and handle streams.
fn decode_xrecord_object(handle: u64, payload_size: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject, xrecord_terminal_fills: &mut Vec<(u8, u8)>) -> Result<(), String> {
    let data_byte_count = data.read_bl().map_err(|error| format!("XRECORD {handle:#x} value byte count: {error}"))? as usize;
    let values = decode_xrecord_values(data, data_byte_count, main_end_bit).map_err(|error| format!("XRECORD {handle:#x}: {error}"))?;
    let cloning_flag = data.read_bs().map_err(|error| format!("XRECORD {handle:#x} cloning flag: {error}"))?;
    if data.bit_position() != main_end_bit {
        return Err(format!("XRECORD {handle:#x} class-main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    let handle_end_bit = payload_size.saturating_mul(8);
    let mut object_id_handles = Vec::new();
    while handle_end_bit.saturating_sub(handle_reader.bit_position()) >= 8 {
        let checkpoint = handle_reader.clone();
        match read_object_handle(handle_reader, handle) {
            Ok(Some(reference)) => object_id_handles.push(reference),
            Ok(None) => break,
            Err(_) => {
                handle_reader = checkpoint;
                break;
            }
        }
    }
    let terminal_padding = handle_end_bit.saturating_sub(handle_reader.bit_position());
    if terminal_padding > 7 {
        return Err(format!("XRECORD {handle:#x} has {terminal_padding} trailing handle-stream bits"));
    }
    let mut padding_value = 0u8;
    for _ in 0..terminal_padding {
        padding_value = (padding_value << 1) | u8::from(handle_reader.read_b().map_err(|error| format!("XRECORD {handle:#x} terminal fill: {error}"))?);
    }
    let expected_padding = (1u8 << terminal_padding).saturating_sub(1);
    if padding_value != expected_padding {
        return Err(format!("XRECORD {handle:#x} terminal handle-stream fill is {padding_value:#x}, expected {expected_padding:#x}"));
    }
    xrecord_terminal_fills.push((terminal_padding as u8, padding_value));
    object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::XRecord(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgXRecordBody { values, object_id_handles, cloning_flag }));

    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2004_object_identities(bytes: &[u8], classes: &[semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgClass]) -> Result<Vec<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject>, String> {
    decode_r2004_object_records(bytes, classes).map(|(objects, _)| objects)
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
