use super::*;

#[test]
fn multiline_editor_matches_neutral_vectors_with_bounded_utf8_history() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(MAXIMUM_BYTES as u64, fixture["maximumBytes"].as_u64().unwrap());
    assert_eq!(HISTORY_BYTES as u64, fixture["historyBytes"].as_u64().unwrap());
    for row in fixture["cases"].as_array().unwrap() {
        let mut editor = EditorState::new(row["initial"].as_str().unwrap(), row["limit"].as_u64().unwrap_or(1_048_576) as usize);
        editor.read_only = row["readOnly"].as_bool().unwrap_or(false);
        editor.cursor = row["cursor"].as_u64().unwrap() as usize;
        for command in row["commands"].as_array().unwrap() {
            let op = command["op"].as_str().unwrap();
            if op == "insert" { editor.insert(command["text"].as_str().unwrap()); } else { editor.command(op); }
            assert_eq!(boundary_at_or_before(&editor.value, editor.cursor), editor.cursor, "grapheme boundary: {}", row["name"]);
            assert!(editor.value.is_char_boundary(editor.cursor), "{}", row["name"]);
            assert!(editor.value.len() <= editor.maximum_bytes);
            assert!(editor.history_bytes <= HISTORY_BYTES);
        }
        assert_eq!(editor.value, row["expected"].as_str().unwrap(), "{}", row["name"]);
        assert_eq!(editor.cursor, row["expectedCursor"].as_u64().unwrap() as usize, "{}", row["name"]);
    }
    println!("[DEBUG] multiline editor neutral cases=13 UTF8=true boundedHistory=true");
}

#[test]
fn multiline_editor_cursor_and_pointer_follow_physical_cells() {
    let mut editor = EditorState::new("a中\nx\ty", 100);
    editor.cursor = "a中\nx\t".len();
    let rect = Rect::new(3, 5, 20, 4);
    assert_eq!(editor_cursor(&editor, rect).unwrap().pos, Pos { x: 7, y: 6 });
    editor_on_mouse(&mut editor, rect, &MouseEvent { kind: MouseKind::Down(MouseButton::Left), pos: Pos { x: 6, y: 5 }, mods: 0, clicks: 1 });
    assert_eq!(editor.cursor, "a中".len());
}
