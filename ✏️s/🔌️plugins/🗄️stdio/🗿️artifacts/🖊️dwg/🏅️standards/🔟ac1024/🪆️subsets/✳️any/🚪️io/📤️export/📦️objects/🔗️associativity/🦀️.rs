//! 🔗️ Associativity object writers: networks, dependencies, variables, constraint groups and evaluation graphs.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::constraint_node_id;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::evaluation_graph_indexes;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::bits::DwgBitWriter;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::append_r2010_string_stream;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::constraint_point3;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_r2010_constraint_node;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::encode_r2010_eed;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::finish_r2010_object_frame;
use semio_s_artifact_stdio_dwg_write_records::standards::v_ac1024::subsets::any::io::export::records::fields::write_object_handle;

//#region 🔖️DwgStructuralCodec
//#region DwgRead
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_associative_dependency_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::AssociativeDependency(dependency)) = object.body.as_ref() else {
        return Err(format!("ACDBASSOCDEPENDENCY {:#x} body missing", object.handle));
    };
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bs(1);
    data.write_bl(0);
    data.write_b(dependency.is_read_dependency);
    data.write_b(dependency.is_write_dependency);
    data.write_b(dependency.is_attached_to_object);
    data.write_b(dependency.is_delegating_to_owning_action);
    data.write_bl(dependency.order as u32);
    data.write_b(dependency.name.is_some());
    data.write_bl(dependency.dependency_body_id as u32);
    let mut strings = DwgBitWriter::new();
    if let Some(name) = &dependency.name {
        strings.write_tu(name);
    }
    append_r2010_string_stream(&mut data, &strings, "ACDBASSOCDEPENDENCY", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    handles.write_handle(4, dependency.dependent_on_object_handle);
    handles.write_handle(4, dependency.read_dependency_handle.unwrap_or_default());
    handles.write_handle(4, dependency.dependency_node_handle.unwrap_or_default());
    handles.write_handle(3, dependency.dependency_body_handle.unwrap_or_default());
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_associative_value_dependency_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::AssociativeValueDependency(value)) = object.body.as_ref() else {
        return Err(format!("ACDBASSOCVALUEDEPENDENCY {:#x} body missing", object.handle));
    };
    let dependency = &value.dependency;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bs(1);
    data.write_bl(0);
    data.write_b(dependency.is_read_dependency);
    data.write_b(dependency.is_write_dependency);
    data.write_b(dependency.is_attached_to_object);
    data.write_b(dependency.is_delegating_to_owning_action);
    data.write_bl(dependency.order as u32);
    data.write_b(dependency.name.is_some());
    data.write_bl(dependency.dependency_body_id as u32);
    data.write_bs(0);
    match value.cached_value {
        semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationVariant::Integer32(cached) => {
            data.write_bs(90);
            data.write_bl(cached as u32);
        }
    }
    let mut strings = DwgBitWriter::new();
    if let Some(name) = &dependency.name {
        strings.write_tu(name);
    }
    strings.write_tu(&value.value_name);
    append_r2010_string_stream(&mut data, &strings, "ACDBASSOCVALUEDEPENDENCY", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    handles.write_handle(4, dependency.dependent_on_object_handle);
    handles.write_handle(4, dependency.read_dependency_handle.unwrap_or_default());
    handles.write_handle(4, dependency.dependency_node_handle.unwrap_or_default());
    handles.write_handle(3, dependency.dependency_body_handle.unwrap_or_default());
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_associative_geometry_dependency_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::AssociativeGeometryDependency(geometry)) = object.body.as_ref() else {
        return Err(format!("ACDBASSOCGEOMDEPENDENCY {:#x} body missing", object.handle));
    };
    let dependency = &geometry.dependency;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bs(1);
    data.write_bl(0);
    data.write_b(dependency.is_read_dependency);
    data.write_b(dependency.is_write_dependency);
    data.write_b(dependency.is_attached_to_object);
    data.write_b(dependency.is_delegating_to_owning_action);
    data.write_bl(dependency.order as u32);
    data.write_b(dependency.name.is_some());
    data.write_bl(dependency.dependency_body_id as u32);
    data.write_bs(0);
    data.write_b(geometry.enabled);
    data.write_b(geometry.dependent_on_compound_object);
    let mut strings = DwgBitWriter::new();
    if let Some(name) = &dependency.name {
        strings.write_tu(name);
    }
    strings.write_tu(&geometry.persistent_subentity_class_name);
    append_r2010_string_stream(&mut data, &strings, "ACDBASSOCGEOMDEPENDENCY", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    handles.write_handle(4, dependency.dependent_on_object_handle);
    handles.write_handle(4, dependency.read_dependency_handle.unwrap_or_default());
    handles.write_handle(4, dependency.dependency_node_handle.unwrap_or_default());
    handles.write_handle(3, dependency.dependency_body_handle.unwrap_or_default());
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_associative_variable_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationVariant;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::AssociativeVariable(variable)) = object.body.as_ref() else {
        return Err(format!("ACDBASSOCVARIABLE {:#x} body missing", object.handle));
    };
    if variable.action.maximum_dependency_index < 0 || variable.action.maximum_dependency_index as usize != variable.referenced_value_dependency_handles.len() {
        return Err(format!("ACDBASSOCVARIABLE {:#x} maximum dependency index must equal the referenced value-dependency count", object.handle));
    }
    if variable.mergeable != variable.mergeable_variable_name.is_some() {
        return Err(format!("ACDBASSOCVARIABLE {:#x} mergeable state and variable name disagree", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bs(1);
    data.write_bl(0);
    data.write_bl(variable.action.action_index as u32);
    data.write_bl(variable.action.maximum_dependency_index as u32);
    data.write_bl(variable.action.dependencies.len() as u32);
    for dependency in &variable.action.dependencies {
        data.write_b(dependency.owned);
    }
    data.write_bl(2);
    match variable.evaluated_value {
        DwgEvaluationVariant::Integer32(value) => {
            data.write_bs(90);
            data.write_bl(value as u32);
        }
    }
    data.write_b(variable.mergeable);
    data.write_b(variable.must_merge);
    if variable.action.maximum_dependency_index > 0 {
        data.write_bl(variable.referenced_value_dependency_handles.len() as u32);
    }
    data.write_bs(0);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&variable.name);
    strings.write_tu(&variable.expression);
    strings.write_tu(&variable.evaluator_id);
    strings.write_tu(&variable.description);
    if let Some(name) = &variable.mergeable_variable_name {
        strings.write_tu(name);
    }
    append_r2010_string_stream(&mut data, &strings, "ACDBASSOCVARIABLE", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    handles.write_handle(4, variable.action.owning_network_handle.unwrap_or_default());
    handles.write_handle(3, variable.action.action_body_handle.unwrap_or_default());
    for dependency in &variable.action.dependencies {
        handles.write_handle(if dependency.owned { 3 } else { 5 }, dependency.dependency_handle);
    }
    for dependency in &variable.referenced_value_dependency_handles {
        handles.write_handle(3, *dependency);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_associative_dimension_dependency_body_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::AssociativeDimensionDependencyBody(body)) = object.body.as_ref() else {
        return Err(format!("ASSOCDIMDEPENDENCYBODY {:#x} body missing", object.handle));
    };
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(object.reactor_handles.len() as u32);
    data.write_b(object.extension_dictionary_handle.is_none());
    data.write_bs(1);
    data.write_bs(1);
    data.write_bs(1);
    let mut strings = DwgBitWriter::new();
    strings.write_tu(&body.name);
    append_r2010_string_stream(&mut data, &strings, "ASSOCDIMDEPENDENCYBODY", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    for reactor in &object.reactor_handles {
        handles.write_handle(4, *reactor);
    }
    if let Some(extension_dictionary) = object.extension_dictionary_handle {
        handles.write_handle(4, extension_dictionary);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_evaluation_graph_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    let Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::EvaluationGraph(graph)) = object.body.as_ref() else {
        return Err(format!("ACAD_EVALUATION_GRAPH {:#x} body missing", object.handle));
    };
    if !object.extended_data.is_empty() || object.extension_dictionary_handle.is_some() || object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] || graph.nodes.is_empty() {
        return Err(format!("ACAD_EVALUATION_GRAPH {:#x} common or graph state is invalid", object.handle));
    }
    let (node_relations, edge_relations, endpoints) = evaluation_graph_indexes(graph)?;
    let watermark = graph.nodes.iter().map(|node| node.id).max().ok_or("evaluation graph has no nodes")?;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bl(watermark);
    data.write_bl(watermark);
    data.write_bl(graph.nodes.len() as u32);
    for (index, node) in graph.nodes.iter().enumerate() {
        data.write_bl(index as u32);
        data.write_bl(32);
        data.write_bl(node.id);
        for relation in node_relations[index] {
            data.write_bl(relation as u32);
        }
    }
    data.write_bl(graph.edges.len() as u32);
    for (index, edge) in graph.edges.iter().enumerate() {
        data.write_bl(index as u32);
        data.write_bl(0);
        data.write_bl(edge.reference_count);
        data.write_bl(endpoints[index].0 as u32);
        data.write_bl(endpoints[index].1 as u32);
        for relation in edge_relations[index] {
            data.write_bl(relation as u32);
        }
    }
    data.write_b(false);
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    for node in &graph.nodes {
        handles.write_handle(3, node.expression_handle);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_assoc_network_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssocNetworkMemberKind;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeActionStatus;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::AssocNetwork(network)) = object.body.as_ref() else { return Err(format!("ACDBASSOCNETWORK {:#x} body missing", object.handle)) };
    let action = &network.action;
    if !matches!(action.status, DwgAssociativeActionStatus::UpToDate)
        || action.maximum_dependency_index != 0
        || !action.dependencies.is_empty()
        || action.action_body_handle.is_some()
        || object.owner_handle.is_none()
        || object.reactor_handles.len() != 1
        || object.extension_dictionary_handle.is_some()
        || network.actions.iter().any(|member| member.handle == 0)
    {
        return Err(format!("ACDBASSOCNETWORK {:#x} logical state is invalid", object.handle));
    }
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(1);
    data.write_b(true);
    data.write_bs(1);
    data.write_bl(0);
    data.write_bl(action.action_index as u32);
    data.write_bl(action.maximum_dependency_index as u32);
    data.write_bl(0);
    data.write_bs(0);
    data.write_bl(network.network_action_index as u32);
    data.write_bl(network.actions.len() as u32);
    for member in &network.actions {
        data.write_b(matches!(member.kind, DwgAssocNetworkMemberKind::Action));
    }
    data.write_bl(0);
    append_r2010_string_stream(&mut data, &DwgBitWriter::new(), "ACDBASSOCNETWORK", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, object.reactor_handles[0]);
    handles.write_handle(4, action.owning_network_handle.unwrap_or_default());
    handles.write_handle(3, 0);
    for member in &network.actions {
        handles.write_handle(if matches!(member.kind, DwgAssocNetworkMemberKind::Network) { 4 } else { 3 }, member.handle);
    }
    finish_r2010_object_frame(&data, handles)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_r2010_assoc_2d_constraint_group_frame(object: &semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<Vec<u8>, String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeActionStatus;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let Some(DwgLogicalObjectBody::Assoc2dConstraintGroup(group)) = object.body.as_ref() else { return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {:#x} body missing", object.handle)) };
    if !matches!(group.action.status, DwgAssociativeActionStatus::UpToDate)
        || object.owner_handle.is_none()
        || !object.reactor_handles.is_empty()
        || object.extension_dictionary_handle.is_some()
        || group.action.action_body_handle.is_some()
        || group.work_plane.len() != 3
        || group.member_action_handles.contains(&0)
        || group.nodes.is_empty()
        || group.nodes.len() > 10_000
    {
        return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {:#x} logical state is invalid", object.handle));
    }
    let work_plane = group.work_plane.iter().enumerate().map(|(index, point)| constraint_point3(point, &format!("work-plane point {index}"))).collect::<Result<Vec<_>, _>>()?;
    let mut data = DwgBitWriter::new();
    data.write_bot(object.type_code);
    data.write_handle(0, object.handle);
    encode_r2010_eed(&mut data, object.handle, &object.extended_data)?;
    data.write_bl(0);
    data.write_b(true);
    data.write_bs(1);
    data.write_bl(0);
    data.write_bl(group.action.action_index as u32);
    data.write_bl(group.action.maximum_dependency_index as u32);
    data.write_bl(group.action.dependencies.len() as u32);
    for dependency in &group.action.dependencies {
        data.write_b(dependency.owned);
    }
    data.write_bl(0);
    data.write_b(group.do_not_check_newly_added_constraints);
    for point in work_plane {
        data.write_3bd(point);
    }
    data.write_bl(group.member_action_handles.len() as u32);
    data.write_bl(group.nodes.iter().map(constraint_node_id).max().and_then(|value| value.checked_add(1)).ok_or("constraint group node watermark is invalid")? as u32);
    data.write_bl(group.nodes.len() as u32);
    let mut strings = DwgBitWriter::new();
    let mut node_handles = DwgBitWriter::new();
    for node in &group.nodes {
        encode_r2010_constraint_node(node, &mut data, &mut strings, &mut node_handles)?;
    }
    append_r2010_string_stream(&mut data, &strings, "ACDBASSOC2DCONSTRAINTGROUP", object.handle)?;
    let mut handles = DwgBitWriter::new();
    write_object_handle(&mut handles, object.handle, object.owner_handle);
    handles.write_handle(4, group.action.owning_network_handle.unwrap_or_default());
    handles.write_handle(3, 0);
    for dependency in &group.action.dependencies {
        handles.write_handle(if dependency.owned { 3 } else { 4 }, dependency.dependency_handle);
    }
    handles.write_handle(3, 0);
    for member in &group.member_action_handles {
        handles.write_handle(3, *member);
    }
    handles.append_bits(&node_handles);
    finish_r2010_object_frame(&data, handles)
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
