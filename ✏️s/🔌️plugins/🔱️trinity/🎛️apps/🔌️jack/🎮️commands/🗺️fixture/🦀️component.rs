//! 🗺️ Trinity Jack app — document-mutating fixture commands (`setFixtureJson`, `deleteSelection`,
//! `patchNodes`, `reorganize`) — dispatched as VCS operations with a true inverse.

use crate::apps::jack::config::JackConfigMutation;
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::JackSnapshot;
use semio_framework_plugin::{Emit, Fault};

/// 📦️ Re-runs force layout on the fixture, returning the repositioned fixture (or `None` if empty).
fn force_layout_fixture(fixture: &JackSnapshot) -> Option<JackSnapshot> {
    let mut fixture = fixture.clone();
    if fixture.nodes.is_empty() {
        return None;
    }
    use math::geometry::Vec2;
    use math::graph::drawing::force::{run_force_layout, ForceLayoutOptions};
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
fn reposition_operations(before: &JackSnapshot, after: &JackSnapshot) -> Vec<TrinityGraphMutation> {
    after
        .nodes
        .iter()
        .filter_map(|node| {
            let prev = before.nodes.iter().find(|entry| entry.id == node.id)?;
            if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
                Some(TrinityGraphMutation::Reposition { id: node.id.clone(), x: node.x, y: node.y })
            } else {
                None
            }
        })
        .collect()
}

pub(crate) fn set_fixture_json(json: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    match JackSnapshot::from_json(json) {
        Ok(next) => Ok(Emit::mutations(vec![TrinityGraphMutation::SetFixture { fixture: next }])),
        Err(_) => Ok(Emit::default()),
    }
}

pub(crate) fn delete_selection(fixture: &JackSnapshot, selected_node_ids: &[String]) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    let deletes: Vec<TrinityGraphMutation> = selected_node_ids.iter().filter(|id| fixture.nodes.iter().any(|node| &node.id == *id)).map(|id| TrinityGraphMutation::DeleteNode { id: id.clone() }).collect();
    if deletes.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit { artifact_mutations: deletes, config_mutations: vec![JackConfigMutation::SetSelection { node_ids: Vec::new() }], ..Default::default() })
    }
}

pub(crate) fn patch_nodes(fixture: &JackSnapshot, node_ids: &[String], field: &str, value: &str) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    if field == "name" && !node_ids.is_empty() && !value.trim().is_empty() {
        let operations: Vec<TrinityGraphMutation> = node_ids.iter().filter(|id| fixture.nodes.iter().any(|node| &node.id == *id)).map(|id| TrinityGraphMutation::Rename { id: id.clone(), name: value.trim().into() }).collect();
        Ok(Emit::mutations(operations))
    } else {
        Ok(Emit::default())
    }
}

pub(crate) fn reorganize(fixture: &JackSnapshot, reorganize_epoch: u64) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    let config_mutations = vec![JackConfigMutation::SetReorganizeEpoch { value: reorganize_epoch + 1 }];
    match force_layout_fixture(fixture) {
        Some(after) => Ok(Emit { artifact_mutations: reposition_operations(fixture, &after), config_mutations, ..Default::default() }),
        None => Ok(Emit::config(config_mutations)),
    }
}
