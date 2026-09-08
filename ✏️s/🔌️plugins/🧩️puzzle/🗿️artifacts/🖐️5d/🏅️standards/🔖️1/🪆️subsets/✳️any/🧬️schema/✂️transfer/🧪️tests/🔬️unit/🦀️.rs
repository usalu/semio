
use super::*;
use crate::{Puzzle5dCatalogPartKind, Puzzle5dCompatSpecificity, Puzzle5dGrip, Puzzle5dGripTemplate, Puzzle5dKindCatalogs, Puzzle5dKindCompatibility, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dPartAnchor};

fn part_at(id: &str, x: f64, y: f64) -> Puzzle5dPart {
    Puzzle5dPart {
        id: id.to_string(),
        anchor: Puzzle5dPartAnchor::Fixed,
        part_kind: None,
        part_2d: Puzzle5dPart2d { x, y, ..Default::default() },
        part_3d: Puzzle5dPart3d { origin: [x, y, 0.0], ..Default::default() },
        grips: vec![Puzzle5dGrip { id: "g0".into(), grip_kind: Some("k".into()), grip_2d: Default::default(), grip_3d: Default::default() }],
    }
}

fn three_part_projection() -> Puzzle5dSnapshot {
    let mut projection = Puzzle5dSnapshot::default();
    projection.parts.push(part_at("p1", 0.0, 0.0));
    projection.parts.push(part_at("p2", 10.0, 0.0));
    projection.parts.push(part_at("p3", 20.0, 0.0));
    projection.fasteners.push(Puzzle5dFastener { id: "f1".into(), source: "p1:g0".into(), target: "p2:g0".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
    projection.fasteners.push(Puzzle5dFastener { id: "f2".into(), source: "p2:g0".into(), target: "p3:g0".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
    projection
}

#[test]
fn copy_selection_pulls_in_fastener_endpoints_and_internal_links() {
    let projection = three_part_projection();
    // Selecting only p1 and p2 (no fastener) should still close over f1 since both endpoints are selected.
    let (parts, fasteners) = copy_selection(&projection, &["p1".into(), "p2".into()], &[]);
    assert_eq!(parts.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), vec!["p1", "p2"]);
    assert_eq!(fasteners.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), vec!["f1"]);
}

#[test]
fn copy_selection_expands_parts_from_selected_fastener() {
    let projection = three_part_projection();
    // Selecting only fastener f2 should pull in its endpoint parts p2 and p3.
    let (parts, fasteners) = copy_selection(&projection, &[], &["f2".into()]);
    assert_eq!(parts.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), vec!["p2", "p3"]);
    assert_eq!(fasteners.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), vec!["f2"]);
}

#[test]
fn centroid_2d_averages_positions() {
    let parts = vec![part_at("a", 0.0, 0.0), part_at("b", 10.0, 0.0)];
    assert_eq!(centroid_2d(&parts), Some((5.0, 0.0)));
    assert_eq!(centroid_2d(&[]), None);
}

#[test]
fn paste_selection_mints_fresh_ids_shifts_positions_and_remaps_fastener_endpoints() {
    let projection = three_part_projection();
    let (fragment_parts, fragment_fasteners) = copy_selection(&projection, &["p1".into(), "p2".into()], &[]);
    let (fresh_parts, fresh_fasteners) = paste_selection(&projection, &fragment_parts, &fragment_fasteners, (100.0, 0.0));
    assert_eq!(fresh_parts.len(), 2);
    // Fresh ids must not collide with the existing p1/p2/p3.
    for part in &fresh_parts {
        assert!(!["p1", "p2", "p3"].contains(&part.id.as_str()));
    }
    assert_eq!(fresh_parts[0].part_2d.x, 100.0);
    assert_eq!(fresh_parts[1].part_2d.x, 110.0);
    assert_eq!(fresh_fasteners.len(), 1);
    let fresh_source_part = owning_part_id(&fresh_fasteners[0].source);
    let fresh_target_part = owning_part_id(&fresh_fasteners[0].target);
    assert_eq!(fresh_source_part, fresh_parts[0].id);
    assert_eq!(fresh_target_part, fresh_parts[1].id);
}

#[test]
fn translate_parts_shifts_selected_parts_only() {
    let projection = three_part_projection();
    let updated = translate_parts(&projection, &["p2".into()], (5.0, 5.0), [5.0, 5.0, 5.0]);
    assert_eq!(updated.len(), 1);
    let (index, part) = &updated[0];
    assert_eq!(*index, 1);
    assert_eq!(part.part_2d.x, 15.0);
    assert_eq!(part.part_2d.y, 5.0);
    assert_eq!(part.part_3d.origin, [15.0, 5.0, 5.0]);
}

#[test]
fn find_replaceable_kinds_walks_kind_compatibility() {
    let mut projection = three_part_projection();
    projection.parts[0].part_kind = Some("kind-a".into());
    projection.kind_compatibility.push(Puzzle5dKindCompatibility { source: "k".into(), target: "k2".into(), bidirectional: false, important: false, specificity: Puzzle5dCompatSpecificity::General });
    let (kind_catalogs, kind_catalogs_extra) = crate::split_and_seed_kind_catalogs(Some(Puzzle5dKindCatalogs {
        parts: vec![
            Puzzle5dCatalogPartKind { id: "kind-a".into(), name: "A".into(), label: "A".into(), grips: vec![], ..Default::default() },
            Puzzle5dCatalogPartKind { id: "kind-b".into(), name: "B".into(), label: "B".into(), grips: vec![Puzzle5dGripTemplate { grip_kind: Some("k2".into()), ..Default::default() }], ..Default::default() },
            Puzzle5dCatalogPartKind { id: "kind-c".into(), name: "C".into(), label: "C".into(), grips: vec![Puzzle5dGripTemplate { grip_kind: Some("unrelated".into()), ..Default::default() }], ..Default::default() },
        ],
        grips: vec![],
        fasteners: vec![],
        ropes: vec![],
    }));
    projection.kind_catalogs = kind_catalogs;
    projection.kind_catalogs_extra = kind_catalogs_extra;
    let replaceable = find_replaceable_kinds(&projection, "p1");
    assert_eq!(replaceable, vec!["kind-b".to_string()]);
}
