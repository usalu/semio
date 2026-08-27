//#region 🧵️MountedLayoutText
//! 🧵️ Fixed-credit mounted layout and text worker with one cursor opportunity per grant.

use crate::wgpu::arena::NodeId;
use crate::wgpu::component::ui::UiNode;
use crate::wgpu::engine::UiSurfaceToken;
use crate::wgpu::flex::{LayoutJobStage, LayoutJobStep};
use crate::wgpu::layout::{gap_for_token, padding_for_token};
use crate::wgpu::theme::Theme;
use crate::wgpu::tree::{AcceptedLayout, NodeFlags, UiTree};

pub(crate) const LAYOUT_NODE_CREDITS: usize = 4_096;
pub(crate) const LAYOUT_GLYPH_CREDITS: usize = 16_384;
pub(crate) const LAYOUT_DEPTH_CREDITS: usize = 64;
pub(crate) const LAYOUT_ATLAS_PAGE_CREDITS: usize = 4;
pub(crate) const LAYOUT_ATLAS_PAGE_BYTES: usize = 16 * 1024;
const DEFAULT_TEXT_SIZE_PX: f32 = 14.0;
const SECTION_HEADER_HEIGHT: f32 = 24.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MountedLayoutFault {
    NodeCredits,
    GlyphCredits,
    DepthCredits,
    Stale,
    RevisionExhausted,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct IntrinsicSize {
    width: f32,
    height: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct ChildAggregate {
    count: usize,
    width_sum: f32,
    height_sum: f32,
    max_width: f32,
    max_height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum LayoutNodeKind {
    Text,
    Stack { horizontal: bool, gap: f32, padding: f32 },
    Field { top: f32 },
    Section { gap: f32 },
    Leaf,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct LayoutInputNode {
    id: NodeId,
    parent: Option<usize>,
    kind: LayoutNodeKind,
    intrinsic: IntrinsicSize,
    children: ChildAggregate,
    child_offset: f32,
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
        RetainedGlyphPreview { scalar: input.scalar, advance, height: DEFAULT_TEXT_SIZE_PX * 1.35, generation: 0, revision: 0, atlas_page: 0, atlas_offset: 0, atlas_length: 0 }
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
    preview_cursor: usize,
    publish_cursor: usize,
    publication_committed: bool,
    close_requested: bool,
    worker_thread_observed: bool,
    fault: Option<MountedLayoutFault>,
    text_worker: DeterministicTextWorker,
}

impl MountedLayoutJob {
    pub(crate) fn try_new(tree: &UiTree, root: NodeId, surface: UiSurfaceToken, generation: u64, revision: u64, theme_revision: u64, viewport_revision: u64, theme: Theme, width: f32, height: f32) -> Result<Self, MountedLayoutFault> {
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
            preview_cursor: 0,
            publish_cursor: 0,
            publication_committed: false,
            close_requested: false,
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
                self.measure_cursor = self.nodes.len();
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
            UiNode::Stack(stack) => LayoutNodeKind::Stack { horizontal: stack.direction == "horizontal", gap: gap_for_token(&self.theme, stack.gap.as_deref()), padding: padding_for_token(&self.theme, stack.padding.as_deref()) },
            UiNode::Field(_) => LayoutNodeKind::Field { top: self.theme.font_size_small + gap_for_token(&self.theme, Some("standard")) },
            UiNode::Section(_) => LayoutNodeKind::Section { gap: self.theme.gap_standard },
            _ => LayoutNodeKind::Leaf,
        };
        let index = self.nodes.len();
        let input = LayoutInputNode { id, parent, kind, intrinsic: IntrinsicSize::default(), children: ChildAggregate::default(), child_offset: 0.0 };
        if let Err(owner) = self.nodes.try_push(input) {
            self.rejected_node = Some(owner);
            self.fault = Some(MountedLayoutFault::NodeCredits);
            return (0, 0);
        }
        if let Err(owner) = self.walk.try_push(WalkFrame { next_child: node.first_child, node: index }) {
            self.rejected_walk = Some(owner);
            self.fault = Some(MountedLayoutFault::DepthCredits);
            return (0, 0);
        }
        if matches!(kind, LayoutNodeKind::Text) {
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
        self.worker_thread_observed = std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-"));
        if self.close_requested || cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if cx.should_yield() {
            return semio_framework_job::StepOutcome::Yield;
        }
        cx.set_stage(self.stage_label());
        match self.stage {
            LayoutJobStage::ShapeText => self.shape_one(),
            LayoutJobStage::MeasureFallback => self.measure_one(),
            LayoutJobStage::ArrangeFallback => self.arrange_one(),
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
            self.stage = LayoutJobStage::MeasureFallback;
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

    fn measure_one(&mut self) -> (usize, usize) {
        let Some(index) = self.measure_cursor.checked_sub(1) else {
            self.stage = LayoutJobStage::ArrangeFallback;
            return (0, 0);
        };
        self.measure_cursor = index;
        let Some(input) = self.nodes.get(index).copied() else {
            self.fault = Some(MountedLayoutFault::Stale);
            return (0, 0);
        };
        let aggregate = input.children;
        let size = match input.kind {
            LayoutNodeKind::Text => input.intrinsic,
            LayoutNodeKind::Stack { horizontal: true, gap, padding } => IntrinsicSize { width: aggregate.width_sum + gap * aggregate.count.saturating_sub(1) as f32 + padding * 2.0, height: aggregate.max_height + padding * 2.0 },
            LayoutNodeKind::Stack { horizontal: false, gap, padding } => IntrinsicSize { width: aggregate.max_width + padding * 2.0, height: aggregate.height_sum + gap * aggregate.count.saturating_sub(1) as f32 + padding * 2.0 },
            LayoutNodeKind::Field { top } => IntrinsicSize { width: aggregate.max_width, height: aggregate.height_sum + top },
            LayoutNodeKind::Section { gap } => IntrinsicSize { width: aggregate.max_width, height: aggregate.height_sum + SECTION_HEADER_HEIGHT + gap * aggregate.count.saturating_sub(1) as f32 },
            LayoutNodeKind::Leaf => IntrinsicSize { width: aggregate.max_width, height: aggregate.height_sum },
        };
        if matches!(input.kind, LayoutNodeKind::Text) {
            if let Err(owner) = self.lines.try_push(RetainedLine { node: index, width: size.width, height: size.height }) {
                self.rejected_line = Some(owner);
                self.fault = Some(MountedLayoutFault::NodeCredits);
                return (0, 0);
            }
            self.line_cursor += 1;
        }
        if let Some(node) = self.nodes.get_mut(index) {
            node.intrinsic = size;
        }
        if let Some(parent) = input.parent.and_then(|parent| self.nodes.get_mut(parent)) {
            parent.children.count += 1;
            parent.children.width_sum += size.width;
            parent.children.height_sum += size.height;
            parent.children.max_width = parent.children.max_width.max(size.width);
            parent.children.max_height = parent.children.max_height.max(size.height);
        }
        (1, 0)
    }

    fn arrange_one(&mut self) -> (usize, usize) {
        let Some(input) = self.nodes.get(self.arrange_cursor).copied() else {
            self.stage = LayoutJobStage::PublishResults;
            return (0, 0);
        };
        let (x, y, width, height) = if input.id == self.root {
            (0.0, 0.0, self.width, self.height)
        } else {
            let Some(parent_index) = input.parent else {
                self.fault = Some(MountedLayoutFault::Stale);
                return (0, 0);
            };
            let Some(parent) = self.nodes.get(parent_index).copied() else {
                self.fault = Some(MountedLayoutFault::Stale);
                return (0, 0);
            };
            let Some(parent_result) = self.results.get(parent_index).copied() else {
                self.fault = Some(MountedLayoutFault::Stale);
                return (0, 0);
            };
            match parent.kind {
                LayoutNodeKind::Stack { horizontal: true, gap, padding } => {
                    let content = (parent_result.width - padding * 2.0).max(0.0);
                    let extra = ((content - parent.children.width_sum - gap * parent.children.count.saturating_sub(1) as f32) / parent.children.count.max(1) as f32).max(0.0);
                    (padding + parent.child_offset, padding, input.intrinsic.width + extra, (parent_result.height - padding * 2.0).max(0.0))
                }
                LayoutNodeKind::Stack { horizontal: false, gap, padding } => {
                    let content = (parent_result.height - padding * 2.0).max(0.0);
                    let extra = ((content - parent.children.height_sum - gap * parent.children.count.saturating_sub(1) as f32) / parent.children.count.max(1) as f32).max(0.0);
                    (padding, padding + parent.child_offset, (parent_result.width - padding * 2.0).max(0.0), input.intrinsic.height + extra)
                }
                LayoutNodeKind::Field { top } => (0.0, top, parent_result.width, (parent_result.height - top).max(0.0)),
                LayoutNodeKind::Section { .. } => (0.0, SECTION_HEADER_HEIGHT + parent.child_offset, parent_result.width, input.intrinsic.height),
                _ => (0.0, parent.child_offset, parent_result.width, input.intrinsic.height),
            }
        };
        if let Err(owner) = self.results.try_push(MountedLayoutResult { id: input.id, x, y, width, height }) {
            self.rejected_result = Some(owner);
            self.fault = Some(MountedLayoutFault::NodeCredits);
            return (0, 0);
        }
        if let Some(parent_index) = input.parent {
            let parent_kind = self.nodes.get(parent_index).map(|parent| parent.kind);
            if let Some(parent) = self.nodes.get_mut(parent_index) {
                parent.child_offset += match parent_kind {
                    Some(LayoutNodeKind::Stack { horizontal: true, gap, .. }) => width + gap,
                    Some(LayoutNodeKind::Stack { horizontal: false, gap, .. }) | Some(LayoutNodeKind::Section { gap }) => height + gap,
                    _ => height,
                };
            }
        }
        self.arrange_cursor += 1;
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
            LayoutJobStage::MeasureFallback => "Layout.MeasureFallback",
            LayoutJobStage::ArrangeFallback => "Layout.ArrangeFallback",
            LayoutJobStage::PublishResults => "Layout.PublishResults",
            _ => "Layout.CursorBoundary",
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
            && self.atlas_candidate.is_empty()
    }
}

impl MountedLayoutFault {
    const fn label(self) -> &'static str {
        match self {
            Self::NodeCredits => "layout.node-credits",
            Self::GlyphCredits => "layout.glyph-credits",
            Self::DepthCredits => "layout.depth-credits",
            Self::Stale => "layout.stale",
            Self::RevisionExhausted => "layout.revision-exhausted",
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
mod tests {
    use super::*;
    use crate::wgpu::component::ui::{UiPresence, UiStackNode, UiTextNode};
    use crate::wgpu::Label;

    fn clock_zero() -> Option<u64> {
        Some(0)
    }

    fn text_tree(value: String) -> (UiTree, NodeId) {
        let mut tree = UiTree::new();
        tree.apply_tree(&UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }));
        let root = tree.root.unwrap_or_else(|| panic!("text tree root"));
        (tree, root)
    }

    fn wide_tree(children: usize) -> (UiTree, NodeId) {
        let children = (0..children).map(|_| UiNode::Text(UiTextNode { value: Label::data(""), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })).collect();
        let mut tree = UiTree::new();
        tree.apply_tree(&UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: None,
            padding: None,
            id: Some("hostile-wide".into()),
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children,
            menu: None,
        }));
        let root = tree.root.unwrap_or_else(|| panic!("wide tree root"));
        (tree, root)
    }

    fn deep_tree(depth: usize) -> (UiTree, NodeId) {
        let mut node = UiNode::Text(UiTextNode { value: Label::data("deep"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None });
        for index in 0..depth {
            node = UiNode::Stack(UiStackNode {
                direction: "vertical".into(),
                gap: None,
                padding: None,
                id: Some(format!("deep-{index}")),
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                children: vec![node],
                menu: None,
            });
        }
        let mut tree = UiTree::new();
        tree.apply_tree(&node);
        let root = tree.root.unwrap_or_else(|| panic!("deep tree root"));
        (tree, root)
    }

    fn text_job(tree: &UiTree, root: NodeId) -> MountedLayoutJob {
        MountedLayoutJob::try_new(tree, root, UiSurfaceToken::new(3, 7), 11, 13, 17, 19, Theme::default(), 640.0, 480.0).unwrap_or_else(|fault| panic!("mounted text job: {fault:?}"))
    }

    fn admit(job: &mut MountedLayoutJob, tree: &UiTree, cancel: &semio_framework_job::CancelToken) -> LayoutJobStep {
        let mut preview = 0;
        loop {
            let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(23), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), clock_zero, &mut preview);
            let step = job.admit_one(tree, &mut cx);
            if job.is_admitted() || matches!(step, LayoutJobStep::Fault(_) | LayoutJobStep::Cancelled) {
                return step;
            }
        }
    }

    #[test]
    fn mounted_layout_node_max_plus_one_returns_the_exact_owner() {
        let mut nodes = ui_contract::UiFixedList::<u64, LAYOUT_NODE_CREDITS>::default();
        for owner in 0..LAYOUT_NODE_CREDITS as u64 {
            assert_eq!(nodes.try_push(owner), Ok(()));
        }
        assert_eq!(nodes.try_push(u64::MAX), Err(u64::MAX));
    }

    #[test]
    fn mounted_layout_glyph_max_plus_one_returns_the_exact_owner() {
        let mut glyphs = ui_contract::UiFixedList::<u64, LAYOUT_GLYPH_CREDITS>::default();
        for owner in 0..LAYOUT_GLYPH_CREDITS as u64 {
            assert_eq!(glyphs.try_push(owner), Ok(()));
        }
        assert_eq!(glyphs.try_push(u64::MAX), Err(u64::MAX));
    }

    #[test]
    fn mounted_layout_actual_glyph_max_plus_one_retains_the_exact_rejected_scalar() {
        let (tree, root) = text_tree("x".repeat(LAYOUT_GLYPH_CREDITS + 1));
        let mut job = text_job(&tree, root);
        let cancel = semio_framework_job::CancelToken::root_now();
        assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Fault("layout.glyph-credits")));
        assert_eq!(job.rejected_glyph().map(|owner| owner.scalar), Some('x'));
        job.begin_close();
        let before = job.glyphs.len();
        assert!(!job.close_one());
        assert_eq!(job.glyphs.len(), before);
        while !job.close_one() {}
        assert!(job.terminal_is_empty());
    }

    #[test]
    fn mounted_layout_actual_node_max_plus_one_retains_the_exact_rejected_tree_owner() {
        let (tree, root) = wide_tree(LAYOUT_NODE_CREDITS);
        let mut job = text_job(&tree, root);
        let cancel = semio_framework_job::CancelToken::root_now();
        assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Fault("layout.node-credits")));
        let rejected = job.rejected_node().unwrap_or_else(|| panic!("rejected node owner"));
        assert!(tree.contains(rejected.id));
        assert!(!job.nodes.iter().any(|retained| retained.id == rejected.id));
        job.begin_close();
        while !job.close_one() {}
        assert!(job.terminal_is_empty());
    }

    #[test]
    fn mounted_layout_deep_tree_depth_refusal_retains_walk_authority_for_close() {
        let (tree, root) = deep_tree(LAYOUT_DEPTH_CREDITS + 1);
        let mut job = text_job(&tree, root);
        let cancel = semio_framework_job::CancelToken::root_now();
        assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Fault("layout.depth-credits")));
        assert!(job.rejected_walk.is_some());
        job.begin_close();
        let retained = job.nodes.len() + job.walk.len() + usize::from(job.rejected_walk.is_some());
        assert!(!job.close_one());
        let after = job.nodes.len() + job.walk.len() + usize::from(job.rejected_walk.is_some());
        assert_eq!(retained - after, 1);
        while !job.close_one() {}
        assert!(job.terminal_is_empty());
    }

    #[test]
    fn mounted_layout_multi_page_unicode_uses_one_glyph_or_atlas_boundary_per_turn() {
        let (tree, root) = text_tree("🙂".repeat(LAYOUT_GLYPH_CREDITS));
        let cancel = semio_framework_job::CancelToken::root_now();
        let mut job = text_job(&tree, root);
        assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Yield { .. }));
        let mut turns = 0;
        let mut preview_sequence = 0;
        while job.stage() == LayoutJobStage::ShapeText {
            let before = job.glyph_cursor;
            let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(27), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), clock_zero, &mut preview_sequence);
            let _ = semio_framework_job::InteractiveJob::step(&mut job, &mut cx);
            assert!(job.glyph_cursor - before <= 1);
            turns += 1;
        }
        assert_eq!(job.glyph_cursor, LAYOUT_GLYPH_CREDITS);
        assert_eq!(job.atlas_candidate.page_cursor, LAYOUT_ATLAS_PAGE_CREDITS - 1);
        assert!(turns > LAYOUT_GLYPH_CREDITS);
        assert!(job.glyph_previews.iter().all(|preview| preview.generation == 11 && preview.revision == 13));
        job.begin_close();
        while !job.close_one() {}
        assert!(job.terminal_is_empty());
    }

    #[test]
    fn mounted_layout_worker_runs_on_shared_user_visible_lane_and_pool_thread() {
        let (tree, root) = text_tree("worker".to_string());
        let mut job = text_job(&tree, root);
        let cancel = semio_framework_job::CancelToken::root_now();
        assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Yield { .. }));
        let params = semio_framework_job::BatchJobParams {
            operation: semio_framework_job::OperationId(29),
            generation: semio_framework_job::Generation(11),
            cancel,
            config: semio_framework_job::BatchDriveConfig { site: "ui.layout-text.worker.law", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
            now_us: semio_framework_job::default_now_us,
        };
        let mut session = semio_framework_job::MountedWorkerJobSession::try_new(job, params).unwrap_or_else(|_| panic!("mounted worker session credit"));
        let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
        let lane = semio_framework_async::Lane::UserVisible;
        for _ in 0..10_000 {
            let _ = session.pump_one(&pool, lane);
            if session.poll() == semio_framework_job::WorkerJobPoll::CheckedOut {
                break;
            }
            std::thread::yield_now();
        }
        assert_eq!(lane, semio_framework_async::Lane::UserVisible);
        assert!(session.checked_out_job_mut().is_some_and(|owner| owner.worker_thread_observed()));
        session.begin_close();
        for _ in 0..LAYOUT_GLYPH_CREDITS + LAYOUT_NODE_CREDITS * 4 {
            let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if session.terminal_is_empty() {
                break;
            }
        }
        assert!(session.terminal_is_empty());
    }

    #[test]
    fn mounted_layout_cancel_before_and_after_owned_text_call_is_typed_and_retained() {
        let (tree, root) = text_tree("cancel".to_string());
        let cancel = semio_framework_job::CancelToken::root_now();
        let mut before = text_job(&tree, root);
        assert!(matches!(admit(&mut before, &tree, &cancel), LayoutJobStep::Yield { .. }));
        cancel.cancel_now();
        let mut preview = 0;
        let mut before_cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(31), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, clock_zero, &mut preview);
        assert!(matches!(semio_framework_job::InteractiveJob::step(&mut before, &mut before_cx), semio_framework_job::StepOutcome::Cancelled));
        assert_eq!(before.glyph_cursor, 0);

        let after_cancel = semio_framework_job::CancelToken::root_now();
        let mut after = text_job(&tree, root);
        assert!(matches!(admit(&mut after, &tree, &after_cancel), LayoutJobStep::Yield { .. }));
        after.cancel_after_shape(after_cancel.clone());
        let mut after_preview = 0;
        let mut after_cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(37), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX), after_cancel, clock_zero, &mut after_preview);
        assert!(matches!(semio_framework_job::InteractiveJob::step(&mut after, &mut after_cx), semio_framework_job::StepOutcome::Cancelled));
        assert_eq!(after.glyph_cursor, 1);
        assert_eq!(after.latest_glyph_preview().map(|preview| preview.generation), Some(11));
        after.begin_close();
        while !after.close_one() {}
        assert!(after.terminal_is_empty());
    }

    #[test]
    fn mounted_layout_deadline_and_partial_close_each_advance_at_most_one_owner() {
        let (tree, root) = text_tree("deadline".to_string());
        let cancel = semio_framework_job::CancelToken::root_now();
        let mut job = text_job(&tree, root);
        assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Yield { .. }));
        let mut preview = 0;
        let mut expired = semio_framework_job::StepContext::new(semio_framework_job::OperationId(41), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, 0), cancel, clock_zero, &mut preview);
        assert!(matches!(semio_framework_job::InteractiveJob::step(&mut job, &mut expired), semio_framework_job::StepOutcome::Yield));
        assert_eq!(job.glyph_cursor, 0);
        let retained = job.glyphs.len() + job.nodes.len() + job.runs.len() + LAYOUT_ATLAS_PAGE_CREDITS;
        job.begin_close();
        assert!(!job.close_one());
        let after_one = job.glyphs.len() + job.nodes.len() + job.runs.len() + job.atlas_candidate.pages.iter().filter(|page| page.is_some()).count();
        assert_eq!(retained - after_one, 1);
        while !job.close_one() {}
        assert!(job.terminal_is_empty());
    }

    #[test]
    fn mounted_layout_publication_rechecks_full_identity_and_repeat_ready_swaps_once() {
        let (mut tree, root) = text_tree("publish".to_string());
        let cancel = semio_framework_job::CancelToken::root_now();
        let mut job = text_job(&tree, root);
        assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Yield { .. }));
        let mut preview_sequence = 0;
        while job.stage() != LayoutJobStage::PublishResults {
            let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(43), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), clock_zero, &mut preview_sequence);
            let _ = semio_framework_job::InteractiveJob::step(&mut job, &mut cx);
        }
        let stale = (UiSurfaceToken::new(3, 8), 11, 13, 17, 19, 640.0, 480.0);
        assert!(matches!(job.publish_one(&mut tree, stale), LayoutJobStep::Fault("layout.stale")));
        job.fault = None;
        let identity = job.identity();
        let before = tree.accepted_layout_generation();
        loop {
            let step = job.publish_one(&mut tree, identity);
            if matches!(step, LayoutJobStep::Complete) {
                break;
            }
            assert_eq!(tree.accepted_layout_generation(), before);
        }
        assert_eq!(tree.accepted_layout_generation(), 11);
        assert!(matches!(job.publish_one(&mut tree, identity), LayoutJobStep::Complete));
        assert_eq!(tree.accepted_layout_generation(), 11);
        job.begin_close();
        while !job.close_one() {}
        assert!(job.terminal_is_empty());
    }
}
//#endregion 🧵️MountedLayoutText
