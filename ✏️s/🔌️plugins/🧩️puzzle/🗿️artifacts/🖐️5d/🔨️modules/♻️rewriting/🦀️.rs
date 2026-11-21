//! 🧩️ Projects a Puzzle 5D document into Jack and admits a rewrite as typed Puzzle mutations.

use crate::Puzzle5dSnapshot;
use crate::standards::v1::subsets::any::schema::document_records::{
    Puzzle5dFastener, Puzzle5dGrip, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dKindCatalogs,
    Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPartAnchor, Puzzle5dPart3d, Puzzle5dScale,
};
use crate::standards::v1::subsets::any::schema::mutations::{self, Puzzle5dMutation};
use protocol::{ColdMutation, DiffAlgebra, MutationAdmission, MutationDiff, apply_diff_cold_into};
use semio_framework_graph_manifest::{PortDirection, PropertyBag, PropertyDef, PropertyKind, PropertyValue};
use semio_framework_value::{FromValue, ToValue, ValueType};
use semio_s_artifact_puzzle_5d_kind_catalogs::standards::v1::subsets::any::schema::graph_manifest::puzzle5d_default::puzzle5d_default_manifest;
pub(crate) use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::executor::QueryExecution;
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::{
    graph::{Camera, Edge, EntityRef, GraphEffect, Node, Port, parse_port_key, port_key},
    manifest::{EdgeKindDef, Manifest, NodeKindDef, PortKindDef},
    memory::Graph,
};
use semio_s_artifact_trinity_rewriting_inferences::{Rule, compile_rule};
use std::collections::{BTreeMap, BTreeSet};

const PART: &str = "Part";
const GRIP: &str = "Grip";
const FASTENER: &str = "Fastener";
const IN_SUFFIX: &str = "@in";
const PART_PROPERTIES: &[(&str, ValueType)] = &[
    ("anchor", ValueType::Text), ("shape", ValueType::Text), ("radius", ValueType::Decimal),
    ("width", ValueType::Decimal), ("height", ValueType::Decimal), ("text", ValueType::Text),
    ("iconKind", ValueType::Text), ("hidden", ValueType::Boolean), ("locked", ValueType::Boolean),
    ("origin", ValueType::Any), ("meshUrl", ValueType::Text), ("orientation", ValueType::Any),
    ("scale", ValueType::Any), ("label", ValueType::Text),
];
const POSE: [&str; 8] = ["gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y"];

/// ⚠️ A Puzzle graph or rewrite effect could not be admitted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Puzzle5dRewriteError(String);

impl std::fmt::Display for Puzzle5dRewriteError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for Puzzle5dRewriteError {}

fn invalid(message: impl Into<String>) -> Puzzle5dRewriteError {
    Puzzle5dRewriteError(message.into())
}

fn valid_id(id: &str) -> Result<(), Puzzle5dRewriteError> {
    if id.is_empty() || id.contains([':', '@']) {
        return Err(invalid(format!("Invalid Puzzle 5D graph identifier {id:?}")));
    }
    Ok(())
}

fn properties(fields: impl IntoIterator<Item = (&'static str, ValueType)>) -> Vec<PropertyDef> {
    fields.into_iter().map(|(name, value_type)| PropertyDef {
        name: name.into(), kind: PropertyKind::Data, value_type, expr: None,
    }).collect()
}

fn field_bag(value: &impl ToValue) -> PropertyBag {
    PropertyBag::from_value(value.to_value()).expect("Puzzle 5D records encode as property bags")
}

fn grip_kind(grip: &Puzzle5dGrip) -> Option<&str> {
    grip.grip_kind.as_deref().filter(|kind| !kind.is_empty())
        .or(grip.grip_2d.grip_kind.as_deref()).filter(|kind| !kind.is_empty())
}

fn grip_ports(grip: &Puzzle5dGrip) -> [Port; 2] {
    let kind = grip_kind(grip).unwrap_or(GRIP).to_owned();
    let gr = field_bag(grip);
    [
        Port { id: grip.id.clone(), kind: kind.clone(), direction: PortDirection::Out, properties: gr.clone() },
        Port { id: format!("{}{IN_SUFFIX}", grip.id), kind, direction: PortDirection::In, properties: gr },
    ]
}

fn puzzle_endpoint(key: &str) -> Result<(&str, &str), Puzzle5dRewriteError> {
    let (part, grip) = key.split_once(':').ok_or_else(|| invalid(format!("Invalid Puzzle grip endpoint {key:?}")))?;
    valid_id(part)?;
    valid_id(grip)?;
    Ok((part, grip))
}

fn pose(fastener: &Puzzle5dFastener) -> [f64; 8] {
    [fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt, fastener.x, fastener.y]
}

fn node_manifest(snapshot: &Puzzle5dSnapshot, catalog: &Puzzle5dKindCatalogs) -> Manifest {
    let shipped = puzzle5d_default_manifest();
    let grip_kinds: BTreeSet<_> = std::iter::once(GRIP)
        .chain(shipped.port_kinds.iter().map(|kind| kind.id.as_str()))
        .chain(catalog.grips.iter().map(|kind| kind.id.as_str()))
        .chain(catalog.parts.iter().flat_map(|kind| &kind.grips).filter_map(|grip| grip.grip_kind.as_deref()))
        .chain(snapshot.parts.iter().flat_map(|part| &part.grips).filter_map(grip_kind))
        .collect();
    let mut part_kinds = BTreeMap::new();
    for kind in &catalog.parts {
        part_kinds.entry(kind.id.as_str()).or_insert((Some(kind), None));
    }
    for part in &snapshot.parts {
        if let Some(name) = part.part_kind.as_deref() {
            part_kinds.entry(name).or_default().1.get_or_insert(part);
        }
    }
    part_kinds.entry(PART).or_default();
    let edge_kinds: BTreeSet<_> = std::iter::once(FASTENER)
        .chain(shipped.edge_kinds.iter().map(|kind| kind.id.as_str()))
        .chain(catalog.fasteners.iter().map(|kind| kind.id.as_str()))
        .chain(snapshot.fasteners.iter().filter_map(|edge| edge.fastener_kind.as_deref()))
        .collect();
    Manifest {
        node_kinds: part_kinds.into_iter().map(|(name, (catalog_kind, placed_part))| {
            let first_grip_kind = catalog_kind
                .and_then(|kind| kind.grips.first()).and_then(|grip| grip.grip_kind.as_deref())
                .or_else(|| placed_part
                    .and_then(|part| part.grips.first()).and_then(grip_kind))
                .unwrap_or(GRIP);
            NodeKindDef {
                name: name.into(),
                properties: properties(PART_PROPERTIES.iter().cloned()),
                port_kinds: vec![first_grip_kind.into()],
            }
        }).collect(),
        edge_kinds: edge_kinds.into_iter().map(|name| EdgeKindDef {
            name: name.into(),
            properties: properties(POSE.map(|key| (key, ValueType::Decimal))),
        }).collect(),
        port_kinds: grip_kinds.into_iter().flat_map(|name| {
            [PortDirection::Out, PortDirection::In].map(|direction| PortKindDef {
                name: name.into(), direction, properties: Vec::new(),
            })
        }).collect(),
    }
}

/// 🕸️ Maps placed parts to nodes, grips to ports, and fasteners to edges with the same directed endpoints.
pub fn graph_from_puzzle5d(snapshot: &Puzzle5dSnapshot, catalog: &Puzzle5dKindCatalogs) -> Result<Graph, Puzzle5dRewriteError> {
    let mut nodes = BTreeMap::new();
    for part in &snapshot.parts {
        valid_id(&part.id)?;
        let mut grip_ids = BTreeSet::new();
        for grip in &part.grips {
            valid_id(&grip.id)?;
            if !grip_ids.insert(&grip.id) {
                return Err(invalid(format!("Duplicate grip {}:{}", part.id, grip.id)));
            }
        }
        let mut properties: PropertyBag = field_bag(&part.part_2d).into_iter()
            .chain(field_bag(&part.part_3d)).collect();
        properties.remove("x");
        properties.remove("y");
        properties.insert("anchor".into(), PropertyValue::from_value(part.anchor.to_value()).map_err(|error| invalid(error.to_string()))?);
        let node = Node {
            id: part.id.clone(),
            kind: part.part_kind.clone().unwrap_or_else(|| PART.into()),
            name: part.part_2d.text.clone().unwrap_or_else(|| part.id.clone()),
            x: part.part_2d.x, y: part.part_2d.y,
            width: part.part_2d.width.unwrap_or_default(), height: part.part_2d.height.unwrap_or_default(),
            properties,
            ports: part.grips.iter().flat_map(grip_ports).collect(),
        };
        if nodes.insert(node.id.clone(), node).is_some() {
            return Err(invalid(format!("Duplicate part {}", part.id)));
        }
    }
    let mut edges = BTreeMap::new();
    for fastener in &snapshot.fasteners {
        valid_id(&fastener.id)?;
        let endpoint = |key: &str, incoming: bool| {
            let (part, grip) = puzzle_endpoint(key)?;
            let port = if incoming { format!("{grip}{IN_SUFFIX}") } else { grip.to_owned() };
            if !nodes.get(part).is_some_and(|node| node.ports.iter().any(|candidate| candidate.id == port)) {
                return Err(invalid(format!("Fastener {} references unknown grip {key}", fastener.id)));
            }
            Ok(port_key(part, &port))
        };
        let edge = Edge {
            id: fastener.id.clone(),
            kind: fastener.fastener_kind.clone().unwrap_or_else(|| FASTENER.into()),
            source: endpoint(&fastener.source, false)?,
            target: endpoint(&fastener.target, true)?,
            properties: POSE.into_iter().zip(pose(fastener))
                .map(|(key, value)| (key.into(), PropertyValue::Number(value))).collect(),
        };
        if edges.insert(edge.id.clone(), edge).is_some() {
            return Err(invalid(format!("Duplicate fastener {}", fastener.id)));
        }
    }
    Ok(Graph {
        name: snapshot.label.clone().unwrap_or_default(),
        manifest_id: None,
        manifest: node_manifest(snapshot, catalog),
        camera: Camera::default(),
        nodes,
        edges,
        root_node_id: None,
        query: String::new(),
    })
}

fn check_finite(value: &PropertyValue) -> bool {
    match value {
        PropertyValue::Number(number) => number.is_finite(),
        PropertyValue::Array(items) => items.iter().all(check_finite),
        PropertyValue::Object(items) => items.values().all(check_finite),
        _ => true,
    }
}

fn decode<T: FromValue>(key: &str, value: &PropertyValue) -> Result<T, Puzzle5dRewriteError> {
    if !check_finite(value) {
        return Err(invalid(format!("Puzzle property {key} requires finite values")));
    }
    T::from_value(value.to_value()).map_err(|error| invalid(format!("Invalid Puzzle property {key}: {error}")))
}

fn number(key: &str, value: &PropertyValue) -> Result<f64, Puzzle5dRewriteError> {
    value.as_f64().filter(|number| number.is_finite()).ok_or_else(|| invalid(format!("Puzzle property {key} requires a finite number")))
}

fn dimensions(id: String, view: &Puzzle5dPart2d) -> Puzzle5dMutation {
    mutations::replace_part_2d_geometry(id, view.shape.clone(), view.radius, view.width, view.height)
}

fn node_property(part: &Puzzle5dPart, key: &str, value: &PropertyValue) -> Result<Puzzle5dMutation, Puzzle5dRewriteError> {
    let id = part.id.clone();
    if part.part_2d.locked == Some(true) && key != "locked" {
        return Err(invalid(format!("Puzzle part {id} is locked")));
    }
    Ok(match key {
        "name" | "text" => mutations::edit_part_2d_text(id, decode(key, value)?),
        "iconKind" => mutations::change_part_2d_icon(id, decode(key, value)?),
        "hidden" => mutations::change_part_2d_hidden(id, decode(key, value)?),
        "locked" => mutations::change_part_2d_locked(id, decode(key, value)?),
        "label" => mutations::edit_part_3d_label(id, decode(key, value)?),
        "meshUrl" => mutations::change_part_3d_mesh(id, decode(key, value)?),
        "origin" => mutations::move_part_3d(id, decode(key, value)?),
        "orientation" => mutations::rotate_part_3d(id, decode(key, value)?),
        "scale" => mutations::scale_part_3d(id, decode::<Option<Puzzle5dScale>>(key, value)?),
        "anchor" => mutations::change_part_anchor(id, decode::<Puzzle5dPartAnchor>(key, value)?),
        "shape" | "radius" | "width" | "height" => {
            let mut view = part.part_2d.clone();
            match key {
                "shape" => view.shape = decode(key, value)?,
                "radius" => view.radius = decode(key, value)?,
                "width" => view.width = decode(key, value)?,
                _ => view.height = decode(key, value)?,
            }
            dimensions(id, &view)
        }
        _ => return Err(invalid(format!("Unsupported Puzzle part property {key}"))),
    })
}

fn fastener_pose(id: String, [gap, shift, rise, rotation, turn, tilt, x, y]: [f64; 8]) -> Puzzle5dMutation {
    mutations::replace_fastener_geometry(mutations::ReplaceFastenerGeometry {
        id, new_gap: gap, new_shift: shift, new_rise: rise, new_rotation: rotation,
        new_turn: turn, new_tilt: tilt, new_x: x, new_y: y,
    })
}

fn find_part<'a>(snapshot: &'a Puzzle5dSnapshot, id: &str) -> Result<&'a Puzzle5dPart, Puzzle5dRewriteError> {
    snapshot.parts.iter().find(|part| part.id == id).ok_or_else(|| invalid(format!("Unknown Puzzle part {id}")))
}

fn find_fastener<'a>(snapshot: &'a Puzzle5dSnapshot, id: &str) -> Result<&'a Puzzle5dFastener, Puzzle5dRewriteError> {
    snapshot.fasteners.iter().find(|fastener| fastener.id == id).ok_or_else(|| invalid(format!("Unknown Puzzle fastener {id}")))
}

fn property_mutation(snapshot: &Puzzle5dSnapshot, entity: &EntityRef, key: &str, value: &PropertyValue) -> Result<Puzzle5dMutation, Puzzle5dRewriteError> {
    match entity {
        EntityRef::Node(id) => node_property(find_part(snapshot, id)?, key, value),
        EntityRef::Edge(id) => {
            let index = POSE.iter().position(|field| *field == key).ok_or_else(|| invalid(format!("Unsupported Puzzle fastener property {key}")))?;
            let mut values = pose(find_fastener(snapshot, id)?);
            values[index] = number(key, value)?;
            Ok(fastener_pose(id.clone(), values))
        }
    }
}

fn create_part(snapshot: &Puzzle5dSnapshot, basis: &Puzzle5dSnapshot, catalog: &Puzzle5dKindCatalogs, node: &Node) -> Result<Puzzle5dMutation, Puzzle5dRewriteError> {
    valid_id(&node.id)?;
    if snapshot.parts.iter().any(|part| part.id == node.id) {
        return Err(invalid(format!("Duplicate Puzzle part {}", node.id)));
    }
    let kind = catalog.parts.iter().find(|kind| kind.id == node.kind);
    let prototype = basis.parts.iter().find(|part| part.part_kind.as_deref() == Some(node.kind.as_str()));
    if kind.is_some_and(|kind| kind.is_abstract) || (node.kind != PART && kind.is_none() && prototype.is_none()) {
        return Err(invalid(format!("Puzzle part kind {} is absent or abstract", node.kind)));
    }
    let templates = if let Some(kind) = kind.filter(|kind| !kind.grips.is_empty()) {
        kind.grips.iter().enumerate().map(|(index, template)| Puzzle5dGrip {
            id: if template.id.is_empty() { format!("v{index}") } else { template.id.clone() },
            grip_kind: template.grip_kind.clone(),
            grip_2d: Puzzle5dGrip2d { angle: template.t.unwrap_or_default(), radius: template.radius, ..Default::default() },
            grip_3d: Puzzle5dGrip3d {
                position: template.point, direction: Some(template.direction), radius: template.radius,
                label: (!template.label.is_empty()).then(|| template.label.clone()),
            },
        }).collect()
    } else {
        prototype.map(|part| part.grips.clone()).unwrap_or_default()
    };
    let mut consumed_templates = BTreeSet::new();
    let shipped = puzzle5d_default_manifest();
    let mut grip_ids = BTreeSet::new();
    let mut grips = Vec::with_capacity(node.ports.len().max(templates.len()));
    for port in &node.ports {
        valid_id(&port.id)?;
        if !grip_ids.insert(port.id.clone()) {
            return Err(invalid(format!("Duplicate grip {}:{}", node.id, port.id)));
        }
        let grip = if port.properties.is_empty() {
            let template = templates.iter().enumerate().find(|(index, template)| {
                !consumed_templates.contains(index) && grip_kind(template).unwrap_or(GRIP) == port.kind
            });
            if let Some((index, template)) = template {
                consumed_templates.insert(index);
                Puzzle5dGrip { id: port.id.clone(), ..template.clone() }
            } else {
                Puzzle5dGrip { id: port.id.clone(), grip_kind: (port.kind != GRIP).then(|| port.kind.clone()), grip_2d: Default::default(), grip_3d: Default::default() }
            }
        } else {
            let grip = Puzzle5dGrip::from_value(port.properties.to_value()).map_err(|error| invalid(error.to_string()))?;
            if grip.id != port.id || grip_kind(&grip).unwrap_or(GRIP) != port.kind
                || field_bag(&grip) != port.properties {
                return Err(invalid(format!("Puzzle grip {}:{} has conflicting graph fields", node.id, port.id)));
            }
            grip
        };
        grips.push(grip);
    }
    for (index, mut grip) in templates.into_iter().enumerate() {
        if consumed_templates.contains(&index) {
            continue;
        }
        valid_id(&grip.id)?;
        while !grip_ids.insert(grip.id.clone()) {
            grip.id.push('-');
        }
        grips.push(grip);
    }
    for kind in grips.iter().filter_map(grip_kind) {
        if shipped.port_kind(kind).is_none()
            && !catalog.grips.iter().any(|grip| grip.id == kind)
            && !basis.parts.iter().flat_map(|part| &part.grips).any(|grip| grip_kind(grip) == Some(kind))
        {
            return Err(invalid(format!("Unknown Puzzle grip kind {kind}")));
        }
    }
    for number in [node.x, node.y, node.width, node.height] {
        if !number.is_finite() {
            return Err(invalid(format!("Puzzle part {} requires finite coordinates and dimensions", node.id)));
        }
    }
    let mut part_2d = field_bag(&Puzzle5dPart2d {
        x: node.x, y: node.y,
        shape: prototype.and_then(|part| part.part_2d.shape.clone()),
        radius: prototype.and_then(|part| part.part_2d.radius),
        width: prototype.and_then(|part| part.part_2d.width).or(Some(node.width)),
        height: prototype.and_then(|part| part.part_2d.height).or(Some(node.height)),
        text: Some(node.name.clone()),
        icon_kind: prototype.and_then(|part| part.part_2d.icon_kind.clone()),
        ..Default::default()
    });
    let mesh_url = kind.and_then(|kind| kind.representations.iter().find(|representation| !representation.url.is_empty()))
        .map(|representation| representation.url.clone())
        .or_else(|| prototype.and_then(|part| part.part_3d.mesh_url.clone()));
    let origin = [node.x * mutations::PUZZLE5D_FLAT_TO_WORLD, -node.y * mutations::PUZZLE5D_FLAT_TO_WORLD, 0.0];
    let mut part_3d = field_bag(&Puzzle5dPart3d {
        origin, mesh_url, scale: prototype.and_then(|part| part.part_3d.scale),
        ..Default::default()
    });
    let mut anchor = Puzzle5dPartAnchor::default();
    for (key, value) in &node.properties {
        if !check_finite(value) {
            return Err(invalid(format!("Puzzle property {key} requires finite values")));
        }
        let projection = match key.as_str() {
            "anchor" => {
                anchor = decode(key, value)?;
                continue;
            }
            "origin" | "meshUrl" | "orientation" | "scale" | "label" => &mut part_3d,
            "shape" | "radius" | "width" | "height" | "text" | "iconKind" | "hidden" | "locked" => &mut part_2d,
            _ => return Err(invalid(format!("Unsupported Puzzle part property {key}"))),
        };
        projection.insert(key.clone(), value.clone());
    }
    let initial = Puzzle5dPart {
        id: node.id.clone(),
        part_kind: (node.kind != PART).then(|| node.kind.clone()),
        grips, anchor,
        part_2d: Puzzle5dPart2d::from_value(part_2d.to_value()).map_err(|error| invalid(error.to_string()))?,
        part_3d: Puzzle5dPart3d::from_value(part_3d.to_value()).map_err(|error| invalid(error.to_string()))?,
    };
    Ok(mutations::CreatePart { part: initial, index: None }.into())
}

fn endpoint_grip<'a>(snapshot: &'a Puzzle5dSnapshot, key: &str) -> Result<(&'a Puzzle5dPart, &'a Puzzle5dGrip), Puzzle5dRewriteError> {
    let (part_id, port_id) = parse_port_key(key).ok_or_else(|| invalid(format!("Puzzle fasteners require grip endpoints, got {key:?}")))?;
    let part = find_part(snapshot, part_id)?;
    let grip_id = port_id.strip_suffix(IN_SUFFIX).unwrap_or(port_id);
    let grip = part.grips.iter().find(|grip| grip.id == grip_id).ok_or_else(|| invalid(format!("Unknown Puzzle grip {key}")))?;
    Ok((part, grip))
}

fn create_fastener(snapshot: &Puzzle5dSnapshot, basis: &Puzzle5dSnapshot, catalog: &Puzzle5dKindCatalogs, edge: &Edge) -> Result<Puzzle5dMutation, Puzzle5dRewriteError> {
    valid_id(&edge.id)?;
    if snapshot.fasteners.iter().any(|candidate| candidate.id == edge.id) {
        return Err(invalid(format!("Duplicate Puzzle fastener {}", edge.id)));
    }
    let (source_part, source_grip) = endpoint_grip(snapshot, &edge.source)?;
    let (target_part, target_grip) = endpoint_grip(snapshot, &edge.target)?;
    let source = format!("{}:{}", source_part.id, source_grip.id);
    let target = format!("{}:{}", target_part.id, target_grip.id);
    if source == target {
        return Err(invalid("A Puzzle fastener requires two different grips"));
    }
    if snapshot.fasteners.iter().any(|candidate| {
        (candidate.source == source && candidate.target == target) || (candidate.source == target && candidate.target == source)
    }) {
        return Err(invalid(format!("Puzzle grips {source} and {target} are already connected")));
    }
    if let (Some(a), Some(b)) = (grip_kind(source_grip), grip_kind(target_grip)) {
        let allows = snapshot.kind_compatibility.is_empty() || snapshot.kind_compatibility.iter().any(|rule| {
            (rule.source == a && rule.target == b) || (rule.bidirectional && rule.source == b && rule.target == a)
        });
        if !allows {
            return Err(invalid(format!("Incompatible Puzzle grip kinds {a} and {b}")));
        }
    }
    let kind = (edge.kind != FASTENER).then(|| edge.kind.clone());
    if let Some(id) = &kind {
        if puzzle5d_default_manifest().edge_kind(id).is_none()
            && !catalog.fasteners.iter().any(|kind| kind.id == *id)
            && !basis.fasteners.iter().any(|fastener| fastener.fastener_kind.as_ref() == Some(id))
        {
            return Err(invalid(format!("Unknown Puzzle fastener kind {id}")));
        }
    }
    let mut values = [0.0; 8];
    for (key, value) in &edge.properties {
        let index = POSE.iter().position(|&field| field == key).ok_or_else(|| invalid(format!("Unsupported Puzzle fastener property {key}")))?;
        values[index] = number(key, value)?;
    }
    let [gap, shift, rise, rotation, turn, tilt, x, y] = values;
    Ok(mutations::connect_grips(edge.id.clone(), source, target, kind, gap, shift, rise, rotation, turn, tilt, x, y, None))
}

/// 🛠️ Admits the ordered effects against a private Puzzle projection before returning any mutations.
pub fn prepare_puzzle5d_effects(
    snapshot: &Puzzle5dSnapshot,
    catalog: &Puzzle5dKindCatalogs,
    effects: &[GraphEffect],
) -> Result<Vec<Puzzle5dMutation>, Puzzle5dRewriteError> {
    if effects.is_empty() {
        return Ok(Vec::new());
    }
    let mut working = snapshot.clone();
    let mut prepared = Vec::with_capacity(effects.len());
    let mut implicit_deletions = BTreeSet::new();
    for effect in effects {
        let mutation = match effect {
            GraphEffect::CreateNode(node) => create_part(&working, snapshot, catalog, node)?,
            GraphEffect::DeleteNode(id) => {
                let part = find_part(&working, id)?;
                if part.part_2d.locked == Some(true) {
                    return Err(invalid(format!("Puzzle part {id} is locked")));
                }
                for edge in &working.fasteners {
                    if [&edge.source, &edge.target].iter().any(|key| key.split_once(':').is_some_and(|(part, _)| part == id)) {
                        implicit_deletions.insert(edge.id.clone());
                    }
                }
                mutations::delete_part(id.clone())
            }
            GraphEffect::CreateEdge(edge) => {
                implicit_deletions.remove(&edge.id);
                create_fastener(&working, snapshot, catalog, edge)?
            }
            GraphEffect::DeleteEdge(id) => {
                if implicit_deletions.remove(id) {
                    continue;
                }
                find_fastener(&working, id)?;
                mutations::disconnect_grips(id.clone())
            }
            GraphEffect::RenameNode { id, name } => {
                node_property(find_part(&working, id)?, "name", &PropertyValue::String(name.clone()))?
            }
            GraphEffect::MoveNode { id, x, y } => {
                let part = find_part(&working, id)?;
                if part.part_2d.locked == Some(true) {
                    return Err(invalid(format!("Puzzle part {id} is locked")));
                }
                if !x.is_finite() || !y.is_finite() {
                    return Err(invalid(format!("Puzzle part {id} requires finite coordinates")));
                }
                mutations::move_part_2d(id.clone(), *x, *y)
            }
            GraphEffect::SetProperty { entity, key, value } => property_mutation(&working, entity, key, value)?,
            GraphEffect::RemoveProperty { entity, key } => property_mutation(&working, entity, key, &PropertyValue::Null)?,
        };
        let outcome = match mutation.diff(&working).admit() {
            MutationAdmission::Admitted(outcome) => outcome,
            MutationAdmission::Refused(outcome) => {
                let (diff, messages) = outcome.into_parts();
                MutationDiff::<Puzzle5dSnapshot>::retire_cold(diff);
                let reason = messages.into_iter().map(|message| message.message).collect::<Vec<_>>().join(", ");
                return Err(invalid(format!("Puzzle mutation refused: {reason}")));
            }
        };
        let (diff, _) = outcome.into_parts();
        if !DiffAlgebra::<Puzzle5dSnapshot>::is_empty(&diff) {
            let applied = apply_diff_cold_into(&diff, &mut working);
            MutationDiff::<Puzzle5dSnapshot>::retire_cold(diff);
            applied.map_err(|error| invalid(format!("Puzzle mutation refused: {error}")))?;
            prepared.push(mutation);
        } else {
            MutationDiff::<Puzzle5dSnapshot>::retire_cold(diff);
        }
    }
    Ok(prepared)
}

/// ♻️ Starts Jack rule execution over the Puzzle graph for a caller that drives it in turns.
pub(crate) fn start_puzzle5d_rewrite(
    snapshot: &Puzzle5dSnapshot,
    catalog: &Puzzle5dKindCatalogs,
    rule: &Rule,
    bindings: &PropertyBag,
) -> Result<QueryExecution, Puzzle5dRewriteError> {
    let query = compile_rule(rule, bindings).map_err(|error| invalid(error.to_string()))?;
    let graph = graph_from_puzzle5d(snapshot, catalog)?;
    Ok(QueryExecution::new(graph, query))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
