//! 🗃️ Symbol-table record writers: table controls and table records.

use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::bits::DwgBitWriter;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_complex_color_value;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_r2010_eed;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::finish_r2010_object_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_object_handle;

//#region 🔖️DwgStructuralCodec
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_table_control_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableControlBody;
    let semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::TableControl(control) = object.body.as_ref().ok_or_else(|| format!("table control {:#x} body missing", object.handle))? else {
        return Err(format!("object {:#x} is not a table control", object.handle));
    };
    let expected_type = match control {
        DwgTableControlBody::Block(_) => 48,
        DwgTableControlBody::Layer(_) => 50,
        DwgTableControlBody::TextStyle(_) => 52,
        DwgTableControlBody::Linetype(_) => 56,
        DwgTableControlBody::View(_) => 60,
        DwgTableControlBody::Ucs(_) => 62,
        DwgTableControlBody::Viewport(_) => 64,
        DwgTableControlBody::RegisteredApplication(_) => 66,
        DwgTableControlBody::DimensionStyle(_) => 68,
    };
    if object.type_code != expected_type {
        return Err(format!("table-control variant expects type {expected_type}, found {}", object.type_code));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    let entries = control.entry_handles();
    if matches!(object.type_code, 48 | 50 | 52 | 60) {
        data.write_bl(entries.len() as u32);
    } else {
        data.write_bs(entries.len() as u16);
    }
    if let DwgTableControlBody::DimensionStyle(value) = control {
        if value.additional_handles.len() > usize::from(u8::MAX) {
            return Err("DIMSTYLE additional handle count exceeds RC".into());
        }
        data.write_rc(value.additional_handles.len() as u8);
    }
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(xdic) = object.extension_dictionary_handle {
        handles.write_handle(3, xdic);
    }
    for entry in entries {
        handles.write_handle(2, entry.handle.unwrap_or_default());
    }
    match control {
        DwgTableControlBody::Block(value) => {
            handles.write_handle(3, value.model_space_handle.ok_or("BLOCK_CONTROL model-space handle missing")?);
            handles.write_handle(3, value.paper_space_handle.ok_or("BLOCK_CONTROL paper-space handle missing")?);
        }
        DwgTableControlBody::Linetype(value) => {
            handles.write_handle(3, value.by_block_handle);
            handles.write_handle(3, value.by_layer_handle);
        }
        DwgTableControlBody::DimensionStyle(value) => {
            for additional in &value.additional_handles {
                handles.write_handle(5, *additional);
            }
        }
        _ => {}
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_table_record_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody;
    let body = match object.body.as_ref() {
        Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::TableRecord(value)) => value,
        _ => return Err(format!("table record {:#x} body missing", object.handle)),
    };
    let (common, expected_type) = match body {
        DwgTableRecordBody::RegisteredApplication(value) => (&value.common, 67),
        DwgTableRecordBody::TextStyle(value) => (&value.common, 53),
        DwgTableRecordBody::Layer(value) => (&value.common, 51),
        DwgTableRecordBody::Linetype(value) => (&value.common, 57),
        DwgTableRecordBody::BlockHeader(value) => (&value.common, 49),
        DwgTableRecordBody::Viewport(value) => (&value.common, 65),
        DwgTableRecordBody::DimensionStyle(value) => (&value.common, 69),
    };
    if object.type_code != expected_type {
        return Err(format!("table-record variant expects type {expected_type}, found {}", object.type_code));
    }
    if !matches!(common.xref_resolution, 0 | 256) {
        return Err(format!("table record {:#x} xref resolution {} is invalid", object.handle, common.xref_resolution));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bs(common.xref_resolution);
    let mut strings = DwgBitWriter::new();
    if let DwgTableRecordBody::BlockHeader(value) = body {
        if value.anonymous {
            let mut characters = common.name.chars();
            let prefix = characters.by_ref().take(2).collect::<String>();
            if !prefix.starts_with('*') || prefix.chars().count() != 2 || !characters.as_str().chars().all(|character| character.is_ascii_digit()) {
                return Err(format!("anonymous block header {:#x} name {:?} is not a standard prefix plus numeric identity", object.handle, common.name));
            }
            strings.write_tu(&prefix);
        } else {
            strings.write_tu(&common.name);
        }
    } else {
        strings.write_tu(&common.name);
    }
    match body {
        DwgTableRecordBody::RegisteredApplication(value) => data.write_rc(value.group_71),
        DwgTableRecordBody::TextStyle(value) => {
            data.write_b(value.is_shape);
            data.write_b(value.is_vertical);
            data.write_bd(value.text_size);
            data.write_bd(value.width_factor);
            data.write_bd(value.oblique_angle);
            data.write_rc(value.generation);
            data.write_bd(value.last_height);
            strings.write_tu(&value.font_file);
            strings.write_tu(&value.big_font_file);
        }
        DwgTableRecordBody::Layer(value) => write_layer_record(value, object, &mut data, &mut strings)?,
        DwgTableRecordBody::Linetype(value) => write_linetype_record(value, object, &mut data, &mut strings)?,
        DwgTableRecordBody::BlockHeader(value) => write_block_header_record(value, object, &mut data, &mut strings)?,
        DwgTableRecordBody::Viewport(value) => write_viewport_record(value, &mut data, &mut strings),
        DwgTableRecordBody::DimensionStyle(value) => write_dimension_style_record(value, &mut data, &mut strings),
    }
    let string_bits = strings.bit_len();
    if string_bits > 0x7fff {
        return Err(format!("table record {:#x} strings exceed compact R2010 size", object.handle));
    }
    data.append_bits(&strings);
    data.write_rs(string_bits as u16);
    data.write_b(true);

    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(xdic) = object.extension_dictionary_handle {
        handles.write_handle(3, xdic);
    }
    handles.write_handle(5, common.xref_handle.unwrap_or_default());
    if let DwgTableRecordBody::Layer(value) = body {
        handles.write_handle(5, value.plot_style_handle.unwrap_or_default());
        handles.write_handle(5, value.material_handle.unwrap_or_default());
        handles.write_handle(5, value.linetype_handle.unwrap_or_default());
    }
    if let DwgTableRecordBody::Linetype(value) = body {
        for dash in &value.dashes {
            handles.write_handle(5, dash.style_handle.unwrap_or_default());
        }
    }
    if let DwgTableRecordBody::BlockHeader(value) = body {
        handles.write_handle(3, value.block_entity_handle);
        for owned in &value.owned_entity_handles {
            handles.write_handle(3, *owned);
        }
        handles.write_handle(3, value.end_block_entity_handle);
        for insert in &value.insert_backreference_handles {
            handles.write_handle(4, *insert);
        }
        handles.write_handle(5, value.layout_handle.unwrap_or_default());
    }
    if let DwgTableRecordBody::Viewport(value) = body {
        handles.write_handle(4, value.background_handle.unwrap_or_default());
        handles.write_handle(5, value.visual_style_handle.unwrap_or_default());
        handles.write_handle(3, value.sun_handle.unwrap_or_default());
        handles.write_handle(5, value.named_ucs_handle.unwrap_or_default());
        handles.write_handle(5, value.base_ucs_handle.unwrap_or_default());
    }
    if let DwgTableRecordBody::DimensionStyle(value) = body {
        handles.write_handle(5, value.text_style_handle.unwrap_or_default());
        handles.write_handle(5, value.leader_arrow_handle.unwrap_or_default());
        handles.write_handle(5, value.arrow_handle.unwrap_or_default());
        handles.write_handle(5, value.arrow_1_handle.unwrap_or_default());
        handles.write_handle(5, value.arrow_2_handle.unwrap_or_default());
        handles.write_handle(5, value.dimension_linetype_handle.unwrap_or_default());
        handles.write_handle(5, value.extension_1_linetype_handle.unwrap_or_default());
        handles.write_handle(5, value.extension_2_linetype_handle.unwrap_or_default());
    }
    finish_r2010_object_frame(&data, handles)
}

/// 🧱️ Writes the main stream fields of a layer table record.
fn write_layer_record(
    value: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgLayerTableRecord,
    object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject,
    data: &mut DwgBitWriter,
    strings: &mut DwgBitWriter,
) -> Result<(), String> {
    if value.lineweight > 31 {
        return Err(format!("layer {:#x} lineweight {} exceeds five bits", object.handle, value.lineweight));
    }
    let flag0 = u16::from(value.frozen) | (u16::from(value.off) << 1) | (u16::from(value.frozen_in_new_viewports) << 2) | (u16::from(value.locked) << 3) | (u16::from(value.plottable) << 4) | (u16::from(value.lineweight) << 5);
    data.write_bs(flag0);
    data.write_bs(value.color.index);
    data.write_bl(encode_complex_color_value(&value.color.value));
    let color_flags = u8::from(value.color.name.is_some()) | (u8::from(value.color.book_name.is_some()) << 1);
    data.write_rc(color_flags);
    if let Some(name) = &value.color.name {
        strings.write_tu(name);
    }
    if let Some(book_name) = &value.color.book_name {
        strings.write_tu(book_name);
    }
    Ok(())
}

/// 🪵️ Writes the main stream fields of a linetype table record.
fn write_linetype_record(
    value: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgLinetypeTableRecord,
    object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject,
    data: &mut DwgBitWriter,
    strings: &mut DwgBitWriter,
) -> Result<(), String> {
    if value.dashes.len() > usize::from(u8::MAX) {
        return Err(format!("linetype {:#x} has too many dashes", object.handle));
    }
    if value.dashes.iter().any(|dash| dash.text.is_some()) {
        return Err(format!("linetype {:#x} textual dash area is not yet supported", object.handle));
    }
    strings.write_tu(&value.description);
    data.write_bd(value.pattern_length);
    data.write_rc(value.alignment);
    data.write_rc(value.dashes.len() as u8);
    for dash in &value.dashes {
        data.write_bd(dash.length);
        data.write_bs(dash.complex_shape_code);
        data.write_rd(dash.x_offset);
        data.write_rd(dash.y_offset);
        data.write_bd(dash.scale);
        data.write_bd(dash.rotation);
        data.write_bs(dash.shape_flags);
    }
    Ok(())
}

/// 🔩️ Writes the main stream fields of a block header table record.
fn write_block_header_record(
    value: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgBlockHeaderTableRecord,
    object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject,
    data: &mut DwgBitWriter,
    strings: &mut DwgBitWriter,
) -> Result<(), String> {
    if (value.is_xref || value.xref_overlaid) && !value.owned_entity_handles.is_empty() {
        return Err(format!("block header {:#x} xref cannot own entities", object.handle));
    }
    if value.insert_backreference_handles.len() > 0xefffff {
        return Err(format!("block header {:#x} has too many insert backreferences", object.handle));
    }
    data.write_b(value.anonymous);
    data.write_b(value.has_attributes);
    data.write_b(value.is_xref);
    data.write_b(value.xref_overlaid);
    data.write_b(value.xref_loaded);
    if !value.is_xref && !value.xref_overlaid {
        data.write_bl(value.owned_entity_handles.len() as u32);
    }
    data.write_3bd(value.base_point);
    strings.write_tu(&value.xref_path);
    for _ in &value.insert_backreference_handles {
        data.write_rc(1);
    }
    data.write_rc(0);
    strings.write_tu(&value.description);
    data.write_bl(0);
    data.write_bs(value.insert_units);
    data.write_b(value.explodable);
    data.write_rc(value.block_scaling);
    Ok(())
}

/// ⚙️ Writes the main stream fields of a viewport table record.
fn write_viewport_record(
    value: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgViewportTableRecord,
    data: &mut DwgBitWriter,
    strings: &mut DwgBitWriter,
) -> () {
    data.write_bd(value.view_height);
    data.write_bd(value.view_width);
    data.write_2rd(value.center);
    data.write_3bd(value.target);
    data.write_3bd(value.direction);
    data.write_bd(value.twist);
    data.write_bd(value.lens_length);
    data.write_bd(value.front_clipping);
    data.write_bd(value.back_clipping);
    for flag in value.view_mode {
        data.write_b(flag);
    }
    data.write_rc(value.render_mode);
    data.write_b(value.use_default_lights);
    data.write_rc(value.default_lighting_type);
    data.write_bd(value.brightness);
    data.write_bd(value.contrast);
    data.write_bs(value.ambient_color.index);
    data.write_bl(encode_complex_color_value(&value.ambient_color.value));
    let color_flags = u8::from(value.ambient_color.name.is_some()) | (u8::from(value.ambient_color.book_name.is_some()) << 1);
    data.write_rc(color_flags);
    if let Some(name) = &value.ambient_color.name {
        strings.write_tu(name);
    }
    if let Some(book_name) = &value.ambient_color.book_name {
        strings.write_tu(book_name);
    }
    data.write_2rd(value.lower_left);
    data.write_2rd(value.upper_right);
    data.write_b(value.ucs_follow);
    data.write_bs(value.circle_zoom);
    data.write_b(value.fast_zoom);
    data.write_bb(value.ucs_icon);
    data.write_b(value.grid_mode);
    data.write_2rd(value.grid_unit);
    data.write_b(value.snap_mode);
    data.write_b(value.snap_style);
    data.write_bs(value.snap_isopair);
    data.write_bd(value.snap_angle);
    data.write_2rd(value.snap_base);
    data.write_2rd(value.snap_unit);
    data.write_b(value.ucs_at_origin);
    data.write_b(value.ucs_viewport);
    data.write_3bd(value.ucs_origin);
    data.write_3bd(value.ucs_x_axis);
    data.write_3bd(value.ucs_y_axis);
    data.write_bd(value.ucs_elevation);
    data.write_bs(value.ucs_orthographic_view);
    data.write_bs(value.grid_flags);
    data.write_bs(value.grid_major);
}

/// 🧲️ Writes the main stream fields of a dimension style table record.
fn write_dimension_style_record(
    value: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionStyleTableRecord,
    data: &mut DwgBitWriter,
    strings: &mut DwgBitWriter,
) -> () {
    let write_color = |data: &mut DwgBitWriter, strings: &mut DwgBitWriter, color: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor| {
        data.write_bs(color.index);
        data.write_bl(encode_complex_color_value(&color.value));
        data.write_rc(u8::from(color.name.is_some()) | (u8::from(color.book_name.is_some()) << 1));
        if let Some(name) = &color.name {
            strings.write_tu(name);
        }
        if let Some(book) = &color.book_name {
            strings.write_tu(book);
        }
    };
    strings.write_tu(&value.dimension_postfix);
    strings.write_tu(&value.alternate_postfix);
    let g = &value.geometry;
    data.write_bd(g.scale);
    data.write_bd(g.arrow_size);
    data.write_bd(g.extension_origin_offset);
    data.write_bd(g.dimension_line_increment);
    data.write_bd(g.extension_line_extension);
    data.write_bd(g.rounding);
    data.write_bd(g.dimension_line_extension);
    data.write_bd(g.plus_tolerance);
    data.write_bd(g.minus_tolerance);
    data.write_bd(g.fixed_extension_length);
    data.write_bd(g.jog_angle);
    data.write_bs(value.fill_mode);
    write_color(data, strings, &value.fill_color);
    let b = &value.behavior;
    data.write_b(b.tolerance);
    data.write_b(b.limits);
    data.write_b(b.text_inside_horizontal);
    data.write_b(b.text_outside_horizontal);
    data.write_b(b.suppress_extension_1);
    data.write_b(b.suppress_extension_2);
    data.write_bs(b.text_vertical_alignment);
    data.write_bs(b.zero_suppression);
    data.write_bs(b.angular_zero_suppression);
    data.write_bs(b.arc_symbol);
    let t = &value.text;
    data.write_bd(t.height);
    data.write_bd(t.center_mark_size);
    data.write_bd(t.tick_size);
    data.write_bd(t.alternate_scale);
    data.write_bd(t.linear_scale);
    data.write_bd(t.vertical_position);
    data.write_bd(t.tolerance_scale);
    data.write_bd(t.gap);
    data.write_bd(t.alternate_rounding);
    data.write_b(t.alternate_enabled);
    data.write_bs(t.alternate_decimals);
    data.write_b(t.text_outside_extensions);
    data.write_b(t.separate_arrowheads);
    data.write_b(t.force_text_inside);
    data.write_b(t.suppress_outside_extensions);
    write_color(data, strings, &t.dimension_line_color);
    write_color(data, strings, &t.extension_line_color);
    write_color(data, strings, &t.text_color);
    let u = &value.units;
    data.write_bs(u.alternate_decimal_places);
    data.write_bs(u.decimal_places);
    data.write_bs(u.tolerance_decimal_places);
    data.write_bs(u.alternate_units);
    data.write_bs(u.alternate_tolerance_decimal_places);
    data.write_bs(u.angular_units);
    data.write_bs(u.fraction_format);
    data.write_bs(u.linear_units);
    data.write_bs(u.decimal_separator);
    data.write_bs(u.text_movement);
    data.write_bs(u.text_horizontal_alignment);
    data.write_b(u.suppress_dimension_line_1);
    data.write_b(u.suppress_dimension_line_2);
    data.write_bs(u.tolerance_vertical_alignment);
    data.write_bs(u.tolerance_zero_suppression);
    data.write_bs(u.alternate_zero_suppression);
    data.write_bs(u.alternate_tolerance_zero_suppression);
    data.write_b(u.user_positioned_text);
    data.write_bs(u.arrow_text_fit);
    let r = &value.r2010;
    data.write_b(r.fixed_extension_enabled);
    data.write_b(r.text_direction);
    data.write_bd(r.alternate_measurement_factor);
    strings.write_tu(&r.alternate_measurement_suffix);
    data.write_bd(r.measurement_factor);
    strings.write_tu(&r.measurement_suffix);
    data.write_bs(r.dimension_lineweight);
    data.write_bs(r.extension_lineweight);
    data.write_b(r.flag);
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
