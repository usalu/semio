//! 🧩️ Dynamic-block object writers: parameters, grips, actions, proxies and representation data.

use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::bits::DwgBitWriter;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::append_r2010_string_stream;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_r2010_block_action;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_r2010_block_element;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_r2010_block_grip;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_r2010_eed;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_r2010_two_point_parameter;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::finish_r2010_object_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_object_handle;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_r2010_action_connection;

//#region 🔖️DwgStructuralCodec
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_block_grip_location_component_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::BlockGripLocationComponent(grip)) = object.body.as_ref() else {
        return Err(format!("BLOCKGRIPLOCATIONCOMPONENT {:#x} body missing", object.handle));
    };
    let expression = &grip.evaluation_expression;
    if matches!(&expression.value, DwgEvaluationExpressionValue::PointGroup10(point) | DwgEvaluationExpressionValue::PointGroup11(point) if point.len() != 2) {
        return Err(format!("BLOCKGRIPLOCATIONCOMPONENT {:#x} point value must contain two coordinates", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bl(expression.parent_id as u32);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    match &expression.value {
        DwgEvaluationExpressionValue::Empty => data.write_bs((-9999i16) as u16),
        DwgEvaluationExpressionValue::Double(value) => {
            data.write_bs(40);
            data.write_bd(*value);
        }
        DwgEvaluationExpressionValue::PointGroup10(point) => {
            data.write_bs(10);
            data.write_2rd([point[0], point[1]]);
        }
        DwgEvaluationExpressionValue::PointGroup11(point) => {
            data.write_bs(11);
            data.write_2rd([point[0], point[1]]);
        }
        DwgEvaluationExpressionValue::String(_) => data.write_bs(1),
        DwgEvaluationExpressionValue::Integer32(value) => {
            data.write_bs(90);
            data.write_bl(*value as u32);
        }
        DwgEvaluationExpressionValue::ObjectReference(_) => data.write_bs(91),
        DwgEvaluationExpressionValue::Integer16(value) => {
            data.write_bs(70);
            data.write_bs(*value as u16);
        }
    }
    data.write_bl(expression.node_id);
    data.write_bl(grip.grip_type);
    let mut strings = DwgBitWriter::new();
    if let DwgEvaluationExpressionValue::String(value) = &expression.value {
        strings.write_tu(value);
    }
    strings.write_tu(&grip.grip_expression);
    append_r2010_string_stream(&mut data, &strings, "BLOCKGRIPLOCATIONCOMPONENT", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    if let DwgEvaluationExpressionValue::ObjectReference(reference) = &expression.value {
        handles.write_handle(5, *reference);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_dynamic_block_proxy_node_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::DynamicBlockProxyNode(proxy)) = object.body.as_ref() else {
        return Err(format!("ACDB_DYNAMICBLOCKPROXYNODE {:#x} body missing", object.handle));
    };
    let expression = &proxy.evaluation_expression;
    if matches!(&expression.value, DwgEvaluationExpressionValue::PointGroup10(point) | DwgEvaluationExpressionValue::PointGroup11(point) if point.len() != 2 || point.iter().any(|value| !value.is_finite())) {
        return Err(format!("ACDB_DYNAMICBLOCKPROXYNODE {:#x} point value must contain two finite coordinates", object.handle));
    }
    if matches!(&expression.value, DwgEvaluationExpressionValue::ObjectReference(0)) {
        return Err(format!("ACDB_DYNAMICBLOCKPROXYNODE {:#x} object reference is null", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bl(expression.parent_id as u32);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    match &expression.value {
        DwgEvaluationExpressionValue::Empty => data.write_bs((-9999i16) as u16),
        DwgEvaluationExpressionValue::Double(value) => {
            data.write_bs(40);
            data.write_bd(*value);
        }
        DwgEvaluationExpressionValue::PointGroup10(point) => {
            data.write_bs(10);
            data.write_2rd([point[0], point[1]]);
        }
        DwgEvaluationExpressionValue::PointGroup11(point) => {
            data.write_bs(11);
            data.write_2rd([point[0], point[1]]);
        }
        DwgEvaluationExpressionValue::String(_) => data.write_bs(1),
        DwgEvaluationExpressionValue::Integer32(value) => {
            data.write_bs(90);
            data.write_bl(*value as u32);
        }
        DwgEvaluationExpressionValue::ObjectReference(_) => data.write_bs(91),
        DwgEvaluationExpressionValue::Integer16(value) => {
            data.write_bs(70);
            data.write_bs(*value as u16);
        }
    }
    data.write_bl(expression.node_id);
    let mut strings = DwgBitWriter::new();
    if let DwgEvaluationExpressionValue::String(value) = &expression.value {
        strings.write_tu(value);
    }
    append_r2010_string_stream(&mut data, &strings, "ACDB_DYNAMICBLOCKPROXYNODE", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    if let DwgEvaluationExpressionValue::ObjectReference(reference) = &expression.value {
        handles.write_handle(5, *reference);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_block_parameter_dependency_body_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::BlockParameterDependencyBody(body)) = object.body.as_ref() else {
        return Err(format!("BLOCKPARAMDEPENDENCYBODY {:#x} body missing", object.handle));
    };
    if !object.extended_data.is_empty() || !object.reactor_handles.is_empty() || object.extension_dictionary_handle.is_some() {
        return Err(format!("BLOCKPARAMDEPENDENCYBODY {:#x} unsupported common state", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    data.write_bs(1);
    data.write_bs(1);
    data.write_bs(0);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.name);
    append_r2010_string_stream(&mut data, &strings, "BLOCKPARAMDEPENDENCYBODY", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_block_representation_data_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::BlockRepresentationData(body)) = object.body.as_ref() else {
        return Err(format!("ACDB_BLOCKREPRESENTATION_DATA {:#x} body missing", object.handle));
    };
    if !object.extended_data.is_empty() || object.extension_dictionary_handle.is_some() || object.owner_handle != Some(object.handle - 1) || object.reactor_handles.as_slice() != [object.handle - 1] || body.represented_block_header_handle == 0 {
        return Err(format!("ACDB_BLOCKREPRESENTATION_DATA {:#x} common or graph state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bs(1);
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    handles.write_handle(8, 0);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(5, body.represented_block_header_handle);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_dynamic_block_purge_preventer_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::DynamicBlockPurgePreventer(body)) = object.body.as_ref() else {
        return Err(format!("ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION {:#x} body missing", object.handle));
    };
    if !object.extended_data.is_empty() || object.extension_dictionary_handle.is_some() || object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] || body.protected_block_header_handle == 0 {
        return Err(format!("ACDB_DYNAMICBLOCKPURGEPREVENTER_VERSION {:#x} common or graph state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bs(1);
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(5, body.protected_block_header_handle);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_block_flip_parameter_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterBaseLocation;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::BlockFlipParameter(body)) = object.body.as_ref() else {
        return Err(format!("BLOCKFLIPPARAMETER {:#x} body missing", object.handle));
    };
    if body.properties.len() != 4
        || body.definition_base.len() != 3
        || body.definition_end.len() != 3
        || body.label_point.len() != 3
        || object.owner_handle.is_none()
        || !object.reactor_handles.is_empty()
        || object.extension_dictionary_handle.is_some()
    {
        return Err(format!("BLOCKFLIPPARAMETER {:#x} logical state is invalid", object.handle));
    }
    let expression = &body.evaluation_expression;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    data.write_bl(expression.parent_id as u32);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    match &expression.value {
        DwgEvaluationExpressionValue::Empty => data.write_bs((-9999i16) as u16),
        DwgEvaluationExpressionValue::Double(value) => {
            data.write_bs(40);
            data.write_bd(*value);
        }
        DwgEvaluationExpressionValue::PointGroup10(point) | DwgEvaluationExpressionValue::PointGroup11(point) if point.len() == 2 => {
            data.write_bs(if matches!(expression.value, DwgEvaluationExpressionValue::PointGroup10(_)) { 10 } else { 11 });
            data.write_2rd([point[0], point[1]]);
        }
        DwgEvaluationExpressionValue::String(_) => data.write_bs(1),
        DwgEvaluationExpressionValue::Integer32(value) => {
            data.write_bs(90);
            data.write_bl(*value as u32);
        }
        DwgEvaluationExpressionValue::ObjectReference(reference) if *reference != 0 => data.write_bs(91),
        DwgEvaluationExpressionValue::Integer16(value) => {
            data.write_bs(70);
            data.write_bs(*value as u16);
        }
        _ => return Err(format!("BLOCKFLIPPARAMETER {:#x} evaluation value is invalid", object.handle)),
    }
    data.write_bl(expression.node_id);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    data.write_bl(0);
    data.write_b(body.show_properties);
    data.write_b(body.chain_actions);
    data.write_3bd([body.definition_base[0], body.definition_base[1], body.definition_base[2]]);
    data.write_3bd([body.definition_end[0], body.definition_end[1], body.definition_end[2]]);
    for property in &body.properties {
        data.write_bl(property.connections.len() as u32);
        for connection in &property.connections {
            data.write_bl(connection.code);
        }
    }
    data.write_bl(body.updated_flip.node_id);
    data.write_bl(0);
    data.write_bl(0);
    data.write_bl(0);
    data.write_bs(match body.base_location {
        DwgBlockParameterBaseLocation::StartPoint => 0,
        DwgBlockParameterBaseLocation::Midpoint => 1,
    });
    data.write_3bd([body.label_point[0], body.label_point[1], body.label_point[2]]);
    data.write_bl(body.updated_flip.node_id);
    let mut strings = DwgBitWriter::new();
    if let DwgEvaluationExpressionValue::String(value) = &expression.value {
        strings.write_tu(value);
    }
    strings.write_tu(&body.name);
    for property in &body.properties {
        for connection in &property.connections {
            strings.write_tu(&connection.name);
        }
    }
    strings.write_tu(&body.label);
    strings.write_tu(&body.description);
    strings.write_tu(&body.value_set.base_label);
    strings.write_tu(&body.value_set.flipped_label);
    strings.write_tu(&body.updated_flip.expression_name);
    append_r2010_string_stream(&mut data, &strings, "BLOCKFLIPPARAMETER", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    if let DwgEvaluationExpressionValue::ObjectReference(reference) = &expression.value {
        handles.write_handle(5, *reference);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_block_visibility_parameter_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgVisibilityEvaluationHistory;
    let Some(DwgLogicalObjectBody::BlockVisibilityParameter(body)) = object.body.as_ref() else {
        return Err(format!("BLOCKVISIBILITYPARAMETER {:#x} body missing", object.handle));
    };
    if body.definition_point.len() != 3
        || body.properties.len() != 2
        || body.eligible_entity_handles.contains(&0)
        || body.states.iter().any(|state| state.name.is_empty() || state.visible_entity_handles.iter().any(|handle| !body.eligible_entity_handles.contains(handle)))
        || object.owner_handle.is_none()
        || !object.reactor_handles.is_empty()
        || object.extension_dictionary_handle.is_some()
    {
        return Err(format!("BLOCKVISIBILITYPARAMETER {:#x} logical state is invalid", object.handle));
    }
    let expression = &body.evaluation_expression;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    data.write_bl(expression.parent_id as u32);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    match &expression.value {
        DwgEvaluationExpressionValue::Empty => data.write_bs((-9999i16) as u16),
        DwgEvaluationExpressionValue::Double(value) => {
            data.write_bs(40);
            data.write_bd(*value);
        }
        DwgEvaluationExpressionValue::PointGroup10(point) | DwgEvaluationExpressionValue::PointGroup11(point) if point.len() == 2 => {
            data.write_bs(if matches!(expression.value, DwgEvaluationExpressionValue::PointGroup10(_)) { 10 } else { 11 });
            data.write_2rd([point[0], point[1]]);
        }
        DwgEvaluationExpressionValue::String(_) => data.write_bs(1),
        DwgEvaluationExpressionValue::Integer32(value) => {
            data.write_bs(90);
            data.write_bl(*value as u32);
        }
        DwgEvaluationExpressionValue::ObjectReference(reference) if *reference != 0 => data.write_bs(91),
        DwgEvaluationExpressionValue::Integer16(value) => {
            data.write_bs(70);
            data.write_bs(*value as u16);
        }
        _ => return Err(format!("BLOCKVISIBILITYPARAMETER {:#x} evaluation value is invalid", object.handle)),
    }
    data.write_bl(expression.node_id);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    data.write_bl(0);
    data.write_b(body.show_properties);
    data.write_b(body.chain_actions);
    data.write_3bd([body.definition_point[0], body.definition_point[1], body.definition_point[2]]);
    for property in &body.properties {
        data.write_bl(property.connections.len() as u32);
        for connection in &property.connections {
            data.write_bl(connection.code);
        }
    }
    data.write_bl(body.updated_visibility_node_id);
    data.write_b(body.initialized);
    data.write_b(matches!(body.evaluation_history, DwgVisibilityEvaluationHistory::Required));
    data.write_bl(body.eligible_entity_handles.len() as u32);
    data.write_bl(body.states.len() as u32);
    for state in &body.states {
        data.write_bl(state.visible_entity_handles.len() as u32);
        data.write_bl(state.controlled_expression_handles.len() as u32);
    }
    let mut strings = DwgBitWriter::new();
    if let DwgEvaluationExpressionValue::String(value) = &expression.value {
        strings.write_tu(value);
    }
    strings.write_tu(&body.element_name);
    for property in &body.properties {
        for connection in &property.connections {
            strings.write_tu(&connection.name);
        }
    }
    strings.write_tu(&body.name);
    strings.write_tu(&body.description);
    for state in &body.states {
        strings.write_tu(&state.name);
    }
    append_r2010_string_stream(&mut data, &strings, "BLOCKVISIBILITYPARAMETER", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    if let DwgEvaluationExpressionValue::ObjectReference(reference) = &expression.value {
        handles.write_handle(5, *reference);
    }
    for handle in &body.eligible_entity_handles {
        handles.write_handle(4, *handle);
    }
    for state in &body.states {
        for handle in &state.visible_entity_handles {
            handles.write_handle(4, *handle);
        }
        for handle in &state.controlled_expression_handles {
            handles.write_handle(4, *handle);
        }
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_block_move_action_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockMoveCoordinateMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationExpressionValue;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::BlockMoveAction(body)) = object.body.as_ref() else { return Err(format!("BLOCKMOVEACTION {:#x} body missing", object.handle)) };
    let action = &body.action;
    let expression = &action.evaluation_expression;
    if !matches!(expression.value, DwgEvaluationExpressionValue::Empty)
        || action.display_location.len() != 3
        || action.display_location.iter().any(|value| !value.is_finite())
        || action.name.is_empty()
        || action.dependencies.len() != 2
        || action.action_node_ids.len() != 1
        || body.x_connection.name.is_empty()
        || body.y_connection.name.is_empty()
        || !body.distance_multiplier.is_finite()
        || body.distance_multiplier <= 0.0
        || !body.angle_offset.is_finite()
        || !matches!(body.coordinate_mode, DwgBlockMoveCoordinateMode::CartesianXy)
        || object.owner_handle.is_none()
        || !object.reactor_handles.is_empty()
        || object.extension_dictionary_handle.is_some()
    {
        return Err(format!("BLOCKMOVEACTION {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    data.write_bl(expression.parent_id as u32);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    data.write_bs((-9999i16) as u16);
    data.write_bl(expression.node_id);
    data.write_bl(expression.major_version);
    data.write_bl(expression.minor_version);
    data.write_bl(0);
    data.write_3bd([action.display_location[0], action.display_location[1], action.display_location[2]]);
    data.write_bl(action.dependencies.len() as u32);
    data.write_bl(action.action_node_ids.len() as u32);
    for node_id in &action.action_node_ids {
        data.write_bl(*node_id);
    }
    data.write_bl(body.x_connection.node_id);
    data.write_bl(body.y_connection.node_id);
    data.write_bd(body.distance_multiplier);
    data.write_bd(body.angle_offset);
    data.write_rc(0);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&action.name);
    strings.write_tu(&body.x_connection.name);
    strings.write_tu(&body.y_connection.name);
    append_r2010_string_stream(&mut data, &strings, "BLOCKMOVEACTION", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for dependency in &action.dependencies {
        handles.write_handle(4, dependency.object_handle);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_dynamic_block_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockParameterBaseLocation;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    if !object.extended_data.is_empty() || object.owner_handle.is_none() || !object.reactor_handles.is_empty() || object.extension_dictionary_handle.is_some() {
        return Err(format!("{} {:#x} common state is invalid", object.class_name, object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    let mut strings = DwgBitWriter::new();
    match object.body.as_ref() {
        Some(DwgLogicalObjectBody::BlockLinearParameter(body)) if object.type_code == 527 => {
            let parameter = &body.parameter;
            if parameter.definition_base.len() != 3
                || parameter.definition_end.len() != 3
                || parameter.properties.len() != 4
                || parameter.property_expression_references.iter().any(|reference| reference.property_index >= 4 || reference.node_id == 0)
                || parameter.property_expression_references.iter().enumerate().any(|(index, reference)| parameter.property_expression_references[..index].iter().any(|candidate| candidate.property_index == reference.property_index))
                || parameter.definition_base.iter().chain(&parameter.definition_end).any(|value| !value.is_finite())
                || body.allowed_values.is_empty()
                || body.allowed_values.len() > u16::MAX as usize
                || body.allowed_values.iter().any(|value| !value.is_finite())
                || !body.label_offset.is_finite()
            {
                return Err(format!("BLOCKLINEARPARAMETER {:#x} logical state is invalid", object.handle));
            }
            encode_r2010_block_element(&mut data, &mut strings, &parameter.element, "BLOCKLINEARPARAMETER")?;
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
            for property_index in 0..4 {
                data.write_bl(parameter.property_expression_references.iter().find(|reference| reference.property_index == property_index).map_or(0, |reference| reference.node_id));
            }
            data.write_bs(match parameter.base_location {
                DwgBlockParameterBaseLocation::StartPoint => 0,
                DwgBlockParameterBaseLocation::Midpoint => 1,
            });
            strings.write_tu(&body.distance_name);
            strings.write_tu(&body.distance_description);
            data.write_bd(body.label_offset);
            data.write_bl(8);
            data.write_bd(0.0);
            data.write_bd(0.0);
            data.write_bd(0.0);
            data.write_bs(body.allowed_values.len() as u16);
            for value in &body.allowed_values {
                data.write_bd(*value);
            }
        }
        Some(DwgLogicalObjectBody::BlockLinearGrip(body)) if object.type_code == 528 => {
            if body.orientation.len() != 3 || body.orientation.iter().any(|value| !value.is_finite()) || body.orientation.iter().all(|value| *value == 0.0) {
                return Err(format!("BLOCKLINEARGRIP {:#x} orientation is invalid", object.handle));
            }
            encode_r2010_block_grip(&mut data, &mut strings, &body.grip, "UpdatedEndX", "UpdatedEndY", "BLOCKLINEARGRIP")?;
            data.write_3bd([body.orientation[0], body.orientation[1], body.orientation[2]]);
        }
        Some(DwgLogicalObjectBody::BlockFlipGrip(body)) if object.type_code == 530 => {
            if body.updated_flip.expression_name != "UpdatedFlip" || body.orientation.len() != 3 || body.orientation.iter().any(|value| !value.is_finite()) || body.orientation.iter().all(|value| *value == 0.0) {
                return Err(format!("BLOCKFLIPGRIP {:#x} state is invalid", object.handle));
            }
            encode_r2010_block_grip(&mut data, &mut strings, &body.grip, "UpdatedBaseX", "UpdatedBaseY", "BLOCKFLIPGRIP")?;
            data.write_bl(body.updated_flip.node_id);
            data.write_3bd([body.orientation[0], body.orientation[1], body.orientation[2]]);
        }
        Some(DwgLogicalObjectBody::BlockVisibilityGrip(body)) if object.type_code == 532 => {
            encode_r2010_block_grip(&mut data, &mut strings, &body.grip, "UpdatedX", "UpdatedY", "BLOCKVISIBILITYGRIP")?;
        }
        _ => return Err(format!("{} {:#x} body missing or mismatched", object.class_name, object.handle)),
    }
    append_r2010_string_stream(&mut data, &strings, &object.class_name, object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_alignment_action_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockActionCoordinateMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::blocks::DwgBlockScaleMode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    if !object.extended_data.is_empty() || object.owner_handle.is_none() || !object.reactor_handles.is_empty() || object.extension_dictionary_handle.is_some() {
        return Err(format!("{} {:#x} common state is invalid", object.class_name, object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    let mut strings = DwgBitWriter::new();
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    match object.body.as_ref() {
        Some(DwgLogicalObjectBody::BlockAlignmentParameter(body)) if object.type_code == 533 => {
            if body.updated_grip_node_id == 0 || !body.parameter.property_expression_references.is_empty() {
                return Err(format!("BLOCKALIGNMENTPARAMETER {:#x} relation state is invalid", object.handle));
            }
            encode_r2010_two_point_parameter(&mut data, &mut strings, &body.parameter, [body.updated_grip_node_id, 0, 0, 0], "BLOCKALIGNMENTPARAMETER")?;
            data.write_b(body.align_perpendicular);
        }
        Some(DwgLogicalObjectBody::BlockAlignmentGrip(body)) if object.type_code == 534 => {
            if body.first_location_node_id == 0
                || body.second_location_node_id == 0
                || body.grip.updated_x.node_id != body.first_location_node_id
                || body.grip.updated_y.node_id != body.second_location_node_id
                || body.orientation.len() != 3
                || body.orientation.iter().any(|value| !value.is_finite())
                || body.orientation.iter().all(|value| *value == 0.0)
            {
                return Err(format!("BLOCKALIGNMENTGRIP {:#x} logical state is invalid", object.handle));
            }
            encode_r2010_block_grip(&mut data, &mut strings, &body.grip, "FirstLocation", "SecondLocation", "BLOCKALIGNMENTGRIP")?;
            data.write_3bd([body.orientation[0], body.orientation[1], body.orientation[2]]);
        }
        Some(DwgLogicalObjectBody::BlockStretchAction(body)) if object.type_code == 535 => {
            if body.points.is_empty()
                || body.points.iter().any(|point| point.len() != 2 || point.iter().any(|value| !value.is_finite()))
                || body.selections.iter().any(|selection| selection.object_handle == 0 || selection.vertex_indices.is_empty())
                || body.selectors.iter().any(|selector| selector.node_id == 0 || selector.point_indices.iter().any(|index| *index as usize >= body.points.len()))
                || !body.distance_multiplier.is_finite()
                || body.distance_multiplier <= 0.0
                || !body.angle_offset.is_finite()
                || !matches!(body.coordinate_mode, DwgBlockActionCoordinateMode::CartesianXy)
            {
                return Err(format!("BLOCKSTRETCHACTION {:#x} logical state is invalid", object.handle));
            }
            encode_r2010_block_action(&mut data, &mut strings, &mut handles, &body.action, "BLOCKSTRETCHACTION")?;
            write_r2010_action_connection(&mut data, &mut strings, &body.x_connection)?;
            write_r2010_action_connection(&mut data, &mut strings, &body.y_connection)?;
            data.write_bl(body.points.len() as u32);
            for point in &body.points {
                data.write_2rd([point[0], point[1]]);
            }
            data.write_bl(body.selections.len() as u32);
            for selection in &body.selections {
                data.write_bs(u16::try_from(selection.vertex_indices.len()).map_err(|_| "BLOCKSTRETCHACTION selection index count exceeds BS")?);
                for index in &selection.vertex_indices {
                    data.write_bl(*index);
                }
            }
            data.write_bl(body.selectors.len() as u32);
            for selector in &body.selectors {
                data.write_bl(selector.node_id);
                data.write_bs(u16::try_from(selector.point_indices.len()).map_err(|_| "BLOCKSTRETCHACTION selector index count exceeds BS")?);
                for index in &selector.point_indices {
                    data.write_bl(*index);
                }
            }
            data.write_bd(body.distance_multiplier);
            data.write_bd(body.angle_offset);
            data.write_rc(0);
            for selection in &body.selections {
                handles.write_handle(4, selection.object_handle);
            }
        }
        Some(DwgLogicalObjectBody::BlockScaleAction(body)) if object.type_code == 536 => {
            let base = &body.base;
            if base.offset.len() != 3 || base.base_point.len() != 3 || base.offset.iter().chain(&base.base_point).any(|value| !value.is_finite()) || !matches!(body.mode, DwgBlockScaleMode::Xy) {
                return Err(format!("BLOCKSCALEACTION {:#x} logical state is invalid", object.handle));
            }
            encode_r2010_block_action(&mut data, &mut strings, &mut handles, &base.action, "BLOCKSCALEACTION")?;
            data.write_3bd([base.offset[0], base.offset[1], base.offset[2]]);
            write_r2010_action_connection(&mut data, &mut strings, &base.x_base_connection)?;
            write_r2010_action_connection(&mut data, &mut strings, &base.y_base_connection)?;
            data.write_b(base.dependent);
            data.write_3bd([base.base_point[0], base.base_point[1], base.base_point[2]]);
            write_r2010_action_connection(&mut data, &mut strings, &body.uniform_scale_connection)?;
            write_r2010_action_connection(&mut data, &mut strings, &body.x_scale_connection)?;
            write_r2010_action_connection(&mut data, &mut strings, &body.y_scale_connection)?;
            data.write_rc(0);
        }
        Some(DwgLogicalObjectBody::BlockFlipAction(body)) if object.type_code == 537 => {
            encode_r2010_block_action(&mut data, &mut strings, &mut handles, &body.action, "BLOCKFLIPACTION")?;
            for connection in [&body.flip_connection, &body.updated_flip_connection, &body.updated_base_connection, &body.updated_end_connection] {
                write_r2010_action_connection(&mut data, &mut strings, connection)?;
            }
        }
        _ => return Err(format!("{} {:#x} body missing or mismatched", object.class_name, object.handle)),
    }
    append_r2010_string_stream(&mut data, &strings, &object.class_name, object.handle)?;
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_final_parameter_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    if !object.extended_data.is_empty() || object.owner_handle.is_none() || object.extension_dictionary_handle.is_some() {
        return Err(format!("{} {:#x} common state is invalid", object.class_name, object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(true);
    let mut strings = DwgBitWriter::new();
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    match object.body.as_ref() {
        Some(DwgLogicalObjectBody::BlockBasePointParameter(body)) if object.type_code == 538 => {
            let parameter = &body.parameter;
            if !object.reactor_handles.is_empty()
                || parameter.definition_point.len() != 3
                || body.point.len() != 3
                || body.base_point.len() != 3
                || parameter.properties.len() != 2
                || parameter.definition_point.iter().chain(&body.point).chain(&body.base_point).any(|value| !value.is_finite())
            {
                return Err(format!("BLOCKBASEPOINTPARAMETER {:#x} logical state is invalid", object.handle));
            }
            encode_r2010_block_element(&mut data, &mut strings, &parameter.element, "BLOCKBASEPOINTPARAMETER")?;
            data.write_b(parameter.show_properties);
            data.write_b(parameter.chain_actions);
            data.write_3bd([parameter.definition_point[0], parameter.definition_point[1], parameter.definition_point[2]]);
            for property in &parameter.properties {
                data.write_bl(property.connections.len() as u32);
                for connection in &property.connections {
                    data.write_bl(connection.code);
                    strings.write_tu(&connection.name);
                }
            }
            data.write_bl(0);
            data.write_3bd([body.point[0], body.point[1], body.point[2]]);
            data.write_3bd([body.base_point[0], body.base_point[1], body.base_point[2]]);
        }
        Some(DwgLogicalObjectBody::BlockVerticalConstraintParameter(body) | DwgLogicalObjectBody::BlockHorizontalConstraintParameter(body)) if matches!(object.type_code, 546 | 548) => {
            if object.reactor_handles.len() != 3
                || body.displacement_grip_node_id == 0
                || body.dependency_handle == 0
                || body.expression_name.is_empty()
                || !body.value.is_finite()
                || body.allowed_values.values.is_empty()
                || body.allowed_values.values.len() > u16::MAX as usize
                || body.allowed_values.values.iter().any(|value| !value.is_finite())
                || !body.parameter.property_expression_references.is_empty()
            {
                return Err(format!("{} {:#x} logical state is invalid", object.class_name, object.handle));
            }
            encode_r2010_two_point_parameter(&mut data, &mut strings, &body.parameter, [0, body.displacement_grip_node_id, 0, 0], &object.class_name)?;
            handles.write_handle(4, body.dependency_handle);
            strings.write_tu(&body.expression_name);
            strings.write_tu(&body.expression_description);
            data.write_bd(body.value);
            data.write_bl(8);
            let delta = body.allowed_values.values.windows(2).next().map(|pair| pair[1] - pair[0]);
            let uniform = delta.filter(|step| *step != 0.0 && body.allowed_values.values.windows(2).all(|pair| pair[1] - pair[0] == *step));
            if let Some(step) = uniform {
                data.write_bd(body.allowed_values.values[0]);
                data.write_bd(*body.allowed_values.values.last().unwrap());
                data.write_bd(step);
            } else {
                data.write_bd(0.0);
                data.write_bd(0.0);
                data.write_bd(0.0);
            }
            data.write_bs(body.allowed_values.values.len() as u16);
            for value in &body.allowed_values.values {
                data.write_bd(*value);
            }
        }
        _ => return Err(format!("{} {:#x} body missing or mismatched", object.class_name, object.handle)),
    }
    append_r2010_string_stream(&mut data, &strings, &object.class_name, object.handle)?;
    finish_r2010_object_frame(&data, handles)
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
