//! 🗺️ Trinity Jack app — document-mutating fixture commands (`setFixtureJson`, `deleteSelection`,
//! `patchNodes`, `reorganize`) — dispatched as VCS operations with a true inverse.

use crate::apps::jack::config::JackConfigOperation;
use crate::artifacts::jack::op::TrinityGraphOperation;
use crate::artifacts::jack::GraphFixture;
use semio_framework_plugin::{Emit, Fault};

/// 📦️ Re-runs force layout on the fixture, returning the repositioned fixture (or `None` if empty).
fn force_layout_fixture(fixture: &GraphFixture) -> Option<GraphFixture> {
    let mut fixture = fixture.clone();
    if fixture.nodes.is_empty() {
        return None;
    }
    use mathematical_geometry::Vec2;
    use mathematical_graph_drawing::force::{run_force_layout, ForceLayoutOptions};
    use std::collections::HashMap;
    let mut positions: Vec<Vec2> = fixture.nodes.iter().map(|node| Vec2::new(node.x, node.y)).collect();
    let radii: Vec<f64> = fixture.nodes.iter().map(|node| (node.width.max(48.0) + node.height.max(24.0)) * 0.25).collect();
    let id_to_index: HashMap<String, usize> = fixture.nodes.iter().enumerate().map(|(index, node)| (node.id.clone(), index)).collect();
    let mut edge_pairs = Vec::new();
    for edge in &fixture.edges {
        let (source_node, _) = crate::apps::jack::split_endpoint(&edge.source);
        let (target_node, _) = crate::apps::jack::split_endpoint(&edge.target);
        if let (Some(a), Some(b)) = (id_to_index.get(&source_node), id_to_index.get(&target_node)) {
            edge_pairs.push((*a, *b));
        }
    }
    let pin = vec![None; positions.len()];
    run_force_layout(&mut positions, &radii, &edge_pairs, &pin, &ForceLayoutOptions { iterations: 120, ..ForceLayoutOptions::default() });
    for (index, node) in fixture.nodes.iter_mut().enumerate() {
        node.x = positions[index].x;
        node.y = positions[index].y;
    }
    Some(fixture)
}

/// 🧭️ Emits a `Reposition` operation for every node whose position differs between `before` and `after`.
fn reposition_operations(before: &GraphFixture, after: &GraphFixture) -> Vec<TrinityGraphOperation> {
    after
        .nodes
        .iter()
        .filter_map(|node| {
            let prev = before.nodes.iter().find(|entry| entry.id == node.id)?;
            if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
                Some(TrinityGraphOperation::Reposition { id: node.id.clone(), x: node.x, y: node.y })
            } else {
                None
            }
        })
        .collect()
}

pub(crate) fn set_fixture_json(json: &str) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    match GraphFixture::from_json(json) {
        Ok(next) => Ok(Emit::operations(vec![TrinityGraphOperation::SetFixture { fixture: next }])),
        Err(_) => Ok(Emit::default()),
    }
}

pub(crate) fn delete_selection(fixture: &GraphFixture, selected_node_ids: &[String]) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    let deletes: Vec<TrinityGraphOperation> = selected_node_ids.iter().filter(|id| fixture.nodes.iter().any(|node| &node.id == *id)).map(|id| TrinityGraphOperation::DeleteNode { id: id.clone() }).collect();
    if deletes.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit { document_operations: deletes, config_operations: vec![JackConfigOperation::SetSelection { node_ids: Vec::new() }], ..Default::default() })
    }
}

pub(crate) fn patch_nodes(fixture: &GraphFixture, node_ids: &[String], field: &str, value: &str) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    if field == "name" && !node_ids.is_empty() && !value.trim().is_empty() {
        let operations: Vec<TrinityGraphOperation> = node_ids.iter().filter(|id| fixture.nodes.iter().any(|node| &node.id == *id)).map(|id| TrinityGraphOperation::Rename { id: id.clone(), name: value.trim().into() }).collect();
        Ok(Emit::operations(operations))
    } else {
        Ok(Emit::default())
    }
}

pub(crate) fn reorganize(fixture: &GraphFixture, reorganize_epoch: u64) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
    let config_operations = vec![JackConfigOperation::SetReorganizeEpoch { value: reorganize_epoch + 1 }];
    match force_layout_fixture(fixture) {
        Some(after) => Ok(Emit { document_operations: reposition_operations(fixture, &after), config_operations, ..Default::default() }),
        None => Ok(Emit::config(config_operations)),
    }
}
