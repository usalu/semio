//! 🧪️ Every details-pane pointer of the GIF87a editor resolves to the concrete kinds of its gesture, and replaying them reaches the edited document.

use super::*;

fn image(left: u32) -> GifImage {
    GifImage { left, top: 0, width: 2, height: 2, interlace: false, lct: None, indices: vec![0, 1, 1, 0] }
}

fn base() -> GifSnapshot {
    GifSnapshot { width: 8, height: 8, images: vec![image(0), image(1), image(2)], ..Default::default() }
}

fn resolve(event: SnapshotEditEvent) -> Vec<GifMutation> {
    let snapshot = base();
    match special(&event, &snapshot).unwrap() {
        Some(mutations) => mutations,
        None => EDIT_RULES.resolve::<GifSnapshot, GifMutation>(&snapshot, &event).unwrap(),
    }
}

fn replay(mutations: &[GifMutation]) -> GifSnapshot {
    let mut snapshot = base();
    for mutation in mutations {
        assert!(apply_gif_mutation(&mut snapshot, mutation).messages().is_empty());
    }
    snapshot
}

#[test]
fn an_image_field_resolves_to_its_own_kind_addressed_by_position() {
    let mutations = resolve(SnapshotEditEvent::SetValue { path: "/images/1/interlace".into(), value: DslValue::Bool(true) });
    assert_eq!(mutations, vec![GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index: 1, interlace: true })]);
    assert!(replay(&mutations).images[1].interlace);
}

#[test]
fn a_geometry_field_carries_the_other_three_and_a_screen_dimension_its_sibling() {
    let geometry = resolve(SnapshotEditEvent::SetValue { path: "/images/2/left".into(), value: DslValue::uint(4) });
    assert_eq!(geometry, vec![GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: 2, left: 4, top: 0, width: 2, height: 2 })]);
    let screen = resolve(SnapshotEditEvent::SetValue { path: "/height".into(), value: DslValue::uint(9) });
    assert_eq!(screen, vec![GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 8, height: 9 })]);
}

#[test]
fn images_insert_remove_and_move_in_place() {
    let removed = resolve(SnapshotEditEvent::RemoveValue { path: "/images/1".into() });
    assert_eq!(replay(&removed).images, vec![image(0), image(2)]);
    let moved = resolve(SnapshotEditEvent::MoveValue { from: "/images/0".into(), path: "/images/2".into() });
    assert_eq!(replay(&moved).images, vec![image(1), image(2), image(0)]);
}

#[test]
fn an_unnamed_pointer_is_refused() {
    let event = SnapshotEditEvent::SetValue { path: "/schema".into(), value: DslValue::String("other".into()) };
    assert_eq!(EDIT_RULES.resolve::<GifSnapshot, GifMutation>(&base(), &event).unwrap_err().code, "snapshot-edit.unsupported-path");
}
