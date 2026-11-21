//! 📦️ Collects a controlled page's qualified program into the native vector lane.

use super::{controlled::{signal, worked}, ObjRef, ObjectSource, PageContentBarrier, PageContentCursor, PageContentEvent as Event, PageContentReceipt};
use crate::standards::v1_7::subsets::base::modules::{content::{FontTable, NoFonts}, lexer::CosGrant};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfOp, PdfTextString, PdfTextArrayItem};
use std::mem::{size_of, take};
use std::vec::IntoIter;

enum Stage { Read, Collect, Migrate, Finish, Ready, Fallback, Failed(&'static str), Cancelled, Closing, Closed }

/// 📦️ Holds a complete native page content program or retires a deferred partial one.
#[must_use]
pub struct PageProgramCursor {
    source: PageContentCursor,
    stage: Stage,
    commands: Vec<PdfOp>,
    old: Option<(IntoIter<PdfOp>, usize)>,
    source_closed: bool,
    fallback: Option<PageContentBarrier>,
}

impl PageProgramCursor {
    /// 🌱 Starts from a real page reference.
    pub fn new(page: ObjRef) -> Self {
        Self {
            source: PageContentCursor::new(page),
            stage: Stage::Read,
            commands: Vec::new(),
            old: None,
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
            Stage::Migrate => self.migrate(grant),
            Stage::Finish => self.finish(grant),
        }
    }

    /// 👀️ Borrows a ready operator before the collector accepts its page resource binding.
    pub fn pending_operator(&self) -> Option<&PdfOp> { self.source.peek_operator() }

    /// 🫴️ Transfers the native page content vector after all input and migration backing settles.
    pub fn take_program(&mut self) -> Option<Vec<PdfOp>> {
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

    /// 🧹️ Retires input, output and any in-flight migration without recursive output drops.
    pub fn close_step(&mut self, grant: CosGrant) -> PageContentReceipt {
        if matches!(self.stage, Stage::Closed) { return signal(Event::Closed); }
        if !matches!(self.stage, Stage::Closing) || grant.work == 0 { return signal(Event::More); }
        self.retire(grant)
    }

    fn collect(&mut self, fonts: &dyn FontTable, grant: CosGrant) -> PageContentReceipt {
        let operator = self.source.peek_operator().expect("ready native page operator");
        let unmapped_text = text_operator(operator) && !self.source.selected_font()
            .is_some_and(|name| fonts.controlled_type1(name).is_some() || fonts.controlled_code_width(name).is_some());
        if unmapped_text || operator_bytes(operator).is_none() {
            return self.begin_fallback(PageContentBarrier::ResourceContext, worked());
        }
        if self.commands.len() == self.commands.capacity() {
            let Some(capacity) = self.commands.capacity().max(4).checked_mul(2)
                .filter(|count| *count <= isize::MAX as usize / size_of::<PdfOp>()) else {
                return self.fail("PDF page operator capacity overflow", worked());
            };
            let bytes = capacity * size_of::<PdfOp>();
            if bytes > grant.allocation_bytes { return signal(Event::NeedsAllocation { bytes }); }
            let mut incoming = Vec::new();
            if incoming.try_reserve_exact(capacity).is_err() {
                return self.fail("cannot allocate retained PDF page operators", worked());
            }
            let backing = std::mem::replace(&mut self.commands, incoming);
            let previous = backing.capacity() * size_of::<PdfOp>();
            self.old = Some((backing.into_iter(), previous));
            self.stage = Stage::Migrate;
            return PageContentReceipt { allocated_bytes: bytes, ..worked() };
        }
        self.commands.push(self.source.take_operator().expect("ready page operator custody"));
        self.stage = Stage::Read;
        worked()
    }

    fn migrate(&mut self, grant: CosGrant) -> PageContentReceipt {
        let (items, bytes) = self.old.as_mut().expect("retained page program backing");
        if let Some(item) = items.next() {
            self.commands.push(item);
            return worked();
        }
        if *bytes > grant.release_bytes { return signal(Event::NeedsRelease { bytes: *bytes }); }
        let released_bytes = *bytes;
        self.old = None;
        self.stage = Stage::Collect;
        PageContentReceipt { released_bytes, ..worked() }
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
        if let Some((old, capacity)) = self.old.as_mut() {
            let bytes = old.as_slice().first().map_or(Some(*capacity), operator_bytes).expect("controlled operator ownership");
            if bytes > grant.release_bytes { return signal(Event::NeedsRelease { bytes }); }
            if old.next().is_none() { self.old = None; }
            return PageContentReceipt { released_bytes: bytes, ..worked() };
        }
        if let Some(op) = self.commands.last() {
            let bytes = operator_bytes(op).expect("controlled operator ownership");
            if bytes > grant.release_bytes { return signal(Event::NeedsRelease { bytes }); }
            self.commands.pop();
            return PageContentReceipt { released_bytes: bytes, ..worked() };
        }
        let bytes = self.commands.capacity() * size_of::<PdfOp>();
        if bytes > grant.release_bytes { return signal(Event::NeedsRelease { bytes }); }
        self.commands = Vec::new();
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
        PageContentReceipt { released_bytes: bytes, event: Ok(event), ..worked() }
    }
}

fn text_operator(op: &PdfOp) -> bool {
    matches!(op, PdfOp::SetFont { .. } | PdfOp::ShowText { .. } | PdfOp::ShowTextArray { .. } | PdfOp::NextLineShowText { .. } | PdfOp::NextLineShowTextSpaced { .. })
}

fn operator_bytes(op: &PdfOp) -> Option<usize> {
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
