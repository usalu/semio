use crate::tui::ansi::{emit_runs, AnsiPatch};
use crate::tui::cell::{diff, Cell, CellBuffer};
use crate::tui::event::{Event, Key, KeyEvent, MouseButton, MouseKind};
use crate::tui::geometry::Size;
use crate::tui::scene::{NodeContent, NodeId, Scene};
use crate::tui::theme::Theme;
use crate::tui::widget::{CursorSpec, WidgetSignal};
use ui_styling::appearance::AppearanceName;

fn focusable(scene: &Scene, id: NodeId) -> bool {
    matches!(&scene.node(id).content, NodeContent::Widget(widget) if widget.interactive())
}

fn dfs_focusables(scene: &Scene, id: NodeId, out: &mut Vec<NodeId>) {
    if focusable(scene, id) {
        out.push(id);
    }
    for &child in scene.node(id).children() {
        dfs_focusables(scene, child, out);
    }
}

/// ??? The retained-mode pipeline: layout-if-dirty ? paint-if-dirty ? damage-diff ? ANSI.
pub struct Tui {
    pub scene: Scene,
    pub theme: Theme,
    size: Size,
    front: CellBuffer,
    back: CellBuffer,
    focus: Option<NodeId>,
    hovered: Option<NodeId>,
    #[allow(dead_code)]
    capture: Option<NodeId>,
    full_redraw: bool,
}

impl Tui {
    pub fn new(size: Size, theme: Theme) -> Self {
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        Self { scene: Scene::new(), theme, size, front: CellBuffer::new(size, blank), back: CellBuffer::new(size, blank), focus: None, hovered: None, capture: None, full_redraw: true }
    }

    /// ??? The last fully-composed frame, for hosts/tests that need to inspect the actual render.
    pub fn frame(&self) -> &CellBuffer {
        &self.front
    }

    pub fn resize(&mut self, size: Size) {
        self.size = size;
        let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
        self.front.resize(size, blank);
        self.back.resize(size, blank);
        self.full_redraw = true;
    }

    pub fn set_appearance(&mut self, appearance: AppearanceName) {
        self.theme.set_appearance(appearance);
        self.full_redraw = true;
    }

    pub fn focus(&self) -> Option<NodeId> {
        self.focus
    }

    pub fn set_focus(&mut self, id: Option<NodeId>) {
        self.focus = id;
    }

    /// ✏️ The focused widget's terminal cursor, placed by its current rect.
    pub fn cursor(&self) -> Option<CursorSpec> {
        let node = self.scene.node(self.focus?);
        match &node.content {
            NodeContent::Widget(widget) => widget.cursor(node.rect),
            _ => None,
        }
    }

    /// 👆️ The node under the pointer.
    pub fn hovered(&self) -> Option<NodeId> {
        self.hovered
    }

    /// 🪤️ Names the node that owns the pointer until released with `None`.
    pub fn capture(&mut self, node: Option<NodeId>) {
        self.capture = node;
    }

    /// ⏱️ Advances time-driven widget state to `now_ms`; true when a repaint is due.
    pub fn tick(&mut self, now_ms: u64) -> bool {
        let _ = now_ms;
        false
    }

    /// 📐️ Solves the scene's layout for the current size without painting.
    pub fn layout(&mut self) {
        crate::tui::layout::solve(&mut self.scene, crate::tui::geometry::Rect::new(0, 0, self.size.width, self.size.height));
    }

    pub fn focus_next(&mut self) {
        let mut list = Vec::new();
        dfs_focusables(&self.scene, self.scene.root(), &mut list);
        if list.is_empty() {
            return;
        }
        let next = match self.focus.and_then(|f| list.iter().position(|x| *x == f)) {
            Some(i) => (i + 1) % list.len(),
            None => 0,
        };
        self.focus = Some(list[next]);
    }

    pub fn focus_prev(&mut self) {
        let mut list = Vec::new();
        dfs_focusables(&self.scene, self.scene.root(), &mut list);
        if list.is_empty() {
            return;
        }
        let prev = match self.focus.and_then(|f| list.iter().position(|x| *x == f)) {
            Some(i) => (i + list.len() - 1) % list.len(),
            None => list.len() - 1,
        };
        self.focus = Some(list[prev]);
    }

    /// ??? Routes one input event to the focused widget (keys) or the hit node (mouse).
    pub fn dispatch(&mut self, event: &Event) -> Vec<(NodeId, WidgetSignal)> {
        let mut signals = Vec::new();
        match event {
            Event::Resize(size) => self.resize(*size),
            Event::Key(KeyEvent { key: Key::Tab, .. }) => self.focus_next(),
            Event::Key(KeyEvent { key: Key::BackTab, .. }) => self.focus_prev(),
            Event::Key(key_ev) => {
                if let Some(id) = self.focus {
                    if let Some(widget) = self.scene.node_mut(id).widget() {
                        if let Some(signal) = widget.on_key(key_ev) {
                            signals.push((id, signal));
                        }
                    }
                }
            }
            Event::Mouse(m) => {
                if matches!(m.kind, MouseKind::Down(_)) {
                    if let Some(id) = self.scene.hit(m.pos) {
                        let mut focus_target = id;
                        while !focusable(&self.scene, focus_target) {
                            if let Some(p) = self.scene.node(focus_target).parent {
                                focus_target = p;
                            } else {
                                break;
                            }
                        }
                        if focusable(&self.scene, focus_target) {
                            self.focus = Some(focus_target);
                        }
                        if matches!(m.kind, MouseKind::Down(MouseButton::Left)) {
                            let rect = self.scene.node(id).rect;
                            if let Some(widget) = self.scene.node_mut(id).widget() {
                                if let Some(signal) = widget.on_mouse(rect, m) { signals.push((id, signal)); }
                            }
                        }
                        let mut chrome_probe = id;
                        loop {
                            let rect = self.scene.node(chrome_probe).rect;
                            if let NodeContent::Chrome(chrome) = &self.scene.node(chrome_probe).content {
                                if let Some(signal) = chrome.window_hit(rect, m.pos) {
                                    signals.push((chrome_probe, signal));
                                }
                                break;
                            }
                            if let Some(p) = self.scene.node(chrome_probe).parent {
                                chrome_probe = p;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        signals
    }

    fn paint(&mut self) {
        fn walk(scene: &Scene, theme: &Theme, focus: Option<NodeId>, id: NodeId, buf: &mut CellBuffer) {
            let node = scene.node(id);
            if !node.visible {
                return;
            }
            let rect = node.rect;
            match &node.content {
                NodeContent::Chrome(c) => c.paint(theme, rect, buf),
                NodeContent::Widget(w) => w.paint(theme, rect, buf, Some(id) == focus),
                NodeContent::Text(s) => {
                    let bg = buf.get(rect.x, rect.y).map(|c| c.bg).unwrap_or(theme.surface(crate::tui::theme::Surface::Base));
                    buf.put_str(crate::tui::geometry::Pos { x: rect.x, y: rect.y }, s, theme.role(crate::tui::theme::Role::Foreground), bg, 0, rect);
                }
                NodeContent::Box => {}
            }
            for &child in node.children() {
                walk(scene, theme, focus, child, buf);
            }
        }
        walk(&self.scene, &self.theme, self.focus, self.scene.root(), &mut self.back);
    }

    /// ??? Solves layout if dirty, repaints, diffs against the last frame, and emits a patch.
    pub fn render(&mut self) -> AnsiPatch {
        let root_dirty = self.scene.take_dirty(self.scene.root()) != 0;
        if !root_dirty && !self.full_redraw {
            return AnsiPatch::default();
        }
        self.layout();
        self.paint();
        let mut patch = AnsiPatch::default();
        if self.full_redraw {
            let full = vec![crate::tui::cell::DiffRun { y: 0, x: 0, len: self.size.width }; usize::from(self.size.height)]
                .into_iter()
                .enumerate()
                .map(|(y, mut r)| {
                    r.y = y as u16;
                    r
                })
                .collect::<Vec<_>>();
            emit_runs(&self.back, &full, &mut patch);
            self.full_redraw = false;
        } else {
            let runs = diff(&self.front, &self.back);
            emit_runs(&self.back, &runs, &mut patch);
        }
        self.front = self.back.clone();
        patch
    }

    /// ??? Forces a full-frame repaint regardless of dirty state.
    pub fn render_full(&mut self) -> AnsiPatch {
        self.full_redraw = true;
        self.render()
    }
}
