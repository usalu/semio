//! 🔩️ Object-record field readers: handles, extended entity data, xrecord values, colors, block elements, constraint nodes, cell styles and string streams.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DwgBitReader;

/// 🔗 Decoded common object relation handles.
pub type DwgObjectRelations = (Option<u64>, Vec<u64>, Option<u64>);
//#region 🔖️DwgStructuralCodec
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_r2004_modular_char(bytes: &[u8], position: &mut usize, signed: bool) -> Result<i64, String> {
    let mut value = 0i64;
    let mut shift = 0u32;
    loop {
        let byte = *bytes.get(*position).ok_or("R2004 modular-char underflow")?;
        *position += 1;
        let negative = signed && byte & 0x80 == 0 && byte & 0x40 != 0;
        value |= ((byte & if negative { 0x3f } else { 0x7f }) as i64) << shift;
        if byte & 0x80 == 0 {
            if negative {
                value = -value;
            }
            return Ok(value);
        }
        shift += 7;
        if shift > 56 {
            return Err("R2004 modular-char overflow".into());
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2004_handle_map(bytes: &[u8]) -> Result<Vec<(u64, usize)>, String> {
    let mut position = 0usize;
    let mut entries = Vec::new();
    while position + 2 <= bytes.len() {
        let block_start = position;
        let block_size = u16::from_be_bytes([bytes[position], bytes[position + 1]]) as usize;
        position += 2;
        if block_size <= 2 {
            break;
        }
        let block_end = block_start.checked_add(block_size).ok_or("R2004 handle-map block overflow")?;
        if block_end > bytes.len() {
            return Err(format!("R2004 handle-map block {block_size} exceeds {} bytes", bytes.len() - block_start));
        }
        let mut handle = 0i64;
        let mut address = 0i64;
        while position < block_end {
            handle += read_r2004_modular_char(bytes, &mut position, false)?;
            address += read_r2004_modular_char(bytes, &mut position, true)?;
            if handle < 0 || address < 0 {
                return Err("R2004 handle-map produced a negative handle or object address".into());
            }
            entries.push((handle as u64, address as usize));
        }
        position = block_end.saturating_add(2).min(bytes.len());
    }
    Ok(entries)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2010_eed(reader: &mut DwgBitReader<'_>, _base: u64) -> Result<Vec<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgExtendedEntityData>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgExtendedEntityData;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgXRecordValue;
    let mut records = Vec::new();
    loop {
        let size = reader.read_bs()? as usize;
        if size == 0 {
            return Ok(records);
        }
        let (handle_code, handle_value) = reader.read_handle()?;
        if handle_code != 5 || handle_value == 0 {
            return Err(format!("R2010 EED application reference must be a non-null hard pointer, found code={handle_code} value={handle_value:#x}"));
        }
        let application_handle = handle_value;
        let end_bit = reader.bit_position().checked_add(size.checked_mul(8).ok_or("R2010 EED size overflow")?).ok_or("R2010 EED end overflow")?;
        if end_bit > reader.bytes.len().saturating_mul(8) {
            return Err("R2010 EED record exceeds object payload".into());
        }
        let mut values = Vec::new();
        while reader.bit_position() < end_bit {
            let code = reader.read_rc()?;
            let value = match code {
                0 => {
                    let length = reader.read_rs()? as usize;
                    let units = (0..length).map(|_| reader.read_rs()).collect::<Result<Vec<_>, _>>()?;
                    DwgXRecordValue::String { group_code: 1000, value: String::from_utf16(&units).map_err(|error| format!("invalid R2010 EED UTF-16 string: {error}"))? }
                }
                2 => {
                    let close = reader.read_rc()?;
                    let value = match close {
                        0 => "{",
                        1 => "}",
                        other => return Err(format!("R2010 EED control-string value {other} is invalid")),
                    };
                    DwgXRecordValue::String { group_code: 1002, value: value.into() }
                }
                3 => DwgXRecordValue::Handle { group_code: 1003, value: reader.read_rll()? },
                4 => {
                    let length = reader.read_rc()? as usize;
                    let octets = (0..length).map(|_| reader.read_rc()).collect::<Result<Vec<_>, _>>()?;
                    DwgXRecordValue::Binary { group_code: 1004, octets }
                }
                5 => {
                    let mut value = 0u64;
                    for _ in 0..8 {
                        value = (value << 8) | u64::from(reader.read_rc()?);
                    }
                    DwgXRecordValue::Handle { group_code: 1005, value }
                }
                10..=15 => DwgXRecordValue::Point3d { group_code: 1000 + i16::from(code), value: [reader.read_rd()?, reader.read_rd()?, reader.read_rd()?] },
                40..=42 => DwgXRecordValue::Real { group_code: 1000 + i16::from(code), value: reader.read_rd()? },
                70 => DwgXRecordValue::Integer16 { group_code: 1070, value: reader.read_rs()? as i16 },
                71 => DwgXRecordValue::Integer32 { group_code: 1071, value: reader.read_rl()? as i32 },
                1 => return Err("R2010 EED application-index item is redundant with its typed application handle".into()),
                other => return Err(format!("unknown R2010 EED item code {other}")),
            };
            value.validate()?;
            if reader.bit_position() > end_bit {
                return Err(format!("R2010 EED item code {code} exceeds its declared record size"));
            }
            values.push(value);
        }
        if reader.bit_position() != end_bit {
            return Err("R2010 EED record is not exactly consumed".into());
        }
        records.push(DwgExtendedEntityData { application_handle, values });
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn fixed_object_name(object_type: u16) -> &'static str {
    match object_type {
        1 => "TEXT",
        2 => "ATTRIB",
        3 => "ATTDEF",
        4 => "BLOCK",
        5 => "ENDBLK",
        6 => "SEQEND",
        7 => "INSERT",
        8 => "MINSERT",
        10 => "VERTEX_2D",
        11 => "VERTEX_3D",
        12 => "VERTEX_MESH",
        13 => "VERTEX_PFACE",
        14 => "VERTEX_PFACE_FACE",
        15 => "POLYLINE_2D",
        16 => "POLYLINE_3D",
        17 => "ARC",
        18 => "CIRCLE",
        19 => "LINE",
        20..=26 => "DIMENSION",
        27 => "POINT",
        28 => "3DFACE",
        29 => "POLYLINE_PFACE",
        30 => "POLYLINE_MESH",
        31 => "SOLID",
        32 => "TRACE",
        33 => "SHAPE",
        34 => "VIEWPORT",
        35 => "ELLIPSE",
        36 => "SPLINE",
        37 => "REGION",
        38 => "3DSOLID",
        39 => "BODY",
        40 => "RAY",
        41 => "XLINE",
        42 => "DICTIONARY",
        43 => "OLEFRAME",
        44 => "MTEXT",
        45 => "LEADER",
        46 => "TOLERANCE",
        47 => "MLINE",
        48 => "BLOCK_CONTROL",
        49 => "BLOCK_HEADER",
        50 => "LAYER_CONTROL",
        51 => "LAYER",
        52 => "STYLE_CONTROL",
        53 => "STYLE",
        56 => "LTYPE_CONTROL",
        57 => "LTYPE",
        60 => "VIEW_CONTROL",
        61 => "VIEW",
        62 => "UCS_CONTROL",
        63 => "UCS",
        64 => "VPORT_CONTROL",
        65 => "VPORT",
        66 => "APPID_CONTROL",
        67 => "APPID",
        68 => "DIMSTYLE_CONTROL",
        69 => "DIMSTYLE",
        70 => "VP_ENT_HDR_CONTROL",
        71 => "VP_ENT_HDR",
        72 => "GROUP",
        73 => "MLINESTYLE",
        74 => "OLE2FRAME",
        75 => "DUMMY",
        76 => "LONG_TRANSACTION",
        77 => "LWPOLYLINE",
        78 => "HATCH",
        79 => "XRECORD",
        80 => "ACDBPLACEHOLDER",
        81 => "VBA_PROJECT",
        82 => "LAYOUT",
        498 => "ACAD_PROXY_ENTITY",
        499 => "ACAD_PROXY_OBJECT",
        _ => "UNKNOWN",
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn object_category(object_type: u16) -> semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgObjectCategory {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgObjectCategory;
    match object_type {
        1..=41 | 43..=47 | 77 | 78 | 498 => DwgObjectCategory::Entity,
        48 | 50 | 52 | 56 | 60 | 62 | 64 | 66 | 68 | 70 => DwgObjectCategory::TableControl,
        49 | 51 | 53 | 57 | 61 | 63 | 65 | 67 | 69 | 71 => DwgObjectCategory::TableRecord,
        42 => DwgObjectCategory::Dictionary,
        500.. => DwgObjectCategory::Custom,
        _ => DwgObjectCategory::Object,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn resolve_object_handle(base: u64, code: u8, value: u64) -> Option<u64> {
    let resolved = match code {
        6 => base.checked_add(1)?,
        8 => base.checked_sub(1)?,
        10 => base.checked_add(value)?,
        12 => base.checked_sub(value)?,
        _ => value,
    };
    (resolved != 0).then_some(resolved)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_object_handle(reader: &mut DwgBitReader<'_>, base: u64) -> Result<Option<u64>, String> {
    let (code, value) = reader.read_handle()?;
    Ok(resolve_object_handle(base, code, value))
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum XRecordStorageKind {
    String,
    Real,
    Boolean,
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Point3d,
    Binary,
    Handle,
    ObjectId,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xrecord_value_kind(group_code: i16) -> Option<XRecordStorageKind> {
    let code = i32::from(group_code);
    Some(match code {
        5 | 105 | 320..=329 | 390..=399 | 1003 | 1005 => XRecordStorageKind::Handle,
        0..=4 | 6..=9 | 100..=104 | 300..=309 | 410..=419 | 430..=439 | 470..=479 | 999 | 1000..=1002 => XRecordStorageKind::String,
        10..=37 | 110..=139 | 210..=269 | 1010..=1015 => XRecordStorageKind::Point3d,
        38..=59 | 140..=149 | 460..=469 | 1040..=1042 => XRecordStorageKind::Real,
        60..=79 | 170..=179 | 270..=279 | 370..=389 | 400..=409 | 1070 => XRecordStorageKind::Integer16,
        90..=99 | 420..=429 | 440..=459 | 1071 => XRecordStorageKind::Integer32,
        160..=169 => XRecordStorageKind::Integer64,
        280..=289 => XRecordStorageKind::Integer8,
        290..=299 => XRecordStorageKind::Boolean,
        310..=319 | 1004 => XRecordStorageKind::Binary,
        330..=369 => XRecordStorageKind::ObjectId,
        _ => return None,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_xrecord_values(data: &mut DwgBitReader<'_>, byte_count: usize, main_end_bit: usize) -> Result<Vec<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgXRecordValue>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgXRecordValue;
    let end_bit = data.bit_position().checked_add(byte_count.checked_mul(8).ok_or("XRECORD value size overflow")?).ok_or("XRECORD value boundary overflow")?;
    if end_bit > main_end_bit {
        return Err("XRECORD values exceed bounded class-main data".into());
    }
    let mut values = Vec::new();
    while data.bit_position() < end_bit {
        if end_bit.saturating_sub(data.bit_position()) < 16 {
            return Err("XRECORD group code is truncated".into());
        }
        let group_code = data.read_rs()? as i16;
        let kind = xrecord_value_kind(group_code).ok_or_else(|| format!("unsupported XRECORD group code {group_code}"))?;
        let value = match kind {
            XRecordStorageKind::String => {
                let length = data.read_rs()? as usize;
                if length > end_bit.saturating_sub(data.bit_position()) / 16 {
                    return Err(format!("XRECORD string for group {group_code} exceeds value boundary"));
                }
                let units = (0..length).map(|_| data.read_rs()).collect::<Result<Vec<_>, _>>()?;
                DwgXRecordValue::String { group_code, value: String::from_utf16(&units).map_err(|error| format!("invalid XRECORD UTF-16 string: {error}"))? }
            }
            XRecordStorageKind::Real => DwgXRecordValue::Real { group_code, value: data.read_rd()? },
            XRecordStorageKind::Boolean => {
                let stored = data.read_rc()?;
                if stored > 1 {
                    return Err(format!("XRECORD boolean group {group_code} has invalid value {stored}"));
                }
                DwgXRecordValue::Boolean { group_code, value: stored != 0 }
            }
            XRecordStorageKind::Integer8 => DwgXRecordValue::Integer8 { group_code, value: data.read_rc()? as i8 },
            XRecordStorageKind::Integer16 => DwgXRecordValue::Integer16 { group_code, value: data.read_rs()? as i16 },
            XRecordStorageKind::Integer32 => DwgXRecordValue::Integer32 { group_code, value: data.read_rl()? as i32 },
            XRecordStorageKind::Integer64 => DwgXRecordValue::Integer64 { group_code, value: data.read_rll()? as i64 },
            XRecordStorageKind::Point3d => DwgXRecordValue::Point3d { group_code, value: [data.read_rd()?, data.read_rd()?, data.read_rd()?] },
            XRecordStorageKind::Binary => {
                let length = data.read_rc()? as usize;
                if length > end_bit.saturating_sub(data.bit_position()) / 8 {
                    return Err(format!("XRECORD binary value for group {group_code} exceeds value boundary"));
                }
                DwgXRecordValue::Binary { group_code, octets: (0..length).map(|_| data.read_rc()).collect::<Result<Vec<_>, _>>()? }
            }
            XRecordStorageKind::Handle => DwgXRecordValue::Handle { group_code, value: data.read_rll()? },
            XRecordStorageKind::ObjectId => {
                let absolute_value = data.read_rll()?;
                DwgXRecordValue::ObjectId { group_code, absolute_value }
            }
        };
        if data.bit_position() > end_bit {
            return Err(format!("XRECORD value for group {group_code} exceeds declared size"));
        }
        values.push(value);
    }
    if data.bit_position() != end_bit {
        return Err("XRECORD values do not consume their declared size".into());
    }
    Ok(values)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_visual_style_operation(data: &mut DwgBitReader<'_>) -> Result<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStylePropertyOperation, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStylePropertyOperation;
    match data.read_bs()? {
        0 => Ok(DwgVisualStylePropertyOperation::Inherit),
        1 => Ok(DwgVisualStylePropertyOperation::Set),
        2 => Ok(DwgVisualStylePropertyOperation::Disable),
        3 => Ok(DwgVisualStylePropertyOperation::Enable),
        value => Err(format!("visual-style property operation {value} is invalid")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_visual_style_color(data: &mut DwgBitReader<'_>) -> Result<(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStyleProperty<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor>, u8), String> {
    let (value, flags) = read_r2010_cmc_main(data)?;
    let operation = read_visual_style_operation(data)?;
    Ok((semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStyleProperty { value, operation }, flags))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2010_block_element(data: &mut DwgBitReader<'_>, strings: &mut DwgBitReader<'_>, class_name: &str) -> Result<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockElement, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockElement;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpression;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    let parent_id = data.read_bl()? as i32;
    let major_version = data.read_bl()?;
    let minor_version = data.read_bl()?;
    let value_code = data.read_bs()? as i16;
    let node_id = data.read_bl()?;
    let name = strings.read_tu()?;
    let repeated_major = data.read_bl()?;
    let repeated_minor = data.read_bl()?;
    let marker = data.read_bl()?;
    if parent_id != -1 || (major_version, minor_version, value_code, repeated_major, repeated_minor, marker) != (29, 2, -9999, 29, 2, 0) || name.is_empty() {
        return Err(format!("{class_name} block-element metadata is invalid"));
    }
    Ok(DwgBlockElement { evaluation_expression: DwgEvaluationExpression { parent_id, major_version, minor_version, value: DwgEvaluationExpressionValue::Empty, node_id }, name })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2010_block_grip(data: &mut DwgBitReader<'_>, strings: &mut DwgBitReader<'_>, x_role: &str, y_role: &str, class_name: &str) -> Result<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockGrip, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockGrip;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgNamedEvaluationNodeReference;
    let element = decode_r2010_block_element(data, strings, class_name)?;
    let updated_x = DwgNamedEvaluationNodeReference { node_id: data.read_bl()?, expression_name: x_role.into() };
    let updated_y = DwgNamedEvaluationNodeReference { node_id: data.read_bl()?, expression_name: y_role.into() };
    let location = data.read_3bd()?.to_vec();
    let insertion_cycling = data.read_b()?;
    let insertion_cycling_weight = data.read_bl()? as i32;
    if location.iter().any(|value| !value.is_finite()) {
        return Err(format!("{class_name} location is invalid"));
    }
    Ok(DwgBlockGrip { element, location, insertion_cycling, insertion_cycling_weight, updated_x, updated_y })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2010_two_point_parameter(data: &mut DwgBitReader<'_>, strings: &mut DwgBitReader<'_>, class_name: &str) -> Result<(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockTwoPointParameter, [u32; 4]), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterBaseLocation;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterConnection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterProperty;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockTwoPointParameter;
    let element = decode_r2010_block_element(data, strings, class_name)?;
    let show_properties = data.read_b()?;
    let chain_actions = data.read_b()?;
    let definition_base = data.read_3bd()?.to_vec();
    let definition_end = data.read_3bd()?.to_vec();
    let mut properties = Vec::with_capacity(4);
    for _ in 0..4 {
        let count = data.read_bl()? as usize;
        if count > 10_000 {
            return Err(format!("{class_name} property connection count is invalid"));
        }
        let codes = (0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
        let connections = codes.into_iter().map(|code| Ok(DwgBlockParameterConnection { code, name: strings.read_tu()? })).collect::<Result<Vec<_>, String>>()?;
        properties.push(DwgBlockParameterProperty { connections });
    }
    let property_node_ids = [data.read_bl()?, data.read_bl()?, data.read_bl()?, data.read_bl()?];
    let base_location = match data.read_bs()? {
        0 => DwgBlockParameterBaseLocation::StartPoint,
        1 => DwgBlockParameterBaseLocation::Midpoint,
        value => return Err(format!("{class_name} base location {value} is unsupported")),
    };
    Ok((DwgBlockTwoPointParameter { element, show_properties, chain_actions, definition_base, definition_end, properties, property_expression_references: Vec::new(), base_location }, property_node_ids))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2010_block_action(data: &mut DwgBitReader<'_>, strings: &mut DwgBitReader<'_>, handles: &mut DwgBitReader<'_>, base: u64, class_name: &str) -> Result<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockAction, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockAction;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionDependency;
    let element = decode_r2010_block_element(data, strings, class_name)?;
    let display_location = data.read_3bd()?.to_vec();
    let dependency_count = data.read_bl()? as usize;
    let action_node_count = data.read_bl()? as usize;
    if dependency_count > 10_000 || action_node_count > 10_000 {
        return Err(format!("{class_name} action count is invalid"));
    }
    let action_node_ids = (0..action_node_count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
    let dependencies = (0..dependency_count).map(|_| read_object_handle(handles, base)?.ok_or_else(|| format!("{class_name} dependency is null")).map(|object_handle| DwgBlockActionDependency { object_handle })).collect::<Result<Vec<_>, String>>()?;
    Ok(DwgBlockAction { evaluation_expression: element.evaluation_expression, name: element.name, display_location, dependencies, action_node_ids })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_object_common_relations(data: &mut DwgBitReader<'_>, handles: &mut DwgBitReader<'_>, base: u64) -> Result<DwgObjectRelations, String> {
    let reactor_count = data.read_bl()? as usize;
    let extension_dictionary_missing = data.read_b()?;
    let owner = read_object_handle(handles, base)?;
    let reactors = (0..reactor_count).map(|_| read_object_handle(handles, base).map(|handle| handle.unwrap_or_default())).collect::<Result<Vec<_>, _>>()?.into_iter().filter(|handle| *handle != 0).collect();
    let extension_dictionary = if extension_dictionary_missing { None } else { read_object_handle(handles, base)? };
    Ok((owner, reactors, extension_dictionary))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2010_constraint_node(class: &str, data: &mut DwgBitReader<'_>, handles: &mut DwgBitReader<'_>, base: u64) -> Result<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintNode, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgAxisConstraint;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstrainedBoundedLine;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstrainedDatumLine;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstrainedImplicitPoint;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintGeometry;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintNode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintNodeCore;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgDistanceConstraint;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgExplicitConstraint;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgGeometricConstraint;
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn core(data: &mut DwgBitReader<'_>) -> Result<DwgConstraintNodeCore, String> {
        let id = data.read_bl()? as i32;
        if id < 0 {
            return Err(format!("constraint node ID {id} is invalid"));
        }
        let count = data.read_bl()? as usize;
        if count > 10_000 {
            return Err(format!("constraint node {id} connection count {count} is invalid"));
        }
        let connected_node_ids = (0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
        Ok(DwgConstraintNodeCore { id, connected_node_ids })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn geometric(data: &mut DwgBitReader<'_>) -> Result<DwgGeometricConstraint, String> {
        Ok(DwgGeometricConstraint { node: core(data)?, owner_node_id: data.read_bl()?, implied: data.read_b()?, active: true })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn geometry(data: &mut DwgBitReader<'_>, handles: &mut DwgBitReader<'_>, base: u64) -> Result<DwgConstraintGeometry, String> {
        Ok(DwgConstraintGeometry { node: core(data)?, geometry_dependency_handle: read_object_handle(handles, base)?, geometry_node_id: data.read_bl()? })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn explicit(data: &mut DwgBitReader<'_>, handles: &mut DwgBitReader<'_>, base: u64) -> Result<DwgExplicitConstraint, String> {
        Ok(DwgExplicitConstraint {
            geometric: geometric(data)?,
            value_dependency_handle: read_object_handle(handles, base)?.ok_or("explicit constraint value dependency is null")?,
            dimension_dependency_handle: read_object_handle(handles, base)?.ok_or("explicit constraint dimension dependency is null")?,
        })
    }
    Ok(match class {
        "AcConstrainedImplicitPoint" => {
            let geometry = geometry(data, handles, base)?;
            let point = geometry.geometry_dependency_handle.is_some().then(|| data.read_3bd().map(|value| value.to_vec())).transpose()?;
            DwgConstraintNode::ConstrainedImplicitPoint(DwgConstrainedImplicitPoint { geometry, point, point_kind: data.read_rc()?, point_index: data.read_bl()? as i32, curve_node_id: data.read_bl()? as i32 })
        }
        "AcPointCurveConstraint" => DwgConstraintNode::PointCurveConstraint(geometric(data)?),
        "AcConstrainedBoundedLine" => {
            let geometry = geometry(data, handles, base)?;
            let origin = data.read_3bd()?.to_vec();
            let direction = data.read_3bd()?.to_vec();
            let ray = data.read_b()?;
            DwgConstraintNode::ConstrainedBoundedLine(DwgConstrainedBoundedLine { geometry, origin, direction, ray, bounded: !ray, start_point: data.read_3bd()?.to_vec(), end_point: data.read_3bd()?.to_vec() })
        }
        "AcPointCoincidenceConstraint" => DwgConstraintNode::PointCoincidenceConstraint(geometric(data)?),
        "AcDistanceConstraint" => {
            let explicit = explicit(data, handles, base)?;
            let direction_kind = data.read_rc()?;
            let direction = (direction_kind != 0).then(|| data.read_3bd().map(|value| value.to_vec())).transpose()?;
            DwgConstraintNode::DistanceConstraint(DwgDistanceConstraint { explicit, direction_kind, direction })
        }
        "AcPerpendicularConstraint" => DwgConstraintNode::PerpendicularConstraint(geometric(data)?),
        "AcHorizontalConstraint" => DwgConstraintNode::HorizontalConstraint(DwgAxisConstraint { geometric: geometric(data)?, datum_line_index: data.read_bl()? as i32 }),
        "AcParallelConstraint" => DwgConstraintNode::ParallelConstraint(geometric(data)?),
        "AcMidPointConstraint" => DwgConstraintNode::MidPointConstraint(geometric(data)?),
        "AcEqualLengthConstraint" => DwgConstraintNode::EqualLengthConstraint(geometric(data)?),
        "AcColinearConstraint" => DwgConstraintNode::ColinearConstraint(geometric(data)?),
        "AcConstrainedDatumLine" => {
            let geometry = geometry(data, handles, base)?;
            DwgConstraintNode::ConstrainedDatumLine(DwgConstrainedDatumLine { geometry, origin: data.read_3bd()?.to_vec(), direction: data.read_3bd()?.to_vec() })
        }
        "AcFixedConstraint" => DwgConstraintNode::FixedConstraint(geometric(data)?),
        "AcVerticalConstraint" => DwgConstraintNode::VerticalConstraint(DwgAxisConstraint { geometric: geometric(data)?, datum_line_index: data.read_bl()? as i32 }),
        other => return Err(format!("unsupported ACDBASSOC2DCONSTRAINTGROUP node class {other}")),
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_r2010_cmc_main(data: &mut DwgBitReader<'_>) -> Result<(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor, u8), String> {
    let index = data.read_bs()?;
    let value = decode_complex_color_value(data.read_bl()?)?;
    let flags = data.read_rc()?;
    if flags > 3 {
        return Err(format!("CMC flags {flags:#x} are invalid"));
    }
    Ok((semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor { index, value, name: None, book_name: None }, flags))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_table_style_color(data: &mut DwgBitReader<'_>, handle: u64) -> Result<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor, String> {
    let (color, flags) = read_r2010_cmc_main(data)?;
    if flags != 0 {
        return Err(format!("TABLESTYLE {handle:#x} named colors are unsupported"));
    }
    Ok(color)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_material_color(data: &mut DwgBitReader<'_>, handle: u64) -> Result<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialColor, String> {
    let source = data.read_rc()?;
    if source > 1 {
        return Err(format!("MATERIAL {handle:#x} color source {source} is unsupported"));
    }
    Ok(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialColor { factor: data.read_bd()?, override_rgb: if source == 1 { Some(data.read_bl()?) } else { None } })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_material_map(data: &mut DwgBitReader<'_>, strings: &mut DwgBitReader<'_>, handle: u64) -> Result<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialMap, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialMap;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialMapSource;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialProjection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialTiling;
    let blend_factor = data.read_bd()?;
    let projection = match data.read_rc()? {
        0 => DwgMaterialProjection::Inherit,
        1 => DwgMaterialProjection::Planar,
        2 => DwgMaterialProjection::Box,
        3 => DwgMaterialProjection::Cylinder,
        4 => DwgMaterialProjection::Sphere,
        value => return Err(format!("MATERIAL {handle:#x} projection {value} is unsupported")),
    };
    let tiling = match data.read_rc()? {
        0 => DwgMaterialTiling::Inherit,
        1 => DwgMaterialTiling::Tile,
        2 => DwgMaterialTiling::Crop,
        3 => DwgMaterialTiling::Clamp,
        4 => DwgMaterialTiling::Mirror,
        value => return Err(format!("MATERIAL {handle:#x} tiling {value} is unsupported")),
    };
    let auto_transform = data.read_rc()?;
    let (scale_to_entity, use_current_block_transform) = if auto_transform == 1 { (false, false) } else { (auto_transform & 2 != 0, auto_transform & 4 != 0) };
    if auto_transform & !7 != 0 {
        return Err(format!("MATERIAL {handle:#x} auto-transform {auto_transform:#x} is unsupported"));
    }
    let transform = (0..16).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
    let source = match data.read_rc()? {
        0 => DwgMaterialMapSource::CurrentScene,
        1 => {
            let filename = strings.read_tu()?;
            if !filename.is_empty() {
                return Err(format!("MATERIAL {handle:#x} external texture file is not yet supported"));
            }
            DwgMaterialMapSource::None
        }
        value => return Err(format!("MATERIAL {handle:#x} map source {value} is unsupported")),
    };
    Ok(DwgMaterialMap { blend_factor, projection, tiling, scale_to_entity, use_current_block_transform, transform, source })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_r2010_cell_style(data: &mut DwgBitReader<'_>, strings: &mut DwgBitReader<'_>, handles: &mut DwgBitReader<'_>, base: u64) -> Result<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellStyle, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellBorder;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellBorders;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellContentFormat;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellMargins;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellStyle;
    if data.read_bl()? != 5 || data.read_bs()? != 1 {
        return Err(format!("TABLESTYLE {base:#x} cell type or data flag is unsupported"));
    }
    let property_override_flags = data.read_bl()?;
    let merge_flags = data.read_bl()?;
    let background_color = read_table_style_color(data, base)?;
    let content_layout = data.read_bl()?;
    let content_property_override_flags = data.read_bl()?;
    let content_property_flags = data.read_bl()?;
    let value_data_type = data.read_bl()?;
    let value_unit_type = data.read_bl()?;
    let value_format_string = strings.read_tu()?;
    let rotation = data.read_bd()?;
    let block_scale = data.read_bd()?;
    let alignment = data.read_bl()?;
    let content_color = read_table_style_color(data, base)?;
    let text_style_handle = read_object_handle(handles, base)?;
    let text_height = data.read_bd()?;
    if data.read_bs()? != 1 {
        return Err(format!("TABLESTYLE {base:#x} cell margins are missing"));
    }
    let margins = DwgCellMargins { vertical: data.read_bd()?, horizontal: data.read_bd()?, bottom: data.read_bd()?, right: data.read_bd()?, horizontal_spacing: data.read_bd()?, vertical_spacing: data.read_bd()? };
    let border_count = data.read_bl()? as usize;
    if border_count > 6 {
        return Err(format!("TABLESTYLE {base:#x} border count {border_count} is invalid"));
    }
    let mut borders = DwgCellBorders::default();
    for _ in 0..border_count {
        let mask = data.read_bl()?;
        let border = DwgCellBorder {
            override_flags: data.read_bl()?,
            border_type: data.read_bl()?,
            color: read_table_style_color(data, base)?,
            lineweight: data.read_bl()? as i32,
            linetype_handle: read_object_handle(handles, base)?,
            visible: data.read_bl()?,
            double_line_spacing: data.read_bd()?,
        };
        let slot = match mask {
            1 => &mut borders.top,
            2 => &mut borders.horizontal_inside,
            4 => &mut borders.bottom,
            8 => &mut borders.left,
            16 => &mut borders.vertical_inside,
            32 => &mut borders.right,
            _ => return Err(format!("TABLESTYLE {base:#x} border mask {mask} is invalid")),
        };
        if slot.replace(border).is_some() {
            return Err(format!("TABLESTYLE {base:#x} repeats border mask {mask}"));
        }
    }
    Ok(DwgCellStyle {
        property_override_flags,
        merge_flags,
        background_color,
        content_layout,
        content_format: DwgCellContentFormat {
            property_override_flags: content_property_override_flags,
            property_flags: content_property_flags,
            value_data_type,
            value_unit_type,
            value_format_string,
            rotation,
            block_scale,
            alignment,
            content_color,
            text_style_handle,
            text_height,
        },
        margins,
        borders,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_complex_color_value(word: u32) -> Result<semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColorValue, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColorValue;
    let value = word & 0x00ff_ffff;
    match word >> 24 {
        0xc0 => Ok(DwgComplexColorValue::ByLayer),
        0xc1 => Ok(DwgComplexColorValue::ByBlock),
        0xc2 => Ok(DwgComplexColorValue::ByColor { red: ((value >> 16) & 0xff) as u8, green: ((value >> 8) & 0xff) as u8, blue: (value & 0xff) as u8 }),
        0xc3 => Ok(DwgComplexColorValue::ByAci { index: u16::try_from(value).map_err(|_| format!("ACI color {value} exceeds u16"))? }),
        0xc4 => Ok(DwgComplexColorValue::ByPen { index: u8::try_from(value).map_err(|_| format!("pen color {value} exceeds u8"))? }),
        0xc5 => Ok(DwgComplexColorValue::Foreground),
        0xc6 => Ok(DwgComplexColorValue::LayerOff),
        0xc7 => Ok(DwgComplexColorValue::LayerFrozen),
        0xc8 if value == 0 => Ok(DwgComplexColorValue::None),
        method => Err(format!("complex-color method {method:#x} with value {value:#x} is unsupported")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn r2010_string_stream(bytes: &[u8], end_bit: usize) -> Result<(DwgBitReader<'_>, usize), String> {
    if end_bit < 17 || end_bit > bytes.len().saturating_mul(8) {
        return Err("R2010 string-stream end is out of bounds".into());
    }
    let mut present = DwgBitReader::at_bit(bytes, end_bit - 1)?;
    if !present.read_b()? {
        return Ok((DwgBitReader::at_bit(bytes, end_bit - 1)?, end_bit - 1));
    }
    let mut size_reader = DwgBitReader::at_bit(bytes, end_bit - 17)?;
    let low = size_reader.read_rs()?;
    let (size_bits, header_bits) = if low & 0x8000 == 0 {
        (usize::from(low), 17usize)
    } else {
        if end_bit < 33 {
            return Err("R2010 extended string-stream size is truncated".into());
        }
        let mut high_reader = DwgBitReader::at_bit(bytes, end_bit - 33)?;
        ((usize::from(low & 0x7fff)) | (usize::from(high_reader.read_rs()?) << 15), 33usize)
    };
    let start = end_bit.checked_sub(header_bits).and_then(|value| value.checked_sub(size_bits)).ok_or("R2010 string-stream size exceeds class data")?;
    Ok((DwgBitReader::at_bit(bytes, start)?, start))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn r2010_string_content_end_bit(bytes: &[u8], end_bit: usize) -> Result<usize, String> {
    let (_, start) = r2010_string_stream(bytes, end_bit)?;
    if start == end_bit.saturating_sub(1) {
        return Ok(start);
    }
    let mut size_reader = DwgBitReader::at_bit(bytes, end_bit.checked_sub(17).ok_or("R2010 string footer is truncated")?)?;
    let low = size_reader.read_rs()?;
    Ok(end_bit - if low & 0x8000 == 0 { 17 } else { 33 })
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
