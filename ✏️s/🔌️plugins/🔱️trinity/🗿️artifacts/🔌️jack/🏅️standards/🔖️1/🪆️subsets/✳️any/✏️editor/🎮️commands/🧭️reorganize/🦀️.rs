//! 🗺️ 🗺️ Trinity Jack app command — `reorganize`.

use crate::standards::v1::subsets::any::schema::mutations::move_node;
use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::{JackSnapshot, Node};
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::Emit;

fn force_layout_nodes(fixture: &JackSnapshot) -> Option<Vec<Node>> {
    let scene = crate::jack_working_scene(fixture);
    if scene.nodes.is_empty() {
        return None;
    }
    let mut nodes = scene.nodes;
    use semio_framework_geometry::Vec2;
    use semio_framework_graph::drawing::force::{run_force_layout, ForceLayoutOptions};
    use std::collections::HashMap;
    let mut positions: Vec<Vec2> = nodes.iter().map(|node| Vec2::new(node.x, node.y)).collect();
    let radii: Vec<f64> = nodes.iter().map(|node| (node.width.max(48.0) + node.height.max(24.0)) * 0.25).collect();
    let id_to_index: HashMap<String, usize> = nodes.iter().enumerate().map(|(index, node)| (node.id.clone(), index)).collect();
    let mut edge_pairs = Vec::new();
    for edge in &scene.edges {
        let (source_node, _) = crate::editor::jack::split_endpoint(&edge.source);
        let (target_node, _) = crate::editor::jack::split_endpoint(&edge.target);
        if let (Some(a), Some(b)) = (id_to_index.get(&source_node), id_to_index.get(&target_node)) {
            edge_pairs.push((*a, *b));
        }
    }
    let pin = vec![None; positions.len()];
    run_force_layout(&mut positions, &radii, &edge_pairs, &pin, &ForceLayoutOptions { iterations: 120, ..ForceLayoutOptions::default() });
    for (index, node) in nodes.iter_mut().enumerate() {
        node.x = positions[index].x;
        node.y = positions[index].y;
    }
    Some(nodes)
}
fn reposition_operations(before: &[Node], after: &[Node]) -> Vec<TrinityGraphMutation> {
    after
        .iter()
        .filter_map(|node| {
            let prev = before.iter().find(|entry| entry.id == node.id)?;
            if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
                Some(move_node(node.id.clone(), node.x, node.y))
            } else {
                None
            }
        })
        .collect()
}

pub(crate) fn reorganize(fixture: &JackSnapshot) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    match force_layout_nodes(fixture) {
        Some(after) => Emit { artifact_mutations: reposition_operations(&fixture.nodes(), &after), ..Default::default() },
        None => Emit::default(),
    }
}
