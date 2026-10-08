//! 🔬️ The overlay layer: menu, dialog, palette and tooltip stack above the scene, trap focus while modal, dismiss on
//! Escape or an outside press, and give focus back (R14, R19; fleet-plan §4.2 T-B).

use crate::tui::dialog::DialogState;
use crate::tui::engine::Tui;
use crate::tui::event::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseKind};
use crate::tui::geometry::{Pos, Size};
use crate::tui::layout::{Constraint, Dimension, Direction};
use crate::tui::menu::MenuItem;
use crate::tui::palette::{PaletteItem, PaletteState};
use crate::tui::scene::{Node, NodeContent, NodeId};
use crate::tui::theme::Theme;
use crate::tui::widget::{ListState, WidgetSignal, WidgetState};
use ui_styling::appearance::AppearanceName;

fn key(key: Key) -> Event {
    Event::Key(KeyEvent { key, mods: 0 })
}

fn press(x: u16, y: u16) -> Event {
    Event::Mouse(MouseEvent { kind: MouseKind::Down(MouseButton::Left), pos: Pos { x, y }, mods: 0, clicks: 1 })
}

fn scene() -> (Tui, NodeId) {
    let mut tui = Tui::new(Size { width: 50, height: 20 }, Theme::new(AppearanceName::Dark));
    let root = tui.scene.root();
    tui.scene.node_mut(root).set_constraint(Constraint { direction: Direction::Column, ..Default::default() });
    let list = tui.scene.add(root, Node::new(NodeContent::Widget(WidgetState::List(ListState::new((0..30).map(|i| format!("row {i}")).collect())))));
    tui.scene.node_mut(list).set_constraint(Constraint { height: Dimension::Weight(1), ..Default::default() });
    tui.set_focus(Some(list));
    tui.render_full();
    (tui, list)
}

fn items() -> Vec<MenuItem> {
    vec![MenuItem::new("copy").with_shortcut("C-c"), MenuItem::separator(), MenuItem::new("paste").disabled(), MenuItem::new("close")]
}

fn frame_text(tui: &Tui) -> String {
    (0..tui.frame().size.height).map(|y| (0..tui.frame().size.width).filter_map(|x| tui.frame().get(x, y).map(|c| c.ch)).collect::<String>()).collect::<Vec<_>>().join("\n")
}

#[test]
fn a_menu_traps_focus_picks_with_the_keyboard_and_returns_focus() {
    let (mut tui, list) = scene();
    let menu = tui.open_menu(Pos { x: 5, y: 3 }, items());
    assert_eq!(tui.focus(), Some(menu));
    assert_eq!(tui.overlays(), vec![menu]);
    tui.render();
    let text = frame_text(&tui);
    assert!(text.contains("copy") && text.contains("C-c") && text.contains("close"), "{text}");
    assert_eq!(tui.dispatch(&key(Key::Tab)), vec![(menu, WidgetSignal::SelectionChanged(3))], "Tab belongs to the menu while it is open, and skips separators and disabled rows");
    assert_eq!(tui.dispatch(&key(Key::Up)), vec![(menu, WidgetSignal::SelectionChanged(0))]);
    assert_eq!(tui.dispatch(&key(Key::Down)), vec![(menu, WidgetSignal::SelectionChanged(3))]);
    assert_eq!(tui.dispatch(&key(Key::Enter)), vec![(menu, WidgetSignal::Activated(3))]);
    assert!(tui.overlays().is_empty(), "choosing an item closes the menu");
    assert_eq!(tui.focus(), Some(list));
    assert!(!tui.render().0.is_empty());
    assert!(!frame_text(&tui).contains("close"));
}

#[test]
fn escape_and_an_outside_press_dismiss_a_menu_without_reaching_the_scene() {
    let (mut tui, list) = scene();
    let menu = tui.open_menu(Pos { x: 5, y: 3 }, items());
    assert_eq!(tui.dispatch(&key(Key::Esc)), vec![(menu, WidgetSignal::Dismissed)]);
    assert_eq!(tui.focus(), Some(list));
    let menu = tui.open_menu(Pos { x: 5, y: 3 }, items());
    let before = match &tui.scene.node(list).content {
        NodeContent::Widget(WidgetState::List(l)) => l.selected,
        _ => unreachable!(),
    };
    assert_eq!(tui.dispatch(&press(40, 15)), vec![(menu, WidgetSignal::Dismissed)]);
    let after = match &tui.scene.node(list).content {
        NodeContent::Widget(WidgetState::List(l)) => l.selected,
        _ => unreachable!(),
    };
    assert_eq!(before, after);
    assert!(tui.overlays().is_empty());
}

#[test]
fn clicking_a_menu_row_chooses_it_and_a_disabled_row_does_nothing() {
    let (mut tui, _list) = scene();
    let menu = tui.open_menu(Pos { x: 5, y: 3 }, items());
    tui.render();
    let rect = tui.scene.rect(menu);
    assert_eq!(tui.dispatch(&press(rect.x + 2, rect.y + 3)), vec![], "the disabled row ignores the press");
    assert_eq!(tui.overlays(), vec![menu], "and the menu stays open");
    assert_eq!(tui.dispatch(&press(rect.x + 2, rect.y + 4)), vec![(menu, WidgetSignal::Activated(3))]);
}

#[test]
fn a_menu_near_the_edge_flips_back_inside_the_terminal() {
    let (mut tui, _list) = scene();
    let menu = tui.open_menu(Pos { x: 49, y: 19 }, items());
    tui.render();
    let rect = tui.scene.rect(menu);
    assert!(rect.x + rect.width <= 50 && rect.y + rect.height <= 20, "{rect:?}");
    assert_eq!((rect.x + rect.width, rect.y + rect.height), (50, 20), "it opens up and to the left of the anchor");
}

#[test]
fn a_dialog_is_modal_cycles_buttons_and_reports_the_chosen_one() {
    let (mut tui, list) = scene();
    let dialog = tui.open_dialog(DialogState::new("Close task", "The task is still running. Close it anyway?", vec!["Cancel".into(), "Close".into()]));
    tui.render();
    let text = frame_text(&tui);
    assert!(text.contains("Close task") && text.contains("Cancel"), "{text}");
    let rect = tui.scene.rect(dialog);
    assert!(rect.x > 0 && rect.y > 0 && rect.x + rect.width < 50, "centred: {rect:?}");
    assert_eq!(tui.dispatch(&press(1, 1)), vec![], "an outside press does not dismiss a dialog that did not ask for it");
    assert_eq!(tui.overlays(), vec![dialog]);
    assert_eq!(tui.dispatch(&key(Key::Tab)), vec![(dialog, WidgetSignal::SelectionChanged(1))]);
    assert_eq!(tui.dispatch(&key(Key::Enter)), vec![(dialog, WidgetSignal::Activated(1))]);
    assert_eq!(tui.focus(), Some(list));
}

#[test]
fn a_dialog_that_allows_it_is_dismissed_by_an_outside_press() {
    let (mut tui, _list) = scene();
    let mut state = DialogState::new("Note", "Hello", vec!["Ok".into()]);
    state.dismiss_outside = true;
    let dialog = tui.open_dialog(state);
    assert_eq!(tui.dispatch(&press(1, 1)), vec![(dialog, WidgetSignal::Dismissed)]);
}

#[test]
fn the_palette_filters_as_you_type_and_reports_the_original_item_index() {
    let (mut tui, _list) = scene();
    let mut state = PaletteState::new(vec![PaletteItem::new("Open settings", "C-,"), PaletteItem::new("Close window", "C-w"), PaletteItem::new("Close all tasks", ""), PaletteItem::new("Zoom window", "z")]);
    state.placeholder = "type a command".into();
    state.empty_text = "nothing".into();
    let palette = tui.open_palette(state);
    tui.render();
    assert!(frame_text(&tui).contains("type a command"));
    assert!(tui.cursor().is_some(), "the palette shows a caret");
    for c in "close w".chars() {
        tui.dispatch(&key(Key::Char(c)));
    }
    let NodeContent::Widget(WidgetState::Palette(p)) = &tui.scene.node(palette).content else { panic!("palette") };
    assert_eq!(p.visible(), &[1], "every word must match, case-insensitively");
    assert_eq!(tui.dispatch(&key(Key::Enter)), vec![(palette, WidgetSignal::Activated(1))]);
    assert!(tui.overlays().is_empty());
}

#[test]
fn the_palette_edits_scrolls_pastes_and_shows_the_empty_text() {
    let (mut tui, _list) = scene();
    let mut state = PaletteState::new((0..40).map(|i| PaletteItem::new(format!("command {i}"), "")).collect());
    state.empty_text = "nothing".into();
    let palette = tui.open_palette(state);
    for _ in 0..15 {
        tui.dispatch(&key(Key::Down));
    }
    tui.render();
    assert!(frame_text(&tui).contains("command 15"), "the selection stays inside the viewport");
    tui.dispatch(&Event::Paste("zz\nz".into()));
    let NodeContent::Widget(WidgetState::Palette(p)) = &tui.scene.node(palette).content else { panic!("palette") };
    assert_eq!(p.query, "zz z");
    assert!(p.visible().is_empty());
    tui.render();
    assert!(frame_text(&tui).contains("nothing"));
    for _ in 0..10 {
        tui.dispatch(&key(Key::Backspace));
    }
    let NodeContent::Widget(WidgetState::Palette(p)) = &tui.scene.node(palette).content else { panic!("palette") };
    assert_eq!(p.visible().len(), 40);
}

#[test]
fn overlays_stack_in_open_order_and_the_topmost_gets_the_input() {
    let (mut tui, list) = scene();
    let menu = tui.open_menu(Pos { x: 2, y: 2 }, items());
    let dialog = tui.open_dialog(DialogState::new("Sure?", "Really?", vec!["No".into(), "Yes".into()]));
    assert_eq!(tui.overlays(), vec![menu, dialog]);
    assert_eq!(tui.focus(), Some(dialog));
    assert_eq!(tui.dispatch(&key(Key::Esc)), vec![(dialog, WidgetSignal::Dismissed)]);
    assert_eq!(tui.focus(), Some(menu), "closing the top overlay gives focus back to the one beneath");
    tui.close_overlay(menu);
    assert_eq!(tui.focus(), Some(list));
    assert!(tui.overlays().is_empty());
}

#[test]
fn closing_an_overlay_whose_owner_vanished_does_not_dangle() {
    let (mut tui, list) = scene();
    let menu = tui.open_menu(Pos { x: 2, y: 2 }, items());
    tui.scene.remove(list);
    tui.close_overlay(menu);
    assert_eq!(tui.focus(), None);
    tui.render();
}

#[test]
fn a_focus_request_made_while_a_modal_is_open_waits_for_it_to_close() {
    let (mut tui, list) = scene();
    let other = tui.scene.add(tui.scene.root(), Node::new(NodeContent::Widget(WidgetState::List(ListState::new(vec!["other".into()])))));
    tui.render();
    let menu = tui.open_menu(Pos { x: 2, y: 2 }, items());
    tui.set_focus(Some(other));
    assert_eq!(tui.focus(), Some(menu), "the modal keeps the keyboard");
    tui.set_focus(None);
    assert_eq!(tui.focus(), Some(menu));
    tui.close_overlay(menu);
    assert_eq!(tui.focus(), Some(other), "the waiting request is honoured, not the older focus ({list:?})");
}
