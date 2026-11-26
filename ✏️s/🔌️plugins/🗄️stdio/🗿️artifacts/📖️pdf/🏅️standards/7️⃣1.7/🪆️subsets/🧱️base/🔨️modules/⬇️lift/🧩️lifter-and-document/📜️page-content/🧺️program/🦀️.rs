//! 📦️ Collects a controlled page's qualified program into the native paged lane.

use super::{controlled::{signal, worked}, ObjRef, ObjectSource, PageContentBarrier, PageContentCursor, PageContentEvent as Event, PageContentReceipt};
use crate::standards::v1_7::subsets::base::modules::{content::{ContentProgramCursor, FontTable, NoFonts}, lexer::CosGrant};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfContent, PdfOp, PdfPropertyList, PdfTextString, PdfTextArrayItem};
use std::mem::{size_of, take};

enum Stage { Read, Collect, Finish, Ready, Fallback, Failed(&'static str), Cancelled, Closing, Closed }

/// 📦️ Holds a complete native page content program or retires a deferred partial one.
#[must_use]
pub struct PageProgramCursor {
    source: PageContentCursor,
    stage: Stage,
    commands: PdfContent,
    source_closed: bool,
    fallback: Option<PageContentBarrier>,
}

impl PageProgramCursor {
    /// 🌱 Starts from a real page reference.
    pub fn new(page: ObjRef) -> Self {
        Self {
            source: PageContentCursor::new(page),
            stage: Stage::Read,
            commands: PdfContent::default(),
            source_closed: false,
            fallback: None,
        }
    }

    /// ▶️ Progresses decoding, backing migration, or settlement under explicit native grants.
    pub fn advance(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, cancelled: bool) -> PageContentReceipt {
        self.advance_with_fonts(source, &NoFonts, grant, cancelled)
    }

    /// 🔡️ Supplies qualified Type1 resource bindings while advancing the text program.
    pub fn advance_with_fonts(&mut self, source: &mut dyn ObjectSource, fonts: &dyn FontTable, grant: CosGrant, cancelled: bool) -> PageContentReceipt {
        if cancelled && !matches!(self.stage, Stage::Closing | Stage::Closed) { self.stage = Stage::Cancelled; }
        match self.stage {
            Stage::Ready => signal(Event::Complete),
            Stage::Fallback => signal(Event::Deferred(self.fallback.expect("deferred page program"))),
            Stage::Failed(reason) => self.fail(reason, signal(Event::More)),
            Stage::Cancelled => signal(Event::Cancelled),
            Stage::Closed => signal(Event::Closed),
            Stage::Closing => self.close_step(grant),
            _ if grant.work == 0 => signal(Event::More),
            Stage::Read => {
                let mut turn = self.source.advance_with_fonts(source, fonts, grant, false);
                match turn.event {
                    Ok(Event::OperatorReady) => self.stage = Stage::Collect,
                    Ok(Event::Deferred(reason)) => return self.begin_fallback(reason, turn),
                    Ok(Event::Complete) => {
                        self.source.begin_close();
                        self.stage = Stage::Finish;
                    }
                    Err(reason) => return self.fail(reason, turn),
                    _ => return turn,
                }
                turn.event = Ok(Event::More);
                turn
            }
            Stage::Collect => self.collect(fonts, grant),
            Stage::Finish => self.finish(grant),
        }
    }

    /// 📦️ Whether the next unit reserves one empty page of the operator list (4,096 bytes, never the whole program): an empty reservation that copies nothing, bounded by the host's release grant rather than its copy grant.
    pub fn reserving_empty_backing(&self) -> bool { matches!(self.stage, Stage::Collect) && !self.commands.has_reserved_slot() }

    /// 📏️ The most one unit of the program copies beside its allocation: an operand move or an operator write.
    pub const UNIT_COPY_MAXIMUM: usize = if ContentProgramCursor::UNIT_COPY_MAXIMUM > size_of::<PdfOp>() { ContentProgramCursor::UNIT_COPY_MAXIMUM } else { size_of::<PdfOp>() };

    /// 📏️ The bytes the next unit copies beside its allocation, or `None` when it moves source bytes one per unit of its grant: collecting moves one operator, reserving a page copies nothing.
    pub fn unit_copy(&self) -> Option<usize> {
        match self.stage {
            Stage::Read => self.source.unit_copy(),
            Stage::Collect if !self.reserving_empty_backing() => Some(size_of::<PdfOp>()),
            _ => Some(0),
        }
    }

    /// 👀️ Borrows a ready operator before the collector accepts its page resource binding.
    pub fn pending_operator(&self) -> Option<&PdfOp> { self.source.peek_operator() }

    /// 🫴️ Transfers the native paged page content after all input backing settles.
    pub fn take_program(&mut self) -> Option<PdfContent> {
        if !matches!(self.stage, Stage::Ready) { return None; }
        self.stage = Stage::Closed;
        Some(take(&mut self.commands))
    }

    /// ♻️ Starts cancelling or retiring an unclaimed page program.
    pub fn begin_close(&mut self) {
        if !matches!(self.stage, Stage::Closed | Stage::Closing) {
            self.source.begin_close();
            self.stage = Stage::Closing;
        }
    }

    /// 🧹️ Retires input and output, one operator or one page per unit, without recursive output drops.
    pub fn close_step(&mut self, grant: CosGrant) -> PageContentReceipt {
        if matches!(self.stage, Stage::Closed) { return signal(Event::Closed); }
        if !matches!(self.stage, Stage::Closing) || grant.work == 0 { return signal(Event::More); }
        self.retire(grant)
    }

    fn collect(&mut self, fonts: &dyn FontTable, grant: CosGrant) -> PageContentReceipt {
        let operator = self.source.peek_operator().expect("ready native page operator");
        let unmapped_text = text_operator(operator) && !self.source.selected_font()
            .is_some_and(|name| fonts.controlled_type1(name).is_some() || fonts.controlled_code_width(name).is_some() || fonts.font(name).is_some());
        if unmapped_text || operator_bytes(operator).is_none() {
            return self.begin_fallback(PageContentBarrier::ResourceContext, worked());
        }
        if !self.commands.has_reserved_slot() {
            let Ok(bytes) = self.commands.next_allocation_bytes() else { return self.fail("PDF page operator capacity overflow", worked()) };
            if bytes > grant.allocation_bytes { return signal(Event::NeedsAllocation { bytes }); }
            return match self.commands.reserve_one_cold(bytes) {
                Ok(progress) if progress.progressed => PageContentReceipt { allocated_bytes: progress.allocated_bytes, ..worked() },
                _ => self.fail("cannot allocate retained PDF page operators", worked()),
            };
        }
        if self.commands.push_reserved(self.source.take_operator().expect("ready page operator custody")).is_err() {
            return self.fail("PDF page operator page lost its reserved slot", worked());
        }
        self.stage = Stage::Read;
        worked()
    }

    fn fail(&mut self, reason: &'static str, turn: PageContentReceipt) -> PageContentReceipt {
        self.stage = Stage::Failed(reason);
        PageContentReceipt { event: Err(reason), ..turn }
    }

    fn begin_fallback(&mut self, reason: PageContentBarrier, turn: PageContentReceipt) -> PageContentReceipt {
        self.fallback = Some(reason);
        self.source.begin_close();
        self.stage = Stage::Finish;
        PageContentReceipt { event: Ok(Event::More), ..turn }
    }

    fn finish(&mut self, grant: CosGrant) -> PageContentReceipt {
        if self.fallback.is_some() { return self.retire(grant); }
        let turn = self.close_source(grant);
        if self.source_closed {
            self.stage = Stage::Ready;
            return PageContentReceipt { event: Ok(Event::Complete), ..turn };
        }
        turn
    }

    fn close_source(&mut self, grant: CosGrant) -> PageContentReceipt {
        let turn = self.source.close_step(grant);
        self.source_closed = turn.event == Ok(Event::Closed);
        if self.source_closed {
            PageContentReceipt { event: Ok(Event::More), ..turn }
        } else {
            turn
        }
    }

    fn retire(&mut self, grant: CosGrant) -> PageContentReceipt {
        if !self.source_closed { return self.close_source(grant); }
        if let Some(op) = self.commands.last() {
            let bytes = operator_bytes(op).expect("controlled operator ownership");
            if bytes > grant.release_bytes { return signal(Event::NeedsRelease { bytes }); }
            self.commands.pop();
            return PageContentReceipt { released_bytes: bytes, ..worked() };
        }
        if !self.commands.terminal_is_empty() {
            let Ok(bytes) = self.commands.next_release_allocation_bytes() else { return self.fail("PDF page operator page release is not quotable", worked()) };
            if bytes > grant.release_bytes { return signal(Event::NeedsRelease { bytes }); }
            return match self.commands.release_empty_page_leased(grant.release_bytes) {
                Ok(progress) if progress.progressed => PageContentReceipt { released_bytes: progress.released_allocation_bytes, ..worked() },
                _ => self.fail("cannot release retained PDF page operators", worked()),
            };
        }
        let event = match (&self.stage, self.fallback) {
            (Stage::Finish, Some(reason)) => {
                self.stage = Stage::Fallback;
                Event::Deferred(reason)
            }
            _ => {
                self.stage = Stage::Closed;
                Event::Closed
            }
        };
        PageContentReceipt { event: Ok(event), ..worked() }
    }
}

fn text_operator(op: &PdfOp) -> bool {
    matches!(op, PdfOp::SetFont { .. } | PdfOp::ShowText { .. } | PdfOp::ShowTextArray { .. } | PdfOp::NextLineShowText { .. } | PdfOp::NextLineShowTextSpaced { .. })
}

/// 📏️ The heap bytes one qualified operator owns beside its inline value; `None` for an operator the controlled program does not collect.
pub fn operator_bytes(op: &PdfOp) -> Option<usize> {
    Some(match op {
        PdfOp::SetDash { array, .. }
        | PdfOp::SetStrokeColor { components: array }
        | PdfOp::SetFillColor { components: array } => array.capacity() * size_of::<f64>(),
        PdfOp::SetStrokeColorN { components, pattern } | PdfOp::SetFillColorN { components, pattern } => {
            components.capacity() * size_of::<f64>() + pattern.as_ref().map_or(0, String::capacity)
        }
        PdfOp::SetRenderingIntent { intent: name }
        | PdfOp::SetExtGState { name }
        | PdfOp::SetStrokeColorSpace { name }
        | PdfOp::SetFillColorSpace { name }
        | PdfOp::PaintShading { name }
        | PdfOp::PaintXObject { name }
        | PdfOp::MarkedContentPoint { tag: name }
        | PdfOp::BeginMarkedContent { tag: name } => name.capacity(),
        PdfOp::SetFont { name, .. } => name.capacity(),
        PdfOp::ShowText { text } | PdfOp::NextLineShowText { text } | PdfOp::NextLineShowTextSpaced { text, .. } => match text {
            PdfTextString::Text { text } => text.capacity(),
            PdfTextString::Codes { codes } => codes.capacity() * size_of::<u32>(),
        },
        PdfOp::ShowTextArray { items } => items.capacity() * size_of::<PdfTextArrayItem>() + items.iter().map(|item| match item {
            PdfTextArrayItem::Text { text } => text.capacity(),
            PdfTextArrayItem::Codes { codes } => codes.capacity() * size_of::<u32>(),
            PdfTextArrayItem::Adjust { .. } => 0,
        }).sum::<usize>(),
        PdfOp::MarkedContentPointWithProperties { tag, properties: PdfPropertyList::Named { name } }
        | PdfOp::BeginMarkedContentWithProperties { tag, properties: PdfPropertyList::Named { name } } => tag.capacity() + name.capacity(),
        PdfOp::InlineImage { .. }
        | PdfOp::MarkedContentPointWithProperties { .. }
        | PdfOp::BeginMarkedContentWithProperties { .. }
        | PdfOp::Unknown { .. } => return None,
        PdfOp::SetLineWidth { .. }
        | PdfOp::SetLineCap { .. }
        | PdfOp::SetLineJoin { .. }
        | PdfOp::SetMiterLimit { .. }
        | PdfOp::SetFlatness { .. }
        | PdfOp::Save
        | PdfOp::Restore
        | PdfOp::Transform { .. }
        | PdfOp::MoveTo { .. }
        | PdfOp::LineTo { .. }
        | PdfOp::CurveTo { .. }
        | PdfOp::CurveToInitial { .. }
        | PdfOp::CurveToFinal { .. }
        | PdfOp::ClosePath
        | PdfOp::Rectangle { .. }
        | PdfOp::Stroke
        | PdfOp::CloseStroke
        | PdfOp::Fill
        | PdfOp::FillEvenOdd
        | PdfOp::FillStroke
        | PdfOp::FillStrokeEvenOdd
        | PdfOp::CloseFillStroke
        | PdfOp::CloseFillStrokeEvenOdd
        | PdfOp::EndPath
        | PdfOp::Clip
        | PdfOp::ClipEvenOdd
        | PdfOp::BeginText
        | PdfOp::EndText
        | PdfOp::SetCharSpacing { .. }
        | PdfOp::SetWordSpacing { .. }
        | PdfOp::SetHorizontalScale { .. }
        | PdfOp::SetLeading { .. }
        | PdfOp::SetTextRenderingMode { .. }
        | PdfOp::SetTextRise { .. }
        | PdfOp::MoveText { .. }
        | PdfOp::MoveTextSetLeading { .. }
        | PdfOp::SetTextMatrix { .. }
        | PdfOp::NextLine
        | PdfOp::SetGlyphWidth { .. }
        | PdfOp::SetGlyphWidthAndBox { .. }
        | PdfOp::SetStrokeGray { .. }
        | PdfOp::SetFillGray { .. }
        | PdfOp::SetStrokeRgb { .. }
        | PdfOp::SetFillRgb { .. }
        | PdfOp::SetStrokeCmyk { .. }
        | PdfOp::SetFillCmyk { .. }
        | PdfOp::EndMarkedContent
        | PdfOp::BeginCompatibility
        | PdfOp::EndCompatibility => 0,
    })
}
