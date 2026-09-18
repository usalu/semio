//#region 🧵️MountedLayoutText
//! 🧵️ Fixed-credit mounted layout and text worker with one cursor opportunity per grant.

use crate::wgpu::arena::NodeId;
use crate::wgpu::component::ui::{UiNode, UiTreeItemNode, UiTreeNode};
use crate::wgpu::engine::UiSurfaceToken;
use crate::wgpu::flex::{FlexRect, FlexTree, LayoutJobStage, LayoutJobStep, LayoutNodeKind, MeasureConstraint};
use crate::wgpu::layout::{gap_for_token, padding_for_token, tree_item_height, tree_node_height, tree_section_header_height, tree_section_height, TreeRowMetrics, TREE_ROW_MAX_DEPTH};
use crate::wgpu::text::{is_wrap_space, may_break_between};
use crate::wgpu::theme::Theme;
use crate::wgpu::tree::{AcceptedLayout, NodeFlags, NodeKey, UiTree};

pub(crate) const LAYOUT_NODE_CREDITS: usize = 4_096;
pub(crate) const LAYOUT_GLYPH_CREDITS: usize = 16_384;
pub(crate) const LAYOUT_DEPTH_CREDITS: usize = 64;
pub(crate) const LAYOUT_ATLAS_PAGE_CREDITS: usize = 4;
pub(crate) const LAYOUT_ATLAS_PAGE_BYTES: usize = 16 * 1024;
const DEFAULT_TEXT_SIZE_PX: f32 = 14.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MountedLayoutFault {
    NodeCredits,
    GlyphCredits,
    DepthCredits,
    Stale,
    /// 📐️ The flex solver refused the tree (an insert, a link, or the solve itself) — reported as a
    /// fault rather than publishing a half-solved surface.
    Solver,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct IntrinsicSize {
    width: f32,
    height: f32,
}

/// 🧩️ The band a host-provided content leaf reserves, read off the slot's own `params_json`
/// (`{"hostContentHeight": <logical px>}`) and clamped to something a panel can actually hold. A slot
/// whose params carry no height (or no JSON at all) falls back to [`HOST_CONTENT_DEFAULT_ROWS`] control
/// rows, so a host that forgets to declare one still gets a visible box rather than a collapsed line.
pub(crate) fn host_content_height(params_json: &str, theme: &Theme) -> f32 {
    let declared = serde_json::from_str::<serde_json::Value>(params_json).ok().and_then(|params| params.get("hostContentHeight").and_then(serde_json::Value::as_f64)).map(|height| height as f32);
    declared.unwrap_or(theme.control_height * HOST_CONTENT_DEFAULT_ROWS).clamp(0.0, HOST_CONTENT_MAX_HEIGHT_PX)
}

/// 🧩️ Control rows a host-provided leaf reserves when its host declares no height.
const HOST_CONTENT_DEFAULT_ROWS: f32 = 6.0;
/// 🧩️ The ceiling one host-provided leaf may reserve, so a malformed number cannot ask for a band no
/// viewport could hold (the host scrolls inside its own rect instead).
const HOST_CONTENT_MAX_HEIGHT_PX: f32 = 4096.0;

/// 🌳️ The `UiTreeNode` spec that owns the row `id` mounts as — the nearest `UiNode::Tree`
/// ancestor, the same walk `events::find_tree_item_spec` does to re-derive a row's authored item.
pub(crate) fn owning_tree_spec(tree: &UiTree, id: NodeId) -> Option<&UiTreeNode> {
    let mut ancestor = tree.node(id)?.parent;
    let mut hops = 0usize;
    while let Some(candidate) = ancestor {
        if hops >= TREE_ROW_MAX_DEPTH {
            return None;
        }
        hops += 1;
        let node = tree.node(candidate)?;
        if let UiNode::Tree(tree_node) = &node.spec.0 {
            return Some(tree_node);
        }
        ancestor = node.parent;
    }
    None
}

pub(crate) fn find_tree_item<'a>(items: &'a [UiTreeItemNode], id: &str, depth: usize) -> Option<&'a UiTreeItemNode> {
    if depth >= TREE_ROW_MAX_DEPTH {
        return None;
    }
    for item in items {
        if item.id == id {
            return Some(item);
        }
        if let Some(found) = item.items.as_deref().and_then(|nested| find_tree_item(nested, id, depth + 1)) {
            return Some(found);
        }
    }
    None
}

/// 🌳️ Classifies the synthesized row `id` against the `Tree` that owns it — a section row when its
/// parent is the tree itself and its key names one of that tree's sections, an item row when its
/// parent is already a row and its key names one of that tree's items. Anything else is an ordinary
/// container that merely happens to live inside a tree, and keeps its own kind.
fn tree_row_kind(tree: &UiTree, id: NodeId, parent_kind: Option<LayoutNodeKind>, metrics: &TreeRowMetrics) -> Option<LayoutNodeKind> {
    let parent_kind = parent_kind?;
    if !matches!(parent_kind, LayoutNodeKind::Tree { .. } | LayoutNodeKind::TreeSection { .. } | LayoutNodeKind::TreeRow { .. }) {
        return None;
    }
    let NodeKey::Explicit(key) = &tree.node(id)?.key else { return None };
    let owner = owning_tree_spec(tree, id)?;
    match parent_kind {
        LayoutNodeKind::Tree { .. } => {
            let section = owner.sections.iter().find(|section| &section.id == key)?;
            Some(LayoutNodeKind::TreeSection { header: tree_section_header_height(section, metrics), height: tree_section_height(section, metrics) })
        }
        parent_kind => {
            let item = owner.sections.iter().find_map(|section| find_tree_item(&section.items, key, 0))?;
            if matches!(parent_kind, LayoutNodeKind::TreeRow { expanded: false, .. }) {
                return Some(LayoutNodeKind::TreeRow { row: 0.0, height: 0.0, expanded: false });
            }
            let height = tree_item_height(item, metrics);
            let expanded = height > 0.0 && item.default_open.unwrap_or(false) && item.items.as_deref().is_some_and(|items| !items.is_empty());
            Some(LayoutNodeKind::TreeRow { row: if expanded { metrics.row_height } else { 0.0 }, height, expanded })
        }
    }
}

/// 🧩️ One admitted node's layout identity: where it sits in the arena, which flex box it became,
/// and — for a `Text` node — the half-open glyph range the shaping stage filled, which is the whole
/// input the intrinsic measurement callback needs (no tree access on the worker thread).
#[derive(Clone, Copy, Debug, PartialEq)]
struct LayoutInputNode {
    id: NodeId,
    parent: Option<usize>,
    first_child: Option<usize>,
    last_child: Option<usize>,
    next_sibling: Option<usize>,
    kind: LayoutNodeKind,
    intrinsic: IntrinsicSize,
    glyph_start: usize,
    glyph_end: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RetainedGlyphInput {
    node: usize,
    scalar: char,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RetainedTextRun {
    node: usize,
    glyph_start: usize,
    glyph_end: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct RetainedLine {
    node: usize,
    width: f32,
    height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RetainedGlyphPreview {
    pub scalar: char,
    pub advance: f32,
    pub height: f32,
    pub generation: u64,
    pub revision: u64,
    pub atlas_page: u8,
    pub atlas_offset: u16,
    pub atlas_length: u8,
}

trait OwnedTextWorker: Send {
    fn shape_one(&mut self, input: RetainedGlyphInput) -> RetainedGlyphPreview;
}

#[derive(Default)]
struct DeterministicTextWorker {
    #[cfg(test)]
    cancel_after_shape: Option<semio_framework_job::CancelToken>,
}

impl OwnedTextWorker for DeterministicTextWorker {
    fn shape_one(&mut self, input: RetainedGlyphInput) -> RetainedGlyphPreview {
        let advance = if input.scalar.is_ascii() { DEFAULT_TEXT_SIZE_PX * 0.625 } else { DEFAULT_TEXT_SIZE_PX };
        #[cfg(test)]
        if let Some(cancel) = self.cancel_after_shape.take() {
            cancel.cancel_now();
        }
        RetainedGlyphPreview { scalar: input.scalar, advance, height: crate::wgpu::text::line_height(DEFAULT_TEXT_SIZE_PX), generation: 0, revision: 0, atlas_page: 0, atlas_offset: 0, atlas_length: 0 }
    }
}

struct RetainedAtlasCandidate {
    pages: [Option<Box<[u8; LAYOUT_ATLAS_PAGE_BYTES]>>; LAYOUT_ATLAS_PAGE_CREDITS],
    page_cursor: usize,
    byte_cursor: usize,
}

impl RetainedAtlasCandidate {
    fn new() -> Self {
        Self { pages: std::array::from_fn(|_| Some(Box::new([0; LAYOUT_ATLAS_PAGE_BYTES]))), page_cursor: 0, byte_cursor: 0 }
    }

    fn retain_one(&mut self, scalar: char, generation: u64, revision: u64, mut preview: RetainedGlyphPreview) -> Result<Option<RetainedGlyphPreview>, MountedLayoutFault> {
        let mut bytes = [0; 4];
        let encoded = scalar.encode_utf8(&mut bytes).as_bytes();
        if self.byte_cursor + encoded.len() > LAYOUT_ATLAS_PAGE_BYTES {
            self.page_cursor = self.page_cursor.checked_add(1).ok_or(MountedLayoutFault::GlyphCredits)?;
            self.byte_cursor = 0;
            if self.page_cursor == LAYOUT_ATLAS_PAGE_CREDITS {
                return Err(MountedLayoutFault::GlyphCredits);
            }
            return Ok(None);
        }
        let page = self.pages.get_mut(self.page_cursor).and_then(Option::as_mut).ok_or(MountedLayoutFault::Stale)?;
        let start = self.byte_cursor;
        page[start..start + encoded.len()].copy_from_slice(encoded);
        self.byte_cursor += encoded.len();
        preview.generation = generation;
        preview.revision = revision;
        preview.atlas_page = self.page_cursor as u8;
        preview.atlas_offset = start as u16;
        preview.atlas_length = encoded.len() as u8;
        Ok(Some(preview))
    }

    fn close_one(&mut self) -> bool {
        let Some(page) = self.pages.iter_mut().find(|page| page.is_some()) else { return true };
        *page = None;
        false
    }

    fn is_empty(&self) -> bool {
        self.pages.iter().all(Option::is_none)
    }
}

/// 📏️ One text node's intrinsic size against the space the solver offers it, from the shaped
/// advances alone — the worker thread holds no tree and no font atlas. This is CSS's own reading of
/// a text run inside a flex item: `MaxContent` is the whole run on one line, `MinContent` is the
/// widest unbreakable word (the floor a flex item may shrink to before it overflows), and a
/// `Definite` width is first-fit greedy wrapping at [`may_break_between`]'s own CSS break
/// opportunities — the SAME predicate the atlas wrap and the retained painter use, so a flex item
/// sized here and a paragraph painted there can no longer disagree about where a line ends. A
/// trailing space hangs and never pushes a line over the edge.
fn measure_text(nodes: &ui_contract::UiFixedList<LayoutInputNode, LAYOUT_NODE_CREDITS>, glyphs: &ui_contract::UiFixedList<RetainedGlyphInput, LAYOUT_GLYPH_CREDITS>, previews: &ui_contract::UiFixedList<RetainedGlyphPreview, LAYOUT_GLYPH_CREDITS>, index: usize, constraint: MeasureConstraint) -> (f32, f32) {
    let Some(node) = nodes.get(index) else { return (0.0, 0.0) };
    let (start, end) = (node.glyph_start, node.glyph_end);
    if end <= start {
        return (0.0, 0.0);
    }
    let line = crate::wgpu::text::line_height(DEFAULT_TEXT_SIZE_PX);
    let advance = |cursor: usize| previews.get(cursor).map_or(0.0, |preview| preview.advance);
    let scalar = |cursor: usize| glyphs.get(cursor).map_or(' ', |glyph| glyph.scalar);
    match constraint {
        MeasureConstraint::MaxContent => ((start..end).map(advance).sum(), line),
        MeasureConstraint::MinContent => {
            let (mut widest, mut run) = (0.0_f32, 0.0_f32);
            for cursor in start..end {
                let ch = scalar(cursor);
                if ch == '\n' || is_wrap_space(ch) || (cursor > start && may_break_between(scalar(cursor - 1), ch)) {
                    widest = widest.max(run);
                    run = 0.0;
                }
                if ch != '\n' && !is_wrap_space(ch) {
                    run += advance(cursor);
                }
            }
            (widest.max(run), line)
        }
        MeasureConstraint::Definite(available) => {
            let (mut widest, mut placed, mut run, mut ink, mut lines) = (0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32, 1_usize);
            for cursor in start..end {
                let ch = scalar(cursor);
                if ch == '\n' {
                    widest = widest.max(ink);
                    (placed, run, ink) = (0.0, 0.0, 0.0);
                    lines += 1;
                    continue;
                }
                if cursor > start && placed + run > 0.0 && may_break_between(scalar(cursor - 1), ch) {
                    placed += run;
                    run = 0.0;
                }
                let advance = advance(cursor);
                if is_wrap_space(ch) {
                    placed += run + advance;
                    run = 0.0;
                    continue;
                }
                if placed + run > 0.0 && placed + run + advance > available {
                    widest = widest.max(ink);
                    if placed > 0.0 {
                        placed = 0.0;
                    } else {
                        run = 0.0;
                    }
                    lines += 1;
                }
                run += advance;
                ink = placed + run;
            }
            (widest.max(ink), line * lines as f32)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MountedLayoutResult {
    pub id: NodeId,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AdmissionPhase {
    Visit,
    Text,
    Unwind,
    Ready,
}

#[derive(Clone, Copy)]
struct WalkFrame {
    next_child: Option<NodeId>,
    node: usize,
}

/// 📦️ Exact fixed working-set owner retained across admission, worker, preview, and close.
pub(crate) struct MountedLayoutJob {
    stage: LayoutJobStage,
    root: NodeId,
    surface: UiSurfaceToken,
    generation: u64,
    revision: u64,
    theme_revision: u64,
    viewport_revision: u64,
    theme: Theme,
    row_metrics: TreeRowMetrics,
    width: f32,
    height: f32,
    admission: AdmissionPhase,
    pending_node: Option<(NodeId, Option<usize>)>,
    walk: Box<ui_contract::UiFixedList<WalkFrame, LAYOUT_DEPTH_CREDITS>>,
    nodes: Box<ui_contract::UiFixedList<LayoutInputNode, LAYOUT_NODE_CREDITS>>,
    glyphs: Box<ui_contract::UiFixedList<RetainedGlyphInput, LAYOUT_GLYPH_CREDITS>>,
    runs: Box<ui_contract::UiFixedList<RetainedTextRun, LAYOUT_NODE_CREDITS>>,
    lines: Box<ui_contract::UiFixedList<RetainedLine, LAYOUT_NODE_CREDITS>>,
    glyph_previews: Box<ui_contract::UiFixedList<RetainedGlyphPreview, LAYOUT_GLYPH_CREDITS>>,
    atlas_candidate: RetainedAtlasCandidate,
    results: Box<ui_contract::UiFixedList<MountedLayoutResult, LAYOUT_NODE_CREDITS>>,
    rejected_node: Option<LayoutInputNode>,
    rejected_walk: Option<WalkFrame>,
    rejected_glyph: Option<RetainedGlyphInput>,
    rejected_preview: Option<RetainedGlyphPreview>,
    rejected_run: Option<RetainedTextRun>,
    rejected_line: Option<RetainedLine>,
    rejected_result: Option<MountedLayoutResult>,
    text_node: Option<usize>,
    text_byte: usize,
    text_glyph_start: usize,
    run_cursor: usize,
    glyph_cursor: usize,
    line_cursor: usize,
    measure_cursor: usize,
    arrange_cursor: usize,
    collect_cursor: usize,
    child_scratch: Vec<usize>,
    flex: FlexTree,
    preview_cursor: usize,
    publish_cursor: usize,
    publication_committed: bool,
    close_requested: bool,
    #[cfg(test)]
    worker_thread_observed: bool,
    fault: Option<MountedLayoutFault>,
    text_worker: DeterministicTextWorker,
}

#[derive(Clone, Copy)]
pub(crate) struct MountedLayoutIdentity {
    pub surface: UiSurfaceToken,
    pub generation: u64,
    pub revision: u64,
    pub theme_revision: u64,
    pub viewport_revision: u64,
}

impl MountedLayoutJob {
    pub(crate) fn try_new(tree: &UiTree, root: NodeId, identity: MountedLayoutIdentity, theme: Theme, width: f32, height: f32) -> Result<Self, MountedLayoutFault> {
        let MountedLayoutIdentity { surface, generation, revision, theme_revision, viewport_revision } = identity;
        let root_node = tree.node(root).ok_or(MountedLayoutFault::Stale)?;
        if !root_node.flags.contains(NodeFlags::DIRTY_LAYOUT) && !root_node.flags.contains(NodeFlags::SUBTREE_DIRTY) {
            return Err(MountedLayoutFault::Stale);
        }
        Ok(Self {
            stage: LayoutJobStage::CollectNodes,
            root,
            surface,
            generation,
            revision,
            theme_revision,
            viewport_revision,
            row_metrics: TreeRowMetrics::from_theme(&theme),
            theme,
            width,
            height,
            admission: AdmissionPhase::Visit,
            pending_node: Some((root, None)),
            walk: Box::new(ui_contract::UiFixedList::default()),
            nodes: Box::new(ui_contract::UiFixedList::default()),
            glyphs: Box::new(ui_contract::UiFixedList::default()),
            runs: Box::new(ui_contract::UiFixedList::default()),
            lines: Box::new(ui_contract::UiFixedList::default()),
            glyph_previews: Box::new(ui_contract::UiFixedList::default()),
            atlas_candidate: RetainedAtlasCandidate::new(),
            results: Box::new(ui_contract::UiFixedList::default()),
            rejected_node: None,
            rejected_walk: None,
            rejected_glyph: None,
            rejected_preview: None,
            rejected_run: None,
            rejected_line: None,
            rejected_result: None,
            text_node: None,
            text_byte: 0,
            text_glyph_start: 0,
            run_cursor: 0,
            glyph_cursor: 0,
            line_cursor: 0,
            measure_cursor: 0,
            arrange_cursor: 0,
            collect_cursor: 0,
            child_scratch: Vec::new(),
            flex: FlexTree::new(),
            preview_cursor: 0,
            publish_cursor: 0,
            publication_committed: false,
            close_requested: false,
            #[cfg(test)]
            worker_thread_observed: false,
            fault: None,
            text_worker: DeterministicTextWorker::default(),
        })
    }

    pub(crate) fn identity(&self) -> (UiSurfaceToken, u64, u64, u64, u64, f32, f32) {
        (self.surface, self.generation, self.revision, self.theme_revision, self.viewport_revision, self.width, self.height)
    }

    pub(crate) fn stage(&self) -> LayoutJobStage {
        self.stage
    }

    pub(crate) fn is_admitted(&self) -> bool {
        self.stage != LayoutJobStage::CollectNodes
    }

    pub(crate) fn admit_one(&mut self, tree: &UiTree, cx: &mut semio_framework_job::StepContext<'_>) -> LayoutJobStep {
        if cx.is_cancelled() {
            return LayoutJobStep::Cancelled;
        }
        if cx.should_yield() {
            return LayoutJobStep::Yield { stage: self.stage, nodes: 0, glyphs: 0 };
        }
        cx.set_stage("Layout.AdmitOne");
        let progress = match self.admission {
            AdmissionPhase::Visit => self.admit_node_one(tree),
            AdmissionPhase::Text => self.admit_text_one(tree),
            AdmissionPhase::Unwind => self.unwind_one(tree),
            AdmissionPhase::Ready => {
                self.stage = LayoutJobStage::ShapeText;
                (0, 0)
            }
        };
        cx.consume_fuel(1);
        if let Some(fault) = self.fault {
            return LayoutJobStep::Fault(fault.label());
        }
        LayoutJobStep::Yield { stage: self.stage, nodes: progress.0, glyphs: progress.1 }
    }

    /// 🧩️ The panel content-projection ESCAPE HATCH. An `ExternalSlot` is the one node whose
    /// pixels this engine does not author: its host paints inside the solved rect. It used to
    /// fall into `Leaf`, which measures from arena children — a slot has none, so the band
    /// collapsed to nothing and any host-provided content (React hosts an arbitrary subtree
    /// here through `Tree`'s `emptyState`: an agent chat transcript, a marketplace list) had no
    /// box at all. The host declares its band in the slot's own `params_json`
    /// (`{"hostContentHeight": <px>}`), so the reservation costs one node and no measurement,
    /// and every other panel's node credit is untouched.
    fn admit_node_one(&mut self, tree: &UiTree) -> (usize, usize) {
        let Some((id, parent)) = self.pending_node.take() else {
            self.admission = AdmissionPhase::Unwind;
            return (0, 0);
        };
        let Some(node) = tree.node(id) else {
            self.fault = Some(MountedLayoutFault::Stale);
            return (0, 0);
        };
        let kind = match &node.spec.0 {
            UiNode::Text(_) => LayoutNodeKind::Text,
            UiNode::Tree(tree_node) => LayoutNodeKind::Tree { height: tree_node_height(tree_node, &self.row_metrics) },
            UiNode::Stack(stack) => tree_row_kind(tree, id, parent.and_then(|index| self.nodes.get(index)).map(|input| input.kind), &self.row_metrics).unwrap_or(LayoutNodeKind::Stack {
                horizontal: stack.direction == "horizontal",
                gap: gap_for_token(&self.theme, stack.gap.as_deref()),
                padding: padding_for_token(&self.theme, stack.padding.as_deref()),
            }),
            UiNode::Field(_) => LayoutNodeKind::Field { top: self.theme.font_size_small + gap_for_token(&self.theme, Some("standard")) },
            UiNode::Section(_) => LayoutNodeKind::Section { gap: self.theme.gap_standard },
            UiNode::Button(_) => LayoutNodeKind::Control { height: self.theme.control_height, label_padding: Some(self.theme.padding_standard) },
            UiNode::Input(_) | UiNode::Select(_) | UiNode::Toggle(_) | UiNode::Slider(_) | UiNode::NumberStepper(_) | UiNode::Ring(_) | UiNode::IconSelect(_) => {
                LayoutNodeKind::Control { height: self.theme.control_height, label_padding: None }
            }
            UiNode::ExternalSlot(slot) => LayoutNodeKind::HostContent { height: host_content_height(&slot.params_json, &self.theme) },
            UiNode::ComponentScene(_) => LayoutNodeKind::EngineSurface,
            _ => LayoutNodeKind::Leaf,
        };
        let index = self.nodes.len();
        let parent_kind = parent.and_then(|index| self.nodes.get(index)).map(|input| input.kind);
        let input = LayoutInputNode { id, parent, first_child: None, last_child: None, next_sibling: None, kind, intrinsic: IntrinsicSize::default(), glyph_start: 0, glyph_end: 0 };
        if let Err(owner) = self.nodes.try_push(input) {
            self.rejected_node = Some(owner);
            self.fault = Some(MountedLayoutFault::NodeCredits);
            return (0, 0);
        }
        let metrics = self.row_metrics;
        if !self.flex.push(kind, parent, node.layout_spec.as_ref(), &metrics, parent_kind) {
            self.fault = Some(MountedLayoutFault::Solver);
            return (0, 0);
        }
        if let Some(parent) = parent {
            let previous = self.nodes.get(parent).and_then(|owner| owner.last_child);
            match previous {
                Some(sibling) => {
                    if let Some(sibling) = self.nodes.get_mut(sibling) {
                        sibling.next_sibling = Some(index);
                    }
                }
                None => {
                    if let Some(owner) = self.nodes.get_mut(parent) {
                        owner.first_child = Some(index);
                    }
                }
            }
            if let Some(owner) = self.nodes.get_mut(parent) {
                owner.last_child = Some(index);
            }
        }
        if let Err(owner) = self.walk.try_push(WalkFrame { next_child: node.first_child, node: index }) {
            self.rejected_walk = Some(owner);
            self.fault = Some(MountedLayoutFault::DepthCredits);
            return (0, 0);
        }
        if matches!(kind, LayoutNodeKind::Text | LayoutNodeKind::Control { label_padding: Some(_), .. }) {
            self.text_node = Some(index);
            self.text_byte = 0;
            self.text_glyph_start = self.glyphs.len();
            self.admission = AdmissionPhase::Text;
        } else {
            self.admission = AdmissionPhase::Unwind;
        }
        (1, 0)
    }

    fn admit_text_one(&mut self, tree: &UiTree) -> (usize, usize) {
        let Some(index) = self.text_node else {
            self.admission = AdmissionPhase::Unwind;
            return (0, 0);
        };
        let Some(input) = self.nodes.get(index) else {
            self.fault = Some(MountedLayoutFault::Stale);
            return (0, 0);
        };
        let value = match tree.node(input.id).map(|node| &node.spec.0) {
            Some(UiNode::Text(text)) => text.value.as_str(),
            Some(UiNode::Button(button)) => button.label.as_str(),
            _ => {
                self.fault = Some(MountedLayoutFault::Stale);
                return (0, 0);
            }
        };
        let Some(scalar) = value.get(self.text_byte..).and_then(|tail| tail.chars().next()) else {
            let run = RetainedTextRun { node: index, glyph_start: self.text_glyph_start, glyph_end: self.glyphs.len() };
            if let Err(owner) = self.runs.try_push(run) {
                self.rejected_run = Some(owner);
                self.fault = Some(MountedLayoutFault::NodeCredits);
                return (0, 0);
            }
            if let Some(node) = self.nodes.get_mut(index) {
                node.glyph_start = run.glyph_start;
                node.glyph_end = run.glyph_end;
            }
            self.text_node = None;
            self.text_byte = 0;
            self.admission = AdmissionPhase::Unwind;
            return (0, 0);
        };
        self.text_byte += scalar.len_utf8();
        if let Err(owner) = self.glyphs.try_push(RetainedGlyphInput { node: index, scalar }) {
            self.rejected_glyph = Some(owner);
            self.fault = Some(MountedLayoutFault::GlyphCredits);
            return (0, 0);
        }
        (0, 1)
    }

    fn unwind_one(&mut self, tree: &UiTree) -> (usize, usize) {
        let Some(frame) = self.walk.last_mut() else {
            self.admission = AdmissionPhase::Ready;
            return (0, 0);
        };
        if let Some(child) = frame.next_child {
            frame.next_child = tree.node(child).and_then(|node| node.next_sibling);
            self.pending_node = Some((child, Some(frame.node)));
            self.admission = AdmissionPhase::Visit;
            return (0, 0);
        }
        self.walk.pop();
        (1, 0)
    }

    fn worker_one(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        #[cfg(test)]
        { self.worker_thread_observed = std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")); }
        if self.close_requested || cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if cx.should_yield() {
            return semio_framework_job::StepOutcome::Yield;
        }
        cx.set_stage(self.stage_label());
        match self.stage {
            LayoutJobStage::ShapeText => self.shape_one(),
            LayoutJobStage::MeasureLayout => self.measure_one(),
            LayoutJobStage::SolveLayout => self.arrange_one(),
            LayoutJobStage::CollectResults => self.collect_one(),
            _ => (0, 0),
        };
        cx.consume_fuel(1);
        if cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if self.fault.is_some() {
            return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
        }
        if cx.deadline_exceeded() {
            return semio_framework_job::StepOutcome::Yield;
        }
        if self.stage == LayoutJobStage::PublishResults {
            semio_framework_job::StepOutcome::PreviewReady(semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Preview))
        } else {
            semio_framework_job::StepOutcome::Yield
        }
    }

    fn shape_one(&mut self) -> (usize, usize) {
        let Some(run) = self.runs.get(self.run_cursor).copied() else {
            if self.flex.len() != self.nodes.len() {
                self.fault = Some(MountedLayoutFault::Solver);
                return (0, 0);
            }
            self.measure_cursor = self.nodes.len();
            self.stage = LayoutJobStage::MeasureLayout;
            return (0, 0);
        };
        if self.glyph_cursor < run.glyph_start {
            self.glyph_cursor = run.glyph_start;
            return (0, 0);
        }
        if self.glyph_cursor >= run.glyph_end {
            self.run_cursor += 1;
            return (0, 0);
        }
        let Some(input) = self.glyphs.get(self.glyph_cursor).copied() else {
            self.fault = Some(MountedLayoutFault::Stale);
            return (0, 0);
        };
        let raw_preview = self.text_worker.shape_one(input);
        let preview = match self.atlas_candidate.retain_one(input.scalar, self.generation, self.revision, raw_preview) {
            Ok(Some(preview)) => preview,
            Ok(None) => return (0, 0),
            Err(fault) => {
                self.fault = Some(fault);
                return (0, 0);
            }
        };
        if let Err(owner) = self.glyph_previews.try_push(preview) {
            self.rejected_preview = Some(owner);
            self.fault = Some(MountedLayoutFault::GlyphCredits);
            return (0, 0);
        }
        if let Some(node) = self.nodes.get_mut(input.node) {
            node.intrinsic.width += preview.advance;
            node.intrinsic.height = node.intrinsic.height.max(preview.height);
        }
        self.glyph_cursor += 1;
        (0, 1)
    }

    /// 👶️ Collects node `index`'s direct children into the reusable scratch buffer, in document
    /// order — the sibling links admission built, so no scan over the node array is ever needed.
    fn gather_children(&mut self, index: usize) {
        self.child_scratch.clear();
        let mut cursor = self.nodes.get(index).and_then(|owner| owner.first_child);
        while let Some(child) = cursor {
            self.child_scratch.push(child);
            cursor = self.nodes.get(child).and_then(|owner| owner.next_sibling);
        }
    }

    /// 📏️ Measures exactly ONE node's intrinsic size, walking the admission order in REVERSE so every
    /// child is already measured when its parent is reached. Work in this grant is that node's child
    /// count, never its subtree.
    fn measure_one(&mut self) -> (usize, usize) {
        let Some(index) = self.measure_cursor.checked_sub(1) else {
            self.arrange_cursor = 0;
            let root = FlexRect { x: 0.0, y: 0.0, width: self.width, height: self.height };
            if !self.flex.set_root_box(0, root) {
                self.fault = Some(MountedLayoutFault::Solver);
            }
            self.stage = LayoutJobStage::SolveLayout;
            return (0, 0);
        };
        self.measure_cursor = index;
        self.gather_children(index);
        let Self { flex, nodes, glyphs, glyph_previews, child_scratch, .. } = self;
        let (nodes_ref, glyphs_ref, previews_ref) = (&**nodes, &**glyphs, &**glyph_previews);
        let mut measure = |node: usize, constraint: MeasureConstraint| measure_text(nodes_ref, glyphs_ref, previews_ref, node, constraint);
        if !flex.measure_one(index, child_scratch, &mut measure) {
            self.fault = Some(MountedLayoutFault::Solver);
            return (0, 0);
        }
        (1, 0)
    }

    /// 📐️ Arranges exactly ONE container's direct children inside its own already-resolved box,
    /// walking the admission order FORWARD so a parent's box is always settled before its children
    /// are placed. This is the grant the eight-millisecond slice law is charged against: its cost is
    /// the container's child count, so a 1,025-node surface is ~1,025 small solves, never one large one.
    fn arrange_one(&mut self) -> (usize, usize) {
        if self.nodes.get(self.arrange_cursor).is_none() {
            self.stage = LayoutJobStage::CollectResults;
            return (0, 0);
        }
        let index = self.arrange_cursor;
        self.arrange_cursor += 1;
        self.gather_children(index);
        if self.child_scratch.is_empty() {
            return (1, 0);
        }
        let Self { flex, nodes, glyphs, glyph_previews, child_scratch, .. } = self;
        let (nodes_ref, glyphs_ref, previews_ref) = (&**nodes, &**glyphs, &**glyph_previews);
        let mut measure = |node: usize, constraint: MeasureConstraint| measure_text(nodes_ref, glyphs_ref, previews_ref, node, constraint);
        if !flex.arrange_one(index, child_scratch, &mut measure) {
            self.fault = Some(MountedLayoutFault::Solver);
            return (0, 0);
        }
        (1, 0)
    }

    /// 📐️ Reads exactly ONE solved box back out of the flex tree, in admission order so
    /// `results[i]` stays the box of `nodes[i]` — the invariant `publish_one` re-checks.
    fn collect_one(&mut self) -> (usize, usize) {
        let Some(input) = self.nodes.get(self.collect_cursor).copied() else {
            self.stage = LayoutJobStage::PublishResults;
            return (0, 0);
        };
        let Some(rect) = self.flex.rect(self.collect_cursor) else {
            self.fault = Some(MountedLayoutFault::Solver);
            return (0, 0);
        };
        let index = self.collect_cursor;
        self.collect_cursor += 1;
        if matches!(input.kind, LayoutNodeKind::Text | LayoutNodeKind::Control { label_padding: Some(_), .. }) {
            if let Err(owner) = self.lines.try_push(RetainedLine { node: index, width: rect.width, height: rect.height }) {
                self.rejected_line = Some(owner);
                self.fault = Some(MountedLayoutFault::NodeCredits);
                return (0, 0);
            }
            self.line_cursor += 1;
        }
        if let Err(owner) = self.results.try_push(MountedLayoutResult { id: input.id, x: rect.x, y: rect.y, width: rect.width, height: rect.height }) {
            self.rejected_result = Some(owner);
            self.fault = Some(MountedLayoutFault::NodeCredits);
            return (0, 0);
        }
        (1, 0)
    }

    pub(crate) fn take_preview_one(&mut self) -> Option<MountedLayoutResult> {
        let preview = self.results.get(self.preview_cursor).copied()?;
        self.preview_cursor += 1;
        Some(preview)
    }

    pub(crate) fn latest_glyph_preview(&self) -> Option<RetainedGlyphPreview> {
        self.glyph_previews.get(self.glyph_cursor.saturating_sub(1)).copied()
    }

    #[cfg(test)]
    pub(crate) fn worker_thread_observed(&self) -> bool {
        self.worker_thread_observed
    }

    #[cfg(test)]
    fn rejected_glyph(&self) -> Option<RetainedGlyphInput> {
        self.rejected_glyph
    }

    #[cfg(test)]
    fn rejected_node(&self) -> Option<LayoutInputNode> {
        self.rejected_node
    }

    #[cfg(test)]
    fn cancel_after_shape(&mut self, cancel: semio_framework_job::CancelToken) {
        self.text_worker.cancel_after_shape = Some(cancel);
    }

    pub(crate) fn publish_one(&mut self, tree: &mut UiTree, identity: (UiSurfaceToken, u64, u64, u64, u64, f32, f32)) -> LayoutJobStep {
        if self.publication_committed {
            return LayoutJobStep::Complete;
        }
        if self.identity() != identity {
            self.fault = Some(MountedLayoutFault::Stale);
            return LayoutJobStep::Fault(MountedLayoutFault::Stale.label());
        }
        let generation = identity.1;
        let Some(result) = self.results.get(self.publish_cursor).copied() else {
            if self.results.len() != self.nodes.len() || self.glyph_previews.len() != self.glyphs.len() || self.lines.len() != self.runs.len() {
                self.fault = Some(MountedLayoutFault::Stale);
                return LayoutJobStep::Fault(MountedLayoutFault::Stale.label());
            }
            tree.commit_inactive_layout(generation);
            self.publication_committed = true;
            if let Some(root) = tree.node_mut(self.root) {
                root.flags.set(NodeFlags::DIRTY_LAYOUT, false);
                root.flags.set(NodeFlags::SUBTREE_DIRTY, false);
            }
            return LayoutJobStep::Complete;
        };
        if !tree.write_inactive_layout(result.id, generation, AcceptedLayout { x: result.x, y: result.y, width: result.width, height: result.height }) {
            self.fault = Some(MountedLayoutFault::Stale);
            return LayoutJobStep::Fault(MountedLayoutFault::Stale.label());
        }
        self.publish_cursor += 1;
        LayoutJobStep::Yield { stage: LayoutJobStage::PublishResults, nodes: 1, glyphs: 0 }
    }

    fn stage_label(&self) -> &'static str {
        match self.stage {
            LayoutJobStage::CollectNodes => "Layout.CollectNodes",
            LayoutJobStage::ShapeText => "Layout.ShapeText",
            LayoutJobStage::MeasureLayout => "Layout.MeasureLayout",
            LayoutJobStage::SolveLayout => "Layout.SolveLayout",
            LayoutJobStage::CollectResults => "Layout.CollectResults",
            LayoutJobStage::PublishResults => "Layout.PublishResults",
        }
    }

    pub(crate) fn begin_close(&mut self) {
        self.close_requested = true;
    }

    pub(crate) fn close_one(&mut self) -> bool {
        if self.rejected_result.take().is_some()
            || self.rejected_line.take().is_some()
            || self.rejected_glyph.take().is_some()
            || self.rejected_preview.take().is_some()
            || self.rejected_run.take().is_some()
            || self.rejected_walk.take().is_some()
            || self.rejected_node.take().is_some()
            || self.results.pop().is_some()
            || self.lines.pop().is_some()
            || self.glyph_previews.pop().is_some()
            || self.glyphs.pop().is_some()
            || self.runs.pop().is_some()
            || self.walk.pop().is_some()
            || self.nodes.pop().is_some()
        {
            return false;
        }
        if !self.child_scratch.is_empty() {
            self.child_scratch.clear();
            return false;
        }
        if !self.flex.release_one() {
            return false;
        }
        self.atlas_candidate.close_one()
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.rejected_result.is_none()
            && self.rejected_line.is_none()
            && self.rejected_glyph.is_none()
            && self.rejected_preview.is_none()
            && self.rejected_run.is_none()
            && self.rejected_walk.is_none()
            && self.rejected_node.is_none()
            && self.results.is_empty()
            && self.lines.is_empty()
            && self.glyph_previews.is_empty()
            && self.glyphs.is_empty()
            && self.runs.is_empty()
            && self.walk.is_empty()
            && self.nodes.is_empty()
            && self.child_scratch.is_empty()
            && self.flex.is_empty()
            && self.atlas_candidate.is_empty()
    }
}

/// 🧪️ Drives ONE whole bounded layout pass to completion in a single call and mirrors the solved
/// boxes into both the accepted (double-buffered) layout the painter reads and the immediate-mode
/// `LayoutBucket` older probes read. It drives the REAL [`MountedLayoutJob`] stage ladder rather than
/// re-deriving geometry, so a suite laying a tree out this way can never disagree with the shipped
/// renderer, which walks the identical stages one grant at a time through `engine::Ui`.
#[cfg(any(test, feature = "testkit"))]
pub(crate) fn layout_tree_now(tree: &mut UiTree, root: NodeId, theme: Theme, width: f32, height: f32) -> bool {
    let identity = MountedLayoutIdentity { surface: UiSurfaceToken::new(0, 1), generation: 1, revision: 0, theme_revision: 0, viewport_revision: 0 };
    let Ok(mut job) = MountedLayoutJob::try_new(tree, root, identity, theme, width, height) else { return false };
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview = 0;
    while !job.is_admitted() {
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(0), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), || Some(0), &mut preview);
        if matches!(job.admit_one(tree, &mut cx), LayoutJobStep::Fault(_) | LayoutJobStep::Cancelled) {
            return false;
        }
    }
    while job.stage() != LayoutJobStage::PublishResults {
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(0), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), || Some(0), &mut preview);
        if matches!(semio_framework_job::InteractiveJob::step(&mut job, &mut cx), semio_framework_job::StepOutcome::Fault(_) | semio_framework_job::StepOutcome::Cancelled) {
            return false;
        }
    }
    while let Some(result) = job.take_preview_one() {
        if let Some(node) = tree.node_mut(result.id) {
            node.layout.x = result.x;
            node.layout.y = result.y;
            node.layout.width = result.width;
            node.layout.height = result.height;
        }
    }
    let published = job.identity();
    loop {
        match job.publish_one(tree, published) {
            LayoutJobStep::Complete => break,
            LayoutJobStep::Fault(_) | LayoutJobStep::Cancelled => return false,
            LayoutJobStep::Yield { .. } => {}
        }
    }
    job.begin_close();
    while !job.close_one() {}
    true
}

impl MountedLayoutFault {
    const fn label(self) -> &'static str {
        match self {
            Self::NodeCredits => "layout.node-credits",
            Self::GlyphCredits => "layout.glyph-credits",
            Self::DepthCredits => "layout.depth-credits",
            Self::Stale => "layout.stale",
            Self::Solver => "layout.solver",
        }
    }
}

impl semio_framework_job::InteractiveJob for MountedLayoutJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        self.worker_one(cx)
    }

    fn begin_close(&mut self) {
        MountedLayoutJob::begin_close(self);
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.close_one() {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        MountedLayoutJob::terminal_is_empty(self)
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs"]
mod tests;
//#endregion 🧵️MountedLayoutText
