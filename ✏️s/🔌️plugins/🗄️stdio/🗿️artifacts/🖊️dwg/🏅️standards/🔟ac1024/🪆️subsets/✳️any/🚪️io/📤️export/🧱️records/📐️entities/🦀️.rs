//! 📐️ Entity record writers: the common entity fields and every entity body frame.

use crate::standards::v_ac1024::subsets::any::io::export::records::bits::{DwgBitWriter};
use crate::standards::v_ac1024::subsets::any::io::export::records::fields::{append_r2010_string_stream, encode_complex_color_value, encode_r2010_eed, finish_r2010_object_frame, logical_point2, logical_point3};

//#region 🔖️DwgStructuralCodec
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_mode_bits(value: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityMode) -> u8 {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityMode;
    match value {
        DwgEntityMode::ExplicitOwner => 0,
        DwgEntityMode::PaperSpace => 1,
        DwgEntityMode::ModelSpace => 2,
        DwgEntityMode::Reserved => 3,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_reference_bits(value: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityReferenceMode) -> u8 {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityReferenceMode;
    match value {
        DwgEntityReferenceMode::ByLayer => 0,
        DwgEntityReferenceMode::ByBlock => 1,
        DwgEntityReferenceMode::Continuous => 2,
        DwgEntityReferenceMode::Explicit => 3,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_entity_common_main(data: &mut DwgBitWriter, object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject, common: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityCommon) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityColorKind;
    data.write_b(false);
    data.write_bb(entity_mode_bits(common.mode));
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    let encoded_color = match common.color.kind {
        DwgEntityColorKind::ByLayer => 256,
        DwgEntityColorKind::ByBlock => 0,
        DwgEntityColorKind::Index => common.color.index,
        DwgEntityColorKind::TrueColor => return Err("R2010 true-color entity writing requires the complete typed ENC string stream".into()),
    };
    data.write_bs(encoded_color);
    data.write_bd(common.linetype_scale);
    data.write_bb(entity_reference_bits(common.linetype));
    data.write_bb(entity_reference_bits(common.plot_style));
    data.write_bb(entity_reference_bits(common.material));
    data.write_rc(common.shadow);
    data.write_b(common.full_visual_style_handle.is_some());
    data.write_b(common.face_visual_style_handle.is_some());
    data.write_b(common.edge_visual_style_handle.is_some());
    data.write_bs(common.invisible);
    data.write_rc(common.lineweight);
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_entity_common_handles(handles: &mut DwgBitWriter, object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject, common: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityCommon) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityColorKind;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityReferenceMode;
    if common.color.kind == DwgEntityColorKind::TrueColor {
        handles.write_handle(5, common.color.color_handle.unwrap_or_default());
    }
    if common.mode == DwgEntityMode::ExplicitOwner {
        let owner = object.owner_handle.ok_or_else(|| format!("entity {:#x} explicit-owner mode has no owner", object.handle))?;
        if owner.saturating_add(1) == object.handle {
            handles.write_handle(8, 0);
        } else {
            let delta = object.handle.checked_sub(owner).ok_or_else(|| format!("entity {:#x} owner {owner:#x} is not lower", object.handle))?;
            handles.write_handle(12, delta);
        }
    } else if object.owner_handle.is_some() {
        return Err(format!("entity {:#x} non-owner mode has an owner handle", object.handle));
    }
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(3, extension_dictionary);
    }
    handles.write_handle(5, common.layer_handle);
    if common.linetype == DwgEntityReferenceMode::Explicit {
        handles.write_handle(5, common.linetype_handle.unwrap_or_default());
    }
    if common.material == DwgEntityReferenceMode::Explicit {
        handles.write_handle(5, common.material_handle.unwrap_or_default());
    }
    if common.shadow == 3 {
        handles.write_handle(5, common.shadow_handle.unwrap_or_default());
    }
    if common.plot_style == DwgEntityReferenceMode::Explicit {
        handles.write_handle(5, common.plot_style_handle.unwrap_or_default());
    }
    for visual in [common.full_visual_style_handle, common.face_visual_style_handle, common.edge_visual_style_handle].into_iter().flatten() {
        handles.write_handle(5, visual);
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dimension_attachment_wire(value: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionTextAttachment) -> u16 {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionTextAttachment::*;
    match value {
        TopCenter => 1,
        TopLeft => 2,
        TopRight => 3,
        MiddleCenter => 4,
        MiddleLeft => 5,
        MiddleRight => 6,
        BottomCenter => 7,
        BottomLeft => 8,
        BottomRight => 9,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dimension_spacing_wire(value: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionLineSpacingStyle) -> u16 {
    match value {
        semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionLineSpacingStyle::AtLeast => 1,
        semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionLineSpacingStyle::Exact => 2,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_block_begin_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject, name: &str) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody::BlockBegin(block))) = object.body.as_ref() else {
        return Err(format!("BLOCK {:#x} typed body missing", object.handle));
    };
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &block.common)?;
    let mut strings = DwgBitWriter::new();
    strings.write_tu(name);
    append_r2010_string_stream(&mut data, &strings, "BLOCK", object.handle)?;
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &block.common)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_block_end_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody::BlockEnd(block))) = object.body.as_ref() else {
        return Err(format!("ENDBLK {:#x} typed body missing", object.handle));
    };
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &block.common)?;
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &block.common)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_insert_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody::Insert(insert))) = object.body.as_ref() else {
        return Err(format!("INSERT {:#x} typed body missing", object.handle));
    };
    let insertion = logical_point3(&insert.insertion, "INSERT insertion")?;
    let scale = logical_point3(&insert.scale, "INSERT scale")?;
    let extrusion = logical_point3(&insert.extrusion, "INSERT extrusion")?;
    if scale.contains(&0.0) || !insert.rotation.is_finite() || extrusion == [0.0; 3] || insert.block_header_handle == 0 {
        return Err(format!("INSERT {:#x} transform or block-header relationship is invalid", object.handle));
    }
    let has_attributes = !insert.attribute_handles.is_empty();
    if has_attributes != insert.sequence_end_handle.is_some() {
        return Err(format!("INSERT {:#x} attributes and SEQEND must be present together", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &insert.common)?;
    data.write_3bd(insertion);
    if scale == [1.0; 3] {
        data.write_bb(3);
    } else if scale[0].to_bits() == scale[1].to_bits() && scale[0].to_bits() == scale[2].to_bits() {
        data.write_bb(2);
        data.write_rd(scale[0]);
    } else if scale[0] == 1.0 {
        data.write_bb(1);
        data.write_dd(scale[1], 1.0);
        data.write_dd(scale[2], 1.0);
    } else {
        data.write_bb(0);
        data.write_rd(scale[0]);
        data.write_dd(scale[1], scale[0]);
        data.write_dd(scale[2], scale[0]);
    }
    data.write_bd(insert.rotation);
    data.write_3bd(extrusion);
    data.write_b(has_attributes);
    if has_attributes {
        data.write_bl(insert.attribute_handles.len() as u32);
    }
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &insert.common)?;
    handles.write_handle(5, insert.block_header_handle);
    for attribute in &insert.attribute_handles {
        handles.write_handle(4, *attribute);
    }
    if let Some(sequence_end) = insert.sequence_end_handle {
        handles.write_handle(3, sequence_end);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_dimension_linear_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody::DimensionLinear(linear))) = object.body.as_ref() else {
        return Err(format!("DIMENSION_LINEAR {:#x} typed body missing", object.handle));
    };
    let dimension = &linear.dimension;
    let extrusion = logical_point3(&dimension.extrusion, "dimension extrusion")?;
    let text_midpoint = logical_point2(&dimension.text_midpoint, "dimension text midpoint")?;
    let insertion_scale = logical_point3(&dimension.insertion_scale, "dimension insertion scale")?;
    let clone_point = logical_point2(&dimension.clone_insertion_point, "dimension clone insertion point")?;
    let extension_line_1 = logical_point3(&linear.extension_line_1, "dimension extension line 1")?;
    let extension_line_2 = logical_point3(&linear.extension_line_2, "dimension extension line 2")?;
    let definition_point = logical_point3(&linear.definition_point, "dimension definition point")?;
    if dimension.dimension_style_handle == 0
        || dimension.line_spacing_factor <= 0.0
        || insertion_scale.contains(&0.0)
        || [dimension.elevation, dimension.text_rotation, dimension.horizontal_direction, dimension.insertion_rotation, dimension.line_spacing_factor, dimension.actual_measurement, linear.oblique_angle, linear.dimension_rotation]
            .iter()
            .any(|value| !value.is_finite())
    {
        return Err(format!("DIMENSION_LINEAR {:#x} contains invalid semantic values", object.handle));
    }
    let flag = 0x08 | (if dimension.status.block_reference_is_exclusive { 0x20 | 0x02 } else { 0 }) | (if dimension.status.user_positioned_text { 0x80 } else { 0x01 });
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &dimension.common)?;
    data.write_rc(0);
    data.write_3bd(extrusion);
    data.write_2rd(text_midpoint);
    data.write_bd(dimension.elevation);
    data.write_rc(flag);
    data.write_bd(dimension.text_rotation);
    data.write_bd(dimension.horizontal_direction);
    data.write_3bd(insertion_scale);
    data.write_bd(dimension.insertion_rotation);
    data.write_bs(dimension_attachment_wire(dimension.attachment));
    data.write_bs(dimension_spacing_wire(dimension.line_spacing_style));
    data.write_bd(dimension.line_spacing_factor);
    data.write_bd(dimension.actual_measurement);
    data.write_b(false);
    data.write_b(dimension.flip_arrow_1);
    data.write_b(dimension.flip_arrow_2);
    data.write_2rd(clone_point);
    data.write_3bd(extension_line_1);
    data.write_3bd(extension_line_2);
    data.write_3bd(definition_point);
    data.write_bd(linear.oblique_angle);
    data.write_bd(linear.dimension_rotation);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&dimension.user_text);
    append_r2010_string_stream(&mut data, &strings, "DIMENSION_LINEAR", object.handle)?;
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &dimension.common)?;
    handles.write_handle(5, dimension.dimension_style_handle);
    handles.write_handle(5, dimension.dimension_block_handle.unwrap_or_default());
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn viewport_status_mask(flags: &[semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgViewportStatusFlag]) -> Result<u32, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgViewportStatusFlag::*;
    let mut mask = 0u32;
    for flag in flags {
        let bit = match flag {
            Perspective => 0,
            FrontClipping => 1,
            BackClipping => 2,
            UcsFollow => 3,
            FrontClipNotAtEye => 4,
            UcsIconVisible => 5,
            UcsIconAtOrigin => 6,
            FastZoom => 7,
            Snap => 8,
            Grid => 9,
            IsometricSnap => 10,
            HidePlot => 11,
            IsoPairTop => 12,
            IsoPairRight => 13,
            ZoomLock => 14,
            AlwaysEnabled => 15,
            NonRectangularClipping => 16,
            ViewportOff => 17,
            GridBeyondDrawingLimits => 18,
            AdaptiveGrid => 19,
            AdaptiveSubdivision => 20,
            GridFollowsWorkplane => 21,
        };
        if mask & (1 << bit) != 0 {
            return Err("VIEWPORT status contains a duplicate flag".into());
        }
        mask |= 1 << bit;
    }
    Ok(mask)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_viewport_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDefaultLightingType;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgOrthographicView;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgShadePlotMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgViewportRenderMode;
    let Some(DwgLogicalObjectBody::Entity(DwgEntityBody::Viewport(viewport))) = object.body.as_ref() else {
        return Err(format!("VIEWPORT {:#x} body missing", object.handle));
    };
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &viewport.common)?;
    data.write_3bd(logical_point3(&viewport.center, "VIEWPORT center")?);
    data.write_bd(viewport.width);
    data.write_bd(viewport.height);
    data.write_3bd(logical_point3(&viewport.view_target, "VIEWPORT view target")?);
    data.write_3bd(logical_point3(&viewport.view_direction, "VIEWPORT view direction")?);
    for value in [viewport.twist_angle, viewport.view_height, viewport.lens_length, viewport.front_clip, viewport.back_clip, viewport.snap_angle] {
        data.write_bd(value);
    }
    data.write_2rd(logical_point2(&viewport.view_center, "VIEWPORT view center")?);
    data.write_2rd(logical_point2(&viewport.snap_base, "VIEWPORT snap base")?);
    data.write_2rd(logical_point2(&viewport.snap_unit, "VIEWPORT snap unit")?);
    data.write_2rd(logical_point2(&viewport.grid_unit, "VIEWPORT grid unit")?);
    data.write_bs(viewport.circle_zoom_percent);
    data.write_bs(viewport.grid_major);
    data.write_bl(u32::try_from(viewport.frozen_layer_handles.len()).map_err(|_| "VIEWPORT frozen-layer count exceeds u32")?);
    data.write_bl(viewport_status_mask(&viewport.status)?);
    data.write_rc(match viewport.render_mode {
        DwgViewportRenderMode::Optimized2d => 0,
        DwgViewportRenderMode::Wireframe => 1,
        DwgViewportRenderMode::HiddenLine => 2,
        DwgViewportRenderMode::FlatShaded => 3,
        DwgViewportRenderMode::GouraudShaded => 4,
        DwgViewportRenderMode::FlatShadedWithWireframe => 5,
        DwgViewportRenderMode::GouraudShadedWithWireframe => 6,
    });
    data.write_b(viewport.ucs_at_origin);
    data.write_b(viewport.ucs_per_viewport);
    data.write_3bd(logical_point3(&viewport.ucs_origin, "VIEWPORT UCS origin")?);
    data.write_3bd(logical_point3(&viewport.ucs_x_axis, "VIEWPORT UCS X axis")?);
    data.write_3bd(logical_point3(&viewport.ucs_y_axis, "VIEWPORT UCS Y axis")?);
    data.write_bd(viewport.ucs_elevation);
    data.write_bs(match viewport.orthographic_view {
        DwgOrthographicView::None => 0,
        DwgOrthographicView::Top => 1,
        DwgOrthographicView::Bottom => 2,
        DwgOrthographicView::Front => 3,
        DwgOrthographicView::Back => 4,
        DwgOrthographicView::Left => 5,
        DwgOrthographicView::Right => 6,
    });
    data.write_bs(match viewport.shade_plot_mode {
        DwgShadePlotMode::AsDisplayed => 0,
        DwgShadePlotMode::Wireframe => 1,
        DwgShadePlotMode::Hidden => 2,
        DwgShadePlotMode::Rendered => 3,
    });
    data.write_b(viewport.use_default_lights);
    data.write_rc(match viewport.default_lighting_type {
        DwgDefaultLightingType::OneDistantLight => 0,
        DwgDefaultLightingType::TwoDistantLights => 1,
    });
    data.write_bd(viewport.brightness);
    data.write_bd(viewport.contrast);
    data.write_bs(viewport.ambient_color.index);
    data.write_bl(encode_complex_color_value(&viewport.ambient_color.value));
    data.write_rc(u8::from(viewport.ambient_color.name.is_some()) | (u8::from(viewport.ambient_color.book_name.is_some()) << 1));
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&viewport.style_sheet);
    if let Some(name) = &viewport.ambient_color.name {
        strings.write_tu(name);
    }
    if let Some(name) = &viewport.ambient_color.book_name {
        strings.write_tu(name);
    }
    append_r2010_string_stream(&mut data, &strings, "VIEWPORT", object.handle)?;
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &viewport.common)?;
    for layer in &viewport.frozen_layer_handles {
        handles.write_handle(5, *layer);
    }
    handles.write_handle(5, viewport.clip_boundary_handle.unwrap_or_default());
    handles.write_handle(5, viewport.named_ucs_handle.unwrap_or_default());
    handles.write_handle(5, viewport.base_ucs_handle.unwrap_or_default());
    handles.write_handle(4, viewport.background_handle.unwrap_or_default());
    handles.write_handle(5, viewport.visual_style_handle.unwrap_or_default());
    handles.write_handle(4, viewport.shade_plot_handle.unwrap_or_default());
    handles.write_handle(3, viewport.sun_handle.unwrap_or_default());
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_line_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody::Line(line))) = object.body.as_ref() else {
        return Err(format!("LINE {:#x} typed body missing", object.handle));
    };
    let start = logical_point3(&line.start, "LINE start")?;
    let end = logical_point3(&line.end, "LINE end")?;
    let extrusion = logical_point3(&line.extrusion, "LINE extrusion")?;
    if !line.thickness.is_finite() {
        return Err("LINE thickness must be finite".into());
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &line.common)?;
    let z_is_zero = start[2] == 0.0 && end[2] == 0.0;
    data.write_b(z_is_zero);
    data.write_rd(start[0]);
    data.write_dd(end[0], start[0]);
    data.write_rd(start[1]);
    data.write_dd(end[1], start[1]);
    if !z_is_zero {
        data.write_rd(start[2]);
        data.write_dd(end[2], start[2]);
    }
    data.write_bt(line.thickness);
    data.write_be(extrusion);
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &line.common)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_arc_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody::Arc(arc))) = object.body.as_ref() else {
        return Err(format!("ARC {:#x} typed body missing", object.handle));
    };
    let center = logical_point3(&arc.center, "ARC center")?;
    let extrusion = logical_point3(&arc.extrusion, "ARC extrusion")?;
    if !arc.radius.is_finite() || arc.radius < 0.0 || !arc.thickness.is_finite() || !arc.start_angle.is_finite() || !arc.end_angle.is_finite() || extrusion == [0.0; 3] {
        return Err("ARC scalar values or extrusion are invalid".into());
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &arc.common)?;
    data.write_3bd(center);
    data.write_bd(arc.radius);
    data.write_bt(arc.thickness);
    data.write_be(extrusion);
    data.write_bd(arc.start_angle);
    data.write_bd(arc.end_angle);
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &arc.common)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_lwpolyline_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::Entity(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody::LwPolyline(polyline))) = object.body.as_ref() else {
        return Err(format!("LWPOLYLINE {:#x} typed body missing", object.handle));
    };
    if polyline.vertices.is_empty() || polyline.vertices.len() > 20_000 {
        return Err(format!("LWPOLYLINE {:#x} vertex count is invalid", object.handle));
    }
    let extrusion = logical_point3(&polyline.extrusion, "LWPOLYLINE extrusion")?;
    let points = polyline
        .vertices
        .iter()
        .map(|vertex| if vertex.point.len() != 2 || vertex.point.iter().any(|value| !value.is_finite()) { Err("LWPOLYLINE vertex must have two finite coordinates".to_string()) } else { Ok([vertex.point[0], vertex.point[1]]) })
        .collect::<Result<Vec<_>, _>>()?;
    let has_bulges = polyline.vertices.iter().any(|vertex| vertex.bulge != 0.0);
    let has_ids = polyline.vertices.iter().any(|vertex| vertex.vertex_id.is_some());
    let has_widths = polyline.vertices.iter().any(|vertex| vertex.start_width.is_some() || vertex.end_width.is_some());
    if has_ids && polyline.vertices.iter().any(|vertex| vertex.vertex_id.is_none()) {
        return Err("LWPOLYLINE vertex IDs must cover every vertex".into());
    }
    if has_widths && polyline.vertices.iter().any(|vertex| vertex.start_width.is_none() || vertex.end_width.is_none()) {
        return Err("LWPOLYLINE widths must cover every vertex".into());
    }
    let mut flags = if polyline.closed { 512 } else { 0 };
    flags |= u16::from(polyline.constant_width.is_some()) * 4;
    flags |= u16::from(polyline.elevation != 0.0) * 8;
    flags |= u16::from(polyline.thickness != 0.0) * 2;
    flags |= u16::from(extrusion != [0.0, 0.0, 1.0]);
    flags |= u16::from(has_bulges) * 16;
    flags |= u16::from(has_widths) * 32;
    flags |= u16::from(has_ids) * 1024;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, &polyline.common)?;
    data.write_bs(flags);
    if let Some(width) = polyline.constant_width {
        data.write_bd(width);
    }
    if polyline.elevation != 0.0 {
        data.write_bd(polyline.elevation);
    }
    if polyline.thickness != 0.0 {
        data.write_bd(polyline.thickness);
    }
    if extrusion != [0.0, 0.0, 1.0] {
        data.write_3bd(extrusion);
    }
    data.write_bl(points.len() as u32);
    if has_bulges {
        data.write_bl(points.len() as u32);
    }
    if has_ids {
        data.write_bl(points.len() as u32);
    }
    if has_widths {
        data.write_bl(points.len() as u32);
    }
    data.write_2rd(points[0]);
    for index in 1..points.len() {
        data.write_dd(points[index][0], points[index - 1][0]);
        data.write_dd(points[index][1], points[index - 1][1]);
    }
    if has_bulges {
        for vertex in &polyline.vertices {
            data.write_bd(vertex.bulge);
        }
    }
    if has_ids {
        for vertex in &polyline.vertices {
            data.write_bl(vertex.vertex_id.unwrap());
        }
    }
    if has_widths {
        for vertex in &polyline.vertices {
            data.write_bd(vertex.start_width.unwrap());
            data.write_bd(vertex.end_width.unwrap());
        }
    }
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, &polyline.common)?;
    finish_r2010_object_frame(&data, handles)
}
/// 🧱️ One R2010 entity frame: type, handle, EED and common data, then `body` writes the entity's own
/// main-stream fields (and strings), then the common handle references and `references`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_r2010_entity_frame(
    object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject,
    common: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityCommon,
    body: impl FnOnce(&mut DwgBitWriter, &mut DwgBitWriter) -> Result<(), String>,
    references: impl FnOnce(&mut DwgBitWriter),
) -> Result<Vec<u8>, String> {
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    encode_r2010_entity_common_main(&mut data, object, common)?;
    let mut strings = DwgBitWriter::new();
    body(&mut data, &mut strings)?;
    append_r2010_string_stream(&mut data, &strings, &object.class_name, object.handle)?;
    let mut handles = DwgBitWriter::new();
    encode_r2010_entity_common_handles(&mut handles, object, common)?;
    references(&mut handles);
    finish_r2010_object_frame(&data, handles)
}
/// ✏️ POINT, CIRCLE, ELLIPSE, TEXT, SPLINE, 3DFACE and the POLYLINE_3D / POLYLINE_PFACE families
/// (their VERTEX_3D, VERTEX_PFACE, VERTEX_PFACE_FACE and SEQEND sub-entities), field for field as
/// ODA §20.4 lays them out for R2010.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_entity_body_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::Entity(body)) = object.body.as_ref() else { return Err(format!("{} {:#x} typed body missing", object.class_name, object.handle)) };
    let name = object.class_name.as_str();
    let finite = |values: &[f64]| values.iter().all(|value| value.is_finite());
    match body {
        DwgEntityBody::Point(point) => {
            let at = logical_point3(&point.point, "POINT position")?;
            let extrusion = logical_point3(&point.extrusion, "POINT extrusion")?;
            if !finite(&[point.thickness, point.x_axis_angle]) {
                return Err("POINT scalars must be finite".into());
            }
            encode_r2010_entity_frame(
                object,
                &point.common,
                |data, _| {
                    data.write_3bd(at);
                    data.write_bt(point.thickness);
                    data.write_be(extrusion);
                    data.write_bd(point.x_axis_angle);
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::Circle(circle) => {
            let center = logical_point3(&circle.center, "CIRCLE center")?;
            let extrusion = logical_point3(&circle.extrusion, "CIRCLE extrusion")?;
            if !finite(&[circle.radius, circle.thickness]) || circle.radius < 0.0 || extrusion == [0.0; 3] {
                return Err("CIRCLE radius, thickness or extrusion is invalid".into());
            }
            encode_r2010_entity_frame(
                object,
                &circle.common,
                |data, _| {
                    data.write_3bd(center);
                    data.write_bd(circle.radius);
                    data.write_bt(circle.thickness);
                    data.write_be(extrusion);
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::Ellipse(ellipse) => {
            let center = logical_point3(&ellipse.center, "ELLIPSE center")?;
            let major_axis = logical_point3(&ellipse.major_axis, "ELLIPSE major axis")?;
            let extrusion = logical_point3(&ellipse.extrusion, "ELLIPSE extrusion")?;
            if !finite(&[ellipse.axis_ratio, ellipse.start_parameter, ellipse.end_parameter]) || ellipse.axis_ratio <= 0.0 || ellipse.axis_ratio > 1.0 {
                return Err("ELLIPSE axis ratio or parameters are invalid".into());
            }
            encode_r2010_entity_frame(
                object,
                &ellipse.common,
                |data, _| {
                    data.write_3bd(center);
                    data.write_3bd(major_axis);
                    data.write_3bd(extrusion);
                    data.write_bd(ellipse.axis_ratio);
                    data.write_bd(ellipse.start_parameter);
                    data.write_bd(ellipse.end_parameter);
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::Text(text) => {
            let insertion = logical_point2(&text.insertion, "TEXT insertion")?;
            let alignment = text.alignment.as_deref().map(|value| logical_point2(value, "TEXT alignment")).transpose()?;
            let extrusion = logical_point3(&text.extrusion, "TEXT extrusion")?;
            if !finite(&[text.elevation, text.thickness, text.oblique_angle, text.rotation, text.height, text.width_factor]) || text.style_handle == 0 {
                return Err("TEXT scalars or style are invalid".into());
            }
            let flags = u8::from(text.elevation == 0.0)
                | (u8::from(alignment.is_none()) << 1)
                | (u8::from(text.oblique_angle == 0.0) << 2)
                | (u8::from(text.rotation == 0.0) << 3)
                | (u8::from(text.width_factor == 1.0) << 4)
                | (u8::from(text.generation == 0) << 5)
                | (u8::from(text.horizontal_alignment == 0) << 6)
                | (u8::from(text.vertical_alignment == 0) << 7);
            encode_r2010_entity_frame(
                object,
                &text.common,
                |data, strings| {
                    data.write_rc(flags);
                    if flags & 0x01 == 0 {
                        data.write_rd(text.elevation);
                    }
                    data.write_2rd(insertion);
                    if let Some(alignment) = alignment {
                        data.write_dd(alignment[0], insertion[0]);
                        data.write_dd(alignment[1], insertion[1]);
                    }
                    data.write_be(extrusion);
                    data.write_bt(text.thickness);
                    if flags & 0x04 == 0 {
                        data.write_rd(text.oblique_angle);
                    }
                    if flags & 0x08 == 0 {
                        data.write_rd(text.rotation);
                    }
                    data.write_rd(text.height);
                    if flags & 0x10 == 0 {
                        data.write_rd(text.width_factor);
                    }
                    strings.write_tu(&text.value);
                    for (bit, value) in [(0x20, text.generation), (0x40, text.horizontal_alignment), (0x80, text.vertical_alignment)] {
                        if flags & bit == 0 {
                            data.write_bs(value);
                        }
                    }
                    Ok(())
                },
                |handles| handles.write_handle(5, text.style_handle),
            )
        }
        DwgEntityBody::Spline(spline) => {
            let point_count = spline.control_points.len() / 3;
            if spline.control_points.len() % 3 != 0 || point_count == 0 || !(spline.weights.is_empty() || spline.weights.len() == point_count) || !finite(&spline.control_points) || !finite(&spline.knots) || !finite(&spline.weights) {
                return Err(format!("SPLINE {:#x} control points, knots or weights are invalid", object.handle));
            }
            encode_r2010_entity_frame(
                object,
                &spline.common,
                |data, _| {
                    data.write_bl(1);
                    data.write_bl(spline.degree);
                    data.write_b(spline.rational);
                    data.write_b(spline.closed);
                    data.write_b(spline.periodic);
                    data.write_bd(spline.knot_tolerance);
                    data.write_bd(spline.control_tolerance);
                    data.write_bl(spline.knots.len() as u32);
                    data.write_bl(point_count as u32);
                    data.write_b(!spline.weights.is_empty());
                    for knot in &spline.knots {
                        data.write_bd(*knot);
                    }
                    for (index, point) in spline.control_points.as_chunks::<3>().0.iter().enumerate() {
                        data.write_3bd(*point);
                        if let Some(weight) = spline.weights.get(index) {
                            data.write_bd(*weight);
                        }
                    }
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::Face3d(face) => {
            if face.corners.len() != 12 || !finite(&face.corners) {
                return Err("3DFACE needs four finite corners".into());
            }
            let corners = face.corners.as_chunks::<3>().0;
            let z_is_zero = corners.iter().all(|corner| corner[2] == 0.0);
            encode_r2010_entity_frame(
                object,
                &face.common,
                |data, _| {
                    data.write_b(face.invisible_edges == 0);
                    data.write_b(z_is_zero);
                    data.write_rd(corners[0][0]);
                    data.write_rd(corners[0][1]);
                    if !z_is_zero {
                        data.write_rd(corners[0][2]);
                    }
                    for index in 1..4 {
                        data.write_dd(corners[index][0], corners[index - 1][0]);
                        data.write_dd(corners[index][1], corners[index - 1][1]);
                        if !z_is_zero {
                            data.write_dd(corners[index][2], corners[index - 1][2]);
                        }
                    }
                    if face.invisible_edges != 0 {
                        data.write_bs(face.invisible_edges);
                    }
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::Polyline3d(polyline) => encode_r2010_entity_frame(
            object,
            &polyline.common,
            |data, _| {
                data.write_rc(polyline.curve_type);
                data.write_rc(polyline.flags);
                data.write_bl(polyline.vertex_handles.len() as u32);
                Ok(())
            },
            |handles| {
                for vertex in &polyline.vertex_handles {
                    handles.write_handle(3, *vertex);
                }
                handles.write_handle(3, polyline.sequence_end_handle);
            },
        ),
        DwgEntityBody::PolyfaceMesh(mesh) => encode_r2010_entity_frame(
            object,
            &mesh.common,
            |data, _| {
                data.write_bs(mesh.vertex_count);
                data.write_bs(mesh.face_count);
                data.write_bl(mesh.vertex_handles.len() as u32);
                Ok(())
            },
            |handles| {
                for vertex in &mesh.vertex_handles {
                    handles.write_handle(3, *vertex);
                }
                handles.write_handle(3, mesh.sequence_end_handle);
            },
        ),
        DwgEntityBody::Vertex(vertex) => {
            let point = logical_point3(&vertex.point, "VERTEX point")?;
            encode_r2010_entity_frame(
                object,
                &vertex.common,
                |data, _| {
                    data.write_rc(vertex.flags);
                    data.write_3bd(point);
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::PolyfaceFace(face) => {
            if face.indices.len() != 4 {
                return Err("VERTEX_PFACE_FACE needs four vertex indices".into());
            }
            encode_r2010_entity_frame(
                object,
                &face.common,
                |data, _| {
                    for index in &face.indices {
                        data.write_bs(*index as u16);
                    }
                    Ok(())
                },
                |_| {},
            )
        }
        DwgEntityBody::SequenceEnd(sequence_end) => encode_r2010_entity_frame(object, &sequence_end.common, |_, _| Ok(()), |_| {}),
        _ => Err(format!("{name} {:#x} is encoded by its own frame writer", object.handle)),
    }
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
