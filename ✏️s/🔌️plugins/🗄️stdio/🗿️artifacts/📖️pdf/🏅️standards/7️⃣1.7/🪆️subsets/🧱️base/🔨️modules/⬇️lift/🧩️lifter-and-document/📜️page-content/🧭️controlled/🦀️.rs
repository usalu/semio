//! 📽️ Projects native page operators from borrowed streams and the exact concatenation of split programs.

use super::{ObjRef, ObjectSource, PdfObject};
use crate::standards::v1_7::subsets::base::modules::content::{ContentProgramCursor, ContentProgramEvent, ContentProgramReceipt, FontTable, NoFonts};
use crate::standards::v1_7::subsets::base::modules::lexer::CosGrant;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfOp, PdfStreamBytes};
use semio_framework_value::ByteReadSource;

#[derive(Clone, Copy)]
enum Location {
    Owner(ObjRef),
    Entry { page: ObjRef, index: usize },
}

impl Location {
    fn inspect(self, source: &mut dyn ObjectSource, read: &mut dyn FnMut(Option<&PdfObject>)) {
        match self {
            Self::Owner(reference) => source.inspect(reference, read),
            Self::Entry { page, index } => source.inspect(page, &mut |value| {
                read(value.and_then(PdfObject::as_dict).and_then(|dict| dict.get(index)).map(|entry| &entry.value));
            }),
        }
    }
}

#[derive(Clone, Copy)]
enum Span {
    Direct(Location),
    Member { array: Location, index: usize },
}

impl Span {
    fn inspect(self, source: &mut dyn ObjectSource, read: &mut dyn FnMut(Option<&PdfStreamBytes>)) {
        let mut stream = |value: Option<&PdfObject>| read(stream_bytes(value));
        match self {
            Self::Direct(location) => location.inspect(source, &mut stream),
            Self::Member { array, index } => array.inspect(source, &mut |value| {
                stream(value.and_then(PdfObject::as_array).and_then(|items| items.get(index)));
            }),
        }
    }
}

fn stream_bytes(value: Option<&PdfObject>) -> Option<&PdfStreamBytes> {
    match value {
        Some(PdfObject::Stream { data, .. }) => Some(data),
        _ => None,
    }
}

enum Member {
    End,
    Skip,
    Reference(ObjRef),
    Inline(usize),
}

fn member(source: &mut dyn ObjectSource, array: Location, index: usize) -> Member {
    let mut member = Member::End;
    array.inspect(source, &mut |value| {
        member = match value.and_then(PdfObject::as_array).and_then(|items| items.get(index)) {
            Some(PdfObject::Ref(reference)) => Member::Reference(*reference),
            Some(PdfObject::Stream { data, .. }) => Member::Inline(data.len()),
            Some(_) => Member::Skip,
            None => Member::End,
        };
    });
    member
}

#[derive(Clone, Copy)]
enum Stage {
    Locate { page: ObjRef, index: usize },
    Inspect { location: Location },
    Measure { array: Location, index: usize },
    MeasureReference { array: Location, index: usize, reference: ObjRef },
    Allocate { array: Location },
    Join { array: Location, index: usize },
    Copy { array: Location, index: usize, span: Span, offset: usize, separator: bool },
    Interpret(Input),
    Deferred(PageContentBarrier),
    Failed(&'static str),
    Complete,
    Cancelled,
    Closing,
    Closed,
}

#[derive(Clone, Copy)]
enum Input { Joined, Empty }

/// 🚧️ Identifies a controlled interpretation barrier without consuming the unavailable operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageContentBarrier {
    Operator { start: usize, end: usize },
    ResourceContext,
}

/// 🛑️ Reports the bounded page-content cursor state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageContentEvent {
    More,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    OperatorReady,
    Deferred(PageContentBarrier),
    Complete,
    Cancelled,
    Closed,
}

/// 🧾️ Reports graph inspection, split-stream assembly, and operator effects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[must_use]
pub struct PageContentReceipt {
    pub event: Result<PageContentEvent, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

/// 📖️ Borrows one native page's decoded streams and yields typed operators without resolving resources.
#[must_use]
pub struct PageContentCursor {
    stage: Stage,
    planned_bytes: usize,
    stream_count: usize,
    first_stream: Option<Span>,
    joined: Vec<u8>,
    program: ContentProgramCursor,
    program_closed: bool,
}

impl PageContentCursor {
    /// 🌱 Opens the page object without cloning its dictionary, array or stream backing.
    pub fn new(page: ObjRef) -> Self {
        Self {
            stage: Stage::Locate { page, index: 0 },
            planned_bytes: 0,
            stream_count: 0,
            first_stream: None,
            joined: Vec::new(),
            program: ContentProgramCursor::new(0),
            program_closed: false,
        }
    }

    /// 📊 Returns the number of resolved stream members after the input is planned.
    pub fn stream_count(&self) -> usize { self.stream_count }

    /// 👁️ Exposes the currently ready operator without transferring it.
    pub fn peek_operator(&self) -> Option<&PdfOp> {
        if !matches!(self.stage, Stage::Interpret(_)) { return None; }
        self.program.peek_operator()
    }

    pub(super) fn selected_font(&self) -> Option<&str> { self.program.selected_font() }

    /// 🫴️ Transfers one ready typed operator and its native backing.
    pub fn take_operator(&mut self) -> Option<PdfOp> {
        if !matches!(self.stage, Stage::Interpret(_)) { return None; }
        self.program.take_operator()
    }

    /// ▶️ Proceeds with the same retained graph on every turn.
    pub fn advance(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, cancelled: bool) -> PageContentReceipt {
        self.advance_with_fonts(source, &NoFonts, grant, cancelled)
    }

    pub(super) fn advance_with_fonts(&mut self, source: &mut dyn ObjectSource, fonts: &dyn FontTable, grant: CosGrant, cancelled: bool) -> PageContentReceipt {
        use PageContentEvent as Event;
        if cancelled && !matches!(self.stage, Stage::Complete | Stage::Closing | Stage::Closed) {
            self.stage = Stage::Cancelled;
        }
        match self.stage {
            Stage::Deferred(reason) => signal(Event::Deferred(reason)),
            Stage::Failed(reason) => failed(reason),
            Stage::Complete => signal(Event::Complete),
            Stage::Cancelled => signal(Event::Cancelled),
            Stage::Closing => self.close_step(grant),
            Stage::Closed => signal(Event::Closed),
            _ if grant.work == 0 => signal(Event::More),
            Stage::Locate { page, index } => {
                self.stage = Stage::Interpret(Input::Empty);
                source.inspect(page, &mut |value| {
                    if let Some(entry) = value.and_then(PdfObject::as_dict).and_then(|dict| dict.get(index)) {
                        self.stage = if entry.key == "Contents" {
                            Stage::Inspect { location: Location::Entry { page, index } }
                        } else {
                            Stage::Locate { page, index: index + 1 }
                        };
                    }
                });
                worked()
            }
            Stage::Inspect { location } => {
                self.stage = Stage::Interpret(Input::Empty);
                location.inspect(source, &mut |value| match value {
                    Some(PdfObject::Stream { data, .. }) => {
                        self.stream_count = 1;
                        self.planned_bytes = data.len();
                        self.first_stream = Some(Span::Direct(location));
                        self.stage = Stage::Allocate { array: location };
                    }
                    Some(PdfObject::Ref(reference)) if matches!(location, Location::Entry { .. }) => {
                        self.stage = Stage::Inspect { location: Location::Owner(*reference) };
                    }
                    Some(PdfObject::Array(_)) => self.stage = Stage::Measure { array: location, index: 0 },
                    _ => {}
                });
                worked()
            }
            Stage::Measure { array, index } => self.measure(source, array, index),
            Stage::MeasureReference { array, index, reference } => {
                let span = Span::Direct(Location::Owner(reference));
                let mut len = None;
                span.inspect(source, &mut |data| { len = data.map(PdfStreamBytes::len); });
                self.record_span(array, index, len.map(|len| (span, len)))
            }
            Stage::Allocate { array } => {
                if self.planned_bytes > grant.allocation_bytes {
                    return signal(Event::NeedsAllocation { bytes: self.planned_bytes });
                }
                if self.joined.try_reserve_exact(self.planned_bytes).is_err() {
                    return self.fail("cannot allocate combined PDF page content");
                }
                self.stage = match (self.stream_count, self.first_stream) {
                    (1, Some(span)) => Stage::Copy { array, index: 0, span, offset: 0, separator: false },
                    _ => Stage::Join { array, index: 0 },
                };
                PageContentReceipt { allocated_bytes: self.joined.capacity(), ..worked() }
            }
            Stage::Join { array, index } => {
                let span = match member(source, array, index) {
                    Member::End => {
                        if self.joined.len() != self.planned_bytes {
                            return self.fail("PDF page content length changed after joining");
                        }
                        self.stage = Stage::Interpret(Input::Joined);
                        return worked();
                    }
                    Member::Skip => {
                        self.stage = Stage::Join { array, index: index + 1 };
                        return worked();
                    }
                    Member::Reference(reference) => Span::Direct(Location::Owner(reference)),
                    Member::Inline(_) => Span::Member { array, index },
                };
                self.stage = Stage::Copy { array, index, span, offset: 0, separator: !self.joined.is_empty() };
                worked()
            }
            Stage::Copy { array, index, span, offset, separator } => {
                self.join(source, grant, array, index, span, offset, separator)
            }
            Stage::Interpret(input) => self.interpret(source, fonts, grant, input),
        }
    }

    /// ♻️ Begins settling partial or completed input and semantic custody.
    pub fn begin_close(&mut self) {
        if !matches!(self.stage, Stage::Closing | Stage::Closed) {
            self.program.begin_close();
            self.stage = Stage::Closing;
        }
    }

    /// 🧹️ Retires the lexer and joined-byte backing within the turn's grants.
    pub fn close_step(&mut self, grant: CosGrant) -> PageContentReceipt {
        use PageContentEvent as Event;
        if matches!(self.stage, Stage::Closed) { return signal(Event::Closed); }
        if !matches!(self.stage, Stage::Closing) || grant.work == 0 { return signal(Event::More); }
        if !self.program_closed {
            let turn = self.program.close_step(grant);
            self.program_closed = turn.event == Ok(ContentProgramEvent::Closed);
            let mut receipt = program_receipt(turn);
            if self.program_closed { receipt.event = Ok(Event::More); }
            return receipt;
        }
        let bytes = self.joined.capacity();
        if bytes > grant.release_bytes { return signal(Event::NeedsRelease { bytes }); }
        self.joined = Vec::new();
        self.stage = Stage::Closed;
        PageContentReceipt { released_bytes: bytes, event: Ok(Event::Closed), ..worked() }
    }

    fn measure(&mut self, source: &mut dyn ObjectSource, array: Location, index: usize) -> PageContentReceipt {
        let measured = match member(source, array, index) {
            Member::End => {
                self.stage = match (self.stream_count, self.first_stream) {
                    (0, _) => Stage::Interpret(Input::Empty),
                    (1, Some(_)) => Stage::Allocate { array },
                    _ => Stage::Allocate { array },
                };
                return worked();
            }
            Member::Skip => None,
            Member::Inline(len) => Some((Span::Member { array, index }, len)),
            Member::Reference(reference) => {
                self.stage = Stage::MeasureReference { array, index, reference };
                return worked();
            }
        };
        self.record_span(array, index, measured)
    }

    fn record_span(&mut self, array: Location, index: usize, measured: Option<(Span, usize)>) -> PageContentReceipt {
        if let Some((span, len)) = measured {
            let Some(total) = self.planned_bytes.checked_add(usize::from(self.planned_bytes > 0))
                .and_then(|sum| sum.checked_add(len)).filter(|sum| *sum <= isize::MAX as usize) else {
                return self.fail("combined PDF page content exceeds native length");
            };
            self.first_stream.get_or_insert(span);
            self.stream_count += 1;
            self.planned_bytes = total;
        }
        self.stage = Stage::Measure { array, index: index + 1 };
        worked()
    }

    fn join(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, array: Location, index: usize, span: Span, offset: usize, separator: bool) -> PageContentReceipt {
        let mut turn = worked();
        let mut next = Stage::Join { array, index: index + 1 };
        span.inspect(source, &mut |bytes| {
            let Some(bytes) = bytes else { return };
            let Some(tail_len) = bytes.len().checked_sub(offset) else {
                next = Stage::Failed("PDF page content source changed while joining");
                return;
            };
            let separator_count = usize::from(separator);
            let count = tail_len.min(grant.work - separator_count);
            let remaining = self.joined.capacity() - self.joined.len();
            if separator_count + count > remaining {
                next = Stage::Failed("PDF page content source exceeds its planned length");
                return;
            }
            if separator { self.joined.push(b'\n'); }
            let start = self.joined.len();
            self.joined.resize(start + count, 0);
            if bytes.read_bytes(offset, &mut self.joined[start..]) != count {
                next = Stage::Failed("PDF page content source changed while joining");
                return;
            }
            turn.work = (separator_count + count).max(1);
            if count < tail_len {
                next = Stage::Copy { array, index, span, offset: offset + count, separator: false };
            }
        });
        self.stage = next;
        if let Stage::Failed(reason) = next { turn.event = Err(reason); }
        turn
    }

    fn interpret(&mut self, source: &mut dyn ObjectSource, fonts: &dyn FontTable, grant: CosGrant, input: Input) -> PageContentReceipt {
        let turn = match input {
            Input::Empty => self.program.advance(&[], fonts, grant, false),
            Input::Joined => self.program.advance(&self.joined, fonts, grant, false),
        };
        let turn = program_receipt(turn);
        self.stage = match turn.event {
            Ok(PageContentEvent::Deferred(reason)) => Stage::Deferred(reason),
            Ok(PageContentEvent::Complete) => Stage::Complete,
            Err(reason) => Stage::Failed(reason),
            _ => self.stage,
        };
        turn
    }

    fn fail(&mut self, reason: &'static str) -> PageContentReceipt {
        self.stage = Stage::Failed(reason);
        PageContentReceipt { work: 1, ..failed(reason) }
    }
}

pub(super) fn signal(event: PageContentEvent) -> PageContentReceipt {
    PageContentReceipt { event: Ok(event), work: 0, allocated_bytes: 0, released_bytes: 0 }
}

pub(super) fn worked() -> PageContentReceipt { PageContentReceipt { work: 1, ..signal(PageContentEvent::More) } }

fn failed(reason: &'static str) -> PageContentReceipt { PageContentReceipt { event: Err(reason), ..signal(PageContentEvent::More) } }

fn program_receipt(turn: ContentProgramReceipt) -> PageContentReceipt {
    use PageContentEvent as Event;
    let event = turn.event.map(|event| match event {
        ContentProgramEvent::More => Event::More,
        ContentProgramEvent::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        ContentProgramEvent::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        ContentProgramEvent::OperatorReady => Event::OperatorReady,
        ContentProgramEvent::Deferred { start, end } => Event::Deferred(PageContentBarrier::Operator { start, end }),
        ContentProgramEvent::Complete => Event::Complete,
        ContentProgramEvent::Cancelled => Event::Cancelled,
        ContentProgramEvent::Closed => Event::Closed,
    });
    PageContentReceipt { event, work: turn.work, allocated_bytes: turn.allocated_bytes, released_bytes: turn.released_bytes }
}
