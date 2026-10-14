//! 📄️ File-level section writers: header variables, classes, summary, application, dependencies, template, auxiliary header, revision history and preview.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_HEADER_BEGIN;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_HEADER_END;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_PREVIEW_BEGIN;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_PREVIEW_END;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DWG_SENTINEL_CLASSES_BEGIN;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::dwg_crc16;
use crate::standards::v_ac1024::subsets::any::io::export::records::bits::{DwgBitWriter, push_u16, push_u32, push_u64};

//#region 🔖️DwgStructuralCodec
//#region DwgWrite
const DWG_SENTINEL_CLASSES_END: [u8; 16] = [0x72, 0x5E, 0x3B, 0x47, 0x3B, 0x56, 0x07, 0x3A, 0x3F, 0x23, 0x0B, 0xA0, 0x18, 0x30, 0x49, 0x75];
//#endregion DwgWrite
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_utf16_z(output: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let units = value.encode_utf16().collect::<Vec<_>>();
    let count = units.len().checked_add(1).ok_or("DWG UTF-16 string length overflow")?;
    push_u16(output, u16::try_from(count).map_err(|_| "DWG UTF-16 string exceeds u16 length")?);
    for unit in units {
        push_u16(output, unit);
    }
    push_u16(output, 0);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_utf16_bytes(output: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let units = value.encode_utf16().collect::<Vec<_>>();
    push_u32(output, u32::try_from(units.len().checked_mul(2).ok_or("DWG UTF-16 byte length overflow")?).map_err(|_| "DWG UTF-16 byte string exceeds u32 length")?);
    for unit in units {
        push_u16(output, unit);
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_bytes_z(output: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let count = value.len().checked_add(1).ok_or("DWG byte string length overflow")?;
    push_u16(output, u16::try_from(count).map_err(|_| "DWG byte string exceeds u16 length")?);
    output.extend_from_slice(value.as_bytes());
    output.push(0);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn checksum_bytes(value: &str) -> Result<[u8; 16], String> {
    if value.is_empty() {
        return Ok([0; 16]);
    }
    let compact = value.chars().filter(|character| *character != '-').collect::<String>();
    if compact.len() != 32 {
        return Err("DWG checksum identifier must contain 32 hexadecimal digits".into());
    }
    let mut bytes = [0; 16];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&compact[index * 2..index * 2 + 2], 16).map_err(|error| format!("invalid DWG checksum identifier: {error}"))?;
    }
    Ok(bytes)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_summary_info(summary: &semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgSummaryInfo) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    for value in [&summary.title, &summary.subject, &summary.author, &summary.keywords, &summary.comments, &summary.last_saved_by, &summary.revision_number, &summary.hyperlink_base] {
        push_utf16_z(&mut output, value)?;
    }
    push_u64(&mut output, summary.total_editing_time);
    push_u32(&mut output, summary.created_at.days);
    push_u32(&mut output, summary.created_at.milliseconds);
    push_u32(&mut output, summary.modified_at.days);
    push_u32(&mut output, summary.modified_at.milliseconds);
    push_u16(&mut output, u16::try_from(summary.custom_properties.len()).map_err(|_| "DWG summary has too many custom properties")?);
    for property in &summary.custom_properties {
        push_utf16_z(&mut output, &property.key)?;
        push_utf16_z(&mut output, &property.value)?;
    }
    push_u32(&mut output, 0);
    push_u32(&mut output, 0);
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_application_info(application: &semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationInfo) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    push_u32(&mut output, 3);
    push_utf16_z(&mut output, &application.name)?;
    push_u32(&mut output, 3);
    output.extend_from_slice(&checksum_bytes(&application.version_checksum)?);
    push_utf16_z(&mut output, &application.version)?;
    output.extend_from_slice(&checksum_bytes(&application.comment_checksum)?);
    push_utf16_z(&mut output, &application.comment)?;
    output.extend_from_slice(&checksum_bytes(&application.product_checksum)?);
    push_utf16_z(&mut output, &application.product)?;
    if !application.application_version.is_empty() {
        push_bytes_z(&mut output, &application.application_version)?;
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_dependencies(dependencies: &[semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgDependency]) -> Result<Vec<u8>, String> {
    let mut features = Vec::<String>::new();
    for dependency in dependencies {
        if !features.contains(&dependency.feature) {
            features.push(dependency.feature.clone());
        }
    }
    let mut output = Vec::new();
    push_u32(&mut output, u32::try_from(features.len()).map_err(|_| "DWG dependency feature count exceeds u32")?);
    for feature in &features {
        push_utf16_bytes(&mut output, feature)?;
    }
    push_u32(&mut output, u32::try_from(dependencies.len()).map_err(|_| "DWG dependency count exceeds u32")?);
    for dependency in dependencies {
        push_utf16_bytes(&mut output, &dependency.full_path)?;
        push_utf16_bytes(&mut output, &dependency.relative_path)?;
        push_utf16_bytes(&mut output, &dependency.fingerprint)?;
        push_utf16_bytes(&mut output, &dependency.version)?;
        push_u32(&mut output, features.iter().position(|feature| feature == &dependency.feature).ok_or("DWG dependency feature is missing")? as u32);
        push_u32(&mut output, dependency.timestamp);
        push_u32(&mut output, dependency.file_size);
        push_u16(&mut output, u16::from(dependency.affects_graphics));
        push_u32(&mut output, dependency.reference_count);
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_template(template: &semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgTemplate) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    push_utf16_z(&mut output, &template.description)?;
    push_u16(
        &mut output,
        match template.measurement {
            semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgMeasurement::English => 0,
            semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgMeasurement::Metric => 1,
        },
    );
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_auxiliary_header(value: &semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgAuxiliaryHeader) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgCompatibilityProfile;
    if value.compatibility_profile != DwgCompatibilityProfile::Autocad2009 {
        return Err("unsupported auxiliary-header compatibility profile".into());
    }
    let mut output = vec![255, 119, 1];
    push_u16(&mut output, 29);
    push_u16(&mut output, 2);
    push_u32(&mut output, value.total_saves);
    output.extend_from_slice(&(-1i32).to_le_bytes());
    push_u16(&mut output, value.save_partition_one);
    push_u16(&mut output, value.save_partition_two);
    push_u32(&mut output, value.save_generation);
    for stamp in [&value.legacy_stamp_one, &value.legacy_stamp_two] {
        push_u16(&mut output, stamp.version);
        push_u16(&mut output, stamp.maintenance);
    }
    for value in [4u16, 1381, 261, 2600, 0, 1] {
        push_u16(&mut output, value);
    }
    for value in [0u32, 0, 0, 16_908_544, 65_538] {
        push_u32(&mut output, value);
    }
    for date in [&value.created_at, &value.updated_at] {
        push_u32(&mut output, date.days);
        push_u32(&mut output, date.milliseconds);
    }
    push_u64(&mut output, value.handle_seed);
    push_u16(&mut output, 0);
    push_u16(&mut output, value.terminal_save_generation);
    for terminal in [0u32, 0, 0, value.total_saves, 0, 0, 0, 0] {
        push_u32(&mut output, terminal);
    }
    if output.len() != 123 {
        return Err(format!("auxiliary header encoded to {} bytes", output.len()));
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_revision_history(value: &semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgRevisionHistory) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    push_u32(&mut output, value.format_major);
    push_u32(&mut output, value.format_minor);
    push_u32(&mut output, u32::try_from(value.revisions.len()).map_err(|_| "revision history exceeds u32 count")?);
    for revision in &value.revisions {
        push_u32(&mut output, *revision);
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_indexed_preview(value: &semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgIndexedPreview, payload_address: u32) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgPreviewOrigin;
    if value.origin != DwgPreviewOrigin::BottomUp || value.palette.len() != 256 {
        return Err("AC1024 preview requires a bottom-up 256-color indexed bitmap".into());
    }
    let width = value.width as usize;
    let height = value.height as usize;
    if value.pixel_indices.len() != width.checked_mul(height).ok_or("preview dimensions overflow")? {
        return Err("preview pixel count disagrees with dimensions".into());
    }
    let stride = (width + 3) & !3;
    let image_size = stride.checked_mul(height).ok_or("preview image size overflow")?;
    let bitmap_size = 40 + 1024 + image_size;
    let overall_size = 1 + 18 + 80 + bitmap_size;
    let mut output = Vec::with_capacity(overall_size + 36);
    output.extend_from_slice(&DWG_PREVIEW_BEGIN);
    push_u32(&mut output, overall_size as u32);
    output.push(2);
    output.push(1);
    push_u32(&mut output, payload_address + 39);
    push_u32(&mut output, 80);
    output.push(2);
    push_u32(&mut output, payload_address + 119);
    push_u32(&mut output, bitmap_size as u32);
    output.extend_from_slice(&[0; 80]);
    push_u32(&mut output, 40);
    output.extend_from_slice(&(value.width as i32).to_le_bytes());
    output.extend_from_slice(&(value.height as i32).to_le_bytes());
    push_u16(&mut output, 1);
    push_u16(&mut output, 8);
    push_u32(&mut output, 0);
    push_u32(&mut output, image_size as u32);
    output.extend_from_slice(&0i32.to_le_bytes());
    output.extend_from_slice(&0i32.to_le_bytes());
    push_u32(&mut output, 256);
    push_u32(&mut output, 0);
    for color in &value.palette {
        output.extend_from_slice(&[color.blue, color.green, color.red, 0]);
    }
    for row in value.pixel_indices.chunks_exact(width.max(1)) {
        output.extend_from_slice(row);
        output.extend(std::iter::repeat_n(value.background_palette_index, stride - width));
    }
    output.extend_from_slice(&DWG_PREVIEW_END);
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_digest128(value: &str) -> Result<[u8; 16], String> {
    if value.len() != 32 {
        return Err("128-bit identifier must contain 32 hex digits".into());
    }
    let mut output = [0u8; 16];
    for (index, pair) in value.as_bytes().as_chunks::<2>().0.iter().enumerate() {
        output[index] = u8::from_str_radix(std::str::from_utf8(pair).map_err(|_| "identifier is not UTF-8")?, 16).map_err(|_| "identifier contains non-hex digits")?;
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn render_application_properties(format_identifier: &str, properties: &[semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationProperty]) -> String {
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationPropertyKind;
    let mut output = format!("<prop_set fmt_id=\"{{{format_identifier}}}\">");
    for property in properties {
        let tag = match property.kind {
            DwgApplicationPropertyKind::String => "string",
            DwgApplicationPropertyKind::DateTime => "datetime",
        };
        output.push_str(&format!("<prop id=\"{}\"><{tag}>{}</{tag}></prop>", property.id, property.value));
    }
    output.push_str("</prop_set>");
    output
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn render_product_information(value: &semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgProductInformation) -> String {
    format!(
        "\"<ProductInformation name =\\\"{}\\\" build_version=\\\"{}\\\" registry_version=\\\"{}\\\" install_id_string=\\\"{}\\\" registry_localeID=\\\"{}\\\"/>\"",
        value.name, value.build_version, value.registry_version, value.install_id, value.locale_id
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_application_history(value: &semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationHistory) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    output.extend_from_slice(&encode_digest128(&value.history_identifier_one)?);
    output.extend_from_slice(&encode_digest128(&value.history_identifier_two)?);
    push_u32(&mut output, value.class_version);
    push_utf16_z(&mut output, "AppInfoDataList")?;
    push_u32(&mut output, 4);
    for (digest, rendered) in [
        (&value.application_version_digest, value.application_version.clone()),
        (&value.trust_comment_digest, value.trust_comment.clone()),
        (&value.property_set_digest, render_application_properties(&value.property_format_identifier, &value.properties)),
        (&value.product_digest, render_product_information(&value.product)),
    ] {
        output.extend_from_slice(&encode_digest128(digest)?);
        push_utf16_z(&mut output, &rendered)?;
    }
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_object_free_space(updated: &semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgJulianDate) -> Vec<u8> {
    let mut output = Vec::with_capacity(89);
    push_u64(&mut output, 0);
    push_u64(&mut output, 679);
    push_u32(&mut output, updated.days);
    push_u32(&mut output, updated.milliseconds);
    output.push(4);
    for (low, high) in [(50u64, 0u64), (100, 0), (512, 0), (0xffff_ffff, 0)] {
        push_u64(&mut output, low);
        push_u64(&mut output, high);
    }
    output
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn header_point3(value: &[f64], name: &str) -> Result<[f64; 3], String> {
    value.try_into().map_err(|_| format!("{name} must contain three coordinates"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn header_point2(value: &[f64], name: &str) -> Result<[f64; 2], String> {
    value.try_into().map_err(|_| format!("{name} must contain two coordinates"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_header_time(writer: &mut DwgBitWriter, value: &semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgJulianDate) {
    writer.write_bl(value.days);
    writer.write_bl(value.milliseconds);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_header_color(writer: &mut DwgBitWriter, index: u16, rgb: u32) {
    writer.write_bs(index);
    writer.write_bl(rgb);
    writer.write_rc(0);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_header_space(writer: &mut DwgBitWriter, value: &semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderSpaceGeometry, name: &str) -> Result<(), String> {
    writer.write_3bd(header_point3(&value.insertion_base, &format!("{name} insertion base"))?);
    writer.write_3bd(header_point3(&value.extents_minimum, &format!("{name} extents minimum"))?);
    writer.write_3bd(header_point3(&value.extents_maximum, &format!("{name} extents maximum"))?);
    writer.write_2rd(header_point2(&value.limits_minimum, &format!("{name} limits minimum"))?);
    writer.write_2rd(header_point2(&value.limits_maximum, &format!("{name} limits maximum"))?);
    writer.write_bd(value.elevation);
    writer.write_3bd(header_point3(&value.ucs_origin, &format!("{name} UCS origin"))?);
    writer.write_3bd(header_point3(&value.ucs_x_axis, &format!("{name} UCS X axis"))?);
    writer.write_3bd(header_point3(&value.ucs_y_axis, &format!("{name} UCS Y axis"))?);
    writer.write_bs(value.ucs_orthographic_view);
    for (point, suffix) in [(&value.ucs_origin_top, "top"), (&value.ucs_origin_bottom, "bottom"), (&value.ucs_origin_left, "left"), (&value.ucs_origin_right, "right"), (&value.ucs_origin_front, "front"), (&value.ucs_origin_back, "back")] {
        writer.write_3bd(header_point3(point, &format!("{name} UCS {suffix}"))?);
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_header_section(value: &semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderVariables) -> Result<Vec<u8>, String> {
    let mut main = DwgBitWriter::new();
    let u = &value.units;
    for number in [u.unit1_conversion, u.unit2_conversion, u.unit3_conversion, u.unit4_conversion] {
        main.write_bd(number);
    }
    main.write_bl(2_454_805);
    main.write_bl(60_784_745);
    let m = &value.modes;
    for flag in [m.dimension_associative, m.dimension_show, m.polyline_generation, m.orthographic_mode, m.regeneration_mode, m.fill_mode, m.quick_text_mode, m.paper_space_linetype_scale, m.limits_check] {
        main.write_b(flag);
    }
    main.write_b(false);
    for flag in [m.user_timer, m.sketch_polyline, m.angle_direction, m.spline_frame, m.mirror_text, m.world_view, m.tile_mode, m.paper_limits_check, m.visual_retain, m.display_silhouette, m.polyline_ellipse] {
        main.write_b(flag);
    }
    let i = &value.integers;
    main.write_bs(i.proxy_graphics);
    main.write_bs(i.tree_depth as u16);
    main.write_bs(i.linear_units);
    main.write_bs(i.linear_precision);
    main.write_bs(i.angular_units);
    main.write_bs(i.angular_precision);
    main.write_bs(i.attribute_mode);
    main.write_bs(i.point_display_mode);
    main.write_bl(0x30303030);
    main.write_bl(0x1d050900);
    main.write_bl(0x4d353930);
    for number in [i.user_integer1, i.user_integer2, i.user_integer3, i.user_integer4, i.user_integer5] {
        main.write_bs(number as u16);
    }
    for number in
        [i.spline_segments, i.surface_u, i.surface_v, i.surface_type, i.surface_tab1, i.surface_tab2, i.spline_type, i.shade_edge, i.shade_difference, i.unit_mode, i.maximum_active_viewports, i.isolines, i.multiline_justification, i.text_quality]
    {
        main.write_bs(number);
    }
    let s = &value.scalars;
    for number in [
        s.linetype_scale,
        s.text_size,
        s.trace_width,
        s.sketch_increment,
        s.fillet_radius,
        s.thickness,
        s.angle_base,
        s.point_display_size,
        s.polyline_width,
        s.user_real1,
        s.user_real2,
        s.user_real3,
        s.user_real4,
        s.user_real5,
        s.chamfer_a,
        s.chamfer_b,
        s.chamfer_c,
        s.chamfer_d,
        s.facet_resolution,
        s.multiline_scale,
        s.current_entity_linetype_scale,
    ] {
        main.write_bd(number);
    }
    write_header_time(&mut main, &value.time.created_at);
    write_header_time(&mut main, &value.time.updated_at);
    main.write_bl(0x44443434);
    main.write_bl(0x140d0102);
    main.write_bl(0x39343531);
    write_header_time(&mut main, &value.time.editing_duration);
    write_header_time(&mut main, &value.time.user_timer_duration);
    write_header_color(&mut main, s.current_entity_color_index, 0xc0000000);
    main.write_handle(0, value.relations.handle_seed);
    main.write_bd(s.paper_space_viewport_scale);
    write_header_space(&mut main, &value.paper_space, "paper space")?;
    write_header_space(&mut main, &value.model_space, "model space")?;
    let d = &value.dimensions;
    for number in [d.scale, d.arrow_size, d.extension_offset, d.line_increment, d.extension, d.rounding, d.line_extension, d.tolerance_plus, d.tolerance_minus, d.fixed_extension_length, d.jog_angle] {
        main.write_bd(number);
    }
    main.write_bs(d.text_fill);
    write_header_color(&mut main, d.text_fill_color_index, 0xc1000000);
    for flag in [d.tolerance, d.limits, d.text_inside_horizontal, d.text_outside_horizontal, d.suppress_extension1, d.suppress_extension2] {
        main.write_b(flag);
    }
    for number in [d.text_above, d.zero_suppression, d.angular_zero_suppression, d.arc_symbol] {
        main.write_bs(number);
    }
    for number in [d.text_height, d.center_mark, d.tick_size, d.alternate_scale, d.linear_factor, d.text_vertical_position, d.text_factor, d.gap, d.alternate_rounding] {
        main.write_bd(number);
    }
    main.write_b(d.alternate_units);
    main.write_bs(d.alternate_decimal_places);
    for flag in [d.text_outside_force_line, d.separate_arrows, d.text_inside, d.suppress_outside] {
        main.write_b(flag);
    }
    for color in [d.line_color_index, d.extension_color_index, d.text_color_index] {
        write_header_color(&mut main, color, 0xc1000000);
    }
    for number in [
        d.angular_decimal_places,
        d.decimal_places,
        d.tolerance_decimal_places,
        d.alternate_units_format,
        d.alternate_tolerance_decimal_places,
        d.angular_unit_format,
        d.fractional_format,
        d.linear_unit_format,
        d.decimal_separator,
        d.text_movement,
        d.justification,
    ] {
        main.write_bs(number);
    }
    for flag in [d.suppress_dimension1, d.suppress_dimension2] {
        main.write_b(flag);
    }
    for number in [d.tolerance_justification, d.tolerance_zero_suppression, d.alternate_zero_suppression, d.alternate_tolerance_zero_suppression] {
        main.write_bs(number);
    }
    main.write_b(d.user_positioned_text);
    main.write_bs(d.fit);
    main.write_b(d.fixed_extension_enabled);
    main.write_b(d.text_direction);
    main.write_bd(d.alternate_measurement_scale);
    main.write_bd(d.measurement_scale);
    main.write_bs(d.dimension_line_weight as u16);
    main.write_bs(d.extension_line_weight as u16);
    let p = &value.policy;
    main.write_bs(p.text_stack_alignment);
    main.write_bs(p.text_stack_size);
    if (p.current_entity_lineweight, p.end_caps, p.join_style, p.lineweight_display, p.external_reference_editing, p.extended_names, p.plot_style_mode, p.ole_startup) != (-1, 0, 0, false, true, true, true, false) {
        return Err("unsupported AC1024 packed drawing policy".into());
    }
    main.write_bl(0x2a1d);
    main.write_bs(p.insertion_units);
    main.write_bs(p.current_plot_style_type);
    for number in [p.sort_entities, p.index_control, p.hide_text, p.xclip_frame, p.dimension_association, p.halo_gap] {
        main.write_rc(number);
    }
    main.write_bs(p.obscured_color);
    main.write_bs(p.intersection_color);
    main.write_rc(p.obscured_linetype);
    main.write_rc(p.intersection_display);
    main.write_b(p.camera_display);
    main.write_bl(0);
    main.write_bl(10);
    main.write_bd(1.0);
    for number in [p.steps_per_second, p.step_size, p.dwf_3d_precision, p.lens_length, p.camera_height] {
        main.write_bd(number);
    }
    main.write_rc(p.solid_history);
    main.write_rc(p.show_history);
    for number in [p.polysolid_width, p.polysolid_height, p.loft_angle1, p.loft_angle2, p.loft_magnitude1, p.loft_magnitude2] {
        main.write_bd(number);
    }
    main.write_bs(p.loft_parameter);
    main.write_rc(p.loft_normals);
    for number in [p.latitude, p.longitude, p.north_direction] {
        main.write_bd(number);
    }
    main.write_bl(p.timezone as u32);
    for number in [p.light_glyph_display, p.tile_mode_light_sync, p.dwf_frame, p.dgn_frame] {
        main.write_rc(number);
    }
    if p.interfere_color_index != 256 {
        return Err("AC1024 INTERFERECOLOR must be the typed ByLayer color".into());
    }
    main.write_b(p.real_world_scale);
    write_header_color(&mut main, 0, 0xc3000001);
    main.write_rc(p.shadow_mode);
    main.write_bd(p.shadow_plane_location);

    let mut strings = DwgBitWriter::new();
    for text in [
        &u.unit1_name,
        &u.unit2_name,
        &u.unit3_name,
        &u.unit4_name,
        &value.strings.menu,
        &value.strings.dimension_postfix,
        &value.strings.dimension_alternate_postfix,
        &value.strings.dimension_alternate_measurement_zero_suffix,
        &value.strings.dimension_measurement_zero_suffix,
        &value.strings.hyperlink_base,
        &value.strings.stylesheet,
        &value.strings.fingerprint_guid,
        &value.strings.version_guid,
        &value.strings.project_name,
    ] {
        strings.write_tu(text);
    }
    let mut handles = DwgBitWriter::new();
    let r = &value.relations;
    let required = [r.current_layer, r.text_style, r.current_linetype, r.current_material, r.dimension_style, r.multiline_style];
    for handle in required {
        handles.write_handle(5, handle);
    }
    for handle in [r.paper_ucs_name, r.paper_ucs_orthographic_reference, r.paper_ucs_base, r.model_ucs_name, r.model_ucs_orthographic_reference, r.model_ucs_base] {
        handles.write_handle(5, handle.unwrap_or(0));
    }
    handles.write_handle(5, r.dimension_text_style);
    for handle in [r.dimension_leader_block, r.dimension_block, r.dimension_block1, r.dimension_block2, r.dimension_linetype, r.dimension_extension_linetype1, r.dimension_extension_linetype2] {
        handles.write_handle(5, handle.unwrap_or(0));
    }
    for handle in [r.block_control, r.layer_control, r.style_control, r.linetype_control, r.view_control, r.ucs_control, r.viewport_control, r.appid_control, r.dimension_style_control] {
        handles.write_handle(3, handle);
    }
    for handle in [r.group_dictionary, r.multiline_style_dictionary] {
        handles.write_handle(5, handle);
    }
    handles.write_handle(3, r.named_objects_dictionary);
    for handle in [
        r.layout_dictionary,
        r.plot_settings_dictionary,
        r.plot_style_name_dictionary,
        r.material_dictionary,
        r.color_dictionary,
        r.visual_style_dictionary,
        r.paper_space_block_record,
        r.model_space_block_record,
        r.by_layer_linetype,
        r.by_block_linetype,
        r.continuous_linetype,
    ] {
        handles.write_handle(5, handle);
    }
    for handle in [r.interfere_object_visual_style, r.interfere_viewport_visual_style, r.drag_visual_style] {
        handles.write_handle(5, handle.unwrap_or(0));
    }
    for value in [0xbfc4, 0x122d, 0xa23e, 0xb717] {
        handles.write_bs(value);
    }
    handles.write_bl(0);
    handles.write_bl(0);
    handles.write_b(true);
    handles.write_b(true);
    handles.write_b(true);
    handles.write_b(true);
    let string_bits = strings.bit_len();
    if string_bits > 0x7fff {
        return Err(format!("AC1024 Header string stream {string_bits} bits exceeds the compact size"));
    }
    main.append_bits(&strings);
    main.write_rs(string_bits as u16);
    main.write_b(true);
    let boundary = main.bit_len();
    main.append_bits(&handles);
    main.pad_to_byte();
    let mut output = Vec::with_capacity(main.bytes.len() + 42);
    output.extend_from_slice(&DWG_HEADER_BEGIN);
    push_u32(&mut output, u32::try_from(main.bytes.len() + 4).map_err(|_| "AC1024 Header exceeds RL size")?);
    push_u32(&mut output, u32::try_from(boundary + 32).map_err(|_| "AC1024 Header exceeds RL bit size")?);
    output.extend_from_slice(&main.bytes);
    let crc = dwg_crc16(0xc0c1, &output[16..]);
    push_u16(&mut output, crc);
    output.extend_from_slice(&DWG_HEADER_END);
    Ok(output)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_classes_section(classes: &[semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgClass]) -> Result<Vec<u8>, String> {
    let mut data = DwgBitWriter::new();
    let maximum_class = classes.iter().map(|class| class.number).max().unwrap_or(499);
    data.write_bl(u32::from(maximum_class));
    data.write_b(true);
    let mut strings = DwgBitWriter::new();
    for class in classes {
        data.write_bs(class.number);
        data.write_bs(class.proxy_flags as u16);
        strings.write_tu(&class.application_name);
        strings.write_tu(&class.cpp_class_name);
        strings.write_tu(&class.dxf_name);
        data.write_b(class.was_zombie);
        data.write_bs(class.item_class_id);
        data.write_bl(class.object_count);
        data.write_bl(class.dwg_version);
        data.write_bl(class.maintenance_version);
        data.write_bl(class.reserved_values.first().copied().unwrap_or(0));
        data.write_bl(class.reserved_values.get(1).copied().unwrap_or(0));
    }
    let string_bits = strings.bit_len();
    data.append_bits(&strings);
    if string_bits <= 0x7fff {
        data.write_rs(string_bits as u16);
    } else {
        let high = string_bits >> 15;
        if high > u16::MAX as usize {
            return Err("R2010 class string stream exceeds extended size".into());
        }
        data.write_rs(high as u16);
        data.write_rs((string_bits as u16 & 0x7fff) | 0x8000);
    }
    data.write_b(true);
    let body_bits = data.bit_len();
    data.pad_to_byte();
    let total_bits = 32usize.checked_add(body_bits).ok_or("R2010 class section size overflow")?;
    let mut output = Vec::new();
    output.extend_from_slice(&DWG_SENTINEL_CLASSES_BEGIN);
    output.extend_from_slice(&(total_bits.div_ceil(8) as u32).to_le_bytes());
    output.extend_from_slice(&(total_bits as u32).to_le_bytes());
    output.extend_from_slice(&data.bytes);
    output.extend_from_slice(&dwg_crc16(0xC0C1, &output[16..]).to_le_bytes());
    output.extend_from_slice(&DWG_SENTINEL_CLASSES_END);
    output.extend_from_slice(&[0; 8]);
    Ok(output)
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
