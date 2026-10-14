//! 🖌️ Style object writers: visual, table, multiline and multileader styles and materials.

use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::bits::DwgBitWriter;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::append_r2010_string_stream;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_r2010_cell_style;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_r2010_eed;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::finish_r2010_object_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_material_color;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_material_map;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_object_handle;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_table_style_color;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_visual_style_color;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_visual_style_operation;

//#region 🔖️DwgStructuralCodec
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_visual_style_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::VisualStyle(style)) = object.body.as_ref() else { return Err(format!("VISUALSTYLE {:#x} body missing", object.handle)) };
    if style.style_type > 22 || matches!(style.style_type, 10 | 17..=19) || style.extension_lighting_model > 3 {
        return Err(format!("VISUALSTYLE {:#x} style or lighting type is invalid", object.handle));
    }
    let p = &style.properties;
    if p.face_lighting_model.value > 3
        || p.face_lighting_quality.value > 3
        || p.face_color_mode.value > 6
        || p.face_modifiers.value & !3 != 0
        || !p.face_opacity.value.is_finite()
        || !(0.0..=1.0).contains(&p.face_opacity.value)
        || !p.face_specular_amount.value.is_finite()
        || p.edge_model.value > 2
        || p.edge_styles.value & !15 != 0
        || !(1..=11).contains(&p.edge_obscured_line_pattern.value)
        || !(1..=11).contains(&p.edge_intersection_line_pattern.value)
        || !p.edge_crease_angle.value.is_finite()
        || !(-360.0..=360.0).contains(&p.edge_crease_angle.value)
        || p.edge_modifiers.value & !(1 | 2 | 4 | 8 | 16 | 64 | 128) != 0
        || !p.edge_opacity.value.is_finite()
        || !(0.0..=1.0).contains(&p.edge_opacity.value)
        || p.edge_isolines.value > 5000
        || p.display_settings.value & !15 != 0
        || !p.display_brightness.value.is_finite()
        || p.display_shadow_type.value > 3
    {
        return Err(format!("VISUALSTYLE {:#x} contains an invalid property value", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bl(style.style_type);
    data.write_bs(style.extension_lighting_model);
    data.write_b(style.internal_only);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&style.description);
    macro_rules! bl {
        ($field:ident) => {{
            data.write_bl(p.$field.value);
            write_visual_style_operation(&mut data, p.$field.operation);
        }};
    }
    macro_rules! bs {
        ($field:ident) => {{
            data.write_bs(p.$field.value);
            write_visual_style_operation(&mut data, p.$field.operation);
        }};
    }
    macro_rules! bd {
        ($field:ident) => {{
            data.write_bd(p.$field.value);
            write_visual_style_operation(&mut data, p.$field.operation);
        }};
    }
    bl!(face_lighting_model);
    bl!(face_lighting_quality);
    bl!(face_color_mode);
    bs!(face_modifiers);
    bd!(face_opacity);
    bd!(face_specular_amount);
    write_visual_style_color(&mut data, &mut strings, &p.face_monochrome_color);
    bl!(edge_model);
    bl!(edge_styles);
    write_visual_style_color(&mut data, &mut strings, &p.edge_intersection_color);
    write_visual_style_color(&mut data, &mut strings, &p.edge_obscured_color);
    bl!(edge_obscured_line_pattern);
    bl!(edge_intersection_line_pattern);
    bd!(edge_crease_angle);
    bl!(edge_modifiers);
    write_visual_style_color(&mut data, &mut strings, &p.edge_color);
    bd!(edge_opacity);
    bl!(edge_width);
    bl!(edge_overhang);
    bl!(edge_jitter);
    write_visual_style_color(&mut data, &mut strings, &p.edge_silhouette_color);
    bl!(edge_silhouette_width);
    bl!(edge_halo_gap);
    bl!(edge_isolines);
    data.write_b(p.hidden_edge_precision.value);
    write_visual_style_operation(&mut data, p.hidden_edge_precision.operation);
    bl!(display_settings);
    bd!(display_brightness);
    bl!(display_shadow_type);
    append_r2010_string_stream(&mut data, &strings, "VISUALSTYLE", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(xdic) = object.extension_dictionary_handle {
        handles.write_handle(4, xdic);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_table_style_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::TableStyle(body)) = object.body.as_ref() else {
        return Err(format!("TABLESTYLE {:#x} body missing", object.handle));
    };
    if object.owner_handle.is_none() || object.reactor_handles.len() != 1 || object.extension_dictionary_handle.is_none() || !object.extended_data.is_empty() {
        return Err(format!("TABLESTYLE {:#x} common state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(false);
    data.write_rc(0);
    data.write_bl(0);
    data.write_bl(body.bit_flags);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.description);
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(3, object.extension_dictionary_handle.unwrap());
    handles.write_handle(3, body.template_style_handle.unwrap_or_default());
    for (style, id, cell_type, name) in [(&body.table, 4, 2, "Table"), (&body.title, 1, 1, "_TITLE"), (&body.header, 2, 1, "_HEADER"), (&body.data, 3, 2, "_DATA")] {
        if id != 4 {
            data.write_bl(id);
        }
        encode_r2010_cell_style(&mut data, &mut strings, &mut handles, style)?;
        data.write_bl(id);
        data.write_bl(cell_type);
        strings.write_tu(name);
        if id == 4 {
            data.write_bl(3);
        }
    }
    append_r2010_string_stream(&mut data, &strings, "TABLESTYLE", object.handle)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_mline_style_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMlineLinetype;
    let Some(DwgLogicalObjectBody::MlineStyle(body)) = object.body.as_ref() else { return Err(format!("MLINESTYLE {:#x} body missing", object.handle)) };
    if body.name.is_empty()
        || body.elements.is_empty()
        || body.elements.len() > u8::MAX as usize
        || body.elements.windows(2).any(|pair| pair[0].offset <= pair[1].offset)
        || object.owner_handle.is_none()
        || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()]
        || object.extension_dictionary_handle.is_some()
        || !object.extended_data.is_empty()
    {
        return Err(format!("MLINESTYLE {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    let mut flags = 0u16;
    flags |= u16::from(body.fill_enabled);
    flags |= u16::from(body.display_miters) << 1;
    flags |= u16::from(body.start_caps.square) << 4;
    flags |= u16::from(body.start_caps.inner_arcs) << 5;
    flags |= u16::from(body.start_caps.round_outer_arcs) << 6;
    flags |= u16::from(body.end_caps.square) << 8;
    flags |= u16::from(body.end_caps.inner_arcs) << 9;
    flags |= u16::from(body.end_caps.round_outer_arcs) << 10;
    data.write_bs(flags);
    write_table_style_color(&mut data, &body.fill_color)?;
    data.write_bd(body.start_angle);
    data.write_bd(body.end_angle);
    data.write_rc(body.elements.len() as u8);
    for element in &body.elements {
        data.write_bd(element.offset);
        write_table_style_color(&mut data, &element.color)?;
        data.write_bs(match element.linetype {
            DwgMlineLinetype::ByLayer => 32767,
            DwgMlineLinetype::ByBlock => 32766,
            DwgMlineLinetype::Continuous => 0,
        });
    }
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.name);
    strings.write_tu(&body.description);
    append_r2010_string_stream(&mut data, &strings, "MLINESTYLE", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_mleader_style_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::*;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationHistory;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationInfo;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationProperty;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationPropertyKind;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgAuxiliaryHeader;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgClass;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgCompatibilityProfile;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgDependency;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgIndexedPreview;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgJulianDate;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgMeasurement;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgPreviewOrigin;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgProductInformation;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgRevisionHistory;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgSummaryInfo;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgTemplate;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssocNetworkMemberKind;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeActionStatus;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationVariant;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockAction;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionConnection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionCoordinateMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockElement;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockGrip;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockMoveCoordinateMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterBaseLocation;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockScaleMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockTwoPointParameter;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgVisibilityEvaluationHistory;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellBorder;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellBorders;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgCellStyle;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderAttachmentDirection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderBlockConnection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderContentType;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderDrawOrder;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderKind;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMLeaderLeaderOrder;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialColor;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialMap;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialMapSource;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialProjection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMaterialTiling;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgMlineLinetype;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStyleProperty;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::styles::DwgVisualStylePropertyOperation;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColorValue;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableControlBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintGeometry;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintNode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgConstraintNodeCore;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgExplicitConstraint;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgGeometricConstraint;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDefaultLightingType;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionLineSpacingStyle;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgDimensionTextAttachment;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityColorKind;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityCommon;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgEntityReferenceMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgOrthographicView;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgShadePlotMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgViewportRenderMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::entities::DwgViewportStatusFlag;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgExtendedEntityData;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::xrecord::DwgXRecordValue;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderSpaceGeometry;
    use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderVariables;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotArea;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotPaperUnit;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgPlotRotation;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgShadePlot;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgShadePlotResolution;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgStandardScale;
    let Some(DwgLogicalObjectBody::MLeaderStyle(body)) = object.body.as_ref() else { return Err(format!("MLEADERSTYLE {:#x} body missing", object.handle)) };
    if body.leader.linetype_style_handle == 0
        || body.text.style_handle == 0
        || body.block.scale.len() != 3
        || body.block.scale.iter().any(|value| !value.is_finite())
        || object.owner_handle.is_none()
        || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()]
        || object.extension_dictionary_handle.is_some()
        || !object.extended_data.is_empty()
    {
        return Err(format!("MLEADERSTYLE {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bs(2);
    data.write_bs(match body.content_type {
        DwgMLeaderContentType::None => 0,
        DwgMLeaderContentType::Block => 1,
        DwgMLeaderContentType::MText => 2,
    });
    data.write_bs(match body.draw_order {
        DwgMLeaderDrawOrder::LeaderFirst => 0,
        DwgMLeaderDrawOrder::ContentFirst => 1,
    });
    data.write_bs(match body.leader_order {
        DwgMLeaderLeaderOrder::HeadFirst => 0,
        DwgMLeaderLeaderOrder::TailFirst => 1,
    });
    data.write_bl(body.maximum_segment_points);
    data.write_bd(body.first_segment_angle);
    data.write_bd(body.second_segment_angle);
    data.write_bs(match body.leader.kind {
        DwgMLeaderKind::Invisible => 0,
        DwgMLeaderKind::Straight => 1,
        DwgMLeaderKind::Spline => 2,
    });
    write_table_style_color(&mut data, &body.leader.color)?;
    data.write_bl(body.leader.lineweight as u32);
    data.write_b(body.landing.enabled);
    data.write_bd(body.landing.gap);
    data.write_b(body.dogleg.enabled);
    data.write_bd(body.dogleg.length);
    data.write_bd(body.arrow.size);
    data.write_bs(body.text.left_attachment as u16);
    data.write_bs(body.text.right_attachment as u16);
    data.write_bs(body.text.angle as u16);
    data.write_bs(body.text.alignment as u16);
    write_table_style_color(&mut data, &body.text.color)?;
    data.write_bd(body.text.height);
    data.write_b(body.text.frame);
    data.write_b(body.text.always_left);
    data.write_bd(body.text.alignment_space);
    write_table_style_color(&mut data, &body.block.color)?;
    data.write_bd(body.block.scale[0]);
    data.write_bd(body.block.scale[1]);
    data.write_bd(body.block.scale[2]);
    data.write_b(body.block.use_scale);
    data.write_bd(body.block.rotation);
    data.write_b(body.block.use_rotation);
    data.write_bs(match body.block.connection {
        DwgMLeaderBlockConnection::Extents => 0,
        DwgMLeaderBlockConnection::BasePoint => 1,
    });
    data.write_bd(body.overall_scale);
    data.write_b(body.property_overrides_changed);
    data.write_b(body.annotative);
    data.write_bd(body.break_size);
    data.write_bs(match body.text.attachment_direction {
        DwgMLeaderAttachmentDirection::Horizontal => 0,
        DwgMLeaderAttachmentDirection::Vertical => 1,
    });
    data.write_bs(body.text.top_attachment as u16);
    data.write_bs(body.text.bottom_attachment as u16);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.description);
    strings.write_tu(&body.text.default_content);
    append_r2010_string_stream(&mut data, &strings, "MLEADERSTYLE", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(5, body.leader.linetype_style_handle);
    handles.write_handle(5, body.arrow.symbol_handle.unwrap_or_default());
    handles.write_handle(5, body.text.style_handle);
    handles.write_handle(5, body.block.content_handle.unwrap_or_default());
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_material_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::Material(body)) = object.body.as_ref() else { return Err(format!("MATERIAL {:#x} body missing", object.handle)) };
    if body.name.is_empty() || object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] {
        return Err(format!("MATERIAL {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(object.extension_dictionary_handle.is_none());
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.name);
    strings.write_tu(&body.description);
    write_material_color(&mut data, &body.ambient);
    write_material_color(&mut data, &body.diffuse);
    write_material_map(&mut data, &mut strings, &body.diffuse_map)?;
    write_material_color(&mut data, &body.specular);
    write_material_map(&mut data, &mut strings, &body.specular_map)?;
    data.write_bd(body.specular_gloss);
    write_material_map(&mut data, &mut strings, &body.reflection_map)?;
    data.write_bd(body.opacity);
    write_material_map(&mut data, &mut strings, &body.opacity_map)?;
    write_material_map(&mut data, &mut strings, &body.bump_map)?;
    data.write_bd(body.refraction_index);
    write_material_map(&mut data, &mut strings, &body.refraction_map)?;
    data.write_bd(body.translucence);
    data.write_bd(body.self_illumination);
    data.write_bd(body.reflectivity);
    data.write_bl(0);
    let channels = &body.enabled_channels;
    data.write_bl(u32::from(channels.diffuse) | u32::from(channels.specular) << 1 | u32::from(channels.reflection) << 2 | u32::from(channels.opacity) << 3 | u32::from(channels.bump) << 4 | u32::from(channels.refraction) << 5);
    data.write_bl(0);
    append_r2010_string_stream(&mut data, &strings, "MATERIAL", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    if let Some(handle) = object.extension_dictionary_handle {
        handles.write_handle(3, handle);
    }
    finish_r2010_object_frame(&data, handles)
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
