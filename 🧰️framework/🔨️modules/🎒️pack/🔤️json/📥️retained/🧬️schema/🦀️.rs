//! 🔣️ Fixed-page retained JSON token, record, string and hexadecimal schema authority.
use semio_framework_value::{ValueError, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use std::mem::{size_of, size_of_val};
#[path="../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/🧬️schema/⚠️diagnostic/🦀️.rs"]
pub mod schema_diagnostic;

pub const OWNED_SCHEMA_DECODE_PAGE_BYTES: usize = 4_096;

pub const OWNED_SCHEMA_DECODE_MAXIMUM_PAGES: usize = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnedSchemaDecodePage {
    bytes: [u8; OWNED_SCHEMA_DECODE_PAGE_BYTES],
    len: u16,
}

impl OwnedSchemaDecodePage {
    /// 🫙️ Fixed inline storage admits no heap allocation before the first source copy.
    pub const fn empty()->Self{Self{bytes:[0;OWNED_SCHEMA_DECODE_PAGE_BYTES],len:0}}
    /// 📋️ Borrows the exact ordered byte prefix without copying or materializing a page.
    pub fn as_slice(&self)->&[u8]{&self.bytes[..self.len()]}
    /// ⛽️ Copies only the separately granted borrowed source prefix into fixed inline capacity.
    pub fn append_granted(&mut self,source:&[u8],maximum_bytes:usize)->usize{
        let copied=source.len().min(maximum_bytes).min(OWNED_SCHEMA_DECODE_PAGE_BYTES-self.len());
        let start=self.len();self.bytes[start..start+copied].copy_from_slice(&source[..copied]);self.len=(start+copied)as u16;copied
    }
    /// 🖋️ Lends only the admitted output prefix to the canonical incremental envelope writer.
    pub fn writable_prefix(&mut self,maximum_bytes:usize)->&mut[u8]{let start=self.len();let end=start+maximum_bytes.min(OWNED_SCHEMA_DECODE_PAGE_BYTES-start);&mut self.bytes[start..end]}
    /// 🔒️ Commits only the writer's exact initialized prefix within original inline capacity.
    pub fn commit_written(&mut self,written:usize)->Result<(),OwnedSchemaDecodePageFault>{if written>OWNED_SCHEMA_DECODE_PAGE_BYTES-self.len(){return Err(OwnedSchemaDecodePageFault{actual_bytes:written});}self.len+=written as u16;Ok(())}
    /// 📄️ Copies one already-admitted canonical page into definitionally shallow storage.
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self, OwnedSchemaDecodePageFault> {
        if bytes.len() > OWNED_SCHEMA_DECODE_PAGE_BYTES {
            return Err(OwnedSchemaDecodePageFault { actual_bytes: bytes.len() });
        }
        let mut page = Self { bytes: [0; OWNED_SCHEMA_DECODE_PAGE_BYTES], len: bytes.len() as u16 };
        page.bytes[..bytes.len()].copy_from_slice(bytes);
        Ok(page)
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnedSchemaDecodePageFault {
    pub actual_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnedSchemaDecodeCredits {
    pub maximum_pages: usize,
    pub maximum_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnedSchemaDecodeAdmissionFault {
    ZeroCapacity,
    PageCapacity,
    ByteCapacity,
    PartialPageBeforeTerminal,
    Sealed,
    Allocation,
}

pub struct OwnedSchemaDecodePages {
    pub(crate) slots: Box<[std::mem::MaybeUninit<OwnedSchemaDecodePage>]>,
    page_count: usize,
    byte_count: usize,
    maximum_bytes: usize,
    sealed: bool,
}

impl OwnedSchemaDecodePages {
    /// 📐️ Names the indivisible boxed slot allocation independently of logical page bytes.
    pub fn allocation_byte_demand(&self)->usize{self.slots.len().checked_mul(size_of::<OwnedSchemaDecodePage>()).expect("admitted decode slots have a representable allocation")}
    /// 📐️ Prices a fixed slot birth before any original source owner transfers.
    pub fn allocation_byte_demand_for(maximum_pages:usize)->Option<usize>{maximum_pages.checked_mul(size_of::<OwnedSchemaDecodePage>())}
    /// 🎫️ Reserves fixed page and byte authority before any input owner is admitted.
    pub fn try_with_credits(credits: OwnedSchemaDecodeCredits) -> Result<Self, OwnedSchemaDecodeAdmissionFault> {
        if credits.maximum_pages == 0 || credits.maximum_bytes == 0 {
            return Err(OwnedSchemaDecodeAdmissionFault::ZeroCapacity);
        }
        if credits.maximum_pages > OWNED_SCHEMA_DECODE_MAXIMUM_PAGES || credits.maximum_bytes > credits.maximum_pages.checked_mul(OWNED_SCHEMA_DECODE_PAGE_BYTES).ok_or(OwnedSchemaDecodeAdmissionFault::ByteCapacity)? {
            return Err(OwnedSchemaDecodeAdmissionFault::ByteCapacity);
        }
        let mut slots = Vec::new();
        slots.try_reserve_exact(credits.maximum_pages).map_err(|_| OwnedSchemaDecodeAdmissionFault::Allocation)?;
        slots.resize_with(credits.maximum_pages, std::mem::MaybeUninit::uninit);
        Ok(Self { slots: slots.into_boxed_slice(), page_count: 0, byte_count: 0, maximum_bytes: credits.maximum_bytes, sealed: false })
    }

    /// 🔎️ Validates one page extent before a producer constructs its fixed owner.
    pub fn preflight_page_bytes(&self, page_bytes: usize) -> Result<(), OwnedSchemaDecodeAdmissionFault> {
        if self.sealed {
            return Err(OwnedSchemaDecodeAdmissionFault::Sealed);
        }
        if self.page_count == self.slots.len() {
            return Err(OwnedSchemaDecodeAdmissionFault::PageCapacity);
        }
        if self.page_count != 0 && unsafe { self.slots[self.page_count - 1].assume_init_ref() }.len() != OWNED_SCHEMA_DECODE_PAGE_BYTES {
            return Err(OwnedSchemaDecodeAdmissionFault::PartialPageBeforeTerminal);
        }
        if page_bytes > OWNED_SCHEMA_DECODE_PAGE_BYTES {
            return Err(OwnedSchemaDecodeAdmissionFault::ByteCapacity);
        }
        let Some(next_bytes) = self.byte_count.checked_add(page_bytes) else { return Err(OwnedSchemaDecodeAdmissionFault::ByteCapacity) };
        if next_bytes > self.maximum_bytes {
            return Err(OwnedSchemaDecodeAdmissionFault::ByteCapacity);
        }
        Ok(())
    }

    /// 📥️ Moves one exact page owner into its pre-reserved slot or returns it untouched.
    pub fn admit_page(&mut self, page: OwnedSchemaDecodePage) -> Result<(), (OwnedSchemaDecodeAdmissionFault, OwnedSchemaDecodePage)> {
        if let Err(fault) = self.preflight_page_bytes(page.len()) {
            return Err((fault, page));
        }
        self.admit_preflighted_page(page);
        Ok(())
    }

    /// 📨️ Commits the page built by a producer callback after the same mutable authority preflighted it.
    pub fn admit_preflighted_page(&mut self, page: OwnedSchemaDecodePage) {
        debug_assert!(self.preflight_page_bytes(page.len()).is_ok());
        let next_bytes = self.byte_count + page.len();
        self.slots[self.page_count].write(page);
        self.page_count += 1;
        self.byte_count = next_bytes;
    }

    /// 🔒️ Seals the exact admitted byte extent before tokenization begins.
    pub fn seal(&mut self) -> Result<(), OwnedSchemaDecodeAdmissionFault> {
        if self.sealed {
            return Err(OwnedSchemaDecodeAdmissionFault::Sealed);
        }
        self.sealed = true;
        Ok(())
    }

    pub fn is_sealed(&self) -> bool {
        self.sealed
    }

    pub fn page_count(&self) -> usize {
        self.page_count
    }
    /// 🔎️ Borrows one admitted page in original source order without transferring authority.
    pub fn admitted_page(&self,index:usize)->Option<&OwnedSchemaDecodePage>{(index<self.page_count).then(||unsafe{self.slots[index].assume_init_ref()})}

    pub fn byte_count(&self) -> usize {
        self.byte_count
    }

    pub fn byte_at(&self, offset: usize) -> Option<u8> {
        if offset >= self.byte_count {
            return None;
        }
        let page = offset / OWNED_SCHEMA_DECODE_PAGE_BYTES;
        let within = offset % OWNED_SCHEMA_DECODE_PAGE_BYTES;
        unsafe { self.slots.get(page)?.assume_init_ref().bytes.get(within).copied() }
    }

    /// 🧹️ Retires one logical admitted page per close grant; page payloads are inline bytes.
    pub fn close_page_bytes(&self) -> Option<usize> {
        (self.page_count != 0).then(|| unsafe { self.slots[self.page_count - 1].assume_init_ref() }.len())
    }

    pub fn close_take_page(&mut self) -> Option<OwnedSchemaDecodePage> {
        if self.page_count == 0 {
            return None;
        }
        self.page_count -= 1;
        let page = unsafe { self.slots[self.page_count].assume_init_read() };
        self.byte_count -= page.len();
        Some(page)
    }

    /// 🧹️ Releases the original whole directory only after its admitted inline pages are retired.
    pub fn close_backing_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.page_count!=0 || self.byte_count!=0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"decode directory still retains inline pages")); }
        if self.slots.is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items==0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth==0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"decode directory requires supplied depth")); }
        let bytes=self.allocation_byte_demand();
        if grant.maximum_release_bytes<bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        drop(std::mem::replace(&mut self.slots,Box::new([])));
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items:1,released_bytes:bytes,..Default::default() }))
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.page_count == 0 && self.byte_count == 0 && self.slots.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnedSchemaPath {
    bytes: [u8; 256],
    len: u8,
}

impl OwnedSchemaPath {
    pub const ROOT: Self = {
        let mut bytes = [0; 256];
        bytes[0] = b'$';
        Self { bytes, len: 1 }
    };

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..self.len as usize]) }
    }

    pub fn field(key: &'static str) -> Option<Self> {
        let len = key.len().checked_add(2)?;
        if len > u8::MAX as usize || !key.is_ascii() {
            return None;
        }
        let mut bytes = [0; 256];
        bytes[0] = b'$';
        bytes[1] = b'.';
        bytes[2..len].copy_from_slice(key.as_bytes());
        Some(Self { bytes, len: len as u8 })
    }
}

pub type OwnedSchemaDecodeDiagnostic=schema_diagnostic::SchemaDecodeDiagnostic<OwnedSchemaPath>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnedSchemaTokenKind {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    Colon,
    Comma,
    String,
    Number,
    True,
    False,
    Null,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnedSchemaToken {
    pub kind: OwnedSchemaTokenKind,
    pub start: u64,
    pub end: u64,
}

#[derive(Clone, Copy)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaLexState {
    Ready,
    String { start: usize, escape: bool, unicode: u8, utf8: u8, next_min: u8, next_max: u8 },
    Number { start: usize, bytes: [u8; 64], len: u8 },
    Literal { start: usize, kind: OwnedSchemaTokenKind, expected: &'static [u8], index: u8 },
    Complete,
    Fault(OwnedSchemaDecodeDiagnostic),
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaTokenStep {
    Pending,
    Token(OwnedSchemaToken),
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

pub struct OwnedSchemaTokenCursor {
    pub(crate) pages: OwnedSchemaDecodePages,
    state: OwnedSchemaLexState,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    offset: usize,
    line: u32,
    column: u32,
}

impl OwnedSchemaTokenCursor {
    /// 🧳️ Returns unchanged pages after schema admission refuses the original cursor.
    pub fn into_pages(self) -> OwnedSchemaDecodePages { self.pages }

    /// 🧬️ Begins tokenization only after exact fixed-page admission is sealed.
    pub fn try_new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, pages: OwnedSchemaDecodePages) -> Result<Self, OwnedSchemaDecodePages> {
        if !pages.is_sealed() {
            return Err(pages);
        }
        Ok(Self { pages, state: OwnedSchemaLexState::Ready, operation, generation, offset: 0, line: 1, column: 1 })
    }

    fn diagnostic(&self, code: &'static str, offset: usize) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: offset as u64, line: self.line, column: self.column, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    fn advance(&mut self, byte: u8) {
        self.offset += 1;
        if byte == b'\n' {
            self.line = self.line.saturating_add(1);
            self.column = 1;
        } else {
            self.column = self.column.saturating_add(1);
        }
    }

    fn punct(&mut self, byte: u8, kind: OwnedSchemaTokenKind, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaTokenStep {
        let start = self.offset;
        self.advance(byte);
        cx.consume_fuel(1);
        OwnedSchemaTokenStep::Token(OwnedSchemaToken { kind, start: start as u64, end: self.offset as u64 })
    }

    fn valid_number(bytes: &[u8]) -> bool {
        let mut cursor = 0;
        if bytes.get(cursor) == Some(&b'-') {
            cursor += 1;
        }
        match bytes.get(cursor) {
            Some(b'0') => cursor += 1,
            Some(b'1'..=b'9') => {
                cursor += 1;
                while matches!(bytes.get(cursor), Some(b'0'..=b'9')) {
                    cursor += 1;
                }
            }
            _ => return false,
        }
        if bytes.get(cursor) == Some(&b'.') {
            cursor += 1;
            let fraction = cursor;
            while matches!(bytes.get(cursor), Some(b'0'..=b'9')) {
                cursor += 1;
            }
            if cursor == fraction {
                return false;
            }
        }
        if matches!(bytes.get(cursor), Some(b'e' | b'E')) {
            cursor += 1;
            if matches!(bytes.get(cursor), Some(b'+' | b'-')) {
                cursor += 1;
            }
            let exponent = cursor;
            while matches!(bytes.get(cursor), Some(b'0'..=b'9')) {
                cursor += 1;
            }
            if cursor == exponent {
                return false;
            }
        }
        cursor == bytes.len()
    }

    fn finish_eof(&mut self) -> OwnedSchemaTokenStep {
        match self.state {
            OwnedSchemaLexState::Ready => {
                self.state = OwnedSchemaLexState::Complete;
                OwnedSchemaTokenStep::Complete
            }
            OwnedSchemaLexState::Number { start, bytes, len } if Self::valid_number(&bytes[..len as usize]) => {
                self.state = OwnedSchemaLexState::Ready;
                OwnedSchemaTokenStep::Token(OwnedSchemaToken { kind: OwnedSchemaTokenKind::Number, start: start as u64, end: self.offset as u64 })
            }
            OwnedSchemaLexState::Complete => OwnedSchemaTokenStep::Complete,
            OwnedSchemaLexState::Cancelled => OwnedSchemaTokenStep::Cancelled,
            OwnedSchemaLexState::Fault(diagnostic) => OwnedSchemaTokenStep::Fault(diagnostic),
            _ => {
                let diagnostic = self.diagnostic("schema-json.truncated-token", self.offset);
                self.state = OwnedSchemaLexState::Fault(diagnostic);
                OwnedSchemaTokenStep::Fault(diagnostic)
            }
        }
    }

    /// ⏭️ Advances lexical validation by budgeted bytes and yields at most one schema token.
    pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaTokenStep {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            let diagnostic = self.diagnostic("schema-json.stale-authority", self.offset);
            self.state = OwnedSchemaLexState::Fault(diagnostic);
            return OwnedSchemaTokenStep::Fault(diagnostic);
        }
        if cx.is_cancelled() {
            self.state = OwnedSchemaLexState::Cancelled;
            return OwnedSchemaTokenStep::Cancelled;
        }
        cx.set_stage("schema-json-token");
        loop {
            if cx.should_yield() {
                return OwnedSchemaTokenStep::Pending;
            }
            let Some(byte) = self.pages.byte_at(self.offset) else {
                return self.finish_eof();
            };
            let state = self.state;
            let outcome = match state {
                OwnedSchemaLexState::Ready => match byte {
                    b' ' | b'\t' | b'\r' | b'\n' => {
                        self.advance(byte);
                        None
                    }
                    b'{' => return self.punct(byte, OwnedSchemaTokenKind::ObjectStart, cx),
                    b'}' => return self.punct(byte, OwnedSchemaTokenKind::ObjectEnd, cx),
                    b'[' => return self.punct(byte, OwnedSchemaTokenKind::ArrayStart, cx),
                    b']' => return self.punct(byte, OwnedSchemaTokenKind::ArrayEnd, cx),
                    b':' => return self.punct(byte, OwnedSchemaTokenKind::Colon, cx),
                    b',' => return self.punct(byte, OwnedSchemaTokenKind::Comma, cx),
                    b'"' => {
                        let start = self.offset;
                        self.advance(byte);
                        self.state = OwnedSchemaLexState::String { start, escape: false, unicode: 0, utf8: 0, next_min: 0x80, next_max: 0xbf };
                        None
                    }
                    b'-' | b'0'..=b'9' => {
                        let mut bytes = [0; 64];
                        bytes[0] = byte;
                        let start = self.offset;
                        self.advance(byte);
                        self.state = OwnedSchemaLexState::Number { start, bytes, len: 1 };
                        None
                    }
                    b't' => {
                        let start = self.offset;
                        self.advance(byte);
                        self.state = OwnedSchemaLexState::Literal { start, kind: OwnedSchemaTokenKind::True, expected: b"true", index: 1 };
                        None
                    }
                    b'f' => {
                        let start = self.offset;
                        self.advance(byte);
                        self.state = OwnedSchemaLexState::Literal { start, kind: OwnedSchemaTokenKind::False, expected: b"false", index: 1 };
                        None
                    }
                    b'n' => {
                        let start = self.offset;
                        self.advance(byte);
                        self.state = OwnedSchemaLexState::Literal { start, kind: OwnedSchemaTokenKind::Null, expected: b"null", index: 1 };
                        None
                    }
                    _ => Some(self.diagnostic("schema-json.unexpected-byte", self.offset)),
                },
                OwnedSchemaLexState::String { start, mut escape, mut unicode, mut utf8, mut next_min, mut next_max } => {
                    if unicode != 0 {
                        if !byte.is_ascii_hexdigit() {
                            Some(self.diagnostic("schema-json.invalid-unicode-escape", self.offset))
                        } else {
                            unicode -= 1;
                            self.advance(byte);
                            self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8, next_min, next_max };
                            None
                        }
                    } else if escape {
                        if !matches!(byte, b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' | b'u') {
                            Some(self.diagnostic("schema-json.invalid-escape", self.offset))
                        } else {
                            escape = false;
                            unicode = usize::from(byte == b'u') as u8 * 4;
                            self.advance(byte);
                            self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8, next_min, next_max };
                            None
                        }
                    } else if utf8 != 0 {
                        if byte < next_min || byte > next_max {
                            Some(self.diagnostic("schema-json.invalid-utf8", self.offset))
                        } else {
                            utf8 -= 1;
                            next_min = 0x80;
                            next_max = 0xbf;
                            self.advance(byte);
                            self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8, next_min, next_max };
                            None
                        }
                    } else {
                        match byte {
                            b'"' => {
                                self.advance(byte);
                                cx.consume_fuel(1);
                                self.state = OwnedSchemaLexState::Ready;
                                return OwnedSchemaTokenStep::Token(OwnedSchemaToken { kind: OwnedSchemaTokenKind::String, start: start as u64, end: self.offset as u64 });
                            }
                            b'\\' => {
                                escape = true;
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8, next_min, next_max };
                                None
                            }
                            0x00..=0x1f => Some(self.diagnostic("schema-json.control-in-string", self.offset)),
                            0x20..=0x7f => {
                                self.advance(byte);
                                None
                            }
                            0xc2..=0xdf => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 1, next_min: 0x80, next_max: 0xbf };
                                None
                            }
                            0xe0 => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 2, next_min: 0xa0, next_max: 0xbf };
                                None
                            }
                            0xe1..=0xec | 0xee..=0xef => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 2, next_min: 0x80, next_max: 0xbf };
                                None
                            }
                            0xed => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 2, next_min: 0x80, next_max: 0x9f };
                                None
                            }
                            0xf0 => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 3, next_min: 0x90, next_max: 0xbf };
                                None
                            }
                            0xf1..=0xf3 => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 3, next_min: 0x80, next_max: 0xbf };
                                None
                            }
                            0xf4 => {
                                self.advance(byte);
                                self.state = OwnedSchemaLexState::String { start, escape, unicode, utf8: 3, next_min: 0x80, next_max: 0x8f };
                                None
                            }
                            _ => Some(self.diagnostic("schema-json.invalid-utf8", self.offset)),
                        }
                    }
                }
                OwnedSchemaLexState::Number { start, mut bytes, mut len } => {
                    if matches!(byte, b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E') {
                        if len as usize == bytes.len() {
                            Some(self.diagnostic("schema-json.number-token-too-long", self.offset))
                        } else {
                            bytes[len as usize] = byte;
                            len += 1;
                            self.advance(byte);
                            self.state = OwnedSchemaLexState::Number { start, bytes, len };
                            None
                        }
                    } else if Self::valid_number(&bytes[..len as usize]) {
                        self.state = OwnedSchemaLexState::Ready;
                        return OwnedSchemaTokenStep::Token(OwnedSchemaToken { kind: OwnedSchemaTokenKind::Number, start: start as u64, end: self.offset as u64 });
                    } else {
                        Some(self.diagnostic("schema-json.invalid-number", start))
                    }
                }
                OwnedSchemaLexState::Literal { start, kind, expected, index } => {
                    if expected.get(index as usize) != Some(&byte) {
                        Some(self.diagnostic("schema-json.invalid-literal", self.offset))
                    } else {
                        self.advance(byte);
                        let index = index + 1;
                        if index as usize == expected.len() {
                            cx.consume_fuel(1);
                            self.state = OwnedSchemaLexState::Ready;
                            return OwnedSchemaTokenStep::Token(OwnedSchemaToken { kind, start: start as u64, end: self.offset as u64 });
                        }
                        self.state = OwnedSchemaLexState::Literal { start, kind, expected, index };
                        None
                    }
                }
                OwnedSchemaLexState::Complete => return OwnedSchemaTokenStep::Complete,
                OwnedSchemaLexState::Fault(diagnostic) => return OwnedSchemaTokenStep::Fault(diagnostic),
                OwnedSchemaLexState::Cancelled => return OwnedSchemaTokenStep::Cancelled,
            };
            cx.consume_fuel(1);
            if let Some(diagnostic) = outcome {
                self.state = OwnedSchemaLexState::Fault(diagnostic);
                return OwnedSchemaTokenStep::Fault(diagnostic);
            }
            if cx.is_cancelled() {
                self.state = OwnedSchemaLexState::Cancelled;
                return OwnedSchemaTokenStep::Cancelled;
            }
        }
    }

    /// 🧮️ Names the inline payload work of the next admitted page without retiring it.
    pub fn next_close_copy_byte_demand(&self) -> usize {
        self.pages.close_page_bytes().unwrap_or(0)
    }

    /// 🧹️ Retires exactly one admitted page from a cancelled or rejected decode, charging its inline bytes as payload work.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty=RetainedCloneProgress::default();
        let Some(bytes)=self.pages.close_page_bytes() else { return self.pages.close_backing_step(grant); };
        if grant.maximum_items==0 || grant.maximum_copy_bytes<bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth==0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"decode page requires supplied depth")); }
        let page=self.pages.close_take_page().expect("observed page remains retained");
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,copied_bytes:page.len(),..empty }))
    }
    pub fn next_close_capacity_byte_demand(&self, _body:usize)->Result<usize,ValueError>{Ok(0)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.pages.close_page_bytes().is_some(){0}else{self.pages.allocation_byte_demand()})}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal_is_empty()))}
    pub fn terminal_is_empty(&self) -> bool { self.pages.terminal_is_empty() }

    /// 📐️ Copies a bounded token span directly from fixed pages without concatenation.
    pub fn copy_token_bytes(&self, token: OwnedSchemaToken, relative_offset: usize, output: &mut [u8]) -> usize {
        let start = token.start as usize;
        let end = token.end as usize;
        let Some(mut offset) = start.checked_add(relative_offset).filter(|offset| *offset < end) else {
            return 0;
        };
        let maximum = output.len().min(OWNED_SCHEMA_DECODE_PAGE_BYTES);
        let mut written = 0;
        while written < maximum && offset < end {
            let Some(byte) = self.pages.byte_at(offset) else { break };
            output[written] = byte;
            written += 1;
            offset += 1;
        }
        written
    }

    fn string_token_equals(&self, token: OwnedSchemaToken, expected: &str) -> bool {
        if token.kind != OwnedSchemaTokenKind::String || !expected.is_ascii() || expected.len() > 253 {
            return false;
        }
        let (Ok(start), Ok(end)) = (usize::try_from(token.start), usize::try_from(token.end)) else { return false };
        let Some(length) = end.checked_sub(start).filter(|length| *length >= 2 && *length <= expected.len() * 6 + 2) else { return false };
        if self.pages.byte_at(start) != Some(b'"') || self.pages.byte_at(start + length - 1) != Some(b'"') {
            return false;
        }
        let read = |offset| if offset < end - 1 { self.pages.byte_at(offset) } else { None };
        let mut offset = start + 1;
        for expected in expected.bytes() {
            let Some(mut byte) = read(offset) else { return false };
            offset += 1;
            if byte == b'\\' {
                let Some(escape) = read(offset) else { return false };
                offset += 1;
                byte = match escape {
                    b'"' | b'\\' | b'/' => escape,
                    b'b' => 8,
                    b'f' => 12,
                    b'n' => b'\n',
                    b'r' => b'\r',
                    b't' => b'\t',
                    b'u' => {
                        let mut value = 0u16;
                        for _ in 0..4 {
                            let Some(digit) = read(offset) else { return false };
                            let digit = match digit {
                                b'0'..=b'9' => digit - b'0',
                                b'a'..=b'f' => digit - b'a' + 10,
                                b'A'..=b'F' => digit - b'A' + 10,
                                _ => return false,
                            };
                            value = value * 16 + u16::from(digit);
                            offset += 1;
                        }
                        if value > 127 {
                            return false;
                        }
                        value as u8
                    }
                    _ => return false,
                };
            }
            if byte != expected {
                return false;
            }
        }
        offset == end - 1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnedSchemaFieldSpec {
    pub id: u16,
    pub key: &'static str,
    pub required: bool,
}

#[derive(Clone, Copy)]
pub struct OwnedSchemaRecordSpec {
    pub fields: &'static [OwnedSchemaFieldSpec],
}

impl OwnedSchemaRecordSpec {
    /// 📋️ Admits a fixed schema whose field IDs and keys form an exact bijection.
    pub fn validate(self) -> Result<Self, &'static str> {
        if self.fields.is_empty() || self.fields.len() > 64 {
            return Err("schema-json.field-capacity");
        }
        for (index, field) in self.fields.iter().enumerate() {
            if field.id == 0 || field.key.is_empty() || field.key.len() > 253 || !field.key.is_ascii() {
                return Err("schema-json.invalid-field-spec");
            }
            if self.fields[..index].iter().any(|candidate| candidate.id == field.id || candidate.key == field.key) {
                return Err("schema-json.duplicate-field-spec");
            }
        }
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaRecordStep {
    Pending,
    FieldToken { field_id: u16, token: OwnedSchemaToken, terminal: bool },
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

#[derive(Clone, Copy)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaRecordState {
    Root,
    Key,
    KeyAfterComma,
    Colon { field_index: u8 },
    Value { field_index: u8 },
    Nested { field_index: u8, depth: u16 },
    Separator,
    Trailing,
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

pub struct OwnedSchemaRecordCursor {
    spec: OwnedSchemaRecordSpec,
    pub(crate) tokens: OwnedSchemaTokenCursor,
    state: OwnedSchemaRecordState,
    seen: u64,
    delimiters: [OwnedSchemaTokenKind; 256],
}

impl OwnedSchemaRecordCursor {
    /// 🧬️ Couples fixed schema authority to the fixed-page token cursor.
    pub fn try_new(spec: OwnedSchemaRecordSpec, tokens: OwnedSchemaTokenCursor) -> Result<Self, (OwnedSchemaTokenCursor, &'static str)> {
        let spec = match spec.validate() {
            Ok(spec) => spec,
            Err(error) => return Err((tokens, error)),
        };
        Ok(Self { spec, tokens, state: OwnedSchemaRecordState::Root, seen: 0, delimiters: [OwnedSchemaTokenKind::ObjectStart; 256] })
    }

    fn fault(&mut self, code: &'static str, token: OwnedSchemaToken, field_index: Option<u8>) -> OwnedSchemaRecordStep {
        let path = field_index.and_then(|index| self.spec.fields.get(index as usize)).and_then(|field| OwnedSchemaPath::field(field.key)).unwrap_or(OwnedSchemaPath::ROOT);
        let diagnostic = OwnedSchemaDecodeDiagnostic { code, offset: token.start, line: self.tokens.line, column: self.tokens.column, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
        self.state = OwnedSchemaRecordState::Fault(diagnostic);
        OwnedSchemaRecordStep::Fault(diagnostic)
    }

    fn field_index(&self, token: OwnedSchemaToken) -> Option<usize> {
        self.spec.fields.iter().position(|field| self.tokens.string_token_equals(token, field.key))
    }

    fn required_complete(&self) -> bool {
        self.spec.fields.iter().enumerate().all(|(index, field)| !field.required || self.seen & (1 << index) != 0)
    }

    /// ⏭️ Advances one schema token, preserving exact field identity and duplicate state.
    pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaRecordStep {
        if matches!(self.state, OwnedSchemaRecordState::Complete) {
            return OwnedSchemaRecordStep::Complete;
        }
        if matches!(self.state, OwnedSchemaRecordState::Cancelled) || cx.is_cancelled() {
            self.state = OwnedSchemaRecordState::Cancelled;
            return OwnedSchemaRecordStep::Cancelled;
        }
        if let OwnedSchemaRecordState::Fault(diagnostic) = self.state {
            return OwnedSchemaRecordStep::Fault(diagnostic);
        }
        cx.set_stage("schema-json-field");
        let token = match self.tokens.step(cx) {
            OwnedSchemaTokenStep::Pending => return OwnedSchemaRecordStep::Pending,
            OwnedSchemaTokenStep::Token(token) => token,
            OwnedSchemaTokenStep::Complete if matches!(self.state, OwnedSchemaRecordState::Trailing) => {
                self.state = OwnedSchemaRecordState::Complete;
                return OwnedSchemaRecordStep::Complete;
            }
            OwnedSchemaTokenStep::Complete => {
                let token = OwnedSchemaToken { kind: OwnedSchemaTokenKind::Null, start: self.tokens.offset as u64, end: self.tokens.offset as u64 };
                return self.fault("schema-json.truncated-record", token, None);
            }
            OwnedSchemaTokenStep::Cancelled => {
                self.state = OwnedSchemaRecordState::Cancelled;
                return OwnedSchemaRecordStep::Cancelled;
            }
            OwnedSchemaTokenStep::Fault(diagnostic) => {
                self.state = OwnedSchemaRecordState::Fault(diagnostic);
                return OwnedSchemaRecordStep::Fault(diagnostic);
            }
        };
        match self.state {
            OwnedSchemaRecordState::Root if token.kind == OwnedSchemaTokenKind::ObjectStart => {
                self.state = OwnedSchemaRecordState::Key;
                OwnedSchemaRecordStep::Pending
            }
            OwnedSchemaRecordState::Root => self.fault("schema-json.expected-object", token, None),
            OwnedSchemaRecordState::Key if token.kind == OwnedSchemaTokenKind::ObjectEnd => {
                if !self.required_complete() {
                    self.fault("schema-json.missing-required-field", token, None)
                } else {
                    self.state = OwnedSchemaRecordState::Trailing;
                    OwnedSchemaRecordStep::Pending
                }
            }
            OwnedSchemaRecordState::KeyAfterComma if token.kind == OwnedSchemaTokenKind::ObjectEnd => self.fault("schema-json.trailing-comma", token, None),
            OwnedSchemaRecordState::Key | OwnedSchemaRecordState::KeyAfterComma if token.kind == OwnedSchemaTokenKind::String => {
                let Some(index) = self.field_index(token) else {
                    return self.fault("schema-json.unknown-field", token, None);
                };
                if self.seen & (1 << index) != 0 {
                    return self.fault("schema-json.duplicate-field", token, Some(index as u8));
                }
                self.seen |= 1 << index;
                self.state = OwnedSchemaRecordState::Colon { field_index: index as u8 };
                OwnedSchemaRecordStep::Pending
            }
            OwnedSchemaRecordState::Key | OwnedSchemaRecordState::KeyAfterComma => self.fault("schema-json.expected-field", token, None),
            OwnedSchemaRecordState::Colon { field_index } if token.kind == OwnedSchemaTokenKind::Colon => {
                self.state = OwnedSchemaRecordState::Value { field_index };
                OwnedSchemaRecordStep::Pending
            }
            OwnedSchemaRecordState::Colon { field_index } => self.fault("schema-json.expected-colon", token, Some(field_index)),
            OwnedSchemaRecordState::Value { field_index } => match token.kind {
                OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart => {
                    self.delimiters[0] = token.kind;
                    self.state = OwnedSchemaRecordState::Nested { field_index, depth: 1 };
                    OwnedSchemaRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false }
                }
                OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd | OwnedSchemaTokenKind::Colon | OwnedSchemaTokenKind::Comma => self.fault("schema-json.expected-value", token, Some(field_index)),
                _ => {
                    self.state = OwnedSchemaRecordState::Separator;
                    OwnedSchemaRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: true }
                }
            },
            OwnedSchemaRecordState::Nested { field_index, depth } => match token.kind {
                OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart => {
                    let Some(depth) = depth.checked_add(1) else {
                        return self.fault("schema-json.maximum-depth", token, Some(field_index));
                    };
                    if depth as usize > self.delimiters.len() {
                        return self.fault("schema-json.maximum-depth", token, Some(field_index));
                    }
                    self.delimiters[depth as usize - 1] = token.kind;
                    self.state = OwnedSchemaRecordState::Nested { field_index, depth };
                    OwnedSchemaRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false }
                }
                OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd => {
                    let opening = self.delimiters[depth as usize - 1];
                    let matching = matches!((opening, token.kind), (OwnedSchemaTokenKind::ObjectStart, OwnedSchemaTokenKind::ObjectEnd) | (OwnedSchemaTokenKind::ArrayStart, OwnedSchemaTokenKind::ArrayEnd));
                    if !matching {
                        self.fault("schema-json.mismatched-delimiter", token, Some(field_index))
                    } else if depth == 1 {
                        self.state = OwnedSchemaRecordState::Separator;
                        OwnedSchemaRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: true }
                    } else {
                        self.state = OwnedSchemaRecordState::Nested { field_index, depth: depth - 1 };
                        OwnedSchemaRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false }
                    }
                }
                _ => OwnedSchemaRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false },
            },
            OwnedSchemaRecordState::Separator if token.kind == OwnedSchemaTokenKind::Comma => {
                self.state = OwnedSchemaRecordState::KeyAfterComma;
                OwnedSchemaRecordStep::Pending
            }
            OwnedSchemaRecordState::Separator if token.kind == OwnedSchemaTokenKind::ObjectEnd => {
                if !self.required_complete() {
                    self.fault("schema-json.missing-required-field", token, None)
                } else {
                    self.state = OwnedSchemaRecordState::Trailing;
                    OwnedSchemaRecordStep::Pending
                }
            }
            OwnedSchemaRecordState::Separator => self.fault("schema-json.expected-separator", token, None),
            OwnedSchemaRecordState::Trailing => self.fault("schema-json.trailing-token", token, None),
            OwnedSchemaRecordState::Complete => OwnedSchemaRecordStep::Complete,
            OwnedSchemaRecordState::Cancelled => OwnedSchemaRecordStep::Cancelled,
            OwnedSchemaRecordState::Fault(diagnostic) => OwnedSchemaRecordStep::Fault(diagnostic),
        }
    }

    pub fn copy_token_bytes(&self, token: OwnedSchemaToken, relative_offset: usize, output: &mut [u8]) -> usize {
        self.tokens.copy_token_bytes(token, relative_offset, output)
    }

    pub fn next_close_copy_byte_demand(&self) -> usize {
        self.tokens.next_close_copy_byte_demand()
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.tokens.close_step(grant)
    }

    pub fn next_close_capacity_byte_demand(&self, body:usize)->Result<usize,ValueError>{self.tokens.next_close_capacity_byte_demand(body)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.tokens.next_close_release_byte_demand()}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.tokens.next_close_depth_demand()}
    pub fn terminal_is_empty(&self) -> bool {
        self.tokens.terminal_is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaNestedRecordStep {
    Pending,
    FieldToken { field_id: u16, token: OwnedSchemaToken, terminal: bool },
    Complete,
    Fault(OwnedSchemaDecodeDiagnostic),
}

/// 🪆 Schema-first nested-object matcher fed by an outer retained record cursor. It
/// owns no bytes and advances exactly one already-budgeted token at a time.
pub struct OwnedSchemaNestedRecordCursor {
    spec: OwnedSchemaRecordSpec,
    state: OwnedSchemaRecordState,
    seen: u64,
    delimiters: [OwnedSchemaTokenKind; 256],
}

impl OwnedSchemaNestedRecordCursor {
    pub fn try_new(spec: OwnedSchemaRecordSpec) -> Result<Self, &'static str> {
        Ok(Self { spec: spec.validate()?, state: OwnedSchemaRecordState::Root, seen: 0, delimiters: [OwnedSchemaTokenKind::ObjectStart; 256] })
    }

    fn fault(&mut self, code: &'static str, token: OwnedSchemaToken, field_index: Option<u8>) -> OwnedSchemaNestedRecordStep {
        let path = field_index.and_then(|index| self.spec.fields.get(index as usize)).and_then(|field| OwnedSchemaPath::field(field.key)).unwrap_or(OwnedSchemaPath::ROOT);
        let diagnostic = OwnedSchemaDecodeDiagnostic { code, offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
        self.state = OwnedSchemaRecordState::Fault(diagnostic);
        OwnedSchemaNestedRecordStep::Fault(diagnostic)
    }

    fn field_index(&self, source: &OwnedSchemaRecordCursor, token: OwnedSchemaToken) -> Option<usize> {
        self.spec.fields.iter().position(|field| source.tokens.string_token_equals(token, field.key))
    }

    fn required_complete(&self) -> bool {
        self.spec.fields.iter().enumerate().all(|(index, field)| !field.required || self.seen & (1 << index) != 0)
    }

    pub fn accept(&mut self, token: OwnedSchemaToken, source: &OwnedSchemaRecordCursor) -> OwnedSchemaNestedRecordStep {
        match self.state {
            OwnedSchemaRecordState::Root if token.kind == OwnedSchemaTokenKind::ObjectStart => {
                self.state = OwnedSchemaRecordState::Key;
                OwnedSchemaNestedRecordStep::Pending
            }
            OwnedSchemaRecordState::Root => self.fault("schema-json.expected-object", token, None),
            OwnedSchemaRecordState::Key if token.kind == OwnedSchemaTokenKind::ObjectEnd => {
                if self.required_complete() {
                    self.state = OwnedSchemaRecordState::Complete;
                    OwnedSchemaNestedRecordStep::Complete
                } else {
                    self.fault("schema-json.missing-required-field", token, None)
                }
            }
            OwnedSchemaRecordState::KeyAfterComma if token.kind == OwnedSchemaTokenKind::ObjectEnd => self.fault("schema-json.trailing-comma", token, None),
            OwnedSchemaRecordState::Key | OwnedSchemaRecordState::KeyAfterComma if token.kind == OwnedSchemaTokenKind::String => {
                let Some(index) = self.field_index(source, token) else { return self.fault("schema-json.unknown-field", token, None) };
                if self.seen & (1 << index) != 0 {
                    return self.fault("schema-json.duplicate-field", token, Some(index as u8));
                }
                self.seen |= 1 << index;
                self.state = OwnedSchemaRecordState::Colon { field_index: index as u8 };
                OwnedSchemaNestedRecordStep::Pending
            }
            OwnedSchemaRecordState::Key | OwnedSchemaRecordState::KeyAfterComma => self.fault("schema-json.expected-field", token, None),
            OwnedSchemaRecordState::Colon { field_index } if token.kind == OwnedSchemaTokenKind::Colon => {
                self.state = OwnedSchemaRecordState::Value { field_index };
                OwnedSchemaNestedRecordStep::Pending
            }
            OwnedSchemaRecordState::Colon { field_index } => self.fault("schema-json.expected-colon", token, Some(field_index)),
            OwnedSchemaRecordState::Value { field_index } => match token.kind {
                OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart => {
                    self.delimiters[0] = token.kind;
                    self.state = OwnedSchemaRecordState::Nested { field_index, depth: 1 };
                    OwnedSchemaNestedRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false }
                }
                OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd | OwnedSchemaTokenKind::Colon | OwnedSchemaTokenKind::Comma => self.fault("schema-json.expected-value", token, Some(field_index)),
                _ => {
                    self.state = OwnedSchemaRecordState::Separator;
                    OwnedSchemaNestedRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: true }
                }
            },
            OwnedSchemaRecordState::Nested { field_index, depth } => match token.kind {
                OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart => {
                    let Some(depth) = depth.checked_add(1) else { return self.fault("schema-json.maximum-depth", token, Some(field_index)) };
                    if depth as usize > self.delimiters.len() {
                        return self.fault("schema-json.maximum-depth", token, Some(field_index));
                    }
                    self.delimiters[depth as usize - 1] = token.kind;
                    self.state = OwnedSchemaRecordState::Nested { field_index, depth };
                    OwnedSchemaNestedRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false }
                }
                OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd => {
                    let opening = self.delimiters[depth as usize - 1];
                    if !matches!((opening, token.kind), (OwnedSchemaTokenKind::ObjectStart, OwnedSchemaTokenKind::ObjectEnd) | (OwnedSchemaTokenKind::ArrayStart, OwnedSchemaTokenKind::ArrayEnd)) {
                        return self.fault("schema-json.mismatched-delimiter", token, Some(field_index));
                    }
                    if depth == 1 {
                        self.state = OwnedSchemaRecordState::Separator;
                        OwnedSchemaNestedRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: true }
                    } else {
                        self.state = OwnedSchemaRecordState::Nested { field_index, depth: depth - 1 };
                        OwnedSchemaNestedRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false }
                    }
                }
                _ => OwnedSchemaNestedRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false },
            },
            OwnedSchemaRecordState::Separator if token.kind == OwnedSchemaTokenKind::Comma => {
                self.state = OwnedSchemaRecordState::KeyAfterComma;
                OwnedSchemaNestedRecordStep::Pending
            }
            OwnedSchemaRecordState::Separator if token.kind == OwnedSchemaTokenKind::ObjectEnd => {
                if self.required_complete() {
                    self.state = OwnedSchemaRecordState::Complete;
                    OwnedSchemaNestedRecordStep::Complete
                } else {
                    self.fault("schema-json.missing-required-field", token, None)
                }
            }
            OwnedSchemaRecordState::Separator => self.fault("schema-json.expected-separator", token, None),
            OwnedSchemaRecordState::Complete => self.fault("schema-json.trailing-token", token, None),
            OwnedSchemaRecordState::Fault(diagnostic) => OwnedSchemaNestedRecordStep::Fault(diagnostic),
            OwnedSchemaRecordState::Trailing | OwnedSchemaRecordState::Cancelled => self.fault("schema-json.invalid-nested-state", token, None),
        }
    }

    pub fn terminal_is_complete(&self) -> bool {
        matches!(self.state, OwnedSchemaRecordState::Complete)
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaStringStep {
    Pending,
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

#[derive(Clone, Copy)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaStringState {
    Raw,
    Escape,
    Unicode { value: u16, digits: u8 },
    SurrogateSlash { high: u16 },
    SurrogateU { high: u16 },
    SurrogateLow { high: u16, value: u16, digits: u8 },
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

pub struct OwnedSchemaStringAuthority<const MAXIMUM_BYTES: usize> {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    token: OwnedSchemaToken,
    path: OwnedSchemaPath,
    relative: usize,
    state: OwnedSchemaStringState,
    bytes: [u8; MAXIMUM_BYTES],
    len: usize,
    taken: bool,
}

impl<const MAXIMUM_BYTES: usize> OwnedSchemaStringAuthority<MAXIMUM_BYTES> {
    /// 🔠️ Binds one string token to fixed caller-chosen semantic storage.
    pub fn try_new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, token: OwnedSchemaToken, path: OwnedSchemaPath) -> Result<Self, OwnedSchemaToken> {
        if token.kind != OwnedSchemaTokenKind::String || token.end.saturating_sub(token.start) < 2 || MAXIMUM_BYTES == 0 {
            return Err(token);
        }
        Ok(Self { operation, generation, token, path, relative: 1, state: OwnedSchemaStringState::Raw, bytes: [0; MAXIMUM_BYTES], len: 0, taken: false })
    }

    fn diagnostic(&self, code: &'static str) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: self.token.start + self.relative as u64, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    fn raw_byte(&self, source: &OwnedSchemaRecordCursor) -> Option<u8> {
        let mut byte = [0];
        (source.copy_token_bytes(self.token, self.relative, &mut byte) == 1).then_some(byte[0])
    }

    fn push_byte(&mut self, byte: u8) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if self.len == MAXIMUM_BYTES {
            return Err(self.diagnostic("schema-json.string-byte-capacity"));
        }
        self.bytes[self.len] = byte;
        self.len += 1;
        Ok(())
    }

    fn push_scalar(&mut self, scalar: u32) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        let Some(value) = char::from_u32(scalar) else {
            return Err(self.diagnostic("schema-json.invalid-unicode-scalar"));
        };
        let mut encoded = [0; 4];
        for byte in value.encode_utf8(&mut encoded).bytes() {
            self.push_byte(byte)?;
        }
        Ok(())
    }

    fn hex(byte: u8) -> Option<u16> {
        match byte {
            b'0'..=b'9' => Some(u16::from(byte - b'0')),
            b'a'..=b'f' => Some(u16::from(byte - b'a' + 10)),
            b'A'..=b'F' => Some(u16::from(byte - b'A' + 10)),
            _ => None,
        }
    }

    fn finish_unicode(&mut self, value: u16) -> Result<OwnedSchemaStringState, OwnedSchemaDecodeDiagnostic> {
        match value {
            0xd800..=0xdbff => Ok(OwnedSchemaStringState::SurrogateSlash { high: value }),
            0xdc00..=0xdfff => Err(self.diagnostic("schema-json.unpaired-low-surrogate")),
            _ => {
                self.push_scalar(u32::from(value))?;
                Ok(OwnedSchemaStringState::Raw)
            }
        }
    }

    /// ⏭️ Unescapes source bytes one semantic byte/scalar at a time under the caller's grant.
    pub fn step(&mut self, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaStringStep {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            let diagnostic = self.diagnostic("schema-json.stale-string-authority");
            self.state = OwnedSchemaStringState::Fault(diagnostic);
            return OwnedSchemaStringStep::Fault(diagnostic);
        }
        if cx.is_cancelled() {
            self.state = OwnedSchemaStringState::Cancelled;
            return OwnedSchemaStringStep::Cancelled;
        }
        cx.set_stage("schema-json-string");
        loop {
            if cx.should_yield() {
                return OwnedSchemaStringStep::Pending;
            }
            match self.state {
                OwnedSchemaStringState::Complete => return OwnedSchemaStringStep::Complete,
                OwnedSchemaStringState::Cancelled => return OwnedSchemaStringStep::Cancelled,
                OwnedSchemaStringState::Fault(diagnostic) => return OwnedSchemaStringStep::Fault(diagnostic),
                _ => {}
            }
            if self.relative + 1 == self.token.end.saturating_sub(self.token.start) as usize {
                if matches!(self.state, OwnedSchemaStringState::Raw) {
                    self.state = OwnedSchemaStringState::Complete;
                    return OwnedSchemaStringStep::Complete;
                }
                let diagnostic = self.diagnostic("schema-json.truncated-string-escape");
                self.state = OwnedSchemaStringState::Fault(diagnostic);
                return OwnedSchemaStringStep::Fault(diagnostic);
            }
            let Some(byte) = self.raw_byte(source) else {
                let diagnostic = self.diagnostic("schema-json.string-source-offset");
                self.state = OwnedSchemaStringState::Fault(diagnostic);
                return OwnedSchemaStringStep::Fault(diagnostic);
            };
            let next = match self.state {
                OwnedSchemaStringState::Raw if byte == b'\\' => Ok(OwnedSchemaStringState::Escape),
                OwnedSchemaStringState::Raw => self.push_byte(byte).map(|_| OwnedSchemaStringState::Raw),
                OwnedSchemaStringState::Escape => match byte {
                    b'"' | b'\\' | b'/' => self.push_byte(byte).map(|_| OwnedSchemaStringState::Raw),
                    b'b' => self.push_byte(0x08).map(|_| OwnedSchemaStringState::Raw),
                    b'f' => self.push_byte(0x0c).map(|_| OwnedSchemaStringState::Raw),
                    b'n' => self.push_byte(b'\n').map(|_| OwnedSchemaStringState::Raw),
                    b'r' => self.push_byte(b'\r').map(|_| OwnedSchemaStringState::Raw),
                    b't' => self.push_byte(b'\t').map(|_| OwnedSchemaStringState::Raw),
                    b'u' => Ok(OwnedSchemaStringState::Unicode { value: 0, digits: 0 }),
                    _ => Err(self.diagnostic("schema-json.invalid-string-escape")),
                },
                OwnedSchemaStringState::Unicode { value, digits } => match Self::hex(byte) {
                    Some(nibble) if digits == 3 => self.finish_unicode((value << 4) | nibble),
                    Some(nibble) => Ok(OwnedSchemaStringState::Unicode { value: (value << 4) | nibble, digits: digits + 1 }),
                    None => Err(self.diagnostic("schema-json.invalid-string-unicode")),
                },
                OwnedSchemaStringState::SurrogateSlash { high } if byte == b'\\' => Ok(OwnedSchemaStringState::SurrogateU { high }),
                OwnedSchemaStringState::SurrogateSlash { .. } => Err(self.diagnostic("schema-json.missing-low-surrogate")),
                OwnedSchemaStringState::SurrogateU { high } if byte == b'u' => Ok(OwnedSchemaStringState::SurrogateLow { high, value: 0, digits: 0 }),
                OwnedSchemaStringState::SurrogateU { .. } => Err(self.diagnostic("schema-json.missing-low-surrogate")),
                OwnedSchemaStringState::SurrogateLow { high, value, digits } => match Self::hex(byte) {
                    Some(nibble) if digits == 3 => {
                        let low = (value << 4) | nibble;
                        if !(0xdc00..=0xdfff).contains(&low) {
                            Err(self.diagnostic("schema-json.invalid-low-surrogate"))
                        } else {
                            let scalar = 0x1_0000 + ((u32::from(high) - 0xd800) << 10) + (u32::from(low) - 0xdc00);
                            self.push_scalar(scalar).map(|_| OwnedSchemaStringState::Raw)
                        }
                    }
                    Some(nibble) => Ok(OwnedSchemaStringState::SurrogateLow { high, value: (value << 4) | nibble, digits: digits + 1 }),
                    None => Err(self.diagnostic("schema-json.invalid-low-surrogate")),
                },
                OwnedSchemaStringState::Complete | OwnedSchemaStringState::Cancelled | OwnedSchemaStringState::Fault(_) => unreachable!("terminal string state returned before source read"),
            };
            self.relative += 1;
            cx.consume_fuel(1);
            match next {
                Ok(state) => self.state = state,
                Err(diagnostic) => {
                    self.state = OwnedSchemaStringState::Fault(diagnostic);
                    return OwnedSchemaStringStep::Fault(diagnostic);
                }
            }
            if cx.is_cancelled() {
                self.state = OwnedSchemaStringState::Cancelled;
                return OwnedSchemaStringStep::Cancelled;
            }
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        matches!(self.state, OwnedSchemaStringState::Complete).then(|| unsafe { std::str::from_utf8_unchecked(&self.bytes[..self.len]) })
    }

    /// 📤️ Publishes the bounded semantic string exactly once after terminal validation.
    pub fn take_string(&mut self) -> Option<String> {
        if self.taken || !matches!(self.state, OwnedSchemaStringState::Complete) {
            return None;
        }
        self.taken = true;
        Some(unsafe { String::from_utf8_unchecked(self.bytes[..self.len].to_vec()) })
    }

    /// 🛑️ Releases the definitionally inline partial scalar without allocating.
    pub fn cancel(&mut self) {
        self.state = OwnedSchemaStringState::Cancelled;
        self.taken = true;
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.taken || matches!(self.state, OwnedSchemaStringState::Cancelled | OwnedSchemaStringState::Fault(_))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaHexStep {
    Pending,
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

/// 🔢 Incremental bounded hex scalar used by owner catalogs for nested pack values.
/// The decoded bytes stay inline and no String/Vec is ever constructed.
pub struct OwnedSchemaHexAuthority<const MAXIMUM_BYTES: usize> {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    token: OwnedSchemaToken,
    path: OwnedSchemaPath,
    relative: usize,
    bytes: Box<[u8; MAXIMUM_BYTES]>,
    len: usize,
    high: Option<u8>,
    complete: bool,
    taken: bool,
}

impl<const MAXIMUM_BYTES: usize> OwnedSchemaHexAuthority<MAXIMUM_BYTES> {
    pub fn try_new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, token: OwnedSchemaToken, path: OwnedSchemaPath) -> Result<Self, OwnedSchemaDecodeDiagnostic> {
        if token.kind != OwnedSchemaTokenKind::String {
            return Err(OwnedSchemaDecodeDiagnostic { code: "schema-json.expected-hex-string", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        Ok(Self { operation, generation, token, path, relative: 1, bytes: Box::new([0; MAXIMUM_BYTES]), len: 0, high: None, complete: false, taken: false })
    }

    fn fault(&self, code: &'static str) -> OwnedSchemaHexStep {
        OwnedSchemaHexStep::Fault(OwnedSchemaDecodeDiagnostic { code, offset: self.token.start + self.relative as u64, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })
    }

    fn nibble(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }

    pub fn step(&mut self, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaHexStep {
        if self.complete {
            return OwnedSchemaHexStep::Complete;
        }
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return self.fault("schema-json.stale-hex-authority");
        }
        if cx.is_cancelled() {
            self.taken = true;
            return OwnedSchemaHexStep::Cancelled;
        }
        cx.set_stage("schema-json-hex");
        while cx.fuel_remaining() != 0 {
            if self.token.start + self.relative as u64 + 1 >= self.token.end {
                if self.high.is_some() {
                    return self.fault("schema-json.odd-hex-length");
                }
                self.complete = true;
                return OwnedSchemaHexStep::Complete;
            }
            let mut byte = [0u8; 1];
            if source.copy_token_bytes(self.token, self.relative, &mut byte) != 1 {
                return self.fault("schema-json.truncated-hex");
            }
            let Some(nibble) = Self::nibble(byte[0]) else { return self.fault("schema-json.invalid-hex") };
            self.relative += 1;
            cx.consume_fuel(1);
            if let Some(high) = self.high.take() {
                if self.len == MAXIMUM_BYTES {
                    return self.fault("schema-json.hex-byte-capacity");
                }
                self.bytes[self.len] = (high << 4) | nibble;
                self.len += 1;
            } else {
                self.high = Some(nibble);
            }
        }
        OwnedSchemaHexStep::Pending
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        (self.complete && !self.taken).then_some(&self.bytes[..self.len])
    }

    pub fn release(&mut self) -> bool {
        if !self.complete || self.taken {
            return false;
        }
        self.taken = true;
        true
    }

    pub fn cancel(&mut self) {
        self.taken = true;
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.taken
    }
}


#[cfg(any(test, feature="testkit"))]
#[path="🧪️testing/🦀️.rs"]
pub mod testing;
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path="../../../../../🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️owned-schema-record/🦀️.rs"]
mod owned_schema_record_tests;

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;
