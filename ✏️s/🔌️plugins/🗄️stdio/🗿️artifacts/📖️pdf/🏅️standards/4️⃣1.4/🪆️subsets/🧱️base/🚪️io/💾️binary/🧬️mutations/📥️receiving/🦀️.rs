//! 📬 Paid receiving for the authored PDF 1.4 fixed-field edit frames.

use super::{insert_page, move_page, remove_page, replace_page_text, resize_page, PdfMutation};
use crate::standards::v1_4::subsets::base::schema::{mutations::{InsertPage, MovePage, RemovePage, ReplacePageText, ResizePage}, snapshot::PageDoc};
use dsl::original_receiving::{OpBinaryReceivingCursor, OriginalReceivingPrimary};
use semio_framework_value::{
    NativeDecodeControl, OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress,
    RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, RetirementPollExit, ValueError, ValueRefusalKind,
    retained_clone::retained_clone_grant_admits,
    retirement::{RetireOwned, RetirementCursor, RetirementStep, erased_cursor_turn},
};
use std::{mem::{ManuallyDrop, size_of}, ops::Range};

const TEXT_TURN: usize = 4096;

#[derive(Clone, Copy)]
enum Fields {
    Insert { index: usize, width: f64, height: f64 },
    Remove { index: usize },
    Move { from: usize, to: usize },
    Resize { index: usize, width: f64, height: f64 },
    Replace { index: usize },
}

impl Fields {
    fn bind(self, text: String) -> PdfMutation {
        match self {
            Self::Insert { index, width, height } => InsertPage { index, page: PageDoc { width, height, text } }.into(),
            Self::Remove { index } => RemovePage { index }.into(),
            Self::Move { from, to } => MovePage { from, to }.into(),
            Self::Resize { index, width, height } => ResizePage { index, width, height }.into(),
            Self::Replace { index } => ReplacePageText { index, text }.into(),
        }
    }
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "invalid PDF 1.4 operation frame")
}

fn field(source: &[u8], offset: usize) -> Result<[u8; 8], ValueError> {
    source.get(offset..offset + 8).and_then(|word| word.try_into().ok()).ok_or_else(invalid)
}

fn index(source: &[u8], offset: usize) -> Result<usize, ValueError> {
    usize::try_from(u64::from_le_bytes(field(source, offset)?)).map_err(|_| invalid())
}

fn number(source: &[u8], offset: usize) -> Result<f64, ValueError> {
    let value = f64::from_le_bytes(field(source, offset)?);
    value.is_finite().then_some(value).ok_or_else(invalid)
}

fn parse(source: &[u8]) -> Result<(Fields, Range<usize>), ValueError> {
    if source.first() != Some(&store::pack_rt::OP_BINARY_FORMAT) { return Err(invalid()); }
    let index_value = index(source, 2)?;
    let (fields, header, has_text) = match source.get(1).copied().ok_or_else(invalid)? {
        insert_page::TAG => (Fields::Insert { index: index_value, width: number(source, 10)?, height: number(source, 18)? }, 34, true),
        remove_page::TAG => (Fields::Remove { index: index_value }, 10, false),
        move_page::TAG => (Fields::Move { from: index_value, to: index(source, 10)? }, 18, false),
        resize_page::TAG => (Fields::Resize { index: index_value, width: number(source, 10)?, height: number(source, 18)? }, 26, false),
        replace_page_text::TAG => (Fields::Replace { index: index_value }, 18, true),
        _ => return Err(invalid()),
    };
    let length = if has_text { index(source, header - 8)? } else { 0 };
    let end = header.checked_add(length).ok_or_else(invalid)?;
    if end != source.len() { return Err(invalid()); }
    Ok((fields, header..end))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Header, Reserve, Text, Ready, Failed, Closed }

/// 📥 Holds one immutable original frame and the single exact typed text backing.
pub struct Pdf14OperationReceiving {
    source: (usize, usize),
    range: (usize, usize),
    phase: Phase,
    fields: Option<Fields>,
    text_at: usize,
    text_end: usize,
    text: ManuallyDrop<String>,
}

impl Pdf14OperationReceiving {
    pub fn admit(source: &[u8], range: (usize, usize), grant: RetainedCloneGrant) -> RetainedWorkOutcome<Option<(Self, RetainedCloneProgress)>> {
        let demand = fixed(0);
        let admitted = if source.get(range.0..range.1).is_none() {
            Err(invalid())
        } else if !retained_clone_grant_admits(grant, demand) {
            Ok(None)
        } else {
            Ok(Some((Self { source: (source.as_ptr() as usize, source.len()), range, phase: Phase::Header,
                fields: None, text_at: 0, text_end: 0, text: ManuallyDrop::new(String::new()) }, receipt(demand))))
        };
        RetainedWorkOutcome { work: u128::from(matches!(&admitted, Ok(Some(_)))), result: admitted }
    }

    fn frame<'a>(&self, source: &'a [u8]) -> Result<&'a [u8], ValueError> {
        if self.source != (source.as_ptr() as usize, source.len()) { return Err(invalid()); }
        source.get(self.range.0..self.range.1).ok_or_else(invalid)
    }

    fn grant(&self) -> RetirementDemand {
        match self.phase {
            Phase::Header => fixed(size_of::<Option<Fields>>() + 2 * size_of::<usize>() + size_of::<Phase>()),
            Phase::Reserve => RetirementDemand { capacity_bytes: self.text_end - self.text_at, ..fixed(size_of::<String>() + size_of::<Phase>()) },
            Phase::Text => fixed(self.text_extent() + size_of::<String>() + size_of::<usize>() + size_of::<Phase>()),
            _ => Default::default(),
        }
    }

    fn text_extent(&self) -> usize { (self.text_end - self.text_at).min(TEXT_TURN) }

    fn work(&self) -> u64 {
        match self.phase {
            Phase::Header => 35,
            Phase::Reserve => 1,
            Phase::Text => 1 + self.text_extent() as u64,
            _ => 0,
        }
    }

    fn advance(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<RetainedCloneProgress, ValueError> {
        let frame = self.frame(source)?;
        let demand = self.grant();
        let work = self.work();
        if work == 0 || !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? < work {
            return Ok(Default::default());
        }
        native.checkpoint()?;
        if !effect.performed(work)? { return Ok(Default::default()); }
        let mut paid = receipt(demand);
        match self.phase {
            Phase::Header => {
                let (fields, text) = parse(frame)?;
                self.fields = Some(fields);
                self.text_at = text.start;
                self.text_end = text.end;
                self.phase = if text.is_empty() { Phase::Ready } else { Phase::Reserve };
            }
            Phase::Reserve => {
                if effect.actual().is_some() { return Err(ValueError::invariant("PDF 1.4 text capacity marker occupied")); }
                *effect.actual() = Some(0);
                self.text.try_reserve_exact(demand.capacity_bytes).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed,"PDF 1.4 operation text reserve refused"))?;
                let actual = self.text.capacity();
                *effect.actual() = Some(actual);
                if actual != demand.capacity_bytes { return Err(ValueError::invariant("PDF 1.4 text backing differs from its quote")); }
                self.phase = Phase::Text;
            }
            Phase::Text => {
                let extent = self.text_extent();
                let page = &frame[self.text_at..self.text_at + extent];
                let text = match std::str::from_utf8(page) {
                    Ok(text) => text,
                    Err(error) if error.error_len().is_none() && error.valid_up_to() != 0 && self.text_at + extent < self.text_end =>
                        std::str::from_utf8(&page[..error.valid_up_to()]).map_err(|_| invalid())?,
                    Err(_) => return Err(invalid()),
                };
                self.text.push_str(text);
                paid.copied_bytes -= extent - text.len();
                self.text_at += text.len();
                if self.text_at == self.text_end { self.phase = Phase::Ready; }
            }
            _ => unreachable!(),
        }
        Ok(paid)
    }

    fn close_demand(&self) -> RetirementDemand {
        if self.phase == Phase::Closed { return Default::default(); }
        RetirementDemand { release_bytes: self.text.capacity(), ..fixed(size_of::<String>() + size_of::<Phase>()) }
    }

    fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        let settled = (|| {
            if self.phase == Phase::Closed { return Ok(RetainedCloneStep::Complete(Default::default())); }
            let demand = self.close_demand();
            if !retained_clone_grant_admits(effect.grant(), demand) || effect.remaining_work_units()? == 0 || !effect.performed(1)? {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            drop(std::mem::take(&mut *self.text));
            self.phase = Phase::Closed;
            Ok(RetainedCloneStep::Complete(receipt(demand)))
        })();
        RetirementPollExit::settle(effect, settled)
    }
}

fn fixed(copy_bytes: usize) -> RetirementDemand { RetirementDemand { copy_bytes, depth: 1, ..Default::default() } }
fn receipt(demand: RetirementDemand) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes,
        released_bytes: demand.release_bytes, ..Default::default() }
}

impl OpBinaryReceivingCursor<PdfMutation> for Pdf14OperationReceiving {
    fn demands(&self, source: &[u8], _: usize) -> Result<RetirementDemand, ValueError> { self.frame(source)?; Ok(self.grant()) }
    fn step_work_demand(&self, source: &[u8], _: usize) -> Result<u64, ValueError> { self.frame(source)?; Ok(self.work()) }
    fn step_in(&mut self, source: &[u8], native: &mut NativeDecodeControl<'_>, _: usize, effect: &mut RetainedAllocationEffect<'_>)
        -> RetainedWorkOutcome<RetainedCloneProgress, OriginalReceivingPrimary> {
        let before = effect.performed_boundary();
        let result = self.advance(source, native, effect).map_err(|error| {
            self.phase = Phase::Failed;
            error.with_retained_progress(RetainedCloneProgress { retained_capacity_bytes: effect.actual().unwrap_or(0), ..Default::default() }).into()
        });
        RetainedWorkOutcome { result, work: effect.performed_since(before).expect("PDF 1.4 receiving work") }
    }

    fn domain_failure(&self) -> Option<&ValueError> { None }
    fn domain_failure_demand(&self) -> Result<RetirementDemand, ValueError> { Ok(Default::default()) }
    fn domain_failure_work_demand(&self) -> Result<u64, ValueError> { Ok(0) }
    fn take_domain_failure_into(&mut self, _: &mut Option<ValueError>, _: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress> {
        RetainedWorkOutcome { result: Ok(Default::default()), work: 0 }
    }
    fn is_ready(&self) -> bool { self.phase == Phase::Ready }
    fn take_into(&mut self, recipient: &mut Option<PdfMutation>, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress> {
        let demand = fixed(size_of::<PdfMutation>() + size_of::<Option<Fields>>() + size_of::<String>() + size_of::<Phase>());
        if !self.is_ready() || recipient.is_some() || !retained_clone_grant_admits(grant, demand) {
            return RetainedWorkOutcome { result: Ok(Default::default()), work: 0 };
        }
        let result = match self.fields.take() {
            Some(fields) => {
                *recipient = Some(fields.bind(std::mem::take(&mut *self.text)));
                self.phase = Phase::Closed;
                Ok(receipt(demand))
            }
            None => Err(ValueError::invariant("PDF 1.4 operation receiver lacks its admitted fields")),
        };
        RetainedWorkOutcome { result, work: 1 }
    }
    fn close_demands(&self, _: usize) -> Result<RetirementDemand, ValueError> { Ok(self.close_demand()) }
    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll { self.close(effect) }
    fn terminal_is_empty(&self) -> bool { self.phase == Phase::Closed }
}

impl Drop for Pdf14OperationReceiving {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "PDF 1.4 operation receiving requires paid closure");
    }
}

/// ♻️ Retires the original PDF 1.4 text backing through its caller's close grant.
pub struct Pdf14ReceivingRetirement(Pdf14OperationReceiving);

impl RetireOwned for Pdf14OperationReceiving {
    type RetirementCursor = Pdf14ReceivingRetirement;
    fn retirement_seed(self) -> Self::RetirementCursor { Pdf14ReceivingRetirement(self) }
    fn controlled_retirement_supported() -> bool { true }
}

impl Pdf14ReceivingRetirement {
    fn close_step(&mut self, _: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetirementStep {
        erased_cursor_turn(self.0.close(effect))
    }
}

impl RetirementCursor for Pdf14ReceivingRetirement {
    semio_framework_value::native_retirement_reply!();
    fn terminal_is_empty(&self) -> bool { self.0.terminal_is_empty() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.0.close_demand().depth) }
    fn next_work_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.0.close_demand().copy_bytes) }
    fn next_close_byte_demand(&self) -> Option<usize> { Some(self.0.close_demand().release_bytes) }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}
