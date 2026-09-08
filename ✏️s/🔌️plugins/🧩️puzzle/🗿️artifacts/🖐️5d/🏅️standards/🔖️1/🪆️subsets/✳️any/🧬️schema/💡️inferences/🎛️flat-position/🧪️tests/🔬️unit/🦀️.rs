
use super::*;
use crate::{Puzzle5dFastener, Puzzle5dGrip, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dMeta, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dPartAnchor, Puzzle5dSnapshot};

#[test]
fn flatten_writes_diagram_offsets_onto_part_2d() {
    let mut snapshot = Puzzle5dSnapshot {
        schema: "puzzle.5d".into(),
        domain: "architecture".into(),
        label: None,
        meta: Puzzle5dMeta { description: String::new() },
        kind_catalogs: None,
        kind_catalogs_extra: None,
        kind_compatibility: Vec::new(),
        parts: vec![
            Puzzle5dPart {
                id: "p".into(),
                part_kind: None,
                anchor: Puzzle5dPartAnchor::Fixed,
                part_2d: Puzzle5dPart2d { x: 10.0, y: 20.0, shape: None, radius: None, width: None, height: None, text: None, icon_kind: None, hidden: None, locked: None },
                part_3d: Puzzle5dPart3d { origin: [0.0, 0.0, 0.0], mesh_url: None, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
                grips: vec![Puzzle5dGrip {
                    id: "top".into(),
                    grip_kind: None,
                    grip_2d: Puzzle5dGrip2d { angle: 0.0, grip_kind: None, radius: None },
                    grip_3d: Puzzle5dGrip3d { position: [0.0, 0.0, 1.0], direction: Some([0.0, 0.0, 1.0]), radius: None, label: None },
                }],
            },
            Puzzle5dPart {
                id: "c".into(),
                part_kind: None,
                anchor: Puzzle5dPartAnchor::Derived,
                part_2d: Puzzle5dPart2d { x: 0.0, y: 0.0, shape: None, radius: None, width: None, height: None, text: None, icon_kind: None, hidden: None, locked: None },
                part_3d: Puzzle5dPart3d { origin: [0.0, 0.0, 0.0], mesh_url: None, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
                grips: vec![Puzzle5dGrip {
                    id: "bottom".into(),
                    grip_kind: None,
                    grip_2d: Puzzle5dGrip2d { angle: 0.0, grip_kind: None, radius: None },
                    grip_3d: Puzzle5dGrip3d { position: [0.0, 0.0, -1.0], direction: Some([0.0, 0.0, -1.0]), radius: None, label: None },
                }],
            },
        ],
        fasteners: vec![Puzzle5dFastener { id: "f".into(), source: "p:top".into(), target: "c:bottom".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 1.5, y: 2.5 }],
    };
    flatten_snapshot_inplace(&mut snapshot);
    let child = snapshot.parts.iter().find(|part| part.id == "c").expect("c");
    assert_eq!(child.part_2d.x, 10.0 + 1.5);
    assert_eq!(child.part_2d.y, 20.0 + 2.5 + DIAGRAM_VERTICAL_V_EXTRA);
}
