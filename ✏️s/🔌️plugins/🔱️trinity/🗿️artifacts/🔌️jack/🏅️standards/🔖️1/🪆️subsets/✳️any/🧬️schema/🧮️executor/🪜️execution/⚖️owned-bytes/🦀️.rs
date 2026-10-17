//! ⚖️ Jack query owned bytes: the result accounting limits, the per-value owned byte measures and the closing owner of the query graph metadata.

use super::*;

pub(super) const QUERY_ENTITY_MAXIMUM: usize = 16_384;

pub const QUERY_RESULT_MAXIMUM_OWNED_BYTES: usize = 1_048_576;

const QUERY_ENTITY_COLLECTION_MAXIMUM: usize = 128;

const QUERY_ENTITY_NESTING_MAXIMUM: usize = 16;

fn add_owned_bytes(bytes: &mut usize, additional: usize, maximum: usize) -> Result<(), ValueError> {
    *bytes = bytes.checked_add(additional).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit,"query entity byte count overflow"))?;
    if *bytes > maximum {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"query entity exceeds its byte grant"));
    }
    Ok(())
}

pub(super) fn property_owned_bytes(value: &PropertyValue, depth: usize, items: &mut usize, bytes: &mut usize, maximum: usize) -> Result<(), ValueError> {
    if depth > QUERY_ENTITY_NESTING_MAXIMUM {
        return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query entity exceeds its nesting admission"));
    }
    *items = items.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit,"query entity item count overflow"))?;
    if *items > QUERY_ENTITY_COLLECTION_MAXIMUM {
        return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query entity exceeds its collection admission"));
    }
    add_owned_bytes(bytes, size_of::<PropertyValue>(), maximum)?;
    match value {
        PropertyValue::String(value) => add_owned_bytes(bytes, value.len(), maximum),
        PropertyValue::Array(values) => {
            for value in values {
                property_owned_bytes(value, depth + 1, items, bytes, maximum)?;
            }
            Ok(())
        }
        PropertyValue::Object(values) => {
            for (key, value) in values {
                add_owned_bytes(bytes, key.len(), maximum)?;
                property_owned_bytes(value, depth + 1, items, bytes, maximum)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn bag_owned_bytes(values: &PropertyBag, items: &mut usize, bytes: &mut usize, maximum: usize) -> Result<(), ValueError> {
    for (key, value) in values {
        add_owned_bytes(bytes, key.len(), maximum)?;
        property_owned_bytes(value, 0, items, bytes, maximum)?;
    }
    Ok(())
}

pub fn node_owned_bytes(node: &Node, maximum: usize) -> Result<usize, ValueError> {
    let mut bytes = size_of::<Node>();
    let mut items = node.ports.len();
    if items > QUERY_ENTITY_COLLECTION_MAXIMUM {
        return Err(ValueError::new(ValueRefusalKind::WorkLimit,"query node exceeds its port admission"));
    }
    for value in [&node.id, &node.id, &node.kind, &node.name] {
        add_owned_bytes(&mut bytes, value.len(), maximum)?;
    }
    bag_owned_bytes(&node.properties, &mut items, &mut bytes, maximum)?;
    for port in &node.ports {
        add_owned_bytes(&mut bytes, size_of::<Port>(), maximum)?;
        add_owned_bytes(&mut bytes, port.id.len(), maximum)?;
        add_owned_bytes(&mut bytes, port.kind.len(), maximum)?;
        bag_owned_bytes(&port.properties, &mut items, &mut bytes, maximum)?;
    }
    Ok(bytes)
}

pub fn edge_owned_bytes(edge: &Edge, maximum: usize) -> Result<usize, ValueError> {
    let mut bytes = size_of::<Edge>();
    let mut items = 0;
    for value in [&edge.id, &edge.id, &edge.kind, &edge.source, &edge.target] {
        add_owned_bytes(&mut bytes, value.len(), maximum)?;
    }
    bag_owned_bytes(&edge.properties, &mut items, &mut bytes, maximum)?;
    Ok(bytes)
}

pub(super) fn reserve_result<T>(values: &mut Vec<T>, owned: &mut usize, maximum: usize) -> Result<(), ValueError> {
    if values.len() < values.capacity() { return Ok(()); }
    let inline = size_of::<T>();
    let previous = values.capacity();
    let growth = previous.max(1).min(maximum.saturating_sub(*owned).checked_div(inline).unwrap_or(usize::MAX)).max(1);
    let admitted = growth.checked_mul(inline).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query result capacity overflow"))?;
    add_result_owned(owned, admitted, maximum)?;
    values.try_reserve_exact(growth).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "query result allocation refused"))?;
    let extra = values.capacity().checked_sub(previous).and_then(|count| count.checked_sub(growth)).and_then(|count| count.checked_mul(inline)).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query result capacity overflow"))?;
    add_result_owned(owned, extra, maximum)
}

pub(super) fn add_result_owned(owned: &mut usize, additional: usize, maximum: usize) -> Result<(), ValueError> {
    *owned = owned.checked_add(additional).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query result byte count overflow"))?;
    if *owned > maximum { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "query result exceeds its retained ownership grant")); }
    Ok(())
}

pub(super) fn graph_result_metadata_owned_bytes(graph: &Graph, maximum: usize) -> Result<usize, ValueError> {
    let mut bytes = size_of::<crate::standards::v1::subsets::any::schema::ast::JackQueryGraphSnapshot>() + 2 * size_of::<usize>();
    add_result_owned(&mut bytes, JackSnapshot::SCHEMA.len() + graph.name.capacity() + graph.manifest_id.as_ref().map_or(0, String::capacity) + graph.root_node_id.as_ref().map_or(0, String::capacity), maximum)?;
    let mut properties = |values: &Vec<semio_framework_graph_manifest::PropertyDef>| -> Result<(), ValueError> {
        add_result_owned(&mut bytes, values.capacity().checked_mul(size_of::<semio_framework_graph_manifest::PropertyDef>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query metadata capacity overflow"))?, maximum)?;
        for value in values {
            add_result_owned(&mut bytes, value.name.capacity() + value.expr.as_ref().map_or(0, String::capacity), maximum)?;
            let mut value_type = &value.value_type;
            let mut depth = 0;
            loop {
                match value_type {
                    semio_framework_value::ValueType::List(inner) => {
                        depth += 1;
                        if depth > QUERY_ENTITY_NESTING_MAXIMUM { return Err(ValueError::new(ValueRefusalKind::WorkLimit, "query result metadata exceeds its nesting admission")); }
                        add_result_owned(&mut bytes, size_of::<semio_framework_value::ValueType>(), maximum)?;
                        value_type = inner;
                    }
                    semio_framework_value::ValueType::Schema(text) => { add_result_owned(&mut bytes, text.capacity(), maximum)?; break; }
                    _ => break,
                }
            }
        }
        Ok(())
    };
    for kind in &graph.manifest.node_kinds { properties(&kind.properties)?; }
    for kind in &graph.manifest.edge_kinds { properties(&kind.properties)?; }
    for kind in &graph.manifest.port_kinds { properties(&kind.properties)?; }
    for (capacity, inline) in [(graph.manifest.node_kinds.capacity(), size_of::<crate::standards::v1::subsets::any::schema::manifest::NodeKindDef>()), (graph.manifest.edge_kinds.capacity(), size_of::<crate::standards::v1::subsets::any::schema::manifest::EdgeKindDef>()), (graph.manifest.port_kinds.capacity(), size_of::<crate::standards::v1::subsets::any::schema::manifest::PortKindDef>())] {
        add_result_owned(&mut bytes, capacity.checked_mul(inline).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query metadata capacity overflow"))?, maximum)?;
    }
    for kind in &graph.manifest.node_kinds {
        add_result_owned(&mut bytes, kind.name.capacity(), maximum)?;
        add_result_owned(&mut bytes, kind.port_kinds.capacity().checked_mul(size_of::<String>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "query metadata capacity overflow"))?, maximum)?;
        for port in &kind.port_kinds { add_result_owned(&mut bytes, port.capacity(), maximum)?; }
    }
    for kind in &graph.manifest.edge_kinds { add_result_owned(&mut bytes, kind.name.capacity(), maximum)?; }
    for kind in &graph.manifest.port_kinds { add_result_owned(&mut bytes, kind.name.capacity(), maximum)?; }
    Ok(bytes)
}

/// ♻️ The one typed owner a closing query cursor holds between its extraction turn and its funded retirement birth.
#[derive(semio_framework_value::RetireOwned)]
pub(super) enum QueryClosingOwner {
    Effect(GraphEffect),
    Query(Query),
    Metadata(JackSnapshot),
}

/// 📤️ The graph metadata the working graph still owns once its entities are gone, moved into a snapshot shell that retires it.
pub(super) fn graph_metadata_owner(graph: &mut Graph) -> JackSnapshot {
    JackSnapshot { name: std::mem::take(&mut graph.name), manifest_id: graph.manifest_id.take(), manifest: std::mem::take(&mut graph.manifest), camera: graph.camera.clone(), root_node_id: graph.root_node_id.take(), query: std::mem::take(&mut graph.query), ..Default::default() }
}

/// 🧹️ Drives one small displaced value to its terminal witness inside the step that displaced it.
pub(super) fn retire_displaced(effect: GraphEffect) -> Result<(), ValueError> {
    crate::standards::v1::subsets::any::schema::content::retire_owned_to_terminal(effect)
}
