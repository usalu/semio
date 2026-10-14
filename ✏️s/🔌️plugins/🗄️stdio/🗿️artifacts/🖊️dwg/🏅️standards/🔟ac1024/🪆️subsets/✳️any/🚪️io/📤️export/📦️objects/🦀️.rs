//! 📦️ The object-record writer: per-class dispatch, layouts, scales, sort tables, dictionaries, xrecords and the handle map.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::dwg_crc16;
use crate::standards::v_ac1024::subsets::any::io::export::objects::associativity::{encode_r2010_assoc_2d_constraint_group_frame, encode_r2010_assoc_network_frame, encode_r2010_associative_dependency_frame, encode_r2010_associative_dimension_dependency_body_frame, encode_r2010_associative_geometry_dependency_frame, encode_r2010_associative_value_dependency_frame, encode_r2010_associative_variable_frame, encode_r2010_evaluation_graph_frame};
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::bits::DwgBitWriter;
use crate::standards::v_ac1024::subsets::any::io::export::objects::blocks::{encode_r2010_alignment_action_frame, encode_r2010_block_flip_parameter_frame, encode_r2010_block_grip_location_component_frame, encode_r2010_block_move_action_frame, encode_r2010_block_parameter_dependency_body_frame, encode_r2010_block_representation_data_frame, encode_r2010_block_visibility_parameter_frame, encode_r2010_dynamic_block_frame, encode_r2010_dynamic_block_proxy_node_frame, encode_r2010_dynamic_block_purge_preventer_frame, encode_r2010_final_parameter_frame};
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::entities::encode_r2010_arc_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::entities::encode_r2010_block_begin_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::entities::encode_r2010_block_end_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::entities::encode_r2010_dimension_linear_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::entities::encode_r2010_entity_body_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::entities::encode_r2010_insert_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::entities::encode_r2010_line_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::entities::encode_r2010_lwpolyline_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::entities::encode_r2010_viewport_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::DwgObjectFrame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::append_r2010_string_stream;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_r2010_eed;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_xrecord_values;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::finish_r2010_object_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_object_handle;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_r2004_signed_modular_char;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_r2004_unsigned_modular_char;
use crate::standards::v_ac1024::subsets::any::io::export::objects::styles::{encode_r2010_material_frame, encode_r2010_mleader_style_frame, encode_r2010_mline_style_frame, encode_r2010_table_style_frame, encode_r2010_visual_style_frame};
use crate::standards::v_ac1024::subsets::any::io::export::objects::tables::{encode_r2010_table_control_frame, encode_r2010_table_record_frame};

#[path = "🗃️tables/🦀️.rs"]
pub mod tables;
#[path = "🖌️styles/🦀️.rs"]
pub mod styles;
#[path = "🧩️blocks/🦀️.rs"]
pub mod blocks;
#[path = "🔗️associativity/🦀️.rs"]
pub mod associativity;

//#region 🔖️DwgStructuralCodec
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_xrecord_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::XRecord(xrecord) = object.body.as_ref().ok_or_else(|| format!("XRECORD {:#x} body missing", object.handle))? else {
        return Err(format!("object {:#x} is not an XRECORD body", object.handle));
    };
    let xdata = encode_xrecord_values(&xrecord.values)?;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bl(xdata.bytes.len() as u32);
    data.append_bits(&xdata);
    data.write_bs(xrecord.cloning_flag);
    data.write_b(false);

    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(handle) = object.extension_dictionary_handle {
        handles.write_handle(4, handle);
    }
    for reference in &xrecord.object_id_handles {
        handles.write_handle(4, *reference);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_dictionary_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Dictionary(dictionary) = object.body.as_ref().ok_or_else(|| format!("dictionary {:#x} body missing", object.handle))? else {
        return Err(format!("object {:#x} is not a dictionary body", object.handle));
    };
    if dictionary.cloning_flag > 5 {
        return Err(format!("dictionary {:#x} cloning flag {} is invalid", object.handle, dictionary.cloning_flag));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bl(dictionary.entries.len() as u32);
    data.write_bs(dictionary.cloning_flag);
    data.write_rc(u8::from(dictionary.hard_owner));
    let mut strings = DwgBitWriter::new();
    for entry in &dictionary.entries {
        strings.write_tu(&entry.name);
    }
    let string_bits = strings.bit_len();
    if string_bits == 0 {
        data.write_b(false);
    } else {
        data.append_bits(&strings);
        if string_bits > 0x7fff {
            return Err(format!("dictionary {:#x} string stream exceeds compact R2010 size", object.handle));
        }
        data.write_rs(string_bits as u16);
        data.write_b(true);
    }

    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(handle) = object.extension_dictionary_handle {
        handles.write_handle(4, handle);
    }
    for entry in &dictionary.entries {
        handles.write_handle(2, entry.handle);
    }
    if let Some(default_entry) = dictionary.default_entry_handle {
        handles.write_handle(5, default_entry);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_placeholder_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    if !matches!(object.body, Some(DwgLogicalObjectBody::Placeholder(_)))
        || !object.extended_data.is_empty()
        || object.owner_handle.is_none()
        || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()]
        || object.extension_dictionary_handle.is_some()
    {
        return Err(format!("ACDBPLACEHOLDER {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_dictionary_variable_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::DictionaryVariable(body)) = object.body.as_ref() else {
        return Err(format!("DICTIONARYVAR {:#x} body missing", object.handle));
    };
    if !object.extended_data.is_empty() || object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] || object.extension_dictionary_handle.is_some() {
        return Err(format!("DICTIONARYVAR {:#x} common state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_rc(0);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.value);
    append_r2010_string_stream(&mut data, &strings, "DICTIONARYVAR", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_annotation_scale_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::AnnotationScale(body)) = object.body.as_ref() else {
        return Err(format!("SCALE {:#x} body missing", object.handle));
    };
    if body.name.is_empty()
        || !body.paper_units.is_finite()
        || body.paper_units <= 0.0
        || !body.drawing_units.is_finite()
        || body.drawing_units <= 0.0
        || body.is_unit_scale && (body.name != "1:1" || body.paper_units != 1.0 || body.drawing_units != 1.0)
        || !object.extended_data.is_empty()
        || object.owner_handle.is_none()
        || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()]
        || object.extension_dictionary_handle.is_some()
    {
        return Err(format!("SCALE {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bs(0);
    data.write_bd(body.paper_units);
    data.write_bd(body.drawing_units);
    data.write_b(body.is_unit_scale);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.name);
    append_r2010_string_stream(&mut data, &strings, "SCALE", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_sort_entities_table_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::SortEntitiesTable(body)) = object.body.as_ref() else {
        return Err(format!("SORTENTSTABLE {:#x} body missing", object.handle));
    };
    let entity_handles = body.entries.iter().map(|entry| entry.entity_handle).collect::<std::collections::BTreeSet<_>>();
    let sort_handles = body.entries.iter().map(|entry| entry.sort_handle).collect::<std::collections::BTreeSet<_>>();
    if body.block_header_handle == 0
        || body.entries.len() > 50_000
        || body.entries.iter().any(|entry| entry.entity_handle == 0)
        || entity_handles.len() != body.entries.len()
        || sort_handles.len() != body.entries.len()
        || !object.extended_data.is_empty()
        || object.owner_handle.is_none()
        || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()]
        || object.extension_dictionary_handle.is_some()
    {
        return Err(format!("SORTENTSTABLE {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bl(body.entries.len() as u32);
    for entry in &body.entries {
        data.write_handle(0, entry.sort_handle);
    }
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(4, body.block_header_handle);
    for entry in &body.entries {
        handles.write_handle(4, entry.entity_handle);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_layout_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgOrthographicView;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotArea;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotPaperUnit;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotRotation;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgShadePlot;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgShadePlotResolution;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgStandardScale;
    let Some(DwgLogicalObjectBody::Layout(layout)) = object.body.as_ref() else { return Err(format!("LAYOUT {:#x} body missing", object.handle)) };
    let dimensions = [
        (&layout.margins, 4),
        (&layout.paper_size, 2),
        (&layout.plot_origin, 2),
        (&layout.plot_window_lower_left, 2),
        (&layout.plot_window_upper_right, 2),
        (&layout.paper_image_origin, 2),
        (&layout.insertion_base, 3),
        (&layout.limits_minimum, 2),
        (&layout.limits_maximum, 2),
        (&layout.ucs_origin, 3),
        (&layout.ucs_x_axis, 3),
        (&layout.ucs_y_axis, 3),
        (&layout.extents_minimum, 3),
        (&layout.extents_maximum, 3),
    ];
    if object.type_code != 82
        || object.owner_handle.is_none()
        || object.reactor_handles.len() != 1
        || object.extension_dictionary_handle.is_none()
        || layout.block_header_handle == 0
        || dimensions.iter().any(|(values, length)| values.len() != *length || values.iter().any(|value| !value.is_finite()))
        || layout.viewport_handles.contains(&0)
        || layout.viewport_handles.iter().enumerate().any(|(index, handle)| layout.viewport_handles[..index].contains(handle))
        || !layout.paper_units.is_finite()
        || !layout.drawing_units.is_finite()
        || !layout.standard_scale_factor.is_finite()
        || !layout.ucs_elevation.is_finite()
    {
        return Err(format!("LAYOUT {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(false);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&layout.page_setup_name);
    strings.write_tu(&layout.printer_configuration);
    let flags = (u16::from(layout.plot_options.use_standard_scale) * 16)
        | (u16::from(layout.plot_options.plot_viewport_borders) * 32)
        | (u16::from(layout.plot_options.plot_with_lineweights) * 128)
        | (u16::from(layout.plot_options.draw_viewports_first) * 512)
        | (u16::from(layout.plot_options.model_type) * 1024)
        | (u16::from(layout.plot_options.update_paper) * 2048)
        | (u16::from(layout.plot_options.initializing) * 8192);
    data.write_bs(flags);
    for value in &layout.margins {
        data.write_bd(*value);
    }
    for value in &layout.paper_size {
        data.write_bd(*value);
    }
    strings.write_tu(&layout.canonical_media_name);
    for value in &layout.plot_origin {
        data.write_bd(*value);
    }
    data.write_bs(match layout.paper_unit {
        DwgPlotPaperUnit::Inches => 0,
    });
    data.write_bs(match layout.rotation {
        DwgPlotRotation::QuarterTurn => 1,
    });
    data.write_bs(match layout.plot_area {
        DwgPlotArea::Display => 0,
        DwgPlotArea::Layout => 5,
    });
    for value in &layout.plot_window_lower_left {
        data.write_bd(*value);
    }
    for value in &layout.plot_window_upper_right {
        data.write_bd(*value);
    }
    data.write_bd(layout.paper_units);
    data.write_bd(layout.drawing_units);
    strings.write_tu(&layout.stylesheet);
    data.write_bs(match layout.standard_scale {
        DwgStandardScale::Custom => 0,
        DwgStandardScale::OneToOne => 16,
    });
    data.write_bd(layout.standard_scale_factor);
    for value in &layout.paper_image_origin {
        data.write_bd(*value);
    }
    data.write_bs(match layout.shade_plot {
        DwgShadePlot::AsDisplayed => 0,
    });
    data.write_bs(match layout.shade_plot_resolution {
        DwgShadePlotResolution::Normal => 2,
    });
    data.write_bs(layout.shade_plot_dpi);
    strings.write_tu(&layout.name);
    data.write_bs(layout.tab_order);
    data.write_bs(u16::from(layout.options.paper_space_linetype_scaling));
    data.write_3bd([layout.insertion_base[0], layout.insertion_base[1], layout.insertion_base[2]]);
    data.write_2rd([layout.limits_minimum[0], layout.limits_minimum[1]]);
    data.write_2rd([layout.limits_maximum[0], layout.limits_maximum[1]]);
    data.write_3bd([layout.ucs_origin[0], layout.ucs_origin[1], layout.ucs_origin[2]]);
    data.write_3bd([layout.ucs_x_axis[0], layout.ucs_x_axis[1], layout.ucs_x_axis[2]]);
    data.write_3bd([layout.ucs_y_axis[0], layout.ucs_y_axis[1], layout.ucs_y_axis[2]]);
    data.write_bd(layout.ucs_elevation);
    data.write_bs(match layout.orthographic_view {
        DwgOrthographicView::None => 0,
        DwgOrthographicView::Top => 1,
        DwgOrthographicView::Bottom => 2,
        DwgOrthographicView::Front => 3,
        DwgOrthographicView::Back => 4,
        DwgOrthographicView::Left => 5,
        DwgOrthographicView::Right => 6,
    });
    data.write_3bd([layout.extents_minimum[0], layout.extents_minimum[1], layout.extents_minimum[2]]);
    data.write_3bd([layout.extents_maximum[0], layout.extents_maximum[1], layout.extents_maximum[2]]);
    data.write_bl(layout.viewport_handles.len() as u32);
    append_r2010_string_stream(&mut data, &strings, "LAYOUT", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(3, object.extension_dictionary_handle.unwrap());
    handles.write_handle(5, layout.plot_view_handle.unwrap_or_default());
    handles.write_handle(4, layout.visual_style_handle.unwrap_or_default());
    handles.write_handle(4, layout.block_header_handle);
    handles.write_handle(4, layout.active_viewport_handle.unwrap_or_default());
    handles.write_handle(5, layout.base_ucs_handle.unwrap_or_default());
    handles.write_handle(5, layout.named_ucs_handle.unwrap_or_default());
    for viewport in &layout.viewport_handles {
        handles.write_handle(4, *viewport);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_object_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject, block_names: &std::collections::BTreeMap<u64, String>) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    match object.body.as_ref().ok_or_else(|| format!("object {:#x} has no typed body", object.handle))? {
        DwgLogicalObjectBody::Dictionary(_) => encode_r2010_dictionary_frame(object),
        DwgLogicalObjectBody::TableControl(_) => encode_r2010_table_control_frame(object),
        DwgLogicalObjectBody::TableRecord(_) => encode_r2010_table_record_frame(object),
        DwgLogicalObjectBody::XRecord(_) => encode_r2010_xrecord_frame(object),
        DwgLogicalObjectBody::Entity(entity) => match entity {
            DwgEntityBody::Line(_) => encode_r2010_line_frame(object),
            DwgEntityBody::Arc(_) => encode_r2010_arc_frame(object),
            DwgEntityBody::LwPolyline(_) => encode_r2010_lwpolyline_frame(object),
            DwgEntityBody::BlockBegin(_) => encode_r2010_block_begin_frame(object, block_names.get(&object.handle).ok_or_else(|| format!("BLOCK {:#x} has no logical block-header name", object.handle))?),
            DwgEntityBody::BlockEnd(_) => encode_r2010_block_end_frame(object),
            DwgEntityBody::Insert(_) => encode_r2010_insert_frame(object),
            DwgEntityBody::DimensionLinear(_) => encode_r2010_dimension_linear_frame(object),
            DwgEntityBody::Viewport(_) => encode_r2010_viewport_frame(object),
            DwgEntityBody::Point(_) | DwgEntityBody::Circle(_) | DwgEntityBody::Ellipse(_) | DwgEntityBody::Text(_) | DwgEntityBody::Spline(_) | DwgEntityBody::Face3d(_) | DwgEntityBody::Polyline3d(_) | DwgEntityBody::PolyfaceMesh(_) | DwgEntityBody::Vertex(_) | DwgEntityBody::PolyfaceFace(_) | DwgEntityBody::SequenceEnd(_) => {
                encode_r2010_entity_body_frame(object)
            }
        },
        DwgLogicalObjectBody::AssociativeDependency(_) => encode_r2010_associative_dependency_frame(object),
        DwgLogicalObjectBody::AssociativeValueDependency(_) => encode_r2010_associative_value_dependency_frame(object),
        DwgLogicalObjectBody::AssociativeGeometryDependency(_) => encode_r2010_associative_geometry_dependency_frame(object),
        DwgLogicalObjectBody::BlockGripLocationComponent(_) => encode_r2010_block_grip_location_component_frame(object),
        DwgLogicalObjectBody::DynamicBlockProxyNode(_) => encode_r2010_dynamic_block_proxy_node_frame(object),
        DwgLogicalObjectBody::AssociativeVariable(_) => encode_r2010_associative_variable_frame(object),
        DwgLogicalObjectBody::AssociativeDimensionDependencyBody(_) => encode_r2010_associative_dimension_dependency_body_frame(object),
        DwgLogicalObjectBody::VisualStyle(_) => encode_r2010_visual_style_frame(object),
        DwgLogicalObjectBody::BlockParameterDependencyBody(_) => encode_r2010_block_parameter_dependency_body_frame(object),
        DwgLogicalObjectBody::BlockRepresentationData(_) => encode_r2010_block_representation_data_frame(object),
        DwgLogicalObjectBody::DynamicBlockPurgePreventer(_) => encode_r2010_dynamic_block_purge_preventer_frame(object),
        DwgLogicalObjectBody::EvaluationGraph(_) => encode_r2010_evaluation_graph_frame(object),
        DwgLogicalObjectBody::BlockFlipParameter(_) => encode_r2010_block_flip_parameter_frame(object),
        DwgLogicalObjectBody::BlockVisibilityParameter(_) => encode_r2010_block_visibility_parameter_frame(object),
        DwgLogicalObjectBody::Placeholder(_) => encode_r2010_placeholder_frame(object),
        DwgLogicalObjectBody::DictionaryVariable(_) => encode_r2010_dictionary_variable_frame(object),
        DwgLogicalObjectBody::AnnotationScale(_) => encode_r2010_annotation_scale_frame(object),
        DwgLogicalObjectBody::SortEntitiesTable(_) => encode_r2010_sort_entities_table_frame(object),
        DwgLogicalObjectBody::TableStyle(_) => encode_r2010_table_style_frame(object),
        DwgLogicalObjectBody::MlineStyle(_) => encode_r2010_mline_style_frame(object),
        DwgLogicalObjectBody::MLeaderStyle(_) => encode_r2010_mleader_style_frame(object),
        DwgLogicalObjectBody::Material(_) => encode_r2010_material_frame(object),
        DwgLogicalObjectBody::BlockMoveAction(_) => encode_r2010_block_move_action_frame(object),
        DwgLogicalObjectBody::AssocNetwork(_) => encode_r2010_assoc_network_frame(object),
        DwgLogicalObjectBody::Assoc2dConstraintGroup(_) => encode_r2010_assoc_2d_constraint_group_frame(object),
        DwgLogicalObjectBody::BlockLinearParameter(_) | DwgLogicalObjectBody::BlockLinearGrip(_) | DwgLogicalObjectBody::BlockFlipGrip(_) | DwgLogicalObjectBody::BlockVisibilityGrip(_) => encode_r2010_dynamic_block_frame(object),
        DwgLogicalObjectBody::BlockAlignmentParameter(_) | DwgLogicalObjectBody::BlockAlignmentGrip(_) | DwgLogicalObjectBody::BlockStretchAction(_) | DwgLogicalObjectBody::BlockScaleAction(_) | DwgLogicalObjectBody::BlockFlipAction(_) => {
            encode_r2010_alignment_action_frame(object)
        }
        DwgLogicalObjectBody::BlockBasePointParameter(_) | DwgLogicalObjectBody::BlockVerticalConstraintParameter(_) | DwgLogicalObjectBody::BlockHorizontalConstraintParameter(_) => encode_r2010_final_parameter_frame(object),
        DwgLogicalObjectBody::Layout(_) => encode_r2010_layout_frame(object),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn finish_r2004_handle_block(output: &mut Vec<u8>, block: &mut Vec<u8>) -> Result<(), String> {
    let size = u16::try_from(block.len()).map_err(|_| "R2004 Handles block exceeds u16")?;
    block[0..2].copy_from_slice(&size.to_be_bytes());
    let crc = dwg_crc16(0xC0C1, block);
    output.extend_from_slice(block);
    output.extend_from_slice(&crc.to_be_bytes());
    block.clear();
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn materialize_r2004_handles(pairs: &[(u64, usize)]) -> Result<Vec<u8>, String> {
    let mut sorted = pairs.to_vec();
    sorted.sort_by_key(|(handle, _)| *handle);
    if sorted.iter().any(|(handle, _)| *handle == 0) || sorted.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Err("R2004 Handles requires unique nonzero handles".into());
    }
    let mut output = Vec::new();
    let mut block = vec![0u8; 2];
    let mut last_handle = 0u64;
    let mut last_address = 0usize;
    for (handle, address) in sorted {
        write_r2004_unsigned_modular_char(&mut block, handle.checked_sub(last_handle).ok_or("R2004 Handles are not strictly increasing")?);
        write_r2004_signed_modular_char(&mut block, i64::try_from(address).map_err(|_| "R2004 object address exceeds i64")? - i64::try_from(last_address).map_err(|_| "R2004 prior object address exceeds i64")?);
        last_handle = handle;
        last_address = address;
        if block.len() > 2030 {
            finish_r2004_handle_block(&mut output, &mut block)?;
            block.extend_from_slice(&[0, 0]);
            last_handle = 0;
            last_address = 0;
        }
    }
    if block.len() > 2 {
        finish_r2004_handle_block(&mut output, &mut block)?;
    }
    let mut terminator = vec![0, 2];
    finish_r2004_handle_block(&mut output, &mut terminator)?;
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn materialize_r2010_objects(objects: &[semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject]) -> Result<DwgObjectFrame, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody;
    let mut seen = std::collections::BTreeSet::new();
    if objects.iter().any(|object| object.handle == 0 || !seen.insert(object.handle)) {
        return Err("AcDbObjects requires unique nonzero handles".into());
    }
    let block_names = objects
        .iter()
        .filter_map(|object| match object.body.as_ref() {
            Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::BlockHeader(header))) => Some((header.block_entity_handle, header.common.name.clone())),
            _ => None,
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut output = vec![0xca, 0x0d, 0x00, 0x00];
    let mut pairs = Vec::with_capacity(objects.len());
    for object in objects {
        pairs.push((object.handle, output.len()));
        output.extend_from_slice(&encode_r2010_object_frame(object, &block_names)?);
    }
    Ok((output, pairs))
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
