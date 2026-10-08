//! 🧪️ Every details-pane pointer of the GIF89a editor resolves to the concrete kinds of its gesture, and replaying them reaches the edited document.

use super::*;
use crate::standards::v89a::subsets::any::schema::snapshot::{GifColorTable, GifDisposal, GifRgb};

fn frame(delay_cs: u16) -> GifFrame {
    GifFrame {
        left: 0,
        top: 0,
        width: 2,
        height: 2,
        interlace: false,
        lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 9, g: 9, b: 9 }; 2] }),
        indices: vec![0, 1, 1, 0],
        delay_cs,
        disposal: GifDisposal::DoNotDispose,
        transparent_index: None,
        user_input: false,
        plain_text: None,
    }
}

fn base() -> GifSnapshot {
    GifSnapshot {
        width: 4,
        height: 4,
        gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 4, g: 5, b: 6 }; 2] }),
        loop_count: Some(0),
        frames: vec![frame(10), frame(20), frame(30)],
        comments: vec!["a".into(), "b".into(), "c".into()],
        ..Default::default()
    }
}

fn number(value: u64) -> DslValue {
    DslValue::uint(value)
}

fn text(value: &str) -> DslValue {
    DslValue::String(value.to_string())
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
fn a_frame_field_resolves_to_its_own_kind_addressed_by_position() {
    let mutations = resolve(SnapshotEditEvent::SetValue { path: "/frames/1/delayCs".into(), value: number(77) });
    assert_eq!(mutations, vec![GifMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: 1, delay_cs: 77 })]);
    assert_eq!(replay(&mutations).frames[1].delay_cs, 77);
}

#[test]
fn a_screen_dimension_carries_its_sibling_from_the_document() {
    let mutations = resolve(SnapshotEditEvent::SetValue { path: "/width".into(), value: number(9) });
    assert_eq!(mutations, vec![GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 9, height: 4 })]);
}

#[test]
fn a_frame_geometry_field_raises_the_geometry_kind_with_all_four_values() {
    let mutations = resolve(SnapshotEditEvent::SetValue { path: "/frames/2/left".into(), value: number(1) });
    assert_eq!(mutations, vec![GifMutation::SetFrameGeometry(set_frame_geometry::SetFrameGeometry { index: 2, left: 1, top: 0, width: 2, height: 2 })]);
}

#[test]
fn a_frame_colour_table_edit_replaces_the_frame_in_place() {
    let mutations = resolve(SnapshotEditEvent::SetValue { path: "/frames/1/lct/colors/0/r".into(), value: number(200) });
    assert_eq!(mutations.len(), 2);
    let edited = replay(&mutations);
    assert_eq!(edited.frames[1].lct.as_ref().unwrap().colors[0].r, 200);
    assert_eq!((edited.frames[0].clone(), edited.frames[2].clone()), (frame(10), frame(30)));
}

#[test]
fn rows_insert_remove_and_move_in_place() {
    let inserted = resolve(SnapshotEditEvent::InsertValue { path: "/comments/1".into(), value: text("x") });
    assert_eq!(replay(&inserted).comments, vec!["a", "x", "b", "c"]);
    let removed = resolve(SnapshotEditEvent::RemoveValue { path: "/frames/1".into() });
    assert_eq!(replay(&removed).frames, vec![frame(10), frame(30)]);
    let moved = resolve(SnapshotEditEvent::MoveValue { from: "/comments/0".into(), path: "/comments/2".into() });
    assert_eq!(replay(&moved).comments, vec!["b", "c", "a"]);
    let reordered = resolve(SnapshotEditEvent::MoveValue { from: "/frames/0".into(), path: "/frames/2".into() });
    assert_eq!(replay(&reordered).frames, vec![frame(20), frame(30), frame(10)]);
}

#[test]
fn a_comment_text_edit_replaces_the_row_and_an_unchanged_one_publishes_nothing() {
    let mutations = resolve(SnapshotEditEvent::SetValue { path: "/comments/1".into(), value: text("B") });
    assert_eq!(replay(&mutations).comments, vec!["a", "B", "c"]);
    assert!(resolve(SnapshotEditEvent::SetValue { path: "/comments/1".into(), value: text("b") }).is_empty());
}

#[test]
fn an_unnamed_pointer_is_refused() {
    let event = SnapshotEditEvent::SetValue { path: "/schema".into(), value: text("other") };
    assert_eq!(EDIT_RULES.resolve::<GifSnapshot, GifMutation>(&base(), &event).unwrap_err().code, "snapshot-edit.unsupported-path");
}
