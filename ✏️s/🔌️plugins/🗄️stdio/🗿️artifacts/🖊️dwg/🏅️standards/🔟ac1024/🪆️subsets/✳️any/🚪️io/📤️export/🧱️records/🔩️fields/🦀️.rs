//! 🔩️ Object-record field writers: handles, extended entity data, xrecord values, colors, cell styles, block elements, constraint nodes and string streams.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::dwg_crc16;
use crate::standards::v_ac1024::subsets::any::io::export::records::bits::{DwgBitWriter};

/// 📦 Serialized objects with handle and byte-offset entries.
pub type DwgObjectFrame = (Vec<u8>, Vec<(u64, usize)>);
//#region 🔖️DwgStructuralCodec
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_object_handle(writer: &mut DwgBitWriter, base: u64, target: Option<u64>) {
    match target {
        None | Some(0) => writer.write_handle(4, 0),
        Some(value) if value == base.saturating_add(1) => writer.write_handle(6, 0),
        Some(value) if value.saturating_add(1) == base => writer.write_handle(8, 0),
        Some(value) if value > base && value - base < value => writer.write_handle(10, value - base),
        Some(value) if value < base && base - value < value => writer.write_handle(12, base - value),
        Some(value) => writer.write_handle(4, value),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_xrecord_values(values: &[semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgXRecordValue]) -> Result<DwgBitWriter, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgXRecordValue;
    let mut data = DwgBitWriter::new();
    for value in values {
        value.validate()?;
        let group_code = match value {
            DwgXRecordValue::String { group_code, .. }
            | DwgXRecordValue::Real { group_code, .. }
            | DwgXRecordValue::Boolean { group_code, .. }
            | DwgXRecordValue::Integer8 { group_code, .. }
            | DwgXRecordValue::Integer16 { group_code, .. }
            | DwgXRecordValue::Integer32 { group_code, .. }
            | DwgXRecordValue::Integer64 { group_code, .. }
            | DwgXRecordValue::Point3d { group_code, .. }
            | DwgXRecordValue::Binary { group_code, .. }
            | DwgXRecordValue::Handle { group_code, .. }
            | DwgXRecordValue::ObjectId { group_code, .. } => *group_code,
        };
        data.write_rs(group_code as u16);
        match value {
            DwgXRecordValue::String { value, .. } => {
                let units = value.encode_utf16().collect::<Vec<_>>();
                data.write_rs(units.len() as u16);
                for unit in units {
                    data.write_rs(unit);
                }
            }
            DwgXRecordValue::Real { value, .. } => data.write_rd(*value),
            DwgXRecordValue::Boolean { value, .. } => data.write_rc(u8::from(*value)),
            DwgXRecordValue::Integer8 { value, .. } => data.write_rc(*value as u8),
            DwgXRecordValue::Integer16 { value, .. } => data.write_rs(*value as u16),
            DwgXRecordValue::Integer32 { value, .. } => data.write_rl(*value as u32),
            DwgXRecordValue::Integer64 { value, .. } => data.write_rll(*value as u64),
            DwgXRecordValue::Point3d { value, .. } => data.write_3rd(*value),
            DwgXRecordValue::Binary { octets, .. } => {
                data.write_rc(octets.len() as u8);
                for octet in octets {
                    data.write_rc(*octet);
                }
            }
            DwgXRecordValue::Handle { value, .. } => data.write_rll(*value),
            DwgXRecordValue::ObjectId { absolute_value, .. } => data.write_rll(*absolute_value),
        }
    }
    if data.bit != 0 {
        return Err("typed XRECORD values must occupy whole bytes".into());
    }
    Ok(data)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_eed(writer: &mut DwgBitWriter, _base: u64, records: &[semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgExtendedEntityData]) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgXRecordValue;
    for record in records {
        let mut values = DwgBitWriter::new();
        for value in &record.values {
            value.validate()?;
            match value {
                DwgXRecordValue::String { group_code: 1000, value } => {
                    values.write_rc(0);
                    let units = value.encode_utf16().collect::<Vec<_>>();
                    values.write_rs(units.len() as u16);
                    for unit in units {
                        values.write_rs(unit);
                    }
                }
                DwgXRecordValue::String { group_code: 1002, value } => {
                    values.write_rc(2);
                    values.write_rc(match value.as_str() {
                        "{" => 0,
                        "}" => 1,
                        _ => return Err("R2010 EED control string must be '{' or '}'".into()),
                    });
                }
                DwgXRecordValue::Handle { group_code: 1003, value } => {
                    values.write_rc(3);
                    values.write_rll(*value);
                }
                DwgXRecordValue::Binary { group_code: 1004, octets } => {
                    values.write_rc(4);
                    values.write_rc(octets.len() as u8);
                    for octet in octets {
                        values.write_rc(*octet);
                    }
                }
                DwgXRecordValue::Handle { group_code: 1005, value } => {
                    values.write_rc(5);
                    for shift in (0..64).step_by(8).rev() {
                        values.write_rc((value >> shift) as u8);
                    }
                }
                DwgXRecordValue::Point3d { group_code, value } if (1010..=1015).contains(group_code) => {
                    values.write_rc((*group_code - 1000) as u8);
                    values.write_3rd(*value);
                }
                DwgXRecordValue::Real { group_code, value } if (1040..=1042).contains(group_code) => {
                    values.write_rc((*group_code - 1000) as u8);
                    values.write_rd(*value);
                }
                DwgXRecordValue::Integer16 { group_code: 1070, value } => {
                    values.write_rc(70);
                    values.write_rs(*value as u16);
                }
                DwgXRecordValue::Integer32 { group_code: 1071, value } => {
                    values.write_rc(71);
                    values.write_rl(*value as u32);
                }
                _ => return Err(format!("group {} is not a standard R2010 EED value", value.group_code())),
            }
        }
        if values.bit != 0 || values.bytes.len() > usize::from(u16::MAX) {
            return Err("R2010 EED application record is not byte-sized".into());
        }
        writer.write_bs(values.bytes.len() as u16);
        writer.write_handle(5, record.application_handle);
        writer.append_bits(&values);
    }
    writer.write_bs(0);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn finish_r2010_object_frame(data: &DwgBitWriter, mut handles: DwgBitWriter) -> Result<Vec<u8>, String> {
    while !(data.bit_len() + handles.bit_len()).is_multiple_of(8) {
        handles.write_b(true);
    }
    let handle_stream_bits = handles.bit_len();
    let mut payload = DwgBitWriter::new();
    payload.append_bits(data);
    payload.append_bits(&handles);
    if payload.bit != 0 {
        return Err("R2010 object frame is not byte-aligned".into());
    }
    let mut framed = DwgBitWriter::new();
    framed.write_ms(payload.bytes.len() as u32);
    framed.write_umc(handle_stream_bits as u64);
    framed.append_bits(&payload);
    if framed.bit != 0 {
        return Err("R2010 outer object frame is not byte-aligned".into());
    }
    let crc = dwg_crc16(0xC0C1, &framed.bytes);
    framed.bytes.extend_from_slice(&crc.to_le_bytes());
    Ok(framed.bytes)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn logical_point3(values: &[f64], name: &str) -> Result<[f64; 3], String> {
    if values.len() != 3 || values.iter().any(|value| !value.is_finite()) {
        return Err(format!("{name} must contain exactly three finite coordinates"));
    }
    Ok([values[0], values[1], values[2]])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn logical_point2(values: &[f64], name: &str) -> Result<[f64; 2], String> {
    if values.len() != 2 || values.iter().any(|value| !value.is_finite()) {
        return Err(format!("{name} must contain exactly two finite coordinates"));
    }
    Ok([values[0], values[1]])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn append_r2010_string_stream(data: &mut DwgBitWriter, strings: &DwgBitWriter, class_name: &str, handle: u64) -> Result<(), String> {
    let string_bits = strings.bit_len();
    if string_bits == 0 {
        data.write_b(false);
    } else {
        if string_bits > 0x3fff_ffff {
            return Err(format!("{class_name} {handle:#x} string stream size exceeds R2010 range"));
        }
        data.append_bits(strings);
        if string_bits <= 0x7fff {
            data.write_rs(string_bits as u16);
        } else {
            data.write_rs((string_bits >> 15) as u16);
            data.write_rs(((string_bits & 0x7fff) as u16) | 0x8000);
        }
        data.write_b(true);
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_visual_style_operation(data: &mut DwgBitWriter, operation: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStylePropertyOperation) {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStylePropertyOperation;
    data.write_bs(match operation {
        DwgVisualStylePropertyOperation::Inherit => 0,
        DwgVisualStylePropertyOperation::Set => 1,
        DwgVisualStylePropertyOperation::Disable => 2,
        DwgVisualStylePropertyOperation::Enable => 3,
    });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_visual_style_color(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, property: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStyleProperty<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor>) {
    data.write_bs(property.value.index);
    data.write_bl(encode_complex_color_value(&property.value.value));
    let flags = u8::from(property.value.name.is_some()) | (u8::from(property.value.book_name.is_some()) << 1);
    data.write_rc(flags);
    if let Some(name) = &property.value.name {
        strings.write_tu(name);
    }
    if let Some(book) = &property.value.book_name {
        strings.write_tu(book);
    }
    write_visual_style_operation(data, property.operation);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_table_style_color(data: &mut DwgBitWriter, color: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor) -> Result<(), String> {
    if color.name.is_some() || color.book_name.is_some() {
        return Err("TABLESTYLE named colors are unsupported".into());
    }
    data.write_bs(color.index);
    data.write_bl(encode_complex_color_value(&color.value));
    data.write_rc(0);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn table_style_borders(borders: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellBorders) -> [(u32, Option<&semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellBorder>); 6] {
    [(1, borders.top.as_ref()), (2, borders.horizontal_inside.as_ref()), (4, borders.bottom.as_ref()), (8, borders.left.as_ref()), (16, borders.vertical_inside.as_ref()), (32, borders.right.as_ref())]
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_cell_style(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, handles: &mut DwgBitWriter, style: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellStyle) -> Result<(), String> {
    data.write_bl(5);
    data.write_bs(1);
    data.write_bl(style.property_override_flags);
    data.write_bl(style.merge_flags);
    write_table_style_color(data, &style.background_color)?;
    data.write_bl(style.content_layout);
    data.write_bl(style.content_format.property_override_flags);
    data.write_bl(style.content_format.property_flags);
    data.write_bl(style.content_format.value_data_type);
    data.write_bl(style.content_format.value_unit_type);
    strings.write_tu(&style.content_format.value_format_string);
    data.write_bd(style.content_format.rotation);
    data.write_bd(style.content_format.block_scale);
    data.write_bl(style.content_format.alignment);
    write_table_style_color(data, &style.content_format.content_color)?;
    handles.write_handle(5, style.content_format.text_style_handle.unwrap_or_default());
    data.write_bd(style.content_format.text_height);
    data.write_bs(1);
    data.write_bd(style.margins.vertical);
    data.write_bd(style.margins.horizontal);
    data.write_bd(style.margins.bottom);
    data.write_bd(style.margins.right);
    data.write_bd(style.margins.horizontal_spacing);
    data.write_bd(style.margins.vertical_spacing);
    let borders = table_style_borders(&style.borders);
    data.write_bl(borders.iter().filter(|(_, border)| border.is_some()).count() as u32);
    for (mask, border) in borders {
        let Some(border) = border else { continue };
        data.write_bl(mask);
        data.write_bl(border.override_flags);
        data.write_bl(border.border_type);
        write_table_style_color(data, &border.color)?;
        data.write_bl(border.lineweight as u32);
        handles.write_handle(5, border.linetype_handle.unwrap_or_default());
        data.write_bl(border.visible);
        data.write_bd(border.double_line_spacing);
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_material_color(data: &mut DwgBitWriter, color: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialColor) {
    data.write_rc(u8::from(color.override_rgb.is_some()));
    data.write_bd(color.factor);
    if let Some(rgb) = color.override_rgb {
        data.write_bl(rgb);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_material_map(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, map: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialMap) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialMapSource;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialProjection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialTiling;
    if map.transform.len() != 16 || map.transform.iter().any(|value| !value.is_finite()) {
        return Err("MATERIAL mapper transform must contain sixteen finite values".into());
    }
    data.write_bd(map.blend_factor);
    data.write_rc(match map.projection {
        DwgMaterialProjection::Inherit => 0,
        DwgMaterialProjection::Planar => 1,
        DwgMaterialProjection::Box => 2,
        DwgMaterialProjection::Cylinder => 3,
        DwgMaterialProjection::Sphere => 4,
    });
    data.write_rc(match map.tiling {
        DwgMaterialTiling::Inherit => 0,
        DwgMaterialTiling::Tile => 1,
        DwgMaterialTiling::Crop => 2,
        DwgMaterialTiling::Clamp => 3,
        DwgMaterialTiling::Mirror => 4,
    });
    data.write_rc(if !map.scale_to_entity && !map.use_current_block_transform { 1 } else { u8::from(map.scale_to_entity) << 1 | u8::from(map.use_current_block_transform) << 2 });
    for value in &map.transform {
        data.write_bd(*value);
    }
    match map.source {
        DwgMaterialMapSource::None => {
            data.write_rc(1);
            strings.write_tu("");
        }
        DwgMaterialMapSource::CurrentScene => data.write_rc(0),
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn constraint_point3(values: &[f64], role: &str) -> Result<[f64; 3], String> {
    if values.len() != 3 || values.iter().any(|value| !value.is_finite()) {
        return Err(format!("{role} must contain three finite coordinates"));
    }
    Ok([values[0], values[1], values[2]])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_constraint_node(node: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintNode, data: &mut DwgBitWriter, strings: &mut DwgBitWriter, handles: &mut DwgBitWriter) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintNode;
    fn core(data: &mut DwgBitWriter, node: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintNodeCore) -> Result<(), String> {
        if node.id < 0 || node.connected_node_ids.len() > 10_000 {
            return Err("constraint node ID or connection count is invalid".into());
        }
        data.write_bl(node.id as u32);
        data.write_bl(node.connected_node_ids.len() as u32);
        for connection in &node.connected_node_ids {
            data.write_bl(*connection);
        }
        Ok(())
    }
    fn geometric(data: &mut DwgBitWriter, value: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgGeometricConstraint) -> Result<(), String> {
        if !value.active {
            return Err("AC1024 geometric constraint must be active".into());
        }
        core(data, &value.node)?;
        data.write_bl(value.owner_node_id);
        data.write_b(value.implied);
        Ok(())
    }
    fn geometry(data: &mut DwgBitWriter, handles: &mut DwgBitWriter, value: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintGeometry) -> Result<(), String> {
        core(data, &value.node)?;
        handles.write_handle(4, value.geometry_dependency_handle.unwrap_or_default());
        data.write_bl(value.geometry_node_id);
        Ok(())
    }
    fn explicit(data: &mut DwgBitWriter, handles: &mut DwgBitWriter, value: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgExplicitConstraint) -> Result<(), String> {
        geometric(data, &value.geometric)?;
        if value.value_dependency_handle == 0 || value.dimension_dependency_handle == 0 {
            return Err("explicit constraint dependency handles must be nonnull".into());
        }
        handles.write_handle(5, value.value_dependency_handle);
        handles.write_handle(5, value.dimension_dependency_handle);
        Ok(())
    }
    let class = match node {
        DwgConstraintNode::ConstrainedImplicitPoint(value) => {
            geometry(data, handles, &value.geometry)?;
            if value.geometry.geometry_dependency_handle.is_some() {
                data.write_3bd(constraint_point3(value.point.as_deref().ok_or("dependent implicit point is missing its point")?, "implicit point")?);
            } else if value.point.is_some() {
                return Err("independent implicit point must not persist a conditional point".into());
            }
            data.write_rc(value.point_kind);
            data.write_bl(value.point_index as u32);
            data.write_bl(value.curve_node_id as u32);
            "AcConstrainedImplicitPoint"
        }
        DwgConstraintNode::PointCurveConstraint(value) => {
            geometric(data, value)?;
            "AcPointCurveConstraint"
        }
        DwgConstraintNode::ConstrainedBoundedLine(value) => {
            geometry(data, handles, &value.geometry)?;
            if value.bounded == value.ray {
                return Err("bounded-line ray and bounded states must be complementary".into());
            }
            data.write_3bd(constraint_point3(&value.origin, "bounded-line origin")?);
            data.write_3bd(constraint_point3(&value.direction, "bounded-line direction")?);
            data.write_b(value.ray);
            data.write_3bd(constraint_point3(&value.start_point, "bounded-line start")?);
            data.write_3bd(constraint_point3(&value.end_point, "bounded-line end")?);
            "AcConstrainedBoundedLine"
        }
        DwgConstraintNode::PointCoincidenceConstraint(value) => {
            geometric(data, value)?;
            "AcPointCoincidenceConstraint"
        }
        DwgConstraintNode::DistanceConstraint(value) => {
            explicit(data, handles, &value.explicit)?;
            data.write_rc(value.direction_kind);
            if value.direction_kind != 0 {
                data.write_3bd(constraint_point3(value.direction.as_deref().ok_or("directed distance is missing its direction")?, "distance direction")?);
            } else if value.direction.is_some() {
                return Err("undirected distance must not persist a conditional direction".into());
            }
            "AcDistanceConstraint"
        }
        DwgConstraintNode::PerpendicularConstraint(value) => {
            geometric(data, value)?;
            "AcPerpendicularConstraint"
        }
        DwgConstraintNode::HorizontalConstraint(value) => {
            geometric(data, &value.geometric)?;
            data.write_bl(value.datum_line_index as u32);
            "AcHorizontalConstraint"
        }
        DwgConstraintNode::ParallelConstraint(value) => {
            geometric(data, value)?;
            "AcParallelConstraint"
        }
        DwgConstraintNode::MidPointConstraint(value) => {
            geometric(data, value)?;
            "AcMidPointConstraint"
        }
        DwgConstraintNode::EqualLengthConstraint(value) => {
            geometric(data, value)?;
            "AcEqualLengthConstraint"
        }
        DwgConstraintNode::ColinearConstraint(value) => {
            geometric(data, value)?;
            "AcColinearConstraint"
        }
        DwgConstraintNode::ConstrainedDatumLine(value) => {
            geometry(data, handles, &value.geometry)?;
            data.write_3bd(constraint_point3(&value.origin, "datum-line origin")?);
            data.write_3bd(constraint_point3(&value.direction, "datum-line direction")?);
            "AcConstrainedDatumLine"
        }
        DwgConstraintNode::FixedConstraint(value) => {
            geometric(data, value)?;
            "AcFixedConstraint"
        }
        DwgConstraintNode::VerticalConstraint(value) => {
            geometric(data, &value.geometric)?;
            data.write_bl(value.datum_line_index as u32);
            "AcVerticalConstraint"
        }
    };
    strings.write_tu(class);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_block_element(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, element: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockElement, class_name: &str) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    let expression = &element.evaluation_expression;
    if expression.parent_id != -1 || expression.major_version != 29 || expression.minor_version != 2 || !matches!(expression.value, DwgEvaluationExpressionValue::Empty) || element.name.is_empty() {
        return Err(format!("{class_name} block-element state is invalid"));
    }
    data.write_bl(expression.parent_id as u32);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    data.write_bs((-9999i16) as u16);
    data.write_bl(expression.node_id);
    strings.write_tu(&element.name);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    data.write_bl(0);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_block_grip(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, grip: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockGrip, x_role: &str, y_role: &str, class_name: &str) -> Result<(), String> {
    if grip.location.len() != 3 || grip.location.iter().any(|value| !value.is_finite()) || grip.updated_x.expression_name != x_role || grip.updated_y.expression_name != y_role {
        return Err(format!("{class_name} grip state is invalid"));
    }
    encode_r2010_block_element(data, strings, &grip.element, class_name)?;
    data.write_bl(grip.updated_x.node_id);
    data.write_bl(grip.updated_y.node_id);
    data.write_3bd([grip.location[0], grip.location[1], grip.location[2]]);
    data.write_b(grip.insertion_cycling);
    data.write_bl(grip.insertion_cycling_weight as u32);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_two_point_parameter(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, parameter: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockTwoPointParameter, property_node_ids: [u32; 4], class_name: &str) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterBaseLocation;
    if parameter.definition_base.len() != 3 || parameter.definition_end.len() != 3 || parameter.properties.len() != 4 || parameter.definition_base.iter().chain(&parameter.definition_end).any(|value| !value.is_finite()) {
        return Err(format!("{class_name} two-point parameter is invalid"));
    }
    encode_r2010_block_element(data, strings, &parameter.element, class_name)?;
    data.write_b(parameter.show_properties);
    data.write_b(parameter.chain_actions);
    data.write_3bd([parameter.definition_base[0], parameter.definition_base[1], parameter.definition_base[2]]);
    data.write_3bd([parameter.definition_end[0], parameter.definition_end[1], parameter.definition_end[2]]);
    for property in &parameter.properties {
        data.write_bl(property.connections.len() as u32);
        for connection in &property.connections {
            data.write_bl(connection.code);
            strings.write_tu(&connection.name);
        }
    }
    for node_id in property_node_ids {
        data.write_bl(node_id);
    }
    data.write_bs(match parameter.base_location {
        DwgBlockParameterBaseLocation::StartPoint => 0,
        DwgBlockParameterBaseLocation::Midpoint => 1,
    });
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_block_action(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, handles: &mut DwgBitWriter, action: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockAction, class_name: &str) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockElement;
    if action.display_location.len() != 3 || action.display_location.iter().any(|value| !value.is_finite()) || action.name.is_empty() {
        return Err(format!("{class_name} action is invalid"));
    }
    encode_r2010_block_element(data, strings, &DwgBlockElement { evaluation_expression: action.evaluation_expression.clone(), name: action.name.clone() }, class_name)?;
    data.write_3bd([action.display_location[0], action.display_location[1], action.display_location[2]]);
    data.write_bl(action.dependencies.len() as u32);
    data.write_bl(action.action_node_ids.len() as u32);
    for node_id in &action.action_node_ids {
        data.write_bl(*node_id);
    }
    for dependency in &action.dependencies {
        if dependency.object_handle == 0 {
            return Err(format!("{class_name} dependency is null"));
        }
        handles.write_handle(4, dependency.object_handle);
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_r2010_action_connection(data: &mut DwgBitWriter, strings: &mut DwgBitWriter, connection: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionConnection) -> Result<(), String> {
    if connection.name.is_empty() {
        return Err("block-action connection name is empty".into());
    }
    data.write_bl(connection.node_id);
    strings.write_tu(&connection.name);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_r2004_unsigned_modular_char(output: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        output.push((value as u8 & 0x7f) | 0x80);
        value >>= 7;
    }
    output.push(value as u8);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_r2004_signed_modular_char(output: &mut Vec<u8>, value: i64) {
    let negative = value < 0;
    let mut magnitude = value.unsigned_abs();
    while magnitude >= 0x40 {
        output.push((magnitude as u8 & 0x7f) | 0x80);
        magnitude >>= 7;
    }
    output.push(magnitude as u8 | if negative { 0x40 } else { 0 });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_complex_color_value(value: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColorValue) -> u32 {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColorValue;
    match value {
        DwgComplexColorValue::ByLayer => 0xc000_0000,
        DwgComplexColorValue::ByBlock => 0xc100_0000,
        DwgComplexColorValue::ByColor { red, green, blue } => 0xc200_0000 | (u32::from(*red) << 16) | (u32::from(*green) << 8) | u32::from(*blue),
        DwgComplexColorValue::ByAci { index } => 0xc300_0000 | u32::from(*index),
        DwgComplexColorValue::ByPen { index } => 0xc400_0000 | u32::from(*index),
        DwgComplexColorValue::Foreground => 0xc500_0000,
        DwgComplexColorValue::LayerOff => 0xc600_0000,
        DwgComplexColorValue::LayerFrozen => 0xc700_0000,
        DwgComplexColorValue::None => 0xc800_0000,
    }
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
