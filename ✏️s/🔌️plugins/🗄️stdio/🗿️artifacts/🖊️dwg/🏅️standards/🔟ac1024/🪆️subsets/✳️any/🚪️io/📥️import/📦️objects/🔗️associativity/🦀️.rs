//! 🔗️ Associativity object decoders: networks, dependencies, variables, constraint groups and evaluation graphs.

use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::DwgBitReader;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::constraint_node_id;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::io::binary::snapshot::evaluation_graph_indexes;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::entities::validate_entity_terminal_fill;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::decode_r2010_constraint_node;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::r2010_string_content_end_bit;
use semio_s_artifact_stdio_dwg_read_records::standards::v_ac1024::subsets::any::io::import::records::fields::read_object_handle;

//#region 🔖️DwgStructuralCodec
//#region DwgRead
/// 🧩️ Decodes one assoc network object body from its main, string and handle streams.
pub fn decode_assoc_network_object(handle: u64, payload_size: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssocNetwork;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssocNetworkMember;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssocNetworkMemberKind;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeAction;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeActionDependency;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeActionStatus;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let action_version = data.read_bs()?;
    let action_status = data.read_bl()?;
    if action_version != 1 || action_status != 0 {
        return Err(format!("ACDBASSOCNETWORK {handle:#x} action version/status is unsupported"));
    }
    let action_index = data.read_bl()? as i32;
    let maximum_dependency_index = data.read_bl()? as i32;
    let dependency_count = data.read_bl()? as usize;
    let dependency_ownership = (0..dependency_count).map(|_| data.read_b()).collect::<Result<Vec<_>, _>>()?;
    let network_version = data.read_bs()?;
    let network_action_index = data.read_bl()? as i32;
    let action_count = data.read_bl()? as usize;
    let action_kinds = (0..action_count).map(|_| data.read_b()).collect::<Result<Vec<_>, _>>()?;
    let owned_action_count = data.read_bl()? as usize;
    if network_version != 0 || owned_action_count != 0 || data.bit_position() != main_end_bit {
        return Err(format!("ACDBASSOCNETWORK {handle:#x} native derived state or main boundary is invalid"));
    }
    let owning_network_handle = read_object_handle(handle_reader, handle)?;
    let action_body_handle = read_object_handle(handle_reader, handle)?;
    let dependencies = dependency_ownership
        .into_iter()
        .map(|owned| read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCNETWORK {handle:#x} dependency is null")).map(|dependency_handle| DwgAssociativeActionDependency { owned, dependency_handle }))
        .collect::<Result<Vec<_>, String>>()?;
    let actions = action_kinds
        .into_iter()
        .map(|owned| {
            read_object_handle(handle_reader, handle)?
                .ok_or_else(|| format!("ACDBASSOCNETWORK {handle:#x} member is null"))
                .map(|member_handle| DwgAssocNetworkMember { handle: member_handle, kind: if owned { DwgAssocNetworkMemberKind::Action } else { DwgAssocNetworkMemberKind::Network } })
        })
        .collect::<Result<Vec<_>, String>>()?;
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "ACDBASSOCNETWORK")?;
    object.body = Some(DwgLogicalObjectBody::AssocNetwork(DwgAssocNetwork {
        action: DwgAssociativeAction { status: DwgAssociativeActionStatus::UpToDate, owning_network_handle, action_body_handle, action_index, maximum_dependency_index, dependencies },
        network_action_index,
        actions,
    }));

    Ok(())
}
/// 🧩️ Decodes one assoc 2D constraint group object body from its main, string and handle streams.
pub fn decode_assoc_2d_constraint_group_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::constraints::DwgAssoc2dConstraintGroup;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeAction;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeActionDependency;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeActionStatus;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let strings = strings.as_mut().ok_or_else(|| format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} class stream missing"))?;
    let action_version = data.read_bs()?;
    let action_status = data.read_bl()?;
    if action_version != 1 || action_status != 0 {
        return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} action version/status is unsupported"));
    }
    let action_index = data.read_bl()? as i32;
    let maximum_dependency_index = data.read_bl()? as i32;
    let dependency_count = data.read_bl()? as usize;
    if dependency_count > 10_000 {
        return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} dependency count is invalid"));
    }
    let dependency_ownership = (0..dependency_count).map(|_| data.read_b()).collect::<Result<Vec<_>, _>>()?;
    if data.read_bl()? != 0 {
        return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} group version is unsupported"));
    }
    let do_not_check_newly_added_constraints = data.read_b()?;
    let work_plane = (0..3).map(|_| data.read_3bd().map(|value| value.to_vec())).collect::<Result<Vec<_>, _>>()?;
    let member_count = data.read_bl()? as usize;
    if member_count > 10_000 {
        return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} member count is invalid"));
    }
    let node_id_watermark = data.read_bl()? as i32;
    let node_count = data.read_bl()? as usize;
    if node_count == 0 || node_count > 10_000 {
        return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} node count {node_count} is invalid"));
    }
    let owning_network_handle = read_object_handle(handle_reader, handle)?;
    let action_body_handle = read_object_handle(handle_reader, handle)?;
    let dependencies = dependency_ownership
        .into_iter()
        .map(|owned| read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} dependency is null")).map(|dependency_handle| DwgAssociativeActionDependency { owned, dependency_handle }))
        .collect::<Result<Vec<_>, String>>()?;
    if read_object_handle(handle_reader, handle)?.is_some() {
        return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} compatibility dimension slot is nonnull"));
    }
    let member_action_handles = (0..member_count).map(|_| read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} member action is null"))).collect::<Result<Vec<_>, String>>()?;
    let mut nodes = Vec::with_capacity(node_count);
    for index in 0..node_count {
        let class = strings.read_tu()?;
        let bit = data.bit_position();
        nodes.push(decode_r2010_constraint_node(&class, data, handle_reader, handle).map_err(|error| format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} node {index} {class} at bit {bit}: {error}"))?);
    }
    let actual_node_id_watermark = nodes.iter().map(constraint_node_id).max().and_then(|value| value.checked_add(1));
    let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
    if actual_node_id_watermark != Some(node_id_watermark) || data.bit_position() != main_end_bit || strings.bit_position() != string_end_bit {
        return Err(format!("ACDBASSOC2DCONSTRAINTGROUP {handle:#x} watermark {actual_node_id_watermark:?}/{node_id_watermark}, main {}/{main_end_bit}, strings {}/{string_end_bit}", data.bit_position(), strings.bit_position()));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "ACDBASSOC2DCONSTRAINTGROUP")?;
    object.body = Some(DwgLogicalObjectBody::Assoc2dConstraintGroup(DwgAssoc2dConstraintGroup {
        action: DwgAssociativeAction { status: DwgAssociativeActionStatus::UpToDate, owning_network_handle, action_body_handle, action_index, maximum_dependency_index, dependencies },
        do_not_check_newly_added_constraints,
        work_plane,
        member_action_handles,
        nodes,
    }));

    Ok(())
}
/// 🧩️ Decodes one evaluation graph object body from its main, string and handle streams.
pub fn decode_evaluation_graph_object(handle: u64, payload_size: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationGraph;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationGraphEdge;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationGraphNode;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let watermark = data.read_bl()?;
    let watermark_copy = data.read_bl()?;
    if watermark != watermark_copy {
        return Err(format!("ACAD_EVALUATION_GRAPH {handle:#x} watermark copies disagree"));
    }
    let node_count = data.read_bl()? as usize;
    let mut nodes = Vec::with_capacity(node_count);
    let mut native_node_relations = Vec::with_capacity(node_count);
    for index in 0..node_count {
        if data.read_bl()? as usize != index || data.read_bl()? != 32 {
            return Err(format!("ACAD_EVALUATION_GRAPH {handle:#x} node {index} native identity is invalid"));
        }
        let id = data.read_bl()?;
        let mut relations = [0i32; 4];
        for relation in &mut relations {
            *relation = data.read_bl()? as i32;
        }
        nodes.push(DwgEvaluationGraphNode { id, expression_handle: 0 });
        native_node_relations.push(relations);
    }
    let edge_count = data.read_bl()? as usize;
    let mut edges = Vec::with_capacity(edge_count);
    let mut native_edge_relations = Vec::with_capacity(edge_count);
    let mut native_endpoints = Vec::with_capacity(edge_count);
    for index in 0..edge_count {
        if data.read_bl()? as usize != index || data.read_bl()? != 0 {
            return Err(format!("ACAD_EVALUATION_GRAPH {handle:#x} edge {index} native identity is invalid"));
        }
        let reference_count = data.read_bl()?;
        let from = data.read_bl()? as usize;
        let to = data.read_bl()? as usize;
        let from_node_id = nodes.get(from).ok_or_else(|| format!("ACAD_EVALUATION_GRAPH {handle:#x} edge {index} source is invalid"))?.id;
        let to_node_id = nodes.get(to).ok_or_else(|| format!("ACAD_EVALUATION_GRAPH {handle:#x} edge {index} target is invalid"))?.id;
        let mut relations = [0i32; 5];
        for relation in &mut relations {
            *relation = data.read_bl()? as i32;
        }
        edges.push(DwgEvaluationGraphEdge { from_node_id, to_node_id, reference_count, invertible: false, suppressed: false });
        native_edge_relations.push(relations);
        native_endpoints.push((from, to));
    }
    if data.bit_position() != main_end_bit || nodes.iter().map(|node| node.id).max() != Some(watermark) {
        return Err(format!("ACAD_EVALUATION_GRAPH {handle:#x} main stream or watermark is invalid"));
    }
    if !object.extended_data.is_empty() || object.extension_dictionary_handle.is_some() || object.owner_handle.is_none() || object.reactor_handles.as_slice() != [object.owner_handle.unwrap_or_default()] {
        return Err(format!("ACAD_EVALUATION_GRAPH {handle:#x} common state is invalid"));
    }
    for node in &mut nodes {
        node.expression_handle = read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACAD_EVALUATION_GRAPH {handle:#x} expression handle is null"))?;
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "ACAD_EVALUATION_GRAPH")?;
    let graph = DwgEvaluationGraph { nodes, edges };
    let (derived_node_relations, derived_edge_relations, derived_endpoints) = evaluation_graph_indexes(&graph)?;
    if native_node_relations != derived_node_relations || native_edge_relations != derived_edge_relations || native_endpoints != derived_endpoints {
        return Err(format!("ACAD_EVALUATION_GRAPH {handle:#x} native indexes do not match the semantic graph"));
    }
    object.body = Some(DwgLogicalObjectBody::EvaluationGraph(graph));

    Ok(())
}
/// 🧩️ Decodes one assoc dimension dependency body object body from its main, string and handle streams.
pub fn decode_assoc_dimension_dependency_body_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    let dependency_version = data.read_bs()?;
    let dimension_base_version = data.read_bs()?;
    let class_version = data.read_bs()?;
    if (dependency_version, dimension_base_version, class_version) != (1, 1, 1) {
        return Err(format!("ASSOCDIMDEPENDENCYBODY {handle:#x} versions {dependency_version}/{dimension_base_version}/{class_version} are unsupported"));
    }
    if data.bit_position() != main_end_bit {
        return Err(format!("ASSOCDIMDEPENDENCYBODY {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("ASSOCDIMDEPENDENCYBODY {handle:#x} string stream missing"))?;
    let name = strings.read_tu()?;
    let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
    if strings.bit_position() != string_end_bit {
        return Err(format!("ASSOCDIMDEPENDENCYBODY {handle:#x} string stream is not exactly consumed"));
    }
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "ASSOCDIMDEPENDENCYBODY")?;
    object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::AssociativeDimensionDependencyBody(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeDimensionDependencyBody { name }));

    Ok(())
}
/// 🧩️ Decodes one assoc variable object body from its main, string and handle streams.
pub fn decode_assoc_variable_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeAction;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeActionDependency;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeActionStatus;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeVariable;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationVariant;
    use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
    let action_version = data.read_bs()?;
    let action_status = data.read_bl()?;
    if action_version != 1 || action_status != 0 {
        return Err(format!("ACDBASSOCVARIABLE {handle:#x} action version/status {action_version}/{action_status} is unsupported"));
    }
    let action_index = data.read_bl()? as i32;
    let maximum_dependency_index = data.read_bl()? as i32;
    let dependency_count = data.read_bl()? as usize;
    let dependency_ownership = (0..dependency_count).map(|_| data.read_b()).collect::<Result<Vec<_>, _>>()?;
    let variable_version = data.read_bl()?;
    let value_code = data.read_bs()?;
    if variable_version != 2 || value_code != 90 {
        return Err(format!("ACDBASSOCVARIABLE {handle:#x} variable version/value code {variable_version}/{value_code} is unsupported"));
    }
    let evaluated_value = DwgEvaluationVariant::Integer32(data.read_bl()? as i32);
    let mergeable = data.read_b()?;
    let must_merge = data.read_b()?;
    let binding_count = if maximum_dependency_index > 0 { data.read_bl()? as usize } else { 0 };
    let binding_version = data.read_bs()?;
    if maximum_dependency_index < 0 || maximum_dependency_index as usize != binding_count || binding_version != 0 {
        return Err(format!("ACDBASSOCVARIABLE {handle:#x} binding index/count/version is inconsistent"));
    }
    if data.bit_position() != main_end_bit {
        return Err(format!("ACDBASSOCVARIABLE {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("ACDBASSOCVARIABLE {handle:#x} string stream missing"))?;
    let name = strings.read_tu()?;
    let expression = strings.read_tu()?;
    let evaluator_id = strings.read_tu()?;
    let description = strings.read_tu()?;
    let mergeable_variable_name = if mergeable { Some(strings.read_tu()?) } else { None };
    let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
    if strings.bit_position() != string_end_bit {
        return Err(format!("ACDBASSOCVARIABLE {handle:#x} string stream is not exactly consumed: {} != {string_end_bit}", strings.bit_position()));
    }
    let owning_network_handle = Some(read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCVARIABLE {handle:#x} owning network is null"))?);
    let action_body_handle = read_object_handle(handle_reader, handle)?;
    let dependencies = dependency_ownership
        .into_iter()
        .map(|owned| read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCVARIABLE {handle:#x} action dependency is null")).map(|dependency_handle| DwgAssociativeActionDependency { owned, dependency_handle }))
        .collect::<Result<Vec<_>, String>>()?;
    let referenced_value_dependency_handles = (0..binding_count).map(|_| read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCVARIABLE {handle:#x} value dependency is null"))).collect::<Result<Vec<_>, String>>()?;
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "ACDBASSOCVARIABLE")?;
    object.body = Some(DwgLogicalObjectBody::AssociativeVariable(DwgAssociativeVariable {
        action: DwgAssociativeAction { status: DwgAssociativeActionStatus::UpToDate, owning_network_handle, action_body_handle, action_index, maximum_dependency_index, dependencies },
        name,
        expression,
        evaluator_id,
        description,
        evaluated_value,
        mergeable,
        mergeable_variable_name,
        must_merge,
        referenced_value_dependency_handles,
    }));

    Ok(())
}
/// 🧩️ Decodes one assoc geometry dependency object body from its main, string and handle streams.
pub fn decode_assoc_geometry_dependency_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    let class_version = data.read_bs()?;
    let status = data.read_bl()?;
    if class_version != 1 || status != 0 {
        return Err(format!("ACDBASSOCGEOMDEPENDENCY {handle:#x} has unsupported dependency version/status {class_version}/{status}"));
    }
    let is_read_dependency = data.read_b()?;
    let is_write_dependency = data.read_b()?;
    let is_attached_to_object = data.read_b()?;
    let is_delegating_to_owning_action = data.read_b()?;
    let order = data.read_bl()? as i32;
    let has_name = data.read_b()?;
    let dependency_body_id = data.read_bl()? as i32;
    let geometry_version = data.read_bs()?;
    let enabled = data.read_b()?;
    let dependent_on_compound_object = data.read_b()?;
    if geometry_version != 0 {
        return Err(format!("ACDBASSOCGEOMDEPENDENCY {handle:#x} geometry version {geometry_version} is unsupported"));
    }
    if data.bit_position() != main_end_bit {
        return Err(format!("ACDBASSOCGEOMDEPENDENCY {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("ACDBASSOCGEOMDEPENDENCY {handle:#x} string stream missing"))?;
    let name = if has_name { Some(strings.read_tu()?) } else { None };
    let persistent_subentity_class_name = strings.read_tu()?;
    let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
    if strings.bit_position() != string_end_bit {
        return Err(format!("ACDBASSOCGEOMDEPENDENCY {handle:#x} string stream is not exactly consumed: {} != {string_end_bit}", strings.bit_position()));
    }
    let dependent_on_object_handle = read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCGEOMDEPENDENCY {handle:#x} dependent-on object is null"))?;
    let read_dependency_handle = read_object_handle(handle_reader, handle)?;
    let dependency_node_handle = read_object_handle(handle_reader, handle)?;
    let dependency_body_handle = read_object_handle(handle_reader, handle)?;
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "ACDBASSOCGEOMDEPENDENCY")?;
    object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::AssociativeGeometryDependency(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeGeometryDependency {
        dependency: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeDependency {
            status: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeDependencyStatus::UpToDate,
            is_read_dependency,
            is_write_dependency,
            is_attached_to_object,
            is_delegating_to_owning_action,
            order,
            dependent_on_object_handle,
            name,
            read_dependency_handle,
            dependency_node_handle,
            dependency_body_handle,
            dependency_body_id,
        },
        enabled,
        persistent_subentity_class_name,
        dependent_on_compound_object,
    }));

    Ok(())
}
/// 🧩️ Decodes one assoc value dependency object body from its main, string and handle streams.
pub fn decode_assoc_value_dependency_object(handle: u64, payload: &[u8], payload_size: usize, data_end_bit: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    let class_version = data.read_bs()?;
    let status = data.read_bl()?;
    if class_version != 1 || status != 0 {
        return Err(format!("ACDBASSOCVALUEDEPENDENCY {handle:#x} has unsupported dependency version/status {class_version}/{status}"));
    }
    let is_read_dependency = data.read_b()?;
    let is_write_dependency = data.read_b()?;
    let is_attached_to_object = data.read_b()?;
    let is_delegating_to_owning_action = data.read_b()?;
    let order = data.read_bl()? as i32;
    let has_name = data.read_b()?;
    let dependency_body_id = data.read_bl()? as i32;
    let value_dependency_version = data.read_bs()?;
    let cached_value_code = data.read_bs()? as i16;
    if value_dependency_version != 0 || cached_value_code != 90 {
        return Err(format!("ACDBASSOCVALUEDEPENDENCY {handle:#x} has unsupported value version/code {value_dependency_version}/{cached_value_code}"));
    }
    let cached_value = semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgEvaluationVariant::Integer32(data.read_bl()? as i32);
    if data.bit_position() != main_end_bit {
        return Err(format!("ACDBASSOCVALUEDEPENDENCY {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    let strings = strings.as_mut().ok_or_else(|| format!("ACDBASSOCVALUEDEPENDENCY {handle:#x} string stream missing"))?;
    let name = if has_name { Some(strings.read_tu()?) } else { None };
    let value_name = strings.read_tu()?;
    let string_end_bit = r2010_string_content_end_bit(payload, data_end_bit)?;
    if strings.bit_position() != string_end_bit {
        return Err(format!("ACDBASSOCVALUEDEPENDENCY {handle:#x} string stream is not exactly consumed: {} != {string_end_bit}", strings.bit_position()));
    }
    let dependent_on_object_handle = read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCVALUEDEPENDENCY {handle:#x} dependent-on object is null"))?;
    let read_dependency_handle = read_object_handle(handle_reader, handle)?;
    let dependency_node_handle = read_object_handle(handle_reader, handle)?;
    let dependency_body_handle = read_object_handle(handle_reader, handle)?;
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "ACDBASSOCVALUEDEPENDENCY")?;
    object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::AssociativeValueDependency(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeValueDependency {
        dependency: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeDependency {
            status: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeDependencyStatus::UpToDate,
            is_read_dependency,
            is_write_dependency,
            is_attached_to_object,
            is_delegating_to_owning_action,
            order,
            dependent_on_object_handle,
            name,
            read_dependency_handle,
            dependency_node_handle,
            dependency_body_handle,
            dependency_body_id,
        },
        cached_value,
        value_name,
    }));

    Ok(())
}
/// 🧩️ Decodes one assoc dependency object body from its main, string and handle streams.
pub fn decode_assoc_dependency_object(handle: u64, payload_size: usize, main_end_bit: usize, data: &mut DwgBitReader<'_>, handle_reader: &mut DwgBitReader<'_>, strings: &mut Option<DwgBitReader<'_>>, object: &mut semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObject) -> Result<(), String> {
    let class_version = data.read_bs().map_err(|error| format!("ACDBASSOCDEPENDENCY {handle:#x} class version: {error}"))?;
    if class_version != 1 {
        return Err(format!("ACDBASSOCDEPENDENCY {handle:#x} class version {class_version} is unsupported"));
    }
    let status = data.read_bl().map_err(|error| format!("ACDBASSOCDEPENDENCY {handle:#x} status: {error}"))?;
    if status != 0 {
        return Err(format!("ACDBASSOCDEPENDENCY {handle:#x} status {status} is unsupported"));
    }
    let is_read_dependency = data.read_b()?;
    let is_write_dependency = data.read_b()?;
    let is_attached_to_object = data.read_b()?;
    let is_delegating_to_owning_action = data.read_b()?;
    let order = data.read_bl()? as i32;
    let has_name = data.read_b()?;
    let dependency_body_id = data.read_bl()? as i32;
    if data.bit_position() != main_end_bit {
        return Err(format!("ACDBASSOCDEPENDENCY {handle:#x} main stream is not exactly consumed: {} != {main_end_bit}", data.bit_position()));
    }
    let name = if has_name { Some(strings.as_mut().ok_or_else(|| format!("ACDBASSOCDEPENDENCY {handle:#x} string stream missing"))?.read_tu()?) } else { None };
    if strings.as_ref().is_some_and(|reader| reader.bit_position() != main_end_bit) {
        return Err(format!("ACDBASSOCDEPENDENCY {handle:#x} string stream is not exactly consumed"));
    }
    let dependent_on_object_handle = read_object_handle(handle_reader, handle)?.ok_or_else(|| format!("ACDBASSOCDEPENDENCY {handle:#x} dependent-on object is null"))?;
    let read_dependency_handle = read_object_handle(handle_reader, handle)?;
    let dependency_node_handle = read_object_handle(handle_reader, handle)?;
    let dependency_body_handle = read_object_handle(handle_reader, handle)?;
    validate_entity_terminal_fill(handle_reader, payload_size * 8, handle, "ACDBASSOCDEPENDENCY")?;
    object.body = Some(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody::AssociativeDependency(semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeDependency {
        status: semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::associativity::DwgAssociativeDependencyStatus::UpToDate,
        is_read_dependency,
        is_write_dependency,
        is_attached_to_object,
        is_delegating_to_owning_action,
        order,
        dependent_on_object_handle,
        name,
        read_dependency_handle,
        dependency_node_handle,
        dependency_body_handle,
        dependency_body_id,
    }));

    Ok(())
}
//#endregion DwgRead
//#endregion 🔖️DwgStructuralCodec
