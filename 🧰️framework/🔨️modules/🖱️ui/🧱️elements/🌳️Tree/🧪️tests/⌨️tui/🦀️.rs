//! 🌳️ Native Tree integration laws share the semantic fixture and independent Skia color oracle.

use super::*;
use semio_framework_ui_tui_core::cell::attr;
use semio_framework_ui_tui_core::event::{MouseButton, MouseEvent, MouseKind};
use semio_framework_ui_tui_core::geometry::Size;
use semio_framework_ui_tui_core::theme::Rgb;
use semio_framework_pack_json::{parse, JsonMemberPolicy, Value};

fn click(x:u16,y:u16,clicks:u8,button:MouseButton)->MouseEvent { MouseEvent {kind:MouseKind::Down(button),pos:Pos{x,y},mods:0,clicks} }

fn tree_fixture() -> Value {
    parse(include_str!("../../../../⌨️tui/🧪️tests/🧫️fixtures/🌳️tree-emphasis/🔣️.json"), JsonMemberPolicy::Reject).expect("tree emphasis fixture")
}

fn fixture_tree(root: &Value) -> crate::tree::TreeState {
    let object = root.as_object().unwrap();
    crate::tree::TreeState::new(object.get("items").and_then(Value::as_array).unwrap().iter().enumerate().map(|(id, item)| {
        let item = item.as_array().unwrap();
        crate::tree::TreeItem::new(id.to_string(), item[0].as_str().unwrap(), item[1].as_u64().unwrap() as u16)
    }).collect())
}

fn fixture_color(theme: &Theme, name: &str) -> Rgb {
    match name {
        "foreground" => theme.role(Role::Foreground),
        "muted" => theme.role(Role::MutedForeground),
        "emphasized" => theme.role(Role::BorderEmphasized),
        "normal" => theme.role(Role::BorderNormal),
        "active" => theme.role(Role::ActiveForeground),
        _ => panic!("unknown color {name}"),
    }
}

fn oracle_cell(pixmap: &mut tiny_skia::Pixmap, x: u16, y: u16, color: Rgb) {
    let mut paint = tiny_skia::Paint::default();
    paint.set_color_rgba8(color[0], color[1], color[2], 255);
    paint.anti_alias = false;
    pixmap.fill_rect(tiny_skia::Rect::from_xywh(f32::from(x), f32::from(y), 1.0, 1.0).unwrap(), &paint, tiny_skia::Transform::identity(), None);
}

#[test]
fn tree_emphasis_fixture_matches_semantic_cells_and_independent_skia_color_planes() {
    let root = tree_fixture();
    assert_eq!(root.as_object().unwrap().get("emphasisAttribute").and_then(Value::as_str), Some("bold"));
    for appearance in [ui_styling::appearance::AppearanceName::Dark, ui_styling::appearance::AppearanceName::Light] {
        for glyphs in [semio_framework_ui_tui_core::theme::GlyphSet::Unicode, semio_framework_ui_tui_core::theme::GlyphSet::Ascii] {
            let mut theme = Theme::new(appearance);
            theme.set_glyphs(glyphs);
            for case in root.as_object().unwrap().get("cases").and_then(Value::as_array).unwrap() {
                let case = case.as_object().unwrap();
                let name = case.get("name").and_then(Value::as_str).unwrap();
                let mut tree = fixture_tree(&root);
                tree.filterable = false;
                for item in case.get("expanded").and_then(Value::as_array).unwrap() {
                    tree.set_expanded(item.as_u64().unwrap() as usize, true);
                }
                tree.set_query(case.get("query").and_then(Value::as_str).unwrap());
                tree.select_item(case.get("selected").and_then(Value::as_u64).unwrap() as usize);
                tree.view.set_hover(case.get("hover").and_then(Value::as_u64).and_then(|item| tree.position_of(item as usize)));
                let expected = case.get("rows").and_then(Value::as_array).unwrap();
                let rect = Rect::new(0, 0, 20, expected.len() as u16);
                let mut buf = CellBuffer::new(Size { width: rect.width, height: rect.height }, Cell::blank([0; 3], [0; 3]));
                crate::tree::paint_tree(&tree, &theme, rect, &mut buf, true);
                let mut foreground = tiny_skia::Pixmap::new(u32::from(rect.width), u32::from(rect.height)).unwrap();
                let mut background = foreground.clone();
                assert_eq!(tree.count(), expected.len(), "{name}");
                for (y, row) in expected.iter().enumerate() {
                    let row = row.as_object().unwrap();
                    let item = row.get("item").and_then(Value::as_u64).unwrap() as usize;
                    assert_eq!(tree.row_item(y), Some(item), "{name}");
                    let fg = fixture_color(&theme, row.get("label").and_then(Value::as_str).unwrap());
                    let bg = match row.get("background").and_then(Value::as_str).unwrap() {
                        "active" => theme.role(Role::ActiveBase),
                        "hover" => theme.role(Role::HoverInteractive),
                        _ => theme.surface(semio_framework_ui_tui_core::theme::Surface::Window),
                    };
                    let guides = row.get("guides").and_then(Value::as_array).unwrap();
                    let indent = tree.item(item).unwrap().depth * 2;
                    let marker = if !tree.is_folder(item) { ' ' } else if tree.is_expanded(item) || !tree.query().is_empty() { theme.glyphs.glyph(Glyph::Expanded).chars().next().unwrap() } else { theme.glyphs.glyph(Glyph::Collapsed).chars().next().unwrap() };
                    assert_eq!(buf.get(indent, y as u16).unwrap().ch, marker, "{name} marker {item}");
                    if row.get("label").and_then(Value::as_str) == Some("emphasized") {
                        assert_ne!(buf.get(indent, y as u16).unwrap().attrs & attr::BOLD, 0, "{name} emphasized marker {item}");
                        assert_ne!(buf.get(indent + 2, y as u16).unwrap().attrs & attr::BOLD, 0, "{name} emphasized label {item}");
                    }
                    for x in 0..rect.width {
                        let color = if x < indent && x % 2 == 0 {
                            assert_eq!(buf.get(x, y as u16).unwrap().ch, theme.glyphs.glyph(Glyph::ScrollTrack).chars().next().unwrap(), "{name} guide {item}:{x}");
                            fixture_color(&theme, guides[usize::from(x / 2)].as_str().unwrap())
                        } else { fg };
                        oracle_cell(&mut foreground, x, y as u16, color);
                        oracle_cell(&mut background, x, y as u16, bg);
                    }
                }
                for y in 0..rect.height {
                    for x in 0..rect.width {
                        let cell = buf.get(x, y).unwrap();
                        let at = (usize::from(y) * usize::from(rect.width) + usize::from(x)) * 4;
                        assert_eq!(cell.fg, foreground.data()[at..at + 3], "{name} foreground {x}:{y}");
                        assert_eq!(cell.bg, background.data()[at..at + 3], "{name} background {x}:{y}");
                    }
                }
            }
        }
    }
}

#[test]
fn tree_rebuild_clears_hover_identity_before_positions_can_point_to_siblings() {
    let root = tree_fixture();
    let mut tree = fixture_tree(&root);
    tree.expand_to_depth(3);
    tree.view.set_hover(tree.position_of(2));
    tree.set_expanded(1, false);
    assert_eq!(tree.view.hover(), None);
    tree.view.set_hover(tree.position_of(4));
    tree.set_query("OtherLeaf");
    assert_eq!(tree.view.hover(), None);
}

#[test]
fn tree_clips_hover_guides_and_retains_keyboard_and_unfocused_selection() {
    let root = tree_fixture();
    let mut tree = fixture_tree(&root);
    tree.filterable = false;
    tree.expand_to_depth(3);
    tree.select_item(2);
    tree.view.set_hover(tree.position_of(2));
    let theme = Theme::new(ui_styling::appearance::AppearanceName::Dark);
    let sentinel = Cell::blank([11, 22, 33], [44, 55, 66]);
    let mut buf = CellBuffer::new(Size { width: 8, height: 5 }, sentinel);
    let rect = Rect::new(2, 1, 3, 2);
    crate::tree::paint_tree(&tree, &theme, rect, &mut buf, false);
    assert_eq!(tree.view.top(), 1);
    assert_eq!(buf.get(2, 2).unwrap().fg, theme.role(Role::BorderEmphasized));
    for y in 0..buf.size.height {
        for x in 0..buf.size.width {
            if !rect.contains(Pos { x, y }) { assert_eq!(*buf.get(x, y).unwrap(), sentinel); }
        }
    }
    let wide = Rect::new(0, 0, 20, 8);
    let mut buf = CellBuffer::new(Size { width: 20, height: 8 }, sentinel);
    crate::tree::paint_tree(&tree, &theme, wide, &mut buf, false);
    let label = buf.get(6, 2).unwrap();
    assert_eq!((label.fg, label.bg, label.attrs), (theme.role(Role::Foreground), theme.surface(semio_framework_ui_tui_core::theme::Surface::Window), attr::BOLD));
    assert_eq!(crate::tree::tree_on_key(&mut tree, &semio_framework_ui_tui_core::event::KeyEvent { key: Key::Left, mods: 0 }), Some(WidgetSignal::SelectionChanged(1)));
    assert_eq!(crate::tree::tree_on_key(&mut tree, &semio_framework_ui_tui_core::event::KeyEvent { key: Key::Right, mods: 0 }), Some(WidgetSignal::SelectionChanged(2)));
}

#[test]
fn tree_hover_clears_over_filter_empty_space_and_scrollbar_and_updates_between_branches() {
    let root = tree_fixture();
    let mut tree = fixture_tree(&root);
    tree.expand_to_depth(3);
    let theme = Theme::new(ui_styling::appearance::AppearanceName::Dark);
    let rect = Rect::new(0, 0, 20, 9);
    let mut buf = CellBuffer::new(Size { width: 20, height: 9 }, Cell::blank([0; 3], [0; 3]));
    crate::tree::paint_tree(&tree, &theme, rect, &mut buf, true);
    assert!(crate::tree::tree_set_hover(&mut tree, rect, Some(Pos { x: 6, y: 3 })));
    assert_eq!(tree.view.hover(), Some(2));
    assert!(crate::tree::tree_set_hover(&mut tree, rect, Some(Pos { x: 6, y: 8 })));
    crate::tree::paint_tree(&tree, &theme, rect, &mut buf, true);
    assert_eq!(buf.get(4, 2).unwrap().fg, theme.role(Role::Foreground));
    assert_eq!(buf.get(2, 7).unwrap().fg, theme.role(Role::BorderEmphasized));
    assert!(crate::tree::tree_set_hover(&mut tree, rect, Some(Pos { x: 0, y: 0 })));
    assert_eq!(tree.view.hover(), None);
    tree.set_expanded(0, false);
    assert!(!crate::tree::tree_set_hover(&mut tree, rect, Some(Pos { x: 6, y: 8 })));
    assert_eq!(tree.view.hover(), None);
    let short = Rect::new(0, 0, 20, 2);
    crate::tree::paint_tree(&tree, &theme, short, &mut buf, true);
    tree.view.set_hover(Some(0));
    assert!(crate::tree::tree_set_hover(&mut tree, short, Some(Pos { x: 19, y: 1 })));
    assert_eq!(tree.view.hover(), None);
}

#[test]
fn tree_filtered_ancestor_selection_keeps_its_foreground_without_hover() {
    let root = tree_fixture();
    let mut tree = fixture_tree(&root);
    tree.filterable = false;
    tree.set_query("OtherLeaf");
    tree.select_item(6);
    let theme = Theme::new(ui_styling::appearance::AppearanceName::Dark);
    let rect = Rect::new(0, 0, 16, 2);
    let mut buf = CellBuffer::new(Size { width: 16, height: 2 }, Cell::blank([0; 3], [0; 3]));
    crate::tree::paint_tree(&tree, &theme, rect, &mut buf, true);
    let label = buf.get(2, 0).unwrap();
    assert_eq!((label.fg, label.bg), (theme.role(Role::ActiveForeground), theme.role(Role::ActiveBase)));
    assert_eq!(tree.selected_item(), Some(6));
}

#[test]
fn tree_hover_path_labels_and_chevrons_are_bold_while_selected_and_unrelated_rows_keep_their_attributes() {
    let root = tree_fixture();
    for appearance in [ui_styling::appearance::AppearanceName::Dark, ui_styling::appearance::AppearanceName::Light] {
        let theme = Theme::new(appearance);
        for (selected, focused) in [(1, true), (1, false), (6, true), (6, false)] {
            let mut tree = fixture_tree(&root);
            tree.filterable = false;
            tree.expand_to_depth(3);
            tree.select_item(selected);
            tree.view.set_hover(tree.position_of(2));
            let rect = Rect::new(0, 0, 20, 8);
            let mut buf = CellBuffer::new(Size { width: 20, height: 8 }, Cell::blank([0; 3], [0; 3]));
            crate::tree::paint_tree(&tree, &theme, rect, &mut buf, focused);
            for item in 0..tree.len() {
                let path = [0, 1, 2].contains(&item);
                let expected = if item == selected {
                    if focused { 0 } else { attr::BOLD }
                } else if path { attr::BOLD } else { 0 };
                let marker = tree.item(item).unwrap().depth * 2;
                assert_eq!(buf.get(marker, item as u16).unwrap().attrs, expected, "{appearance:?} marker {item} selected={selected} focused={focused}");
                assert_eq!(buf.get(marker + 2, item as u16).unwrap().attrs, expected, "{appearance:?} label {item} selected={selected} focused={focused}");
            }
        }
    }
}
