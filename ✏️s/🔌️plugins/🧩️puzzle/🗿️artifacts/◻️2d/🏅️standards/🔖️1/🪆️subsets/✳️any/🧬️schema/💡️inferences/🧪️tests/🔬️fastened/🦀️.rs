use super::*;
use crate::{Puzzle2dCamera, Puzzle2dEdge, Puzzle2dMeta, Puzzle2dNode, Puzzle2dNodeAnchor, Puzzle2dSnapshot};

#[test]
fn fastened_layout_places_child_from_origin_parent_by_handle_angle() {
    let mut snapshot = Puzzle2dSnapshot {
        schema: "puzzle.2d".into(),
        camera: Puzzle2dCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            Puzzle2dNode {
                id: "p".into(),
                node_kind: None,
                shape: None,
                x: 0.0,
                y: 0.0,
                radius: None,
                width: None,
                height: None,
                text: None,
                icon_kind: None,
                root: None,
                scale: None,
                visible: None,
                locked: None,
                anchor: Puzzle2dNodeAnchor::Fixed,
                handles: vec![crate::Puzzle2dHandle { id: "h".into(), handle_kind: None, angle: 0.0, radius: None, color: None, icon_kind: None, scale: None, visible: None, locked: None }],
            },
            Puzzle2dNode {
                id: "c".into(),
                node_kind: None,
                shape: None,
                x: 0.0,
                y: 0.0,
                radius: None,
                width: None,
                height: None,
                text: None,
                icon_kind: None,
                root: None,
                scale: None,
                visible: None,
                locked: None,
                anchor: Puzzle2dNodeAnchor::Derived,
                handles: vec![crate::Puzzle2dHandle { id: "h".into(), handle_kind: None, angle: 0.0, radius: None, color: None, icon_kind: None, scale: None, visible: None, locked: None }],
            },
        ],
        edges: vec![Puzzle2dEdge {
            id: "e".into(),
            source: "p:h".into(),
            target: "c:h".into(),
            edge_kind: None,
            source_tip: None,
            target_tip: None,
            visible: None,
            locked: None,
            gap: 0.0,
            shift: 0.0,
            rise: 0.0,
            rotation: 0.0,
            turn: 0.0,
            tilt: 0.0,
            x: 0.0,
            y: 0.0,
        }],
        meta: Puzzle2dMeta::default(),
    };
    fastened_layout_snapshot(&mut snapshot);
    let child = snapshot.nodes.iter().find(|node| node.id == "c").expect("c");
    assert_eq!(child.x, 0.0);
    assert_eq!(child.y, DIAGRAM_RADIUS);
}
