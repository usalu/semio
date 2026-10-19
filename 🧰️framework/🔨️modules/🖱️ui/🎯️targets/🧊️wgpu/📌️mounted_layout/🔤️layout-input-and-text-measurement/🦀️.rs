//! 🔤️ Layout input and text measurement: the layout input nodes, the glyph ranges and previews, the text worker, the atlas candidate and the text and glyph measuring. See [`super`].

use super::*;

//#region 🧵️MountedLayoutText
/// 🧩️ One admitted node's layout identity: where it sits in the arena, which flex box it became,
/// and — for a `Text` node — the half-open glyph range the shaping stage filled, which is the whole
/// input the intrinsic measurement callback needs (no tree access on the worker thread).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct LayoutInputNode {
    pub(super) id: NodeId,
    pub(super) parent: Option<usize>,
    pub(super) first_child: Option<usize>,
    pub(super) last_child: Option<usize>,
    pub(super) next_sibling: Option<usize>,
    pub(super) kind: LayoutNodeKind,
    pub(super) intrinsic: IntrinsicSize,
    pub(super) glyph_start: usize,
    pub(super) glyph_end: usize,
    pub(super) chrome: ChromeTextRanges,
    pub(super) content: Option<TreeContentFlow>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct GlyphRange {
    pub(super) start: usize,
    pub(super) end: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum ChromeTextRanges {
    #[default]
    None,
    Section {
        title: Option<GlyphRange>,
    },
    Field {
        label: GlyphRange,
        description: Option<GlyphRange>,
        error: Option<GlyphRange>,
    },
    SliderUnit {
        label: GlyphRange,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct RetainedGlyphInput {
    pub(super) node: usize,
    pub(super) scalar: char,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct RetainedTextRun {
    pub(super) node: usize,
    pub(super) glyph_start: usize,
    pub(super) glyph_end: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct RetainedLine {
    pub(super) node: usize,
    pub(super) width: f32,
    pub(super) height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RetainedGlyphPreview {
    pub scalar: char,
    pub advance: f32,
    /// 🤝️ The pair kerning between this scalar and the next scalar of its run at the worker's size — nothing for a run's last.
    pub kerning: f32,
    pub height: f32,
    pub generation: u64,
    pub revision: u64,
    pub atlas_page: u8,
    pub atlas_offset: u16,
    pub atlas_length: u8,
}

pub(super) trait OwnedTextWorker: Send {
    fn shape_one(&mut self, input: RetainedGlyphInput, next: Option<char>) -> RetainedGlyphPreview;
}

#[derive(Default)]
pub(super) struct DeterministicTextWorker {
    pub(super) kerning: semio_framework_ui_text::PairKerning,
    #[cfg(test)]
    pub(super) cancel_after_shape: Option<semio_framework_job::CancelToken>,
    #[cfg(test)]
    cancel_retirement: Option<semio_framework_async::CancelTokenRetirement>,
}

impl DeterministicTextWorker{
    pub(super) fn terminal_is_empty(&self)->bool{self.kerning.terminal_is_empty()&&self.cancel_lease_is_empty()}
    fn cancel_lease_is_empty(&self)->bool{#[cfg(test)]{self.cancel_after_shape.is_none()&&self.cancel_retirement.is_none()}#[cfg(not(test))]{true}}
    pub(super) fn next_close_release_bytes(&self)->Result<usize,semio_framework_value::ValueError>{
        if self.kerning.terminal_is_empty(){Ok(0)}else{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original layout kerning requires its controlled issuer"))}}
    pub(super) fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::{InteractiveJobCloseStep as Step,OriginalTurnFailure,RetainedCloneProgress};
        if self.terminal_is_empty(){return Ok(Step::Complete{progress:Default::default()})}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(Step::Pending{progress:Default::default()})}
        #[cfg(test)]{
            let metadata=RetainedCloneProgress{copied_items:1,..Default::default()};
            if self.cancel_after_shape.is_some(){
                cx.consume_retained(metadata)?;self.cancel_retirement=Some(semio_framework_async::CancelTokenRetirement::from_token(self.cancel_after_shape.take().expect("retained original text alias")));cx.consume_fuel(1);return Ok(Step::Pending{progress:metadata})
            }
            if let Some(original)=self.cancel_retirement.as_mut(){
                if original.terminal_is_empty(){cx.consume_retained(metadata)?;self.cancel_retirement.take();cx.consume_fuel(1);return Ok(Step::Pending{progress:metadata})}
                let step=original.return_alias_step(cx.original_cancel_token(),grant).map_err(|error|match error{
                    semio_framework_async::CancelTokenRetirementError::Refused(error)=>OriginalTurnFailure::debit_original(error,cx),
                    semio_framework_async::CancelTokenRetirementError::Blocked(_)=>OriginalTurnFailure::from(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original text alias return is blocked"))
                })?;cx.consume_retained(step.progress())?;cx.consume_fuel(1);return Ok(Step::Pending{progress:step.progress()})
            }
        }
        Err(OriginalTurnFailure::from(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original text owner requires its retained close cursor")))
    }
}

impl OwnedTextWorker for DeterministicTextWorker {
    fn shape_one(&mut self, input: RetainedGlyphInput, next: Option<char>) -> RetainedGlyphPreview {
        let advance = semio_framework_ui_text::font_advance_em(semio_framework_ui_text::TextFace::Sans, input.scalar) * DEFAULT_TEXT_SIZE_PX;
        let kerning = next.map_or(0.0, |right| self.kerning.em(semio_framework_ui_text::TextFace::Sans, input.scalar, right) * DEFAULT_TEXT_SIZE_PX);
        #[cfg(test)]
        if let Some(cancel) = self.cancel_after_shape.as_ref() {
            cancel.cancel_now();
        }
        RetainedGlyphPreview { scalar: input.scalar, advance, kerning, height: semio_framework_ui_text::line_height(DEFAULT_TEXT_SIZE_PX), generation: 0, revision: 0, atlas_page: 0, atlas_offset: 0, atlas_length: 0 }
    }
}

pub(super) struct RetainedAtlasCandidate {
    pub(super) pages: [Option<Box<[u8; LAYOUT_ATLAS_PAGE_BYTES]>>; LAYOUT_ATLAS_PAGE_CREDITS],
    pub(super) page_cursor: usize,
    byte_cursor: usize,
}

impl RetainedAtlasCandidate {
    pub(super) fn new() -> Self {
        Self { pages: std::array::from_fn(|_| Some(Box::new([0; LAYOUT_ATLAS_PAGE_BYTES]))), page_cursor: 0, byte_cursor: 0 }
    }

    pub(super) fn retain_one(&mut self, scalar: char, generation: u64, revision: u64, mut preview: RetainedGlyphPreview) -> Result<Option<RetainedGlyphPreview>, MountedLayoutFault> {
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

    pub(super) fn close_one(&mut self) -> bool {
        let Some(page) = self.pages.iter_mut().find(|page| page.is_some()) else { return true };
        *page = None;
        false
    }

    pub(super) fn is_empty(&self) -> bool {
        self.pages.iter().all(Option::is_none)
    }
}

/// 📏️ One text node's intrinsic size against the space the solver offers it, from the shaped
/// advances and pair kerning alone — the worker thread holds no tree and no font atlas. A pair's
/// kerning counts only while both scalars stay on one line, the rule the atlas and painter apply. This is CSS's own reading of
/// a text run inside a flex item: `MaxContent` is the whole run on one line, `MinContent` is the
/// widest unbreakable word (the floor a flex item may shrink to before it overflows), and a
/// `Definite` width is first-fit greedy wrapping at [`may_break_between`]'s own CSS break
/// opportunities — the SAME predicate the atlas wrap and the retained painter use, so a flex item
/// sized here and a paragraph painted there can no longer disagree about where a line ends. A
/// trailing space hangs and never pushes a line over the edge.
pub(super) fn measure_text(
    nodes: &ui_contract::UiFixedList<LayoutInputNode, LAYOUT_NODE_CREDITS>,
    glyphs: &ui_contract::UiFixedList<RetainedGlyphInput, LAYOUT_GLYPH_CREDITS>,
    previews: &ui_contract::UiFixedList<RetainedGlyphPreview, LAYOUT_GLYPH_CREDITS>,
    index: usize,
    constraint: MeasureConstraint,
) -> (f32, f32) {
    let Some(node) = nodes.get(index) else { return (0.0, 0.0) };
    measure_glyph_range(glyphs, previews, GlyphRange { start: node.glyph_start, end: node.glyph_end }, DEFAULT_TEXT_SIZE_PX, constraint, false)
}

fn measure_glyph_range(
    glyphs: &ui_contract::UiFixedList<RetainedGlyphInput, LAYOUT_GLYPH_CREDITS>,
    previews: &ui_contract::UiFixedList<RetainedGlyphPreview, LAYOUT_GLYPH_CREDITS>,
    range: GlyphRange,
    size: f32,
    constraint: MeasureConstraint,
    clipped: bool,
) -> (f32, f32) {
    let (start, end) = (range.start, range.end);
    if end <= start {
        return (0.0, 0.0);
    }
    let line = semio_framework_ui_text::line_height(size);
    let scale = size / DEFAULT_TEXT_SIZE_PX;
    let advance = |cursor: usize| previews.get(cursor).map_or(0.0, |preview| preview.advance * scale);
    let kerning = |cursor: usize| if cursor > start { previews.get(cursor - 1).map_or(0.0, |preview| preview.kerning * scale) } else { 0.0 };
    let scalar = |cursor: usize| glyphs.get(cursor).map_or(' ', |glyph| glyph.scalar);
    let single_line = || (start..end).map(|cursor| kerning(cursor) + advance(cursor)).sum::<f32>();
    if clipped {
        let width = single_line();
        return (
            match constraint {
                MeasureConstraint::Definite(available) => width.min(available.max(0.0)),
                MeasureConstraint::MinContent | MeasureConstraint::MaxContent => width,
            },
            line,
        );
    }
    match constraint {
        MeasureConstraint::MaxContent => (single_line(), line),
        MeasureConstraint::MinContent => {
            let (mut widest, mut run) = (0.0_f32, 0.0_f32);
            for cursor in start..end {
                let ch = scalar(cursor);
                if ch == '\n' || is_wrap_space(ch) || (cursor > start && may_break_between(scalar(cursor - 1), ch)) {
                    widest = widest.max(run);
                    run = 0.0;
                }
                if ch != '\n' && !is_wrap_space(ch) {
                    run += advance(cursor) + if run > 0.0 { kerning(cursor) } else { 0.0 };
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
                let (advance, mut kerning) = (advance(cursor), if placed + run > 0.0 { kerning(cursor) } else { 0.0 });
                if is_wrap_space(ch) {
                    placed += run + kerning + advance;
                    run = 0.0;
                    continue;
                }
                if placed + run > 0.0 && placed + run + kerning + advance > available {
                    widest = widest.max(ink);
                    if placed > 0.0 {
                        placed = 0.0;
                    } else {
                        run = 0.0;
                    }
                    if run == 0.0 {
                        kerning = 0.0;
                    }
                    lines += 1;
                }
                run += kerning + advance;
                ink = placed + run;
            }
            (widest.max(ink), line * lines as f32)
        }
    }
}

pub(super) fn composite_chrome(node: &LayoutInputNode, glyphs: &ui_contract::UiFixedList<RetainedGlyphInput, LAYOUT_GLYPH_CREDITS>, previews: &ui_contract::UiFixedList<RetainedGlyphPreview, LAYOUT_GLYPH_CREDITS>, width: f32, gap: f32) -> Option<(f32, f32)> {
    let width = width.max(1.0);
    match node.chrome {
        ChromeTextRanges::None => None,
        ChromeTextRanges::Section { title } => {
            let title_height = title.map(|range| measure_glyph_range(glyphs, previews, range, SECTION_TITLE_FONT_SIZE, MeasureConstraint::Definite(width), false).1.max(semio_framework_ui_wgpu_layout::flex::SECTION_TITLE_LINE_HEIGHT));
            let metrics = section_chrome_metrics(title_height, 0.0);
            Some((metrics.top, metrics.bottom))
        }
        ChromeTextRanges::Field { description, error, .. } => {
            let detail_height = |range: GlyphRange| measure_glyph_range(glyphs, previews, range, FIELD_DETAIL_FONT_SIZE, MeasureConstraint::Definite(width), false).1.max(semio_framework_ui_text::line_height(FIELD_DETAIL_FONT_SIZE));
            let metrics = field_chrome_metrics(description.map(detail_height), 0.0, error.map(detail_height), gap);
            Some((metrics.top, metrics.bottom))
        }
        ChromeTextRanges::SliderUnit { .. } => None,
    }
}
//#endregion 🧵️MountedLayoutText
