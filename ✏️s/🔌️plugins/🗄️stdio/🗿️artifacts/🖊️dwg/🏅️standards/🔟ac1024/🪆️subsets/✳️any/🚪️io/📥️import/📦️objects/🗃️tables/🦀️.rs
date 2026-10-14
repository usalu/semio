//! 🗃️ Symbol-table object decoders: table controls and table records.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DwgBitReader;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_complex_color_value;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::read_object_handle;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::read_r2010_cmc_main;

//#region 🔖️DwgStructuralCodec
//#region DwgRead
/// 🧩️ Decodes one table control object body from its main, string and handle streams.
pub fn decode_table_control_object(handle: u64, type_code: u16, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgBlockTableControl;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionStyleTableControl;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgLinetypeTableControl;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableControlBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableControlEntries;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableControlEntry;
    let entry_count = match type_code {
        48 | 50 | 52 | 60 => data.read_bl().map(|value| value as usize),
        _ => data.read_bs().map(|value| value as usize),
    }
    .map_err(|error| format!("table control {handle:#x} entry count: {error}"))?;
    let additional_count = if type_code == 68 { data.read_rc().map_err(|error| format!("DIMSTYLE control {handle:#x} additional count: {error}"))? as usize } else { 0 };
    let mut entry_handles = Vec::with_capacity(entry_count);
    for index in 0..entry_count {
        entry_handles.push(DwgTableControlEntry { handle: read_object_handle(handle_reader, handle).map_err(|error| format!("table control {handle:#x} entry {index}: {error}"))? });
    }
    let body = match type_code {
        48 => DwgTableControlBody::Block(DwgBlockTableControl {
            entry_handles,
            model_space_handle: Some(read_object_handle(handle_reader, handle)?.ok_or("BLOCK_CONTROL model-space handle is null")?),
            paper_space_handle: Some(read_object_handle(handle_reader, handle)?.ok_or("BLOCK_CONTROL paper-space handle is null")?),
        }),
        50 => DwgTableControlBody::Layer(DwgTableControlEntries { entry_handles }),
        52 => DwgTableControlBody::TextStyle(DwgTableControlEntries { entry_handles }),
        56 => DwgTableControlBody::Linetype(DwgLinetypeTableControl {
            entry_handles,
            by_block_handle: read_object_handle(handle_reader, handle)?.ok_or("LTYPE_CONTROL ByBlock handle is null")?,
            by_layer_handle: read_object_handle(handle_reader, handle)?.ok_or("LTYPE_CONTROL ByLayer handle is null")?,
        }),
        60 => DwgTableControlBody::View(DwgTableControlEntries { entry_handles }),
        62 => DwgTableControlBody::Ucs(DwgTableControlEntries { entry_handles }),
        64 => DwgTableControlBody::Viewport(DwgTableControlEntries { entry_handles }),
        66 => DwgTableControlBody::RegisteredApplication(DwgTableControlEntries { entry_handles }),
        68 => {
            let mut additional_handles = Vec::with_capacity(additional_count);
            for index in 0..additional_count {
                additional_handles.push(
                    read_object_handle(handle_reader, handle)
                        .map_err(|error| format!("DIMSTYLE control {handle:#x} additional handle {index}: {error}"))?
                        .ok_or_else(|| format!("DIMSTYLE control {handle:#x} additional handle {index} is null"))?,
                );
            }
            DwgTableControlBody::DimensionStyle(DwgDimensionStyleTableControl { entry_handles, additional_handles })
        }
        _ => return Err(format!("unsupported table control type {type_code}")),
    };
    object.body = Some(DwgLogicalObjectBody::TableControl(body));

    Ok(())
}
/// 🧩️ Decodes one table record object body from its main, string and handle streams.
pub fn decode_table_record_object(handle: u64, type_code: u16, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    let xref_resolution = data.read_bs().map_err(|error| format!("table record {handle:#x} xref resolution: {error}"))?;
    if !matches!(xref_resolution, 0 | 256) {
        return Err(format!("table record {handle:#x} xref resolution {xref_resolution} is invalid"));
    }
    let text_style = if type_code == 53 {
        Some((
            data.read_b().map_err(|error| format!("text style {handle:#x} shape flag: {error}"))?,
            data.read_b().map_err(|error| format!("text style {handle:#x} vertical flag: {error}"))?,
            data.read_bd().map_err(|error| format!("text style {handle:#x} text size: {error}"))?,
            data.read_bd().map_err(|error| format!("text style {handle:#x} width factor: {error}"))?,
            data.read_bd().map_err(|error| format!("text style {handle:#x} oblique angle: {error}"))?,
            data.read_rc().map_err(|error| format!("text style {handle:#x} generation: {error}"))?,
            data.read_bd().map_err(|error| format!("text style {handle:#x} last height: {error}"))?,
        ))
    } else {
        None
    };
    let viewport = if type_code == 65 {
        let view_height = data.read_bd().map_err(|error| format!("viewport table {handle:#x} height: {error}"))?;
        let view_width = data.read_bd().map_err(|error| format!("viewport table {handle:#x} width: {error}"))?;
        let center = data.read_2rd().map_err(|error| format!("viewport table {handle:#x} center: {error}"))?;
        let target = data.read_3bd().map_err(|error| format!("viewport table {handle:#x} target: {error}"))?;
        let direction = data.read_3bd().map_err(|error| format!("viewport table {handle:#x} direction: {error}"))?;
        let twist = data.read_bd().map_err(|error| format!("viewport table {handle:#x} twist: {error}"))?;
        let lens_length = data.read_bd().map_err(|error| format!("viewport table {handle:#x} lens: {error}"))?;
        let front_clipping = data.read_bd().map_err(|error| format!("viewport table {handle:#x} front clipping: {error}"))?;
        let back_clipping = data.read_bd().map_err(|error| format!("viewport table {handle:#x} back clipping: {error}"))?;
        let view_mode = [data.read_b()?, data.read_b()?, data.read_b()?, data.read_b()?];
        let render_mode = data.read_rc()?;
        let use_default_lights = data.read_b()?;
        let default_lighting_type = data.read_rc()?;
        let brightness = data.read_bd()?;
        let contrast = data.read_bd()?;
        let ambient_index = data.read_bs()?;
        let ambient_rgb = data.read_bl()?;
        let ambient_flags = data.read_rc()?;
        if ambient_flags > 3 {
            return Err(format!("viewport table {handle:#x} ambient color flags {ambient_flags:#x} are invalid"));
        }
        Some((
            semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgViewportTableRecord {
                common: Default::default(),
                view_height,
                view_width,
                center,
                target,
                direction,
                twist,
                lens_length,
                front_clipping,
                back_clipping,
                view_mode,
                render_mode,
                use_default_lights,
                default_lighting_type,
                brightness,
                contrast,
                ambient_color: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor { index: ambient_index, value: decode_complex_color_value(ambient_rgb)?, name: None, book_name: None },
                lower_left: data.read_2rd()?,
                upper_right: data.read_2rd()?,
                ucs_follow: data.read_b()?,
                circle_zoom: data.read_bs()?,
                fast_zoom: data.read_b()?,
                ucs_icon: data.read_bb()?,
                grid_mode: data.read_b()?,
                grid_unit: data.read_2rd()?,
                snap_mode: data.read_b()?,
                snap_style: data.read_b()?,
                snap_isopair: data.read_bs()?,
                snap_angle: data.read_bd()?,
                snap_base: data.read_2rd()?,
                snap_unit: data.read_2rd()?,
                ucs_at_origin: data.read_b()?,
                ucs_viewport: data.read_b()?,
                ucs_origin: data.read_3bd()?,
                ucs_x_axis: data.read_3bd()?,
                ucs_y_axis: data.read_3bd()?,
                ucs_elevation: data.read_bd()?,
                ucs_orthographic_view: data.read_bs()?,
                grid_flags: data.read_bs()?,
                grid_major: data.read_bs()?,
                background_handle: None,
                visual_style_handle: None,
                sun_handle: None,
                named_ucs_handle: None,
                base_ucs_handle: None,
            },
            ambient_flags,
        ))
    } else {
        None
    };
    let dimension_style = if type_code == 69 {
        use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionBehavior;
        use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionGeometry;
        use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionR2010;
        use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionStyleTableRecord;
        use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionText;
        use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgDimensionUnits;
        let geometry = DwgDimensionGeometry {
            scale: data.read_bd()?,
            arrow_size: data.read_bd()?,
            extension_origin_offset: data.read_bd()?,
            dimension_line_increment: data.read_bd()?,
            extension_line_extension: data.read_bd()?,
            rounding: data.read_bd()?,
            dimension_line_extension: data.read_bd()?,
            plus_tolerance: data.read_bd()?,
            minus_tolerance: data.read_bd()?,
            fixed_extension_length: data.read_bd()?,
            jog_angle: data.read_bd()?,
        };
        let fill_mode = data.read_bs()?;
        let (fill_color, fill_flags) = read_r2010_cmc_main(data)?;
        let behavior = DwgDimensionBehavior {
            tolerance: data.read_b()?,
            limits: data.read_b()?,
            text_inside_horizontal: data.read_b()?,
            text_outside_horizontal: data.read_b()?,
            suppress_extension_1: data.read_b()?,
            suppress_extension_2: data.read_b()?,
            text_vertical_alignment: data.read_bs()?,
            zero_suppression: data.read_bs()?,
            angular_zero_suppression: data.read_bs()?,
            arc_symbol: data.read_bs()?,
        };
        let mut text = DwgDimensionText {
            height: data.read_bd()?,
            center_mark_size: data.read_bd()?,
            tick_size: data.read_bd()?,
            alternate_scale: data.read_bd()?,
            linear_scale: data.read_bd()?,
            vertical_position: data.read_bd()?,
            tolerance_scale: data.read_bd()?,
            gap: data.read_bd()?,
            alternate_rounding: data.read_bd()?,
            alternate_enabled: data.read_b()?,
            alternate_decimals: data.read_bs()?,
            text_outside_extensions: data.read_b()?,
            separate_arrowheads: data.read_b()?,
            force_text_inside: data.read_b()?,
            suppress_outside_extensions: data.read_b()?,
            ..Default::default()
        };
        let (c1, c1f) = read_r2010_cmc_main(data)?;
        let (c2, c2f) = read_r2010_cmc_main(data)?;
        let (c3, c3f) = read_r2010_cmc_main(data)?;
        text.dimension_line_color = c1;
        text.extension_line_color = c2;
        text.text_color = c3;
        let units = DwgDimensionUnits {
            alternate_decimal_places: data.read_bs()?,
            decimal_places: data.read_bs()?,
            tolerance_decimal_places: data.read_bs()?,
            alternate_units: data.read_bs()?,
            alternate_tolerance_decimal_places: data.read_bs()?,
            angular_units: data.read_bs()?,
            fraction_format: data.read_bs()?,
            linear_units: data.read_bs()?,
            decimal_separator: data.read_bs()?,
            text_movement: data.read_bs()?,
            text_horizontal_alignment: data.read_bs()?,
            suppress_dimension_line_1: data.read_b()?,
            suppress_dimension_line_2: data.read_b()?,
            tolerance_vertical_alignment: data.read_bs()?,
            tolerance_zero_suppression: data.read_bs()?,
            alternate_zero_suppression: data.read_bs()?,
            alternate_tolerance_zero_suppression: data.read_bs()?,
            user_positioned_text: data.read_b()?,
            arrow_text_fit: data.read_bs()?,
        };
        let r2010 = DwgDimensionR2010 {
            fixed_extension_enabled: data.read_b()?,
            text_direction: data.read_b()?,
            alternate_measurement_factor: data.read_bd()?,
            alternate_measurement_suffix: String::new(),
            measurement_factor: data.read_bd()?,
            measurement_suffix: String::new(),
            dimension_lineweight: data.read_bs()?,
            extension_lineweight: data.read_bs()?,
            flag: data.read_b()?,
        };
        Some((
            DwgDimensionStyleTableRecord {
                common: Default::default(),
                dimension_postfix: String::new(),
                alternate_postfix: String::new(),
                geometry,
                fill_mode,
                fill_color,
                behavior,
                text,
                units,
                r2010,
                text_style_handle: None,
                leader_arrow_handle: None,
                arrow_handle: None,
                arrow_1_handle: None,
                arrow_2_handle: None,
                dimension_linetype_handle: None,
                extension_1_linetype_handle: None,
                extension_2_linetype_handle: None,
            },
            [fill_flags, c1f, c2f, c3f],
        ))
    } else {
        None
    };
    let block_header = if type_code == 49 {
        let anonymous = data.read_b().map_err(|error| format!("block header {handle:#x} anonymous flag: {error}"))?;
        let has_attributes = data.read_b().map_err(|error| format!("block header {handle:#x} attribute flag: {error}"))?;
        let is_xref = data.read_b().map_err(|error| format!("block header {handle:#x} xref flag: {error}"))?;
        let xref_overlaid = data.read_b().map_err(|error| format!("block header {handle:#x} overlay flag: {error}"))?;
        let xref_loaded = data.read_b().map_err(|error| format!("block header {handle:#x} loaded flag: {error}"))?;
        let owned_count = if !is_xref && !xref_overlaid { data.read_bl().map_err(|error| format!("block header {handle:#x} owned count: {error}"))? as usize } else { 0 };
        let base_point = data.read_3bd().map_err(|error| format!("block header {handle:#x} base point: {error}"))?;
        let mut insert_count = 0usize;
        loop {
            let marker = data.read_rc().map_err(|error| format!("block header {handle:#x} insert marker: {error}"))?;
            if marker == 0 {
                break;
            }
            if marker != 1 || insert_count >= 0xefffff {
                return Err(format!("block header {handle:#x} insert marker {marker} is invalid"));
            }
            insert_count += 1;
        }
        let preview_size = data.read_bl().map_err(|error| format!("block header {handle:#x} preview size: {error}"))? as usize;
        if preview_size != 0 {
            return Err(format!("block header {handle:#x} has unsupported semantic preview of {preview_size} bytes"));
        }
        let insert_units = data.read_bs().map_err(|error| format!("block header {handle:#x} insert units: {error}"))?;
        let explodable = data.read_b().map_err(|error| format!("block header {handle:#x} explodable flag: {error}"))?;
        let block_scaling = data.read_rc().map_err(|error| format!("block header {handle:#x} block scaling: {error}"))?;
        Some((anonymous, has_attributes, is_xref, xref_overlaid, xref_loaded, owned_count, base_point, insert_count, insert_units, explodable, block_scaling))
    } else {
        None
    };
    let linetype = if type_code == 57 {
        let pattern_length = data.read_bd().map_err(|error| format!("linetype {handle:#x} pattern length: {error}"))?;
        let alignment = data.read_rc().map_err(|error| format!("linetype {handle:#x} alignment: {error}"))?;
        let dash_count = data.read_rc().map_err(|error| format!("linetype {handle:#x} dash count: {error}"))? as usize;
        let mut dashes = Vec::with_capacity(dash_count);
        for index in 0..dash_count {
            dashes.push((
                data.read_bd().map_err(|error| format!("linetype {handle:#x} dash {index} length: {error}"))?,
                data.read_bs().map_err(|error| format!("linetype {handle:#x} dash {index} shape code: {error}"))?,
                data.read_rd().map_err(|error| format!("linetype {handle:#x} dash {index} X offset: {error}"))?,
                data.read_rd().map_err(|error| format!("linetype {handle:#x} dash {index} Y offset: {error}"))?,
                data.read_bd().map_err(|error| format!("linetype {handle:#x} dash {index} scale: {error}"))?,
                data.read_bd().map_err(|error| format!("linetype {handle:#x} dash {index} rotation: {error}"))?,
                data.read_bs().map_err(|error| format!("linetype {handle:#x} dash {index} shape flags: {error}"))?,
            ));
        }
        Some((pattern_length, alignment, dashes))
    } else {
        None
    };
    let layer = if type_code == 51 {
        let flag0 = data.read_bs().map_err(|error| format!("layer {handle:#x} flags: {error}"))?;
        let color_index = data.read_bs().map_err(|error| format!("layer {handle:#x} color index: {error}"))?;
        let color_rgb = data.read_bl().map_err(|error| format!("layer {handle:#x} color RGB: {error}"))?;
        let color_flags = data.read_rc().map_err(|error| format!("layer {handle:#x} color flags: {error}"))?;
        if color_flags > 3 {
            return Err(format!("layer {handle:#x} color flags {color_flags:#x} are invalid"));
        }
        Some((flag0, color_index, color_rgb, color_flags))
    } else {
        None
    };
    let group_71 = if type_code == 67 { Some(data.read_rc().map_err(|error| format!("registered-application {handle:#x} group-71 marker: {error}"))?) } else { None };
    if data.bit_position() != main_end_bit {
        return Err(format!("table record {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("table record {handle:#x} string stream missing"))?;
    let name = strings.read_tu().map_err(|error| format!("table record {handle:#x} name: {error}"))?;
    let xref_handle = read_object_handle(handle_reader, handle).map_err(|error| format!("table record {handle:#x} xref handle: {error}"))?;
    let common = semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordCommon { name, xref_resolution, xref_handle };
    let body = if let Some((mut value, color_flags)) = dimension_style {
        value.common = common;
        value.dimension_postfix = strings.read_tu()?;
        value.alternate_postfix = strings.read_tu()?;
        let mut read_color_strings = |color: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor, flags: u8| -> Result<(), String> {
            if flags & 1 != 0 {
                color.name = Some(strings.read_tu()?);
            }
            if flags & 2 != 0 {
                color.book_name = Some(strings.read_tu()?);
            }
            Ok(())
        };
        read_color_strings(&mut value.fill_color, color_flags[0])?;
        read_color_strings(&mut value.text.dimension_line_color, color_flags[1])?;
        read_color_strings(&mut value.text.extension_line_color, color_flags[2])?;
        read_color_strings(&mut value.text.text_color, color_flags[3])?;
        value.r2010.alternate_measurement_suffix = strings.read_tu()?;
        value.r2010.measurement_suffix = strings.read_tu()?;
        value.text_style_handle = read_object_handle(handle_reader, handle)?;
        value.leader_arrow_handle = read_object_handle(handle_reader, handle)?;
        value.arrow_handle = read_object_handle(handle_reader, handle)?;
        value.arrow_1_handle = read_object_handle(handle_reader, handle)?;
        value.arrow_2_handle = read_object_handle(handle_reader, handle)?;
        value.dimension_linetype_handle = read_object_handle(handle_reader, handle)?;
        value.extension_1_linetype_handle = read_object_handle(handle_reader, handle)?;
        value.extension_2_linetype_handle = read_object_handle(handle_reader, handle)?;
        semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody::DimensionStyle(value)
    } else if let Some((mut value, ambient_flags)) = viewport {
        value.common = common;
        if ambient_flags & 1 != 0 {
            value.ambient_color.name = Some(strings.read_tu().map_err(|error| format!("viewport table {handle:#x} ambient color name: {error}"))?);
        }
        if ambient_flags & 2 != 0 {
            value.ambient_color.book_name = Some(strings.read_tu().map_err(|error| format!("viewport table {handle:#x} ambient color book: {error}"))?);
        }
        value.background_handle = read_object_handle(handle_reader, handle)?;
        value.visual_style_handle = read_object_handle(handle_reader, handle)?;
        value.sun_handle = read_object_handle(handle_reader, handle)?;
        value.named_ucs_handle = read_object_handle(handle_reader, handle)?;
        value.base_ucs_handle = read_object_handle(handle_reader, handle)?;
        semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody::Viewport(value)
    } else if let Some((anonymous, has_attributes, is_xref, xref_overlaid, xref_loaded, owned_count, base_point, insert_count, insert_units, explodable, block_scaling)) = block_header {
        let xref_path = strings.read_tu().map_err(|error| format!("block header {handle:#x} xref path: {error}"))?;
        let description = strings.read_tu().map_err(|error| format!("block header {handle:#x} description: {error}"))?;
        let block_entity_handle = read_object_handle(handle_reader, handle).map_err(|error| format!("block header {handle:#x} block entity: {error}"))?.ok_or_else(|| format!("block header {handle:#x} block entity is null"))?;
        let mut owned_entity_handles = Vec::with_capacity(owned_count);
        for index in 0..owned_count {
            owned_entity_handles
                .push(read_object_handle(handle_reader, handle).map_err(|error| format!("block header {handle:#x} owned entity {index}: {error}"))?.ok_or_else(|| format!("block header {handle:#x} owned entity {index} is null"))?);
        }
        let end_block_entity_handle =
            read_object_handle(handle_reader, handle).map_err(|error| format!("block header {handle:#x} end-block entity: {error}"))?.ok_or_else(|| format!("block header {handle:#x} end-block entity is null"))?;
        let mut insert_backreference_handles = Vec::with_capacity(insert_count);
        for index in 0..insert_count {
            insert_backreference_handles.push(
                read_object_handle(handle_reader, handle)
                    .map_err(|error| format!("block header {handle:#x} insert backreference {index}: {error}"))?
                    .ok_or_else(|| format!("block header {handle:#x} insert backreference {index} is null"))?,
            );
        }
        let layout_handle = read_object_handle(handle_reader, handle).map_err(|error| format!("block header {handle:#x} layout: {error}"))?;
        semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody::BlockHeader(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgBlockHeaderTableRecord {
            common,
            anonymous,
            has_attributes,
            is_xref,
            xref_overlaid,
            xref_loaded,
            owned_entity_handles,
            base_point,
            xref_path,
            insert_backreference_handles,
            description,
            insert_units,
            explodable,
            block_scaling,
            block_entity_handle,
            end_block_entity_handle,
            layout_handle,
        })
    } else if let Some((pattern_length, alignment, dash_values)) = linetype {
        let description = strings.read_tu().map_err(|error| format!("linetype {handle:#x} description: {error}"))?;
        let mut dashes = Vec::with_capacity(dash_values.len());
        for (index, (length, complex_shape_code, x_offset, y_offset, scale, rotation, shape_flags)) in dash_values.into_iter().enumerate() {
            let style_handle = read_object_handle(handle_reader, handle).map_err(|error| format!("linetype {handle:#x} dash {index} style: {error}"))?;
            dashes.push(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgLinetypeDash { length, complex_shape_code, style_handle, x_offset, y_offset, scale, rotation, shape_flags, text: None });
        }
        semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody::Linetype(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgLinetypeTableRecord { common, description, pattern_length, alignment, dashes })
    } else if let Some((flag0, color_index, color_rgb, color_flags)) = layer {
        let color_name = if color_flags & 1 != 0 { Some(strings.read_tu().map_err(|error| format!("layer {handle:#x} color name: {error}"))?) } else { None };
        let color_book_name = if color_flags & 2 != 0 { Some(strings.read_tu().map_err(|error| format!("layer {handle:#x} color book name: {error}"))?) } else { None };
        let plot_style_handle = read_object_handle(handle_reader, handle).map_err(|error| format!("layer {handle:#x} plot-style handle: {error}"))?;
        let material_handle = read_object_handle(handle_reader, handle).map_err(|error| format!("layer {handle:#x} material handle: {error}"))?;
        let linetype_handle = read_object_handle(handle_reader, handle).map_err(|error| format!("layer {handle:#x} linetype handle: {error}"))?;
        semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody::Layer(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgLayerTableRecord {
            common,
            frozen: flag0 & 1 != 0,
            off: flag0 & 2 != 0,
            frozen_in_new_viewports: flag0 & 4 != 0,
            locked: flag0 & 8 != 0,
            plottable: flag0 & 16 != 0,
            lineweight: ((flag0 & 0x03e0) >> 5) as u8,
            color: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgComplexColor { index: color_index, value: decode_complex_color_value(color_rgb)?, name: color_name, book_name: color_book_name },
            plot_style_handle,
            material_handle,
            linetype_handle,
        })
    } else if let Some((is_shape, is_vertical, text_size, width_factor, oblique_angle, generation, last_height)) = text_style {
        let font_file = strings.read_tu().map_err(|error| format!("text style {handle:#x} font file: {error}"))?;
        let big_font_file = strings.read_tu().map_err(|error| format!("text style {handle:#x} big-font file: {error}"))?;
        semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody::TextStyle(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTextStyleTableRecord { common, is_shape, is_vertical, text_size, width_factor, oblique_angle, generation, last_height, font_file, big_font_file })
    } else {
        semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgTableRecordBody::RegisteredApplication(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::tables::DwgRegisteredApplicationTableRecord { common, group_71: group_71.unwrap() })
    };
    object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::TableRecord(body));

    Ok(())
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
