//! 🧩️ Dynamic-block object decoders: parameters, grips, actions, visibility and flip states, proxies and representation data.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DwgBitReader;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::validate_entity_terminal_fill;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_r2010_block_action;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_r2010_block_element;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_r2010_block_grip;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_r2010_two_point_parameter;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::r2010_string_content_end_bit;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::read_object_handle;

//#region 🔖️DwgStructuralCodec
//#region DwgRead
/// 🧩️ Decodes one block linear parameter object body from its main, string and handle streams.
pub fn decode_block_linear_parameter_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockLinearParameter;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterBaseLocation;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterConnection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterProperty;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockTwoPointParameter;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgPropertyExpressionReference;
    let strings = strings.as_mut().ok_or_else(|| format!("BLOCKLINEARPARAMETER {handle:#x} string stream missing"))?;
    let element = decode_r2010_block_element(data, strings, "BLOCKLINEARPARAMETER")?;
    let show_properties = data.read_b()?;
    let chain_actions = data.read_b()?;
    let definition_base = data.read_3bd()?.to_vec();
    let definition_end = data.read_3bd()?.to_vec();
    let mut properties = Vec::with_capacity(4);
    for _ in 0..4 {
        let count = data.read_bl()? as usize;
        if count > 10_000 {
            return Err(format!("BLOCKLINEARPARAMETER {handle:#x} connection count is invalid"));
        }
        let codes = (0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
        let connections = codes.into_iter().map(|code| Ok(DwgBlockParameterConnection { code, name: strings.read_tu()? })).collect::<Result<Vec<_>, String>>()?;
        properties.push(DwgBlockParameterProperty { connections });
    }
    let property_expression_references =
        (0..4).map(|property_index| data.read_bl().map(|node_id| (node_id != 0).then_some(DwgPropertyExpressionReference { property_index, node_id }))).collect::<Result<Vec<_>, _>>()?.into_iter().flatten().collect();
    let base_location = match data.read_bs()? {
        0 => DwgBlockParameterBaseLocation::StartPoint,
        1 => DwgBlockParameterBaseLocation::Midpoint,
        value => return Err(format!("BLOCKLINEARPARAMETER {handle:#x} base location {value} is unsupported")),
    };
    let distance_name = strings.read_tu()?;
    let distance_description = strings.read_tu()?;
    let label_offset = data.read_bd()?;
    let value_flags = data.read_bl()?;
    let minimum = data.read_bd()?;
    let maximum = data.read_bd()?;
    let increment = data.read_bd()?;
    let value_count = data.read_bs()? as usize;
    let allowed_values = (0..value_count).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
    if value_flags != 8
        || (minimum, maximum, increment) != (0.0, 0.0, 0.0)
        || allowed_values.is_empty()
        || definition_base.iter().chain(&definition_end).chain(&allowed_values).any(|value| !value.is_finite())
        || !label_offset.is_finite()
        || data.bit_position() != main_end_bit
        || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)?
    {
        return Err(format!("BLOCKLINEARPARAMETER {handle:#x} logical value or stream boundary is invalid"));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "BLOCKLINEARPARAMETER")?;
    object.body = Some(DwgLogicalObjectBody::BlockLinearParameter(DwgBlockLinearParameter {
        parameter: DwgBlockTwoPointParameter { element, show_properties, chain_actions, definition_base, definition_end, properties, property_expression_references, base_location },
        distance_name,
        distance_description,
        label_offset,
        allowed_values,
    }));

    Ok(())
}
/// 🧩️ Decodes one block parameter grip object body from its main, string and handle streams.
pub fn decode_block_parameter_grip_object(handle: u64, type_code: u16, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockFlipGrip;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockLinearGrip;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockVisibilityGrip;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgNamedEvaluationNodeReference;
    let strings = strings.as_mut().ok_or_else(|| format!("{} {handle:#x} string stream missing", object.class_name))?;
    object.body = Some(match type_code {
        528 => {
            let grip = decode_r2010_block_grip(data, strings, "UpdatedEndX", "UpdatedEndY", "BLOCKLINEARGRIP")?;
            let orientation = data.read_3bd()?.to_vec();
            if orientation.iter().any(|value| !value.is_finite()) || orientation.iter().all(|value| *value == 0.0) {
                return Err(format!("BLOCKLINEARGRIP {handle:#x} orientation is invalid"));
            }
            DwgLogicalObjectBody::BlockLinearGrip(DwgBlockLinearGrip { grip, orientation })
        }
        530 => {
            let grip = decode_r2010_block_grip(data, strings, "UpdatedBaseX", "UpdatedBaseY", "BLOCKFLIPGRIP")?;
            let updated_flip = DwgNamedEvaluationNodeReference { node_id: data.read_bl()?, expression_name: "UpdatedFlip".into() };
            let orientation = data.read_3bd()?.to_vec();
            if orientation.iter().any(|value| !value.is_finite()) || orientation.iter().all(|value| *value == 0.0) {
                return Err(format!("BLOCKFLIPGRIP {handle:#x} orientation is invalid"));
            }
            DwgLogicalObjectBody::BlockFlipGrip(DwgBlockFlipGrip { grip, updated_flip, orientation })
        }
        532 => {
            let grip = decode_r2010_block_grip(data, strings, "UpdatedX", "UpdatedY", "BLOCKVISIBILITYGRIP")?;
            DwgLogicalObjectBody::BlockVisibilityGrip(DwgBlockVisibilityGrip { grip })
        }
        _ => return Err(format!("{} {handle:#x} type code is invalid", object.class_name)),
    });
    if data.bit_position() != main_end_bit || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
        return Err(format!("{} {handle:#x} stream boundary is invalid", object.class_name));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, &object.class_name)?;

    Ok(())
}
/// 🧩️ Decodes one block alignment and action object body from its main, string and handle streams.
pub fn decode_block_alignment_and_action_object(handle: u64, type_code: u16, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionConnection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionCoordinateMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionWithBasePoint;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockAlignmentGrip;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockAlignmentParameter;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockFlipAction;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockScaleAction;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockScaleMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockStretchAction;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgStretchSelection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgStretchSelector;
    let strings = strings.as_mut().ok_or_else(|| format!("{} {handle:#x} string stream missing", object.class_name))?;
    object.body = Some(match type_code {
        533 => {
            let (parameter, property_node_ids) = decode_r2010_two_point_parameter(data, strings, "BLOCKALIGNMENTPARAMETER")?;
            let align_perpendicular = data.read_b()?;
            if property_node_ids[0] == 0 || property_node_ids[1..] != [0, 0, 0] {
                return Err(format!("BLOCKALIGNMENTPARAMETER {handle:#x} property relation is invalid"));
            }
            DwgLogicalObjectBody::BlockAlignmentParameter(DwgBlockAlignmentParameter { parameter, updated_grip_node_id: property_node_ids[0], align_perpendicular })
        }
        534 => {
            let grip = decode_r2010_block_grip(data, strings, "FirstLocation", "SecondLocation", "BLOCKALIGNMENTGRIP")?;
            let orientation = data.read_3bd()?.to_vec();
            if orientation.iter().any(|value| !value.is_finite()) || orientation.iter().all(|value| *value == 0.0) {
                return Err(format!("BLOCKALIGNMENTGRIP {handle:#x} orientation is invalid"));
            }
            let first_location_node_id = grip.updated_x.node_id;
            let second_location_node_id = grip.updated_y.node_id;
            DwgLogicalObjectBody::BlockAlignmentGrip(DwgBlockAlignmentGrip { grip, first_location_node_id, second_location_node_id, orientation })
        }
        535 => {
            let action = decode_r2010_block_action(data, strings, handle_reader, handle, "BLOCKSTRETCHACTION")?;
            let x_node_id = data.read_bl()?;
            let x_name = strings.read_tu()?;
            let y_node_id = data.read_bl()?;
            let y_name = strings.read_tu()?;
            let point_count = data.read_bl()? as usize;
            if point_count == 0 || point_count > 10_000 {
                return Err(format!("BLOCKSTRETCHACTION {handle:#x} point count is invalid"));
            }
            let points = (0..point_count).map(|_| data.read_2rd().map(|point| point.to_vec())).collect::<Result<Vec<_>, _>>()?;
            let selection_count = data.read_bl()? as usize;
            if selection_count > 10_000 {
                return Err(format!("BLOCKSTRETCHACTION {handle:#x} selection count is invalid"));
            }
            let selection_indices = (0..selection_count)
                .map(|_| {
                    let count = data.read_bs()? as usize;
                    (0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, String>>()?;
            let selector_count = data.read_bl()? as usize;
            if selector_count > 10_000 {
                return Err(format!("BLOCKSTRETCHACTION {handle:#x} selector count is invalid"));
            }
            let selectors = (0..selector_count)
                .map(|_| {
                    let node_id = data.read_bl()?;
                    let count = data.read_bs()? as usize;
                    let point_indices = (0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
                    Ok(DwgStretchSelector { node_id, point_indices })
                })
                .collect::<Result<Vec<_>, String>>()?;
            let distance_multiplier = data.read_bd()?;
            let angle_offset = data.read_bd()?;
            if data.read_rc()? != 0 {
                return Err(format!("BLOCKSTRETCHACTION {handle:#x} coordinate mode is unsupported"));
            }
            let selections = selection_indices
                .into_iter()
                .map(|vertex_indices| read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("BLOCKSTRETCHACTION {handle:#x} selection is null")).map(|object_handle| DwgStretchSelection { object_handle, vertex_indices }))
                .collect::<Result<Vec<_>, String>>()?;
            DwgLogicalObjectBody::BlockStretchAction(DwgBlockStretchAction {
                action,
                x_connection: DwgBlockActionConnection { node_id: x_node_id, name: x_name },
                y_connection: DwgBlockActionConnection { node_id: y_node_id, name: y_name },
                points,
                selections,
                selectors,
                distance_multiplier,
                angle_offset,
                coordinate_mode: DwgBlockActionCoordinateMode::CartesianXy,
            })
        }
        536 => {
            let action = decode_r2010_block_action(data, strings, handle_reader, handle, "BLOCKSCALEACTION")?;
            let offset = data.read_3bd()?.to_vec();
            let x_base_connection = DwgBlockActionConnection { node_id: data.read_bl()?, name: strings.read_tu()? };
            let y_base_connection = DwgBlockActionConnection { node_id: data.read_bl()?, name: strings.read_tu()? };
            let dependent = data.read_b()?;
            let base_point = data.read_3bd()?.to_vec();
            let uniform_scale_connection = DwgBlockActionConnection { node_id: data.read_bl()?, name: strings.read_tu()? };
            let x_scale_connection = DwgBlockActionConnection { node_id: data.read_bl()?, name: strings.read_tu()? };
            let y_scale_connection = DwgBlockActionConnection { node_id: data.read_bl()?, name: strings.read_tu()? };
            if data.read_rc()? != 0 {
                return Err(format!("BLOCKSCALEACTION {handle:#x} scale mode is unsupported"));
            }
            DwgLogicalObjectBody::BlockScaleAction(DwgBlockScaleAction {
                base: DwgBlockActionWithBasePoint { action, offset, x_base_connection, y_base_connection, dependent, base_point },
                uniform_scale_connection,
                x_scale_connection,
                y_scale_connection,
                mode: DwgBlockScaleMode::Xy,
            })
        }
        537 => {
            let action = decode_r2010_block_action(data, strings, handle_reader, handle, "BLOCKFLIPACTION")?;
            let mut read_connection = || -> Result<DwgBlockActionConnection, String> { Ok(DwgBlockActionConnection { node_id: data.read_bl()?, name: strings.read_tu()? }) };
            DwgLogicalObjectBody::BlockFlipAction(DwgBlockFlipAction {
                action,
                flip_connection: read_connection()?,
                updated_flip_connection: read_connection()?,
                updated_base_connection: read_connection()?,
                updated_end_connection: read_connection()?,
            })
        }
        _ => return Err(format!("{} {handle:#x} type code is invalid", object.class_name)),
    });
    if data.bit_position() != main_end_bit || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
        return Err(format!("{} {handle:#x} stream boundary is invalid", object.class_name));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, &object.class_name)?;

    Ok(())
}
/// 🧩️ Decodes one block constraint parameter object body from its main, string and handle streams.
pub fn decode_block_constraint_parameter_object(handle: u64, type_code: u16, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockBasePointParameter;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockLinearConstraintParameter;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockOnePointParameter;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterAllowedValues;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterConnection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterProperty;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let strings = strings.as_mut().ok_or_else(|| format!("{} {handle:#x} string stream missing", object.class_name))?;
    object.body = Some(if type_code == 538 {
        let element = decode_r2010_block_element(data, strings, "BLOCKBASEPOINTPARAMETER")?;
        let show_properties = data.read_b()?;
        let chain_actions = data.read_b()?;
        let definition_point = data.read_3bd()?.to_vec();
        let mut properties = Vec::with_capacity(2);
        for _ in 0..2 {
            let count = data.read_bl()? as usize;
            let codes = (0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
            properties.push(DwgBlockParameterProperty { connections: codes.into_iter().map(|code| Ok(DwgBlockParameterConnection { code, name: strings.read_tu()? })).collect::<Result<Vec<_>, String>>()? });
        }
        if data.read_bl()? != 0 {
            return Err(format!("BLOCKBASEPOINTPARAMETER {handle:#x} property-info count is invalid"));
        }
        let point = data.read_3bd()?.to_vec();
        let base_point = data.read_3bd()?.to_vec();
        DwgLogicalObjectBody::BlockBasePointParameter(DwgBlockBasePointParameter { parameter: DwgBlockOnePointParameter { element, show_properties, chain_actions, definition_point, properties }, point, base_point })
    } else {
        let (parameter, property_node_ids) = decode_r2010_two_point_parameter(data, strings, &object.class_name)?;
        if property_node_ids[1] == 0 || property_node_ids[0] != 0 || property_node_ids[2] != 0 || property_node_ids[3] != 0 {
            return Err(format!("{} {handle:#x} property state is invalid", object.class_name));
        }
        let dependency_handle = read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("{} {handle:#x} dependency is null", object.class_name))?;
        let expression_name = strings.read_tu()?;
        let expression_description = strings.read_tu()?;
        let value = data.read_bd()?;
        let flags = data.read_bl()?;
        let minimum = data.read_bd()?;
        let maximum = data.read_bd()?;
        let increment = data.read_bd()?;
        let count = data.read_bs()? as usize;
        let values = (0..count).map(|_| data.read_bd()).collect::<Result<Vec<_>, _>>()?;
        let delta = values.windows(2).next().map(|pair| pair[1] - pair[0]);
        let derived = delta.filter(|step| *step != 0.0 && values.windows(2).all(|pair| pair[1] - pair[0] == *step));
        let expected = derived.map_or((0.0, 0.0, 0.0), |step| (values[0], *values.last().unwrap(), step));
        if flags != 8 || values.is_empty() || (minimum, maximum, increment) != expected {
            return Err(format!("{} {handle:#x} allowed-value helpers are invalid", object.class_name));
        }
        let body = DwgBlockLinearConstraintParameter { parameter, displacement_grip_node_id: property_node_ids[1], dependency_handle, expression_name, expression_description, value, allowed_values: DwgBlockParameterAllowedValues { values } };
        if type_code == 546 {
            DwgLogicalObjectBody::BlockVerticalConstraintParameter(body)
        } else {
            DwgLogicalObjectBody::BlockHorizontalConstraintParameter(body)
        }
    });
    if data.bit_position() != main_end_bit || strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
        return Err(format!("{} {handle:#x} stream boundary is invalid", object.class_name));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, &object.class_name)?;

    Ok(())
}
/// 🧩️ Decodes one block move action object body from its main, string and handle streams.
pub fn decode_block_move_action_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockAction;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionConnection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionDependency;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockMoveAction;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockMoveCoordinateMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpression;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let parent_id = data.read_bl()? as i32;
    let major_version = data.read_bl()?;
    let minor_version = data.read_bl()?;
    let value_code = data.read_bs()? as i16;
    if value_code != -9999 {
        return Err(format!("BLOCKMOVEACTION {handle:#x} evaluation value {value_code} is unsupported"));
    }
    let node_id = data.read_bl()?;
    let repeated_major = data.read_bl()?;
    let repeated_minor = data.read_bl()?;
    let application_marker = data.read_bl()?;
    if (repeated_major, repeated_minor, application_marker) != (major_version, minor_version, 0) {
        return Err(format!("BLOCKMOVEACTION {handle:#x} repeated block-element metadata is inconsistent"));
    }
    let display_location = data.read_3bd()?.to_vec();
    let dependency_count = data.read_bl()? as usize;
    let action_node_count = data.read_bl()? as usize;
    let action_node_ids = (0..action_node_count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?;
    let x_code = data.read_bl()?;
    let y_code = data.read_bl()?;
    let distance_multiplier = data.read_bd()?;
    let angle_offset = data.read_bd()?;
    let coordinate_mode = match data.read_rc()? {
        0 => DwgBlockMoveCoordinateMode::CartesianXy,
        value => return Err(format!("BLOCKMOVEACTION {handle:#x} coordinate mode {value} is unsupported")),
    };
    if dependency_count != 2 || action_node_count != 1 || data.bit_position() != main_end_bit {
        return Err(format!("BLOCKMOVEACTION {handle:#x} semantic counts or main boundary are invalid"));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("BLOCKMOVEACTION {handle:#x} string stream missing"))?;
    let name = strings.read_tu()?;
    let x_name = strings.read_tu()?;
    let y_name = strings.read_tu()?;
    if strings.bit_position() != r2010_string_content_end_bit(payload, data_end_bit)? {
        return Err(format!("BLOCKMOVEACTION {handle:#x} string stream is not exactly consumed"));
    }
    let dependencies = (0..dependency_count)
        .map(|_| read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("BLOCKMOVEACTION {handle:#x} dependency is null")).map(|object_handle| DwgBlockActionDependency { object_handle }))
        .collect::<Result<Vec<_>, String>>()?;
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "BLOCKMOVEACTION")?;
    object.body = Some(DwgLogicalObjectBody::BlockMoveAction(DwgBlockMoveAction {
        action: DwgBlockAction { evaluation_expression: DwgEvaluationExpression { parent_id, major_version, minor_version, value: DwgEvaluationExpressionValue::Empty, node_id }, name, display_location, dependencies, action_node_ids },
        x_connection: DwgBlockActionConnection { node_id: x_code, name: x_name },
        y_connection: DwgBlockActionConnection { node_id: y_code, name: y_name },
        distance_multiplier,
        angle_offset,
        coordinate_mode,
    }));

    Ok(())
}
/// 🧩️ Decodes one block visibility parameter object body from its main, string and handle streams.
pub fn decode_block_visibility_parameter_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterConnection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterProperty;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockVisibilityParameter;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpression;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgVisibilityEvaluationHistory;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgVisibilityState;
    let parent_id = data.read_bl()? as i32;
    let major_version = data.read_bl()?;
    let minor_version = data.read_bl()?;
    let value_code = data.read_bs()? as i16;
    let data_value = match value_code {
        -9999 => Some(DwgEvaluationExpressionValue::Empty),
        40 => Some(DwgEvaluationExpressionValue::Double(data.read_bd()?)),
        10 => Some(DwgEvaluationExpressionValue::PointGroup10(data.read_2rd()?.to_vec())),
        11 => Some(DwgEvaluationExpressionValue::PointGroup11(data.read_2rd()?.to_vec())),
        1 | 91 => None,
        90 => Some(DwgEvaluationExpressionValue::Integer32(data.read_bl()? as i32)),
        70 => Some(DwgEvaluationExpressionValue::Integer16(data.read_bs()? as i16)),
        _ => return Err(format!("BLOCKVISIBILITYPARAMETER {handle:#x} evaluation discriminator {value_code} is unsupported")),
    };
    let node_id = data.read_bl()?;
    if data.read_bl()? != major_version || data.read_bl()? != minor_version || data.read_bl()? != 0 {
        return Err(format!("BLOCKVISIBILITYPARAMETER {handle:#x} block-element version is invalid"));
    }
    let show_properties = data.read_b()?;
    let chain_actions = data.read_b()?;
    let definition_point = data.read_3bd()?.to_vec();
    let mut connection_codes = Vec::with_capacity(2);
    for _ in 0..2 {
        let count = data.read_bl()? as usize;
        connection_codes.push((0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?);
    }
    let updated_visibility_node_id = data.read_bl()?;
    let initialized = data.read_b()?;
    let evaluation_history = if data.read_b()? { DwgVisibilityEvaluationHistory::Required } else { DwgVisibilityEvaluationHistory::Stateless };
    let eligible_count = data.read_bl()? as usize;
    let state_count = data.read_bl()? as usize;
    let state_counts = (0..state_count).map(|_| Ok((data.read_bl()? as usize, data.read_bl()? as usize))).collect::<Result<Vec<_>, String>>()?;
    if data.bit_position() != main_end_bit {
        return Err(format!("BLOCKVISIBILITYPARAMETER {handle:#x} main stream is not exactly consumed"));
    }
    if object.owner_handle.is_none() || !object.reactor_handles.is_empty() || object.extension_dictionary_handle.is_some() {
        return Err(format!("BLOCKVISIBILITYPARAMETER {handle:#x} common state is invalid"));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("BLOCKVISIBILITYPARAMETER {handle:#x} string stream missing"))?;
    let string_value = if value_code == 1 { Some(strings.read_tu()?) } else { None };
    let element_name = strings.read_tu()?;
    let mut properties = Vec::with_capacity(2);
    for codes in connection_codes {
        let connections = codes.into_iter().map(|code| Ok(DwgBlockParameterConnection { code, name: strings.read_tu()? })).collect::<Result<Vec<_>, String>>()?;
        properties.push(DwgBlockParameterProperty { connections });
    }
    let name = strings.read_tu()?;
    let description = strings.read_tu()?;
    let state_names = (0..state_count).map(|_| strings.read_tu()).collect::<Result<Vec<_>, _>>()?;
    let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
    if strings.bit_position() != string_end_bit {
        return Err(format!("BLOCKVISIBILITYPARAMETER {handle:#x} string stream is not exactly consumed"));
    }
    let value = match value_code {
        1 => DwgEvaluationExpressionValue::String(string_value.unwrap()),
        91 => DwgEvaluationExpressionValue::ObjectReference(read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("BLOCKVISIBILITYPARAMETER {handle:#x} evaluation object reference is null"))?),
        _ => data_value.unwrap(),
    };
    let eligible_entity_handles = (0..eligible_count).map(|_| read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("BLOCKVISIBILITYPARAMETER {handle:#x} eligible entity is null"))).collect::<Result<Vec<_>, String>>()?;
    let mut states = Vec::with_capacity(state_count);
    for ((visible_count, controlled_count), name) in state_counts.into_iter().zip(state_names) {
        let visible_entity_handles = (0..visible_count).map(|_| read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("BLOCKVISIBILITYPARAMETER {handle:#x} visible entity is null"))).collect::<Result<Vec<_>, String>>()?;
        let controlled_expression_handles =
            (0..controlled_count).map(|_| read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("BLOCKVISIBILITYPARAMETER {handle:#x} controlled expression is null"))).collect::<Result<Vec<_>, String>>()?;
        states.push(DwgVisibilityState { name, visible_entity_handles, controlled_expression_handles });
    }
    if states.iter().any(|state| state.visible_entity_handles.iter().any(|member| !eligible_entity_handles.contains(member))) {
        return Err(format!("BLOCKVISIBILITYPARAMETER {handle:#x} state contains an ineligible entity"));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "BLOCKVISIBILITYPARAMETER")?;
    object.body = Some(DwgLogicalObjectBody::BlockVisibilityParameter(DwgBlockVisibilityParameter {
        evaluation_expression: DwgEvaluationExpression { parent_id, major_version, minor_version, value, node_id },
        element_name,
        show_properties,
        chain_actions,
        definition_point,
        properties,
        updated_visibility_node_id,
        initialized,
        name,
        description,
        evaluation_history,
        eligible_entity_handles,
        states,
    }));

    Ok(())
}
/// 🧩️ Decodes one block flip parameter object body from its main, string and handle streams.
pub fn decode_block_flip_parameter_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockFlipParameter;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockFlipValueSet;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterBaseLocation;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterConnection;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterProperty;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpression;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgNamedEvaluationNodeReference;
    let parent_id = data.read_bl()? as i32;
    let major_version = data.read_bl()?;
    let minor_version = data.read_bl()?;
    let value_code = data.read_bs()? as i16;
    let data_value = match value_code {
        -9999 => Some(DwgEvaluationExpressionValue::Empty),
        40 => Some(DwgEvaluationExpressionValue::Double(data.read_bd()?)),
        10 => Some(DwgEvaluationExpressionValue::PointGroup10(data.read_2rd()?.to_vec())),
        11 => Some(DwgEvaluationExpressionValue::PointGroup11(data.read_2rd()?.to_vec())),
        1 | 91 => None,
        90 => Some(DwgEvaluationExpressionValue::Integer32(data.read_bl()? as i32)),
        70 => Some(DwgEvaluationExpressionValue::Integer16(data.read_bs()? as i16)),
        _ => return Err(format!("BLOCKFLIPPARAMETER {handle:#x} evaluation discriminator {value_code} is unsupported")),
    };
    let node_id = data.read_bl()?;
    if data.read_bl()? != major_version || data.read_bl()? != minor_version || data.read_bl()? != 0 {
        return Err(format!("BLOCKFLIPPARAMETER {handle:#x} block-element version is invalid"));
    }
    let show_properties = data.read_b()?;
    let chain_actions = data.read_b()?;
    let definition_base = data.read_3bd()?.to_vec();
    let definition_end = data.read_3bd()?.to_vec();
    let mut connection_codes = Vec::with_capacity(4);
    for _ in 0..4 {
        let count = data.read_bl()? as usize;
        connection_codes.push((0..count).map(|_| data.read_bl()).collect::<Result<Vec<_>, _>>()?);
    }
    let property_states = [data.read_bl()?, data.read_bl()?, data.read_bl()?, data.read_bl()?];
    let base_location = match data.read_bs()? {
        0 => DwgBlockParameterBaseLocation::StartPoint,
        1 => DwgBlockParameterBaseLocation::Midpoint,
        value => return Err(format!("BLOCKFLIPPARAMETER {handle:#x} base location {value} is unsupported")),
    };
    let label_point = data.read_3bd()?.to_vec();
    let updated_node_id = data.read_bl()?;
    if property_states != [updated_node_id, 0, 0, 0] || data.bit_position() != main_end_bit {
        return Err(format!("BLOCKFLIPPARAMETER {handle:#x} duplicated update state or main boundary is invalid"));
    }
    if object.owner_handle.is_none() || !object.reactor_handles.is_empty() || object.extension_dictionary_handle.is_some() {
        return Err(format!("BLOCKFLIPPARAMETER {handle:#x} common state is invalid"));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("BLOCKFLIPPARAMETER {handle:#x} string stream missing"))?;
    let string_value = if value_code == 1 { Some(strings.read_tu()?) } else { None };
    let name = strings.read_tu()?;
    let mut properties = Vec::with_capacity(4);
    for codes in connection_codes {
        let connections = codes.into_iter().map(|code| Ok(DwgBlockParameterConnection { code, name: strings.read_tu()? })).collect::<Result<Vec<_>, String>>()?;
        properties.push(DwgBlockParameterProperty { connections });
    }
    let label = strings.read_tu()?;
    let description = strings.read_tu()?;
    let base_label = strings.read_tu()?;
    let flipped_label = strings.read_tu()?;
    let expression_name = strings.read_tu()?;
    let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
    if strings.bit_position() != string_end_bit {
        return Err(format!("BLOCKFLIPPARAMETER {handle:#x} string stream is not exactly consumed"));
    }
    let value = match value_code {
        1 => DwgEvaluationExpressionValue::String(string_value.unwrap()),
        91 => DwgEvaluationExpressionValue::ObjectReference(read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("BLOCKFLIPPARAMETER {handle:#x} evaluation object reference is null"))?),
        _ => data_value.unwrap(),
    };
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "BLOCKFLIPPARAMETER")?;
    object.body = Some(DwgLogicalObjectBody::BlockFlipParameter(DwgBlockFlipParameter {
        evaluation_expression: DwgEvaluationExpression { parent_id, major_version, minor_version, value, node_id },
        name,
        show_properties,
        chain_actions,
        definition_base,
        definition_end,
        properties,
        base_location,
        label,
        description,
        value_set: DwgBlockFlipValueSet { base_label, flipped_label },
        label_point,
        updated_flip: DwgNamedEvaluationNodeReference { node_id: updated_node_id, expression_name },
    }));

    Ok(())
}
/// 🧩️ Decodes one dynamic block purge preventer object body from its main, string and handle streams.
pub fn decode_dynamic_block_purge_preventer_object(handle: u64, payload_size: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    let marker = data.read_bs()?;
    if marker != 1 {
        return Err(format!("ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION {handle:#x} marker {marker} is unsupported"));
    }
    if data.bit_position() != main_end_bit {
        return Err(format!("ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    if !object.extended_data.is_empty() || object.extension_dictionary_handle.is_some() || object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] {
        return Err(format!("ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION {handle:#x} common state is invalid"));
    }
    let protected_block_header_handle = read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION {handle:#x} protected block header is null"))?;
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION")?;
    object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::DynamicBlockPurgePreventer(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgDynamicBlockPurgePreventer { protected_block_header_handle }));

    Ok(())
}
/// 🧩️ Decodes one block representation data object body from its main, string and handle streams.
pub fn decode_block_representation_data_object(handle: u64, payload_size: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    let marker = data.read_bs()?;
    if marker != 1 {
        return Err(format!("ACDB_BLOCKREPRESENTATION_DATA {handle:#x} marker {marker} is unsupported"));
    }
    if data.bit_position() != main_end_bit {
        return Err(format!("ACDB_BLOCKREPRESENTATION_DATA {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    if !object.extended_data.is_empty() || object.extension_dictionary_handle.is_some() || object.owner_handle != Some(handle - 1) || object.reactor_handles.as_slice() != [handle - 1] {
        return Err(format!("ACDB_BLOCKREPRESENTATION_DATA {handle:#x} common state is invalid"));
    }
    let represented_block_header_handle = read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACDB_BLOCKREPRESENTATION_DATA {handle:#x} represented block header is null"))?;
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "ACDB_BLOCKREPRESENTATION_DATA")?;
    object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::BlockRepresentationData(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockRepresentationData { represented_block_header_handle }));

    Ok(())
}
/// 🧩️ Decodes one block parameter dependency body object body from its main, string and handle streams.
pub fn decode_block_parameter_dependency_body_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    let dependency_version = data.read_bs()?;
    let dimension_base_version = data.read_bs()?;
    let class_version = data.read_bs()?;
    if (dependency_version, dimension_base_version, class_version) != (1, 1, 0) {
        return Err(format!("BLOCKPARAMDEPENDENCYBODY {handle:#x} versions {dependency_version}/{dimension_base_version}/{class_version} are unsupported"));
    }
    if data.bit_position() != main_end_bit {
        return Err(format!("BLOCKPARAMDEPENDENCYBODY {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("BLOCKPARAMDEPENDENCYBODY {handle:#x} string stream missing"))?;
    let name = strings.read_tu()?;
    let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
    if strings.bit_position() != string_end_bit {
        return Err(format!("BLOCKPARAMDEPENDENCYBODY {handle:#x} string stream is not exactly consumed"));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "BLOCKPARAMDEPENDENCYBODY")?;
    object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::BlockParameterDependencyBody(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterDependencyBody { name }));

    Ok(())
}
/// 🧩️ Decodes one dynamic block proxy node object body from its main, string and handle streams.
pub fn decode_dynamic_block_proxy_node_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgDynamicBlockProxyNode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpression;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let parent_id = data.read_bl()? as i32;
    let major_version = data.read_bl()?;
    let minor_version = data.read_bl()?;
    let value_code = data.read_bs()? as i16;
    let data_value = match value_code {
        -9999 => Some(DwgEvaluationExpressionValue::Empty),
        40 => Some(DwgEvaluationExpressionValue::Double(data.read_bd()?)),
        10 => Some(DwgEvaluationExpressionValue::PointGroup10(data.read_2rd()?.to_vec())),
        11 => Some(DwgEvaluationExpressionValue::PointGroup11(data.read_2rd()?.to_vec())),
        1 | 91 => None,
        90 => Some(DwgEvaluationExpressionValue::Integer32(data.read_bl()? as i32)),
        70 => Some(DwgEvaluationExpressionValue::Integer16(data.read_bs()? as i16)),
        _ => return Err(format!("ACDB_DYNAMICBLOCKPROXYNODE {handle:#x} value discriminator {value_code} is unsupported")),
    };
    let node_id = data.read_bl()?;
    if data.bit_position() != main_end_bit {
        return Err(format!("ACDB_DYNAMICBLOCKPROXYNODE {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    let value = match value_code {
        1 => {
            let strings = strings.as_mut().ok_or_else(|| format!("ACDB_DYNAMICBLOCKPROXYNODE {handle:#x} string stream missing"))?;
            let value = DwgEvaluationExpressionValue::String(strings.read_tu()?);
            let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
            if strings.bit_position() != string_end_bit {
                return Err(format!("ACDB_DYNAMICBLOCKPROXYNODE {handle:#x} string stream is not exactly consumed"));
            }
            value
        }
        91 => DwgEvaluationExpressionValue::ObjectReference(read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACDB_DYNAMICBLOCKPROXYNODE {handle:#x} object reference is null"))?),
        _ => data_value.unwrap(),
    };
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "ACDB_DYNAMICBLOCKPROXYNODE")?;
    object.body = Some(DwgLogicalObjectBody::DynamicBlockProxyNode(DwgDynamicBlockProxyNode { evaluation_expression: DwgEvaluationExpression { parent_id, major_version, minor_version, value, node_id } }));

    Ok(())
}
/// 🧩️ Decodes one block grip location component object body from its main, string and handle streams.
pub fn decode_block_grip_location_component_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgBlockGripLocationComponent;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpression;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let parent_id = data.read_bl()? as i32;
    let major_version = data.read_bl()?;
    let minor_version = data.read_bl()?;
    let value_code = data.read_bs()? as i16;
    let data_value = match value_code {
        -9999 => Some(DwgEvaluationExpressionValue::Empty),
        40 => Some(DwgEvaluationExpressionValue::Double(data.read_bd()?)),
        10 => Some(DwgEvaluationExpressionValue::PointGroup10(data.read_2rd()?.to_vec())),
        11 => Some(DwgEvaluationExpressionValue::PointGroup11(data.read_2rd()?.to_vec())),
        1 | 91 => None,
        90 => Some(DwgEvaluationExpressionValue::Integer32(data.read_bl()? as i32)),
        70 => Some(DwgEvaluationExpressionValue::Integer16(data.read_bs()? as i16)),
        _ => return Err(format!("BLOCKGRIPLOCATIONCOMPONENT {handle:#x} value discriminator {value_code} is unsupported")),
    };
    let node_id = data.read_bl()?;
    let grip_type = data.read_bl()?;
    if data.bit_position() != main_end_bit {
        return Err(format!("BLOCKGRIPLOCATIONCOMPONENT {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("BLOCKGRIPLOCATIONCOMPONENT {handle:#x} string stream missing"))?;
    let string_value = if value_code == 1 { Some(strings.read_tu()?) } else { None };
    let grip_expression = strings.read_tu()?;
    let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
    if strings.bit_position() != string_end_bit {
        return Err(format!("BLOCKGRIPLOCATIONCOMPONENT {handle:#x} string stream is not exactly consumed: {} != {string_end_bit}", strings.bit_position()));
    }
    let value = match value_code {
        1 => DwgEvaluationExpressionValue::String(string_value.unwrap()),
        91 => DwgEvaluationExpressionValue::ObjectReference(read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("BLOCKGRIPLOCATIONCOMPONENT {handle:#x} object reference is null"))?),
        _ => data_value.unwrap(),
    };
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "BLOCKGRIPLOCATIONCOMPONENT")?;
    object.body =
        Some(DwgLogicalObjectBody::BlockGripLocationComponent(DwgBlockGripLocationComponent { evaluation_expression: DwgEvaluationExpression { parent_id, major_version, minor_version, value, node_id }, grip_type, grip_expression }));

    Ok(())
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
