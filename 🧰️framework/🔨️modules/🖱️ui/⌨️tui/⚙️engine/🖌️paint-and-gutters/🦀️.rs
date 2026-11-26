//! 🖌️ Paint and gutters: the paint pass, the diffed and full renders and the gutter hit testing and painting. See [`super`].

use super::*;
use semio_ui_tui_core::theme::Surface;

type BorderCapture = (Rect, std::collections::HashMap<(u16, u16), Cell>, std::collections::HashSet<(u16, u16)>);

fn border_mask(ch: char) -> u8 {
    match ch {
        '─' | '━' => 10, '│' | '┃' => 5,
        '┌' | '┏' => 6, '┐' | '┓' => 12, '└' | '┗' => 3, '┘' | '┛' => 9,
        '├' | '┣' => 7, '┤' | '┫' => 13, '┬' | '┳' => 14, '┴' | '┻' => 11,
        '┼' | '╋' => 15, _ => 0,
    }
}

fn border_glyph(mask: u8, heavy: bool) -> char {
    match (mask, heavy) {
        (5, false) => '│', (5, true) => '┃', (10, false) => '─', (10, true) => '━',
        (6, false) => '┌', (6, true) => '┏', (12, false) => '┐', (12, true) => '┓',
        (3, false) => '└', (3, true) => '┗', (9, false) => '┘', (9, true) => '┛',
        (7, false) => '├', (7, true) => '┣', (13, false) => '┤', (13, true) => '┫',
        (14, false) => '┬', (14, true) => '┳', (11, false) => '┴', (11, true) => '┻',
        (15, false) => '┼', (15, true) => '╋', _ => ' ',
    }
}

fn compose_shared_borders(scene: &Scene, focus: Option<NodeId>, theme: &Theme, windows: &[BorderCapture], buf: &mut CellBuffer) {
    use std::collections::HashMap;
    fn collect(scene: &Scene, id: NodeId, focus: Option<NodeId>, states: &mut HashMap<(u16, u16), u8>) {
        let node = scene.node(id);
        if !node.visible { return; }
        if let NodeContent::Axis(axis) = &node.content {
            for (index, pair) in node.children().windows(2).enumerate() {
                let priority = if axis.hover == Some(index) { 2 } else if focus == Some(id) && axis.selected == index { 1 } else { 0 };
                if priority == 0 { continue; }
                let overlap = scene.node(pair[0]).rect.intersect(scene.node(pair[1]).rect);
                for y in overlap.y..overlap.y + overlap.height {
                    for x in overlap.x..overlap.x + overlap.width { states.entry((x, y)).and_modify(|state| *state = (*state).max(priority)).or_insert(priority); }
                }
            }
        }
        for &child in node.children() { collect(scene, child, focus, states); }
    }
    let mut states = HashMap::new();
    collect(scene, scene.root(), focus, &mut states);
    let mut shared: HashMap<(u16, u16), Vec<Cell>> = HashMap::new();
    for (index, (a, first, _)) in windows.iter().enumerate() {
        for (b, second, _) in &windows[index + 1..] {
            let overlap = a.intersect(*b);
            if overlap.width == 0 || overlap.height == 0 || (overlap.width != 1 && overlap.height != 1) { continue; }
            for y in overlap.y..overlap.y + overlap.height {
                for x in overlap.x..overlap.x + overlap.width {
                    let cells = shared.entry((x, y)).or_default();
                    cells.extend(first.get(&(x, y)).copied());
                    cells.extend(second.get(&(x, y)).copied());
                }
            }
        }
    }
    for ((x, y), cells) in shared {
        let Some(current) = buf.get(x, y).copied() else { continue };
        let last_border = windows.iter().rev().find(|(_, _, writes)| writes.contains(&(x, y))).is_some_and(|(_, borders, _)| borders.contains_key(&(x, y)));
        if current.ch != ' ' && !last_border { continue; }
        let mask = cells.iter().fold(0, |mask, cell| mask | border_mask(cell.ch));
        let heavy = cells.iter().any(|cell| matches!(cell.ch, '━' | '┃' | '┏' | '┓' | '┗' | '┛' | '┣' | '┫' | '┳' | '┻' | '╋'));
        let role = |cell: &&Cell| if cell.fg == theme.role(Role::BorderEmphasized) { 2 } else if cell.fg == theme.role(Role::ActiveBase) { 1 } else { 0 };
        if let Some(cell) = cells.iter().max_by_key(role) {
            let ch = border_glyph(mask, heavy);
            if ch != ' ' {
                let priority = states.get(&(x, y)).copied().unwrap_or(0).max(role(&cell));
                let fg = if priority == 2 { theme.role(Role::BorderEmphasized) } else if priority == 1 { theme.role(Role::ActiveBase) } else { cell.fg };
                buf.put(x, y, Cell { ch, fg, ..*cell });
            }
        }
    }
}

/// 🧩️ Which layer of a chrome node one paint step draws: the frame below the children, the chips above them, or both for a node without children.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ChromePart {
    Frame,
    Chips,
    Both,
}

/// 🫟️ The read-only inputs of one paint pass; `projected` paints a canvas with the windows a dock release would mount instead of its children.
#[derive(Clone, Copy)]
struct PaintPass<'a> {
    scene: &'a Scene,
    theme: &'a Theme,
    focus: Option<NodeId>,
    hovered_chrome: &'a [NodeId],
    projected: Option<(NodeId, &'a [(NodeId, Rect, WindowState)])>,
}

impl PaintPass<'_> {
    fn chrome(&self, id: NodeId, chrome: &ChromeState, rect: Rect, buf: &mut CellBuffer, borders: &mut Vec<BorderCapture>, part: ChromePart) {
        let outlined = matches!(chrome, ChromeState::Window(_) | ChromeState::Navbar(_) | ChromeState::Footer(_));
        if outlined { buf.begin_borders(rect); }
        let emphasized = self.hovered_chrome.contains(&id);
        if part != ChromePart::Chips { chrome.paint_frame_emphasized(self.theme, rect, buf, emphasized); }
        if part != ChromePart::Frame { chrome.paint_chips_emphasized(self.theme, rect, buf, emphasized); }
        if outlined { let (outline, writes) = buf.take_borders(); borders.push((rect, outline, writes)); }
    }

    fn walk(&self, id: NodeId, owner: Surface, buf: &mut CellBuffer, borders: &mut Vec<BorderCapture>) {
        let node = self.scene.node(id);
        if !node.visible {
            return;
        }
        let rect = node.rect;
        let owner = match &node.content {
            NodeContent::Chrome(ChromeState::Window(_)) => Surface::Window,
            NodeContent::Chrome(ChromeState::Navbar(_) | ChromeState::Footer(_)) => Surface::Panel,
            NodeContent::Widget(WidgetState::Dialog(_) | WidgetState::Palette(_)) => Surface::Dialog,
            NodeContent::Widget(WidgetState::Menu(_) | WidgetState::Tooltip(_)) => Surface::Menu,
            _ => owner,
        };
        buf.set_owner(owner, self.theme);
        match &node.content {
            NodeContent::Chrome(c) => self.chrome(id, c, rect, buf, borders, ChromePart::Frame),
            NodeContent::Widget(w) => w.paint(self.theme, rect, buf, Some(id) == self.focus),
            NodeContent::Text(s) => {
                let bg = buf.owner_background();
                buf.put_str(Pos { x: rect.x, y: rect.y }, s, self.theme.role(Role::Foreground), bg, 0, rect);
            }
            NodeContent::Axis(_) => {}
            NodeContent::Box => {}
        }
        if let Some((_, windows)) = self.projected.filter(|(canvas, _)| *canvas == id) {
            for (window, rect, state) in windows {
                self.chrome(*window, &ChromeState::Window(Box::new(state.clone())), *rect, buf, borders, ChromePart::Both);
            }
            return;
        }
        let child_owner = if matches!(node.content, NodeContent::Widget(WidgetState::Chip(_))) { owner.above() } else { owner };
        for &child in node.children() {
            self.walk(child, child_owner, buf, borders);
        }
        if let NodeContent::Chrome(c) = &node.content {
            self.chrome(id, c, rect, buf, borders, ChromePart::Chips);
        }
    }
}

impl Tui {
    fn paint(&mut self) {
        let blank = Cell::blank(self.theme.role(Role::Foreground), self.theme.surface(Surface::Base));
        self.back.resize(self.size, blank);
        let pass = PaintPass { scene: &self.scene, theme: &self.theme, focus: self.focus(), hovered_chrome: &self.hovered_chrome, projected: None };
        let mut borders = Vec::new();
        pass.walk(self.scene.root(), Surface::Base, &mut self.back, &mut borders);
        let preview = self.dock_preview.as_ref().map(|preview| {
            let mut committed = self.back.clone();
            let mut outlines = Vec::new();
            PaintPass { projected: Some((preview.canvas, preview.windows.as_slice())), ..pass }.walk(self.scene.root(), Surface::Base, &mut committed, &mut outlines);
            compose_shared_borders(&self.scene, pass.focus, &self.theme, &outlines, &mut committed);
            (preview.rect, committed)
        });
        compose_shared_borders(&self.scene, pass.focus, &self.theme, &borders, &mut self.back);
        pass.walk(self.scene.overlay_root(), Surface::Base, &mut self.back, &mut Vec::new());
        if let Some((rect, committed)) = preview {
            self.back.copy_rect(&committed, rect);
        }
    }

    /// 🖨️ Solves layout if dirty, repaints, diffs against the last frame and emits the patch; an unchanged scene emits nothing.
    pub fn render(&mut self) -> AnsiPatch {
        self.sanitize();
        self.sync_chrome();
        if !self.dirty() {
            return AnsiPatch::default();
        }
        self.ensure_layout();
                let held = match &self.press { Some(Press::Tab { chrome, source, pos, moved: true, .. }) => Some((*chrome, source.clone(), *pos)), _ => None };
        if let Some((chrome, source, pos)) = held {
            self.dock_preview = self.scene.hit(pos).and_then(|hit| self.window_of(hit)).and_then(|node| {
                self.propose_dock(node, chrome, &source, pos)
            });
        }
        if let Some(pos) = self.hover_pos {
            let hit = self.scene.hit(pos).filter(|&hit| self.inside_modal(Some(hit)));
            self.update_hover(hit, Some(pos));
        }
self.paint();
        let mut patch = AnsiPatch::default();
        if self.full_redraw {
            let rows: Vec<DiffRun> = (0..self.size.height).map(|y| DiffRun { y, x: 0, len: self.size.width }).collect();
            emit_runs(&self.back, &rows, &mut patch);
            self.full_redraw = false;
        } else {
            let runs = diff(&self.front, &self.back);
            emit_runs(&self.back, &runs, &mut patch);
        }
        std::mem::swap(&mut self.front, &mut self.back);
        self.scene.clear_dirty(LAYOUT_DIRTY | PAINT_DIRTY);
        self.last_render_ms = Some(self.now_ms);
        patch
    }

    /// ♻️ Forces a full-frame repaint regardless of dirty state.
    pub fn render_full(&mut self) -> AnsiPatch {
        self.full_redraw = true;
        self.render()
    }
}

/// 📏️ The index of the shared splitter cell between adjacent children containing `pos`.
pub(super) fn gutter_at(scene: &Scene, _axis: &AxisState, rect: Rect, children: &[NodeId], pos: Pos) -> Option<usize> {
    if !rect.contains(pos) {
        return None;
    }
    children.windows(2).position(|pair| {
        let (a, b) = (scene.node(pair[0]).rect, scene.node(pair[1]).rect);
        a.contains(pos) && b.contains(pos)
    })
}
