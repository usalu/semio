//! 🔎️ tui paint functions for the Window element: one rectangular frame, painted below the content, and the chips inside it, painted above the content.
//! `window_chip_layout` and `WindowChipLayout` stay in the `chrome` mod (shared with `ChromeState::window_target`).

use semio_ui_tui_core::cell::{attr, Cell, CellBuffer};
use crate::chrome::{window_chip_layout, window_drag_glyph, ChromeMode, TabKind, WindowCornerTab, WindowHit, WindowState};
use semio_ui_tui_core::geometry::{Pos, Rect};
use semio_ui_tui_core::theme::{Glyph, GlyphSet, Rgb, Role, Surface, Theme};

/// 🖋️ The line-drawing set a window outline uses: heavy for the focused stack so focus shows without colour.
struct Lines {
    h: char,
    v: char,
    tl: char,
    tr: char,
    bl: char,
    br: char,
    up: char,
    down: char,
}

const LIGHT: Lines = Lines { h: '\u{2500}', v: '\u{2502}', tl: '\u{250c}', tr: '\u{2510}', bl: '\u{2514}', br: '\u{2518}', up: '\u{2534}', down: '\u{252c}' };
const HEAVY: Lines = Lines { h: '\u{2501}', v: '\u{2503}', tl: '\u{250f}', tr: '\u{2513}', bl: '\u{2517}', br: '\u{251b}', up: '\u{253b}', down: '\u{2533}' };

fn lines(focused: bool) -> &'static Lines {
    if focused {
        &HEAVY
    } else {
        &LIGHT
    }
}

/// 🗗️ The restore glyph of a zoomed window; the theme's repertoire has maximize only.
fn restore_glyph(set: GlyphSet) -> &'static str {
    match set {
        GlyphSet::Unicode => "\u{2921}",
        GlyphSet::Ascii => "v",
    }
}

/// 🎨️ Colours one chip uses, resolved from the tab state; the chip surface is one elevation step above the content, the opaque capability fallback of a glass chip.
struct ChipPaint {
    fg: Rgb,
    bg: Rgb,
    attrs: u8,
}

fn chip_paint(w: &WindowState, theme: &Theme, tab: &WindowCornerTab) -> ChipPaint {
    let chip_bg = theme.surface(Surface::Pane);
    let hovered = chip_hovered(w, tab);
    if tab.kind == TabKind::Tab && tab.index == w.active_stack_tab {
        return ChipPaint { fg: theme.role(Role::Foreground), bg: chip_bg, attrs: attr::BOLD };
    }
    if hovered || (tab.kind == TabKind::Tab && w.drop_target == Some(tab.index)) {
        return ChipPaint { fg: theme.role(Role::Foreground), bg: theme.role(Role::HoverInteractive), attrs: 0 };
    }
    ChipPaint { fg: theme.role(Role::MutedForeground), bg: chip_bg, attrs: 0 }
}

/// 🏆 Whether the pointer's innermost interactive box is this chip itself; a control glyph inside a chip owns its own hover and never hovers the chip.
fn chip_hovered(w: &WindowState, tab: &WindowCornerTab) -> bool {
    match (w.hover, tab.kind) {
        (Some(WindowHit::Tab(i)), TabKind::Tab) => i == tab.index,
        (Some(WindowHit::OverflowPrev(t)), TabKind::OverflowPrev(target)) => t == target,
        (Some(WindowHit::OverflowNext(t)), TabKind::OverflowNext(target)) => t == target,
        _ => false,
    }
}

fn chip_border_role(w: &WindowState, tab: &WindowCornerTab, emphasized: bool) -> Role {
    let hovered = chip_hovered(w, tab);
    if emphasized || hovered { Role::BorderEmphasized } else if tab.kind == TabKind::Tab && tab.index == w.active_stack_tab { Role::ActiveBase } else { Role::BorderNormal }
}

/// 🪟 Paints the interior text of one chip and lights the hovered control glyph.
fn paint_chip_text(buf: &mut CellBuffer, w: &WindowState, theme: &Theme, tab: &WindowCornerTab, y: u16, paint: &ChipPaint) {
    let clip = Rect::new(tab.x + 1, y, tab.interior_width, 1);
    buf.fill_rect(clip, Cell::blank(paint.fg, paint.bg));
    buf.put_str(Pos { x: clip.x, y }, &tab.interior, paint.fg, paint.bg, paint.attrs, clip);
    let set = theme.glyphs;
    let mut draw = |x: u16, glyph: &str, fg: Rgb| {
        buf.put_str(Pos { x, y }, glyph, fg, paint.bg, paint.attrs, clip);
    };
    match tab.kind {
        TabKind::OverflowPrev(_) => draw(tab.x + 1 + if tab.interior_width == 1 { 0 } else { 1 }, set.glyph(Glyph::PreviousOption), paint.fg),
        TabKind::OverflowNext(_) => draw(tab.x + 1 + if tab.interior_width == 1 { 0 } else { tab.interior_width - 2 }, set.glyph(Glyph::NextOption), paint.fg),
        _ => {}
    }
    if let (TabKind::Tab, Some(status)) = (tab.kind, tab.status) {
        let active = tab.index == w.active_stack_tab;
        draw(tab.x + 2, status.glyph(set, w.spin), if active { paint.fg } else { theme.role(status.role()) });
    }
    if let Some(x) = tab.close_x {
        draw(x, set.glyph(Glyph::Close), paint.fg);
    }
    if let Some(x) = tab.maximize_x {
        draw(x, if w.zoomed { restore_glyph(set) } else { set.glyph(Glyph::Maximize) }, paint.fg);
    }
    if let Some(x) = tab.drag_x {
        draw(x, window_drag_glyph(set), paint.fg);
    }
    if let Some(x) = tab.drag_x {
        if let Some(cell) = buf.get(x, y).copied() {
            let attrs = (paint.attrs & !attr::BOLD) | if w.hover_drag == Some(tab.index) { attr::BOLD } else { 0 };
            buf.put(x, y, Cell { attrs, ..cell });
        }
    }
    let lit = match (w.hover, tab.kind) {
        (Some(WindowHit::Close(i)), TabKind::Tab) if i == tab.index => tab.close_x,
        (Some(WindowHit::Maximize(i)), TabKind::Tab) if i == tab.index => tab.maximize_x,
        (Some(WindowHit::NewTab(_)), TabKind::Controls) => tab.new_x,
        _ => None,
    };
    if let Some(x) = lit {
        if let Some(cell) = buf.get(x, y).copied() {
            buf.put(x, y, Cell { fg: theme.role(Role::Foreground), bg: theme.role(Role::HoverInteractive), attrs: attr::BOLD, ..cell });
        }
    }
}

/// 🏅️ The state priority of a border role: emphasized over active over normal.
fn rank(role: Role) -> u8 {
    match role { Role::BorderEmphasized => 2, Role::ActiveBase => 1, _ => 0 }
}

fn rank_color(theme: &Theme, rank: u8) -> Rgb {
    theme.role(match rank { 2 => Role::BorderEmphasized, 1 => Role::ActiveBase, _ => Role::BorderNormal })
}

/// 🖌️ Paints the window frame: one rectangle of cells with its outline, whatever chips it hosts.
pub fn paint_window_frame(w: &WindowState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, emphasized: bool) {
    if rect.width < 2 || rect.height < 2 {
        return;
    }
    let bg = theme.surface(Surface::Window);
    let border = theme.role(if emphasized { Role::BorderEmphasized } else if w.focused { Role::ActiveBase } else { Role::BorderNormal });
    let g = lines(w.focused);
    let bottom_y = rect.y + rect.height - 1;
    let right_x = rect.x + rect.width - 1;
    buf.fill_rect(rect, Cell::blank(theme.role(Role::Foreground), bg));
    buf.border_hline(Pos { x: rect.x + 1, y: rect.y }, rect.width.saturating_sub(2), g.h, border, bg);
    buf.border_hline(Pos { x: rect.x + 1, y: bottom_y }, rect.width.saturating_sub(2), g.h, border, bg);
    buf.border_vline(Pos { x: rect.x, y: rect.y + 1 }, rect.height.saturating_sub(2), g.v, border, bg);
    buf.border_vline(Pos { x: right_x, y: rect.y + 1 }, rect.height.saturating_sub(2), g.v, border, bg);
    for (x, y, ch) in [(rect.x, rect.y, g.tl), (right_x, rect.y, g.tr), (rect.x, bottom_y, g.bl), (right_x, bottom_y, g.br)] {
        buf.put_border(x, y, Cell { ch, fg: border, bg, attrs: 0, width: 1 });
    }
}

/// 🪙️ Paints the chips inside the frame, above the content. A chip covers its own cells of one interior row and draws only its inner edges: its walls and the junctions where they meet the outline; its outer edges are the frame outline. Cells of the row no chip covers keep the content.
pub fn paint_window_chips(w: &WindowState, theme: &Theme, rect: Rect, buf: &mut CellBuffer, emphasized: bool) {
    let layout = window_chip_layout(w, rect);
    if layout.mode == ChromeMode::Flat {
        return;
    }
    let bg = theme.surface(Surface::Window);
    let g = lines(w.focused);
    let body = if emphasized { 2 } else { u8::from(w.focused) };
    let (left_x, right_x, bottom_y) = (rect.x, rect.x + rect.width - 1, rect.y + rect.height - 1);
    let mut walls: std::collections::BTreeMap<(bool, u16), u8> = std::collections::BTreeMap::new();
    for group in &layout.groups {
        let top = group.corner.is_top();
        let (y, edge_y) = (layout.text_y(group.corner, rect), if top { rect.y } else { bottom_y });
        for tab in &group.tabs {
            let paint = chip_paint(w, theme, tab);
            paint_chip_text(buf, w, theme, tab, y, &paint);
            let tab_rank = rank(chip_border_role(w, tab, emphasized));
            buf.border_hline(Pos { x: tab.x + 1, y: edge_y }, tab.interior_width, g.h, rank_color(theme, tab_rank.max(body)), bg);
            for x in [tab.x, tab.x + tab.interior_width + 1] {
                walls.entry((top, x)).and_modify(|rank| *rank = (*rank).max(tab_rank)).or_insert(tab_rank);
            }
        }
    }
    for ((top, x), wall_rank) in walls {
        let (y, edge_y) = if top { (rect.y + 1, rect.y) } else { (bottom_y - 1, bottom_y) };
        let on_edge = x == left_x || x == right_x;
        let joint = rank_color(theme, wall_rank.max(body));
        let ch = match (x == left_x, x == right_x, top) {
            (true, _, true) => g.tl,
            (_, true, true) => g.tr,
            (true, _, false) => g.bl,
            (_, true, false) => g.br,
            (_, _, true) => g.down,
            (_, _, false) => g.up,
        };
        buf.put_border(x, y, Cell { ch: g.v, fg: if on_edge { joint } else { rank_color(theme, wall_rank) }, bg: if on_edge { bg } else { theme.surface(Surface::Pane) }, attrs: 0, width: 1 });
        buf.put_border(x, edge_y, Cell { ch, fg: joint, bg, attrs: 0, width: 1 });
    }
}
