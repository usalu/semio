//! 🧬️ Owned schema decoding: paged, credit-bounded decoding of artifact envelopes and records into retained owners,
//! with field decoder registries, reservations and completed-record tickets.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use super::ArtifactStoreOperationRowsRetirement;
use super::ArtifactStoreDecodedEditRetirementFactory;
use super::ArtifactStoreChangeRetirementFactory;
use super::genesis_pack_admission::ArtifactGenesisPackCapture;
use crate::os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use crate::os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ValueError;
use crate::os_spr::{Edit, Mutation};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use super::snapshot_read::{artifact_retirement_admit_owned, artifact_retirement_box_close_step, artifact_retirement_box_demands, artifact_retirement_owned_birth_demands, artifact_retirement_owner_close, artifact_retirement_owner_demands};
use super::snapshot_read::{ArtifactStoreAlternativeRetirementFactory, ArtifactStoreCheckpointRetirementFactory, ArtifactStoreEnvelopeRetirement, ArtifactStoreHistoryLedgerRetirement, ArtifactStoreOperationRowOwners, ArtifactStoreVcsRetirement, artifact_history_ledger_birth, artifact_history_ledger_handoff};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use crate::os_vcs::{
    Alternative, ArtifactHistoryLedger, ArtifactHistoryReservation, ArtifactVcs, Change, Checkpoint, Identified,
};
use super::store_schema::{ArtifactEnvelope, ArtifactEnvelopeOwners};
#[cfg(target_arch = "wasm32")]
use super::serialization::{ArtifactCodecApplyFuture};
use crate::os_pack::pack_rt;
use super::ledger::ArtifactEditMessageLedger;
#[cfg(target_arch = "wasm32")]
use super::backbone::resolve_backbone;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use super::backbone::local_storage_backbone_key;

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

    pub(crate) fn byte_at(&self, offset: usize) -> Option<u8> {
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

pub type OwnedSchemaDecodeDiagnostic=super::schema_diagnostic::SchemaDecodeDiagnostic<OwnedSchemaPath>;

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
pub(crate) enum OwnedSchemaLexState {
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
pub(crate) enum OwnedSchemaRecordState {
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

pub const ARTIFACT_ENVELOPE_SCHEMA_FIELD: u16 = 1;

pub const ARTIFACT_ENVELOPE_ID_FIELD: u16 = 2;

pub const ARTIFACT_ENVELOPE_VCS_FIELD: u16 = 3;

pub const ARTIFACT_ENVELOPE_BACKBONE_FIELD: u16 = 4;

pub const ARTIFACT_ENVELOPE_ACTIVE_ALTERNATIVE_FIELD: u16 = 5;

pub const ARTIFACT_ENVELOPE_CURSOR_FIELD: u16 = 6;

pub const ARTIFACT_ENVELOPE_DIALECT_FIELD: u16 = 7;

pub const ARTIFACT_ENVELOPE_MIGRATED_FROM_FIELD: u16 = 8;

pub const ARTIFACT_ENVELOPE_OWNER_FIELD: u16 = 9;

pub const ARTIFACT_ENVELOPE_LANES_FIELD: u16 = 10;

pub const ARTIFACT_ENVELOPE_EDIT_MESSAGES_FIELD: u16 = 11;

pub const ARTIFACT_ENVELOPE_CONFLICTS_FIELD: u16 = 12;

pub(crate) const ARTIFACT_ENVELOPE_OWNED_SCHEMA_FIELDS: &[OwnedSchemaFieldSpec] = &[
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_SCHEMA_FIELD, key: "schema", required: true },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_ID_FIELD, key: "id", required: true },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_VCS_FIELD, key: "vcs", required: true },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_BACKBONE_FIELD, key: "backbone", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_ACTIVE_ALTERNATIVE_FIELD, key: "activeAlternativeId", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_CURSOR_FIELD, key: "cursor", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_DIALECT_FIELD, key: "dialect", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_MIGRATED_FROM_FIELD, key: "migratedFrom", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_OWNER_FIELD, key: "owner", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_LANES_FIELD, key: "lanes", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_EDIT_MESSAGES_FIELD, key: "editMessages", required: true },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_CONFLICTS_FIELD, key: "conflicts", required: true },
];

/// 🧬️ Returns the sole field-ID/key contract for owned envelope decode.
pub fn artifact_envelope_owned_schema() -> OwnedSchemaRecordSpec {
    OwnedSchemaRecordSpec { fields: ARTIFACT_ENVELOPE_OWNED_SCHEMA_FIELDS }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactEnvelopeFieldDecodeStep {
    /// The exact token remains owned by the field authority and must be offered again.
    Pending,
    /// The exact token was consumed, but the enclosing field still owns later tokens.
    TokenComplete,
    /// The exact token and its enclosing field were both completed.
    FieldComplete,
    RecordComplete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeFieldReservation {
    field_id: u16,
    generation: u64,
}

impl ArtifactEnvelopeFieldReservation {
    pub fn new(field_id: u16, generation: u64) -> Option<Self> {
        (field_id != 0 && generation != 0).then_some(Self { field_id, generation })
    }

    pub fn field_id(self) -> u16 {
        self.field_id
    }

    pub fn generation(self) -> u64 {
        self.generation
    }
}

/// 🎯️ Exact pre-admission target for one decoded envelope field.
pub trait ArtifactEnvelopeDecodedRecordTarget<P, Mutation>: Send {
    fn reserve_field(&mut self, field_id: u16) -> Result<ArtifactEnvelopeFieldReservation, OwnedSchemaDecodeDiagnostic>;
    fn publish_vcs_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>);
    fn publish_edit_messages_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: ArtifactEditMessageLedger);
    fn publish_conflicts_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: Box<dyn ArtifactEnvelopeSprConflictAuthority>);
    fn cancel_field_reservation(&mut self, reservation: ArtifactEnvelopeFieldReservation) -> Result<(), OwnedSchemaDecodeDiagnostic>;
}

/// 🧬️ Terminalizable owned decoder for the nested VCS record.
pub trait ArtifactEnvelopeVcsFieldAuthority<P, Mutation>: Send {
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn publish_reserved(
        &mut self,
        target: &mut dyn ArtifactEnvelopeDecodedRecordTarget<P, Mutation>,
        reservation: ArtifactEnvelopeFieldReservation,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn maximum_close_byte_demand(&self) -> usize;
    fn maximum_retained_close_bytes(&self) -> usize;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🪨️ Terminalizable owner-supplied decoder for a domain snapshot value.
pub trait ArtifactEnvelopeSnapshotFieldAuthority<P>: Send {
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn publish_reserved(
        &mut self,
        target: &mut dyn ArtifactEnvelopeSnapshotFieldTarget<P>,
        reservation: ArtifactEnvelopeFieldReservation,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn maximum_close_byte_demand(&self) -> usize;
    fn maximum_retained_close_bytes(&self) -> usize;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🪨️ Exact pre-admission target for a nested domain snapshot.
pub trait ArtifactEnvelopeSnapshotFieldTarget<P>: Send {
    fn reserve_snapshot(&mut self) -> Result<ArtifactEnvelopeFieldReservation, OwnedSchemaDecodeDiagnostic>;
    fn publish_snapshot_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: P);
    fn cancel_snapshot_reservation(&mut self, reservation: ArtifactEnvelopeFieldReservation) -> Result<(), OwnedSchemaDecodeDiagnostic>;
}

/// 🧬️ Terminalizable owner-supplied decoder for one domain mutation.
pub trait ArtifactEnvelopeMutationFieldAuthority<Mutation>: Send {
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn publish_reserved(
        &mut self,
        target: &mut dyn ArtifactEnvelopeMutationFieldTarget<Mutation>,
        reservation: ArtifactEnvelopeFieldReservation,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🧬️ Exact pre-admission target for one nested domain mutation.
pub trait ArtifactEnvelopeMutationFieldTarget<Mutation>: Send {
    fn reserve_mutation(&mut self) -> Result<ArtifactEnvelopeFieldReservation, OwnedSchemaDecodeDiagnostic>;
    fn publish_mutation_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: Mutation);
    fn cancel_mutation_reservation(&mut self, reservation: ArtifactEnvelopeFieldReservation) -> Result<(), OwnedSchemaDecodeDiagnostic>;
}

/// ⚔️ Terminalizable repository-owned SPR conflict record authority.
pub trait ArtifactEnvelopeSprConflictAuthority: Send {
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🏭️ Mandatory domain catalog for every nested envelope decoder; there is no default implementation.
pub trait ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>: semio_framework_value::FactoryRetirement + Send + Sync {
    fn begin_vcs(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
    ) -> Result<Box<dyn ArtifactEnvelopeVcsFieldAuthority<P, Mutation>>, Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>>>;
    fn maximum_vcs_close_byte_demand(&self) -> usize;
    fn maximum_retained_vcs_close_bytes(&self) -> usize;
    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>>;
    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeMutationFieldAuthority<Mutation>>;
    fn begin_spr_conflict(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSprConflictAuthority>;
    fn edit_history_decoder(&self) -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>>>;
}

/// 🧬️ Owner-supplied decoder for one pre-admitted history entry. The framework
/// never invokes `DeserializeOwned` for a domain edit behind the catalog's back.
pub trait ArtifactOwnedHistoryEntryDecoder<T>: semio_framework_value::FactoryRetirement + Send + Sync {
    fn begin_entry(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,
    ) -> Box<dyn ArtifactOwnedHistoryEntryAuthority<T>>;
}

/// 🧬 Terminalizable schema cursor for one pre-reserved history entry.
pub trait ArtifactOwnedHistoryEntryAuthority<T>: Send {
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn take_value(&mut self) -> Option<T>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(crate) struct ArtifactRepositoryHistoryEntryDecoder<T>(PhantomData<fn() -> T>);

impl<T> ArtifactRepositoryHistoryEntryDecoder<T> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

/// 🧱 Retained source-bounded repository decoder used only by cohorts whose domain cursor
/// remains explicitly fail-closed pending a schema-owned implementation.
pub fn artifact_bounded_history_entry_decoder<T>() -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<T>>
where
    T: FromValue + Send + Sync + 'static,
{
    Arc::new(ArtifactRepositoryHistoryEntryDecoder::new())
}

pub(crate) const ARTIFACT_OWNED_SPR_EDIT_FIELDS: &[OwnedSchemaFieldSpec] = &[
    OwnedSchemaFieldSpec { id: 1, key: "id", required: true },
    OwnedSchemaFieldSpec { id: 2, key: "actor", required: false },
    OwnedSchemaFieldSpec { id: 3, key: "forwards", required: true },
    OwnedSchemaFieldSpec { id: 4, key: "inverse", required: true },
    OwnedSchemaFieldSpec { id: 5, key: "mutationMeta", required: false },
    OwnedSchemaFieldSpec { id: 8, key: "sequenceNumber", required: true },
    OwnedSchemaFieldSpec { id: 9, key: "startedAt", required: true },
    OwnedSchemaFieldSpec { id: 10, key: "finishedAt", required: false },
    OwnedSchemaFieldSpec { id: 11, key: "verb", required: false },
    OwnedSchemaFieldSpec { id: 12, key: "line", required: true },
];

pub(crate) struct ArtifactOwnedSprMutationTarget<Mutation>
where
    Mutation: Send,
{
    next_generation: u64,
    reservation: Option<ArtifactEnvelopeFieldReservation>,
    value: std::mem::ManuallyDrop<Option<Mutation>>,
}

impl<Mutation: Send> ArtifactOwnedSprMutationTarget<Mutation> {
    fn new() -> Self {
        Self { next_generation: 0, reservation: None, value: std::mem::ManuallyDrop::new(None) }
    }
}

impl<Mutation: Send> ArtifactEnvelopeMutationFieldTarget<Mutation> for ArtifactOwnedSprMutationTarget<Mutation> {
    fn reserve_mutation(&mut self) -> Result<ArtifactEnvelopeFieldReservation, OwnedSchemaDecodeDiagnostic> {
        if self.reservation.is_some() || self.value.is_some() {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-target-occupied", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.next_generation = self.next_generation.checked_add(1).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-target-generation", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        let reservation = ArtifactEnvelopeFieldReservation::new(1, self.next_generation).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-target-reservation", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        self.reservation = Some(reservation);
        Ok(reservation)
    }

    fn publish_mutation_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: Mutation) {
        assert_eq!(self.reservation, Some(reservation), "SPR mutation publication requires its exact reservation");
        assert!(self.value.is_none(), "SPR mutation publication cannot replace an owner");
        self.reservation = None;
        *self.value = Some(value);
    }

    fn cancel_mutation_reservation(&mut self, reservation: ArtifactEnvelopeFieldReservation) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if self.reservation != Some(reservation) {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-target-stale", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.reservation = None;
        Ok(())
    }
}

impl<Mutation: Send> Drop for ArtifactOwnedSprMutationTarget<Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.reservation.is_none() && self.value.is_none()), "SPR mutation target reached Drop with a live reservation or value owner");
    }
}

pub(crate) enum ArtifactOwnedSprMutationArrayState {
    AwaitStart,
    Entries,
    Publishing,
    Complete,
    Closing,
}

pub(crate) struct ArtifactOwnedSprMutationArrayAuthority<P, Mutation: Send> {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: OwnedSchemaPath,
    catalog: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>>>,
    mutation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    factories_close: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
    rows_retirement: std::mem::ManuallyDrop<Option<ArtifactStoreOperationRowsRetirement<Mutation>>>,
    target: ArtifactOwnedSprMutationTarget<Mutation>,
    reservation: Option<ArtifactEnvelopeFieldReservation>,
    active: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEnvelopeMutationFieldAuthority<Mutation>>>>,
    values: std::mem::ManuallyDrop<Option<ArtifactStoreOperationRowOwners<Mutation>>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    depth: usize,
    scalar_entry: bool,
    expect_entry: bool,
    state: ArtifactOwnedSprMutationArrayState,
    taken: bool,
}

impl<P, Mutation: Send + 'static> ArtifactOwnedSprMutationArrayAuthority<P, Mutation> {
    fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::RetirementDemand;
        let diagnostic = self.diagnostic("artifact-spr.mutation-array-close-demand", 0);
        if let Some(owner) = self.active.as_ref() {
            if owner.terminal_is_empty() { return Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }); }
            return Ok(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(copy)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()?.checked_add(1).ok_or(diagnostic)? });
        }
        if self.reservation.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(value) = self.target.value.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.mutation_factory.as_ref().expect("mutation target retains its factory").retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
        if let Some(owner) = self.retirement.as_ref() { return artifact_retirement_box_demands(owner, copy).map_err(|_| diagnostic); }
        if self.values.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<ArtifactStoreOperationRowOwners<Mutation>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.rows_retirement.as_ref() {
            let mut demand = owner.demands(copy).map_err(|_| diagnostic)?;
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        if self.catalog.is_some() || self.mutation_factory.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.factories_close.iter().flatten().next() {
            let mut demand = owner.demands(copy).map_err(|_| diagnostic)?;
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
    fn try_new(
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
        mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
        inverse: bool,
    ) -> Result<Self, OwnedSchemaDecodeDiagnostic> {
        let values = if inverse { ArtifactStoreOperationRowOwners::Inverse(Default::default()) } else {
            let mut values = Vec::new();
            values.try_reserve_exact(ARTIFACT_ENVELOPE_HISTORY_ITEMS).map_err(|_| OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-array-admission", offset: 0, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
            ArtifactStoreOperationRowOwners::Forwards(values)
        };
        Ok(Self {
            operation,
            generation,
            path,
            catalog: std::mem::ManuallyDrop::new(Some(catalog)),
            mutation_factory: std::mem::ManuallyDrop::new(Some(mutation_factory)),
            factories_close: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)),
            rows_retirement: std::mem::ManuallyDrop::new(None),
            target: ArtifactOwnedSprMutationTarget::new(),
            reservation: None,
            active: std::mem::ManuallyDrop::new(None),
            values: std::mem::ManuallyDrop::new(Some(values)),
            retirement: std::mem::ManuallyDrop::new(None),
            depth: 0,
            scalar_entry: false,
            expect_entry: true,
            state: ArtifactOwnedSprMutationArrayState::AwaitStart,
            taken: false,
        })
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    fn accept(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("artifact-spr.mutation-array-stale", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("artifact-spr.mutation-array-cancelled", token.start));
        }
        if matches!(self.state, ArtifactOwnedSprMutationArrayState::Publishing) {
            if let Some(ArtifactStoreOperationRowOwners::Inverse(values)) = self.values.as_mut() {
                if !values.has_reserved_slot() {
                    let diagnostic = OwnedSchemaDecodeDiagnostic { code: "artifact-spr.inverse-page-admission", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
                    let demand = values.next_allocation_bytes().map_err(|_| diagnostic)?;
                    if demand > ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES { return Err(diagnostic); }
                    let progress = values.reserve_one(demand).map_err(|_| diagnostic)?;
                    cx.consume_fuel(progress.allocated_bytes.max(1) as u64);
                    return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);
                }
            }
            let reservation = self.reservation.ok_or_else(|| self.diagnostic("artifact-spr.mutation-array-reservation-missing", token.start))?;
            let path = self.path;
            let active = self.active.as_mut().ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-array-owner-missing", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
            return match active.publish_reserved(&mut self.target, reservation, cx)? {
                ArtifactEnvelopeFieldDecodeStep::Pending => Ok(ArtifactEnvelopeFieldDecodeStep::Pending),
                ArtifactEnvelopeFieldDecodeStep::FieldComplete | ArtifactEnvelopeFieldDecodeStep::TokenComplete => {
                    self.reservation = None;
                    let value = self.target.value.take().ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-array-value-missing", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                    let values = self.values.as_mut().ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-array-values-missing", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                    match values {
                        ArtifactStoreOperationRowOwners::Forwards(values) => {
                            if values.len() == values.capacity() {
                                *self.target.value = Some(value);
                                return Err(self.diagnostic("artifact-spr.mutation-array-capacity", token.start));
                            }
                            values.push(value);
                        }
                        ArtifactStoreOperationRowOwners::Inverse(values) => {
                            if let Err(value) = values.push_reserved(value) {
                                *self.target.value = Some(value);
                                return Err(self.diagnostic("artifact-spr.inverse-page-reservation", token.start));
                            }
                        }
                    }
                    if !active.terminal_is_empty() {
                        return Err(self.diagnostic("artifact-spr.mutation-array-live-after-publish", token.start));
                    }
                    drop(self.active.take());
                    self.scalar_entry = false;
                    self.expect_entry = false;
                    self.state = ArtifactOwnedSprMutationArrayState::Entries;
                    Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                }
                ArtifactEnvelopeFieldDecodeStep::RecordComplete => Err(self.diagnostic("artifact-spr.mutation-array-record-terminal", token.start)),
            };
        }
        if let Some(active) = self.active.as_mut() {
            let entry_terminal = self.scalar_entry || (self.depth == 1 && matches!(token.kind, OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd));
            return match active.accept_token(token, entry_terminal, source, cx)? {
                ArtifactEnvelopeFieldDecodeStep::Pending => Ok(ArtifactEnvelopeFieldDecodeStep::Pending),
                ArtifactEnvelopeFieldDecodeStep::FieldComplete if entry_terminal => {
                    self.depth = 0;
                    self.state = ArtifactOwnedSprMutationArrayState::Publishing;
                    Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                }
                ArtifactEnvelopeFieldDecodeStep::TokenComplete if !entry_terminal => {
                    match token.kind {
                        OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart => self.depth += 1,
                        OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd => self.depth -= 1,
                        _ => {}
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                }
                _ => Err(self.diagnostic("artifact-spr.mutation-array-terminal-discipline", token.start)),
            };
        }
        match self.state {
            ArtifactOwnedSprMutationArrayState::AwaitStart if token.kind == OwnedSchemaTokenKind::ArrayStart && !terminal => {
                self.expect_entry = true;
                self.state = ArtifactOwnedSprMutationArrayState::Entries;
                Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
            }
            ArtifactOwnedSprMutationArrayState::Entries if token.kind == OwnedSchemaTokenKind::ArrayEnd && terminal && self.values.as_ref().is_some_and(|values| match values { ArtifactStoreOperationRowOwners::Forwards(values) => values.is_empty(), ArtifactStoreOperationRowOwners::Inverse(values) => values.is_empty() }) => {
                self.state = ArtifactOwnedSprMutationArrayState::Complete;
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            ArtifactOwnedSprMutationArrayState::Entries if token.kind == OwnedSchemaTokenKind::ArrayEnd && terminal && !self.expect_entry => {
                self.state = ArtifactOwnedSprMutationArrayState::Complete;
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            ArtifactOwnedSprMutationArrayState::Entries if token.kind == OwnedSchemaTokenKind::Comma && !self.expect_entry => {
                self.expect_entry = true;
                Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
            }
            ArtifactOwnedSprMutationArrayState::Entries if self.expect_entry && matches!(token.kind, OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart | OwnedSchemaTokenKind::String) => {
                let reservation = self.target.reserve_mutation()?;
                self.reservation = Some(reservation);
                self.scalar_entry = token.kind == OwnedSchemaTokenKind::String;
                *self.active = Some(self.catalog.as_ref().expect("live original mutation catalog").begin_mutation(self.operation, self.generation, self.path));
                self.accept(token, self.scalar_entry, source, cx)
            }
            ArtifactOwnedSprMutationArrayState::AwaitStart => Err(self.diagnostic("artifact-spr.mutation-array-start", token.start)),
            ArtifactOwnedSprMutationArrayState::Entries => Err(self.diagnostic("artifact-spr.mutation-array-entry", token.start)),
            ArtifactOwnedSprMutationArrayState::Complete => Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete),
            ArtifactOwnedSprMutationArrayState::Publishing => unreachable!("publishing handled before token admission"),
            ArtifactOwnedSprMutationArrayState::Closing => Err(self.diagnostic("artifact-spr.mutation-array-closing", token.start)),
        }
    }

    fn take_values(&mut self) -> Option<ArtifactStoreOperationRowOwners<Mutation>> {
        if !matches!(self.state, ArtifactOwnedSprMutationArrayState::Complete) || self.active.is_some() || self.reservation.is_some() || self.taken {
            return None;
        }
        self.taken = true;
        self.values.take()
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress, admit_retained_clone_close};
        let diagnostic = self.diagnostic("artifact-spr.mutation-array-close-authority", 0);
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(diagnostic); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.state = ArtifactOwnedSprMutationArrayState::Closing;
        if let Some(owner) = self.active.as_mut() {
            if owner.terminal_is_empty() { drop(self.active.take()); self.depth = 0; self.scalar_entry = false; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty })); }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.close_step(child)?;
            admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original SPR mutation decoder").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(reservation) = self.reservation {
            self.target.cancel_mutation_reservation(reservation)?;
            self.reservation = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(value) = self.target.value.take() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            return match self.mutation_factory.as_ref().expect("mutation target retains original factory").retire_owned(value, child) {
                Ok((owner, progress)) => { *self.retirement = Some(owner); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(diagnostic.with_progress(progress)); } Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, value)) => { *self.target.value = Some(value); Err(diagnostic.with_native(error)) }
            };
        }
        if self.retirement.is_some() { return artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (diagnostic).with_native(error)); }
        if let Some(values) = self.values.take() {
            *self.rows_retirement = Some(ArtifactStoreOperationRowsRetirement::new(values, self.mutation_factory.as_ref().expect("original operation rows retain their factory").clone()));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(owner) = self.rows_retirement.as_mut() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.close_step(child).map_err(|error| (diagnostic).with_native(error))?;
            admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original SPR row owner").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if owner.terminal_is_empty() { drop(self.rows_retirement.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(factory) = self.mutation_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factories_close[0] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(factory) = self.catalog.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factories_close[1] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(slot) = self.factories_close.iter_mut().find(|slot| slot.is_some()) {
            let owner = slot.as_mut().expect("observed original mutation factory");
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.step(child).map_err(|error| (diagnostic).with_native(error))?;
            admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original SPR mutation factory").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if owner.terminal_is_empty() { drop(slot.take()); }
            self.taken = self.factories_close.iter().all(Option::is_none);
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        self.taken = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.taken && self.active.is_none() && self.reservation.is_none() && self.target.reservation.is_none() && self.target.value.is_none() && self.values.is_none() && self.retirement.is_none() && self.rows_retirement.is_none() && self.catalog.is_none() && self.mutation_factory.is_none() && self.factories_close.iter().all(Option::is_none)
    }
}

impl<P, Mutation: Send> Drop for ArtifactOwnedSprMutationArrayAuthority<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.taken && self.active.is_none() && self.reservation.is_none() && self.target.reservation.is_none() && self.target.value.is_none() && self.values.is_none() && self.retirement.is_none() && self.rows_retirement.is_none() && self.catalog.is_none() && self.mutation_factory.is_none() && self.factories_close.iter().all(Option::is_none)), "SPR mutation array reached Drop before every exact mutation owner was published or cursor-retired");
    }
}

#[expect(clippy::large_enum_variant, reason = "The active decoder retains its admitted fixed string authority inline across bounded steps without a new allocation.")]
pub(crate) enum ArtifactOwnedSprEditActive<P, Mutation: Send> {
    String { field_id: u16, authority: OwnedSchemaStringAuthority<ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES> },
    Mutations { field_id: u16, authority: ArtifactOwnedSprMutationArrayAuthority<P, Mutation> },
    EmptyMetadata(OwnedSchemaEmptyArrayAuthority),
}

pub(crate) struct ArtifactOwnedSprEditAuthority<P, Mutation>
where
    P: Send + Sync + 'static,
    Mutation: Send + 'static,
{
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: OwnedSchemaPath,
    catalog: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>>>,
    mutation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>>,
    cursor: OwnedSchemaNestedRecordCursor,
    pending: Option<(OwnedSchemaToken, bool)>,
    active: std::mem::ManuallyDrop<Option<ArtifactOwnedSprEditActive<P, Mutation>>>,
    completed_arrays: [std::mem::ManuallyDrop<Option<ArtifactOwnedSprMutationArrayAuthority<P, Mutation>>>; 2],
    factories_close: [Option<semio_framework_value::FactoryAuthority>; 3],
    strings: [std::mem::ManuallyDrop<Option<String>>; 6],
    actor: std::mem::ManuallyDrop<Option<semio_framework_value::SharedUtf8>>,
    actor_close: Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>,
    forwards: std::mem::ManuallyDrop<Option<Vec<Mutation>>>,
    inverse: std::mem::ManuallyDrop<Option<semio_framework_value::list::PagedList<Mutation, {usize::MAX}>>>,
    sequence_number: Option<i32>,
    value: std::mem::ManuallyDrop<Option<Edit<Mutation>>>,
    pub(crate) retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    close_string: usize,
    terminal: bool,
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactOwnedSprEditAuthority<P, Mutation> {
    pub(crate) fn new(
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
        mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
        retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
    ) -> Self {
        Self {
            operation,
            generation,
            path,
            catalog: std::mem::ManuallyDrop::new(Some(catalog)),
            mutation_factory: std::mem::ManuallyDrop::new(Some(mutation_factory)),
            retirement_factory: std::mem::ManuallyDrop::new(Some(retirement_factory)),
            cursor: OwnedSchemaNestedRecordCursor::try_new(OwnedSchemaRecordSpec { fields: ARTIFACT_OWNED_SPR_EDIT_FIELDS }).expect("SPR edit schema is a validated static catalog"),
            pending: None,
            active: std::mem::ManuallyDrop::new(None),
            completed_arrays: std::array::from_fn(|_| std::mem::ManuallyDrop::new(None)),
            factories_close: std::array::from_fn(|_| None),
            strings: std::array::from_fn(|_| std::mem::ManuallyDrop::new(None)),
            actor: std::mem::ManuallyDrop::new(None),
            actor_close: None,
            forwards: std::mem::ManuallyDrop::new(None),
            inverse: std::mem::ManuallyDrop::new(None),
            sequence_number: None,
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            close_string: 0,
            terminal: false,
        }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    /// 🌱️ Installs the exact owner for one edit field, or finishes a scalar field inline. Returns
    /// `Some(step)` when the field needed no owner at all.
    fn begin_edit_field(&mut self, field_id: u16, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor) -> Result<Option<ArtifactEnvelopeFieldDecodeStep>, OwnedSchemaDecodeDiagnostic> {
        if Self::string_index(field_id).is_some() {
            if token.kind == OwnedSchemaTokenKind::Null && matches!(field_id, 2 | 10 | 11 | 12) {
                return Ok(Some(ArtifactEnvelopeFieldDecodeStep::TokenComplete));
            }
            if !terminal {
                return Err(self.diagnostic("artifact-spr.edit-string-scalar", token.start));
            }
            let authority = OwnedSchemaStringAuthority::try_new(self.operation, self.generation, token, self.path).map_err(|token| self.diagnostic("artifact-spr.edit-string", token.start))?;
            *self.active = Some(ArtifactOwnedSprEditActive::String { field_id, authority });
            return Ok(None);
        }
        if matches!(field_id, 3 | 4) {
            let authority = ArtifactOwnedSprMutationArrayAuthority::try_new(self.operation, self.generation, self.path, self.catalog.as_ref().expect("live original edit catalog").clone(), self.mutation_factory.as_ref().expect("live original edit mutation factory").clone(), field_id == 4)?;
            *self.active = Some(ArtifactOwnedSprEditActive::Mutations { field_id, authority });
            return Ok(None);
        }
        if field_id == 5 {
            *self.active = Some(ArtifactOwnedSprEditActive::EmptyMetadata(OwnedSchemaEmptyArrayAuthority::new(self.path)));
            return Ok(None);
        }
        if field_id == 8 && terminal && token.kind == OwnedSchemaTokenKind::Number {
            let mut bytes = [0u8; 64];
            let len = usize::try_from(token.end.saturating_sub(token.start)).unwrap_or(usize::MAX);
            if len == 0 || len > bytes.len() || source.copy_token_bytes(token, 0, &mut bytes[..len]) != len {
                return Err(self.diagnostic("artifact-spr.edit-sequence-token", token.start));
            }
            self.sequence_number = std::str::from_utf8(&bytes[..len]).ok().and_then(|value| value.parse().ok());
            if self.sequence_number.is_none() {
                return Err(self.diagnostic("artifact-spr.edit-sequence-value", token.start));
            }
            return Ok(Some(ArtifactEnvelopeFieldDecodeStep::TokenComplete));
        }
        Err(self.diagnostic("artifact-spr.edit-field", token.start))
    }

    /// 🎯️ Routes one cursor-attributed token into the live edit-field owner.
    fn accept_active_edit_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let Some(active) = self.active.as_mut() else {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-field", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        };
        match active {
            ArtifactOwnedSprEditActive::String { field_id, authority } => match authority.step(source, cx) {
                OwnedSchemaStringStep::Pending => return Ok(ArtifactEnvelopeFieldDecodeStep::Pending),
                OwnedSchemaStringStep::Complete => {
                    let Some(index) = Self::string_index(*field_id) else {
                        return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-string-field", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
                    };
                    if index==1 {
                        let grant=cx.retained_grant();
                        if self.strings[1].is_none(){
                            let original=authority.as_str().ok_or(OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-text",offset:token.start,line:0,column:0,path, refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                            let copied_bytes=original.len().checked_add(size_of::<Option<String>>()).ok_or(OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-overflow",offset:token.start,line:0,column:0,path, refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                            if grant.maximum_items==0||grant.maximum_depth==0||copied_bytes>grant.maximum_copy_bytes||original.len()>grant.maximum_capacity_bytes{return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);}
                            let mut value=String::new();value.try_reserve_exact(original.len()).map_err(|_|OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-allocation",offset:token.start,line:0,column:0,path, refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;value.push_str(original);
                            let receipt=RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes:value.capacity(),released_bytes:0};
                            *self.strings[1]=Some(value);cx.consume_retained(receipt).map_err(|error|OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-receipt",offset:token.start,line:0,column:0,path,refusal_kind:error.kind,retained_progress:receipt})?;
                            return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);
                        }
                        let placement=size_of::<Option<semio_framework_value::SharedUtf8>>();
                        if grant.maximum_copy_bytes<placement{return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);}
                        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-placement,..grant};
                        match semio_framework_value::SharedUtf8::admit(self.strings[1].take().expect("original actor text retained"),child){
                            Ok((original,mut receipt))=>{receipt.copied_bytes+=placement;*self.actor=Some(original);cx.consume_retained(receipt).map_err(|error|OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-receipt",offset:token.start,line:0,column:0,path,refusal_kind:error.kind,retained_progress:receipt})?;}
                            Err((_,original))=>{*self.strings[1]=Some(original);return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);}
                        }
                    }else{*self.strings[index] = authority.take_string();}
                }
                OwnedSchemaStringStep::Cancelled => return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-string-cancelled", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }),
                OwnedSchemaStringStep::Fault(diagnostic) => return Err(diagnostic),
            },
            ArtifactOwnedSprEditActive::Mutations { field_id, authority } => match authority.accept(token, terminal, source, cx) {
                Err(diagnostic) => return Err(diagnostic),
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => {
                    let index = match *field_id { 3 => 0, 4 => 1, _ => return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-native-array-owner", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }) };
                    if self.completed_arrays[index].is_some() { return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-array-custody", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }); }
                    let Some(values) = authority.take_values() else {
                        return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-mutation-values", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
                    };
                    match (*field_id, values) {
                        (3, ArtifactStoreOperationRowOwners::Forwards(values)) => *self.forwards = Some(values),
                        (4, ArtifactStoreOperationRowOwners::Inverse(values)) => *self.inverse = Some(values),
                        (_, values) => {
                            *authority.values = Some(values);
                            authority.taken = false;
                            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-native-array-owner", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
                        }
                    }
                }
                Ok(step) => return Ok(step),
            },
            ArtifactOwnedSprEditActive::EmptyMetadata(authority) => match authority.accept(token, terminal) {
                Err(diagnostic) => return Err(diagnostic),
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => {}
                Ok(step) => return Ok(step),
            },
        }
        if let Some(ArtifactOwnedSprEditActive::Mutations { field_id, authority }) = self.active.take() {
            *self.completed_arrays[usize::from(field_id == 4)] = Some(authority);
        }
        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
    }

    fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::RetirementDemand;
        let diagnostic = self.diagnostic("artifact-spr.edit-close-demand", 0);
        if let Some(active) = self.active.as_ref() {
            let mut demand = match active {
                ArtifactOwnedSprEditActive::Mutations { authority, .. } => authority.close_demands(copy)?,
                _ => RetirementDemand::default(),
            };
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        if let Some(owner) = self.completed_arrays.iter().find_map(|slot| slot.as_ref()) {
            let mut demand = owner.close_demands(copy)?;
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        if let Some(owner) = self.retirement.as_ref() { return artifact_retirement_box_demands(owner, copy).map_err(|_| diagnostic); }
        if let Some(value) = self.value.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.retirement_factory.as_ref().expect("edit retains value factory").retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
        if self.inverse.is_some() || self.forwards.is_some() { return Ok(RetirementDemand { capacity_bytes: size_of::<ArtifactStoreOperationRowsRetirement<Mutation>>(), depth: 2, ..Default::default() }); }
        if let Some(owner)=self.actor_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of_val(&self.actor_close),depth:1,..Default::default()});}return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand().map_err(|error|diagnostic.with_native(error))?,capacity_bytes:owner.next_capacity_byte_demand(copy).map_err(|error|diagnostic.with_native(error))?,release_bytes:owner.next_release_byte_demand().map_err(|error|diagnostic.with_native(error))?,depth:owner.next_depth_demand().map_err(|error|diagnostic.with_native(error))?.checked_add(1).ok_or(diagnostic)?});}
        if self.actor.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_value::SharedUtf8>()+size_of_val(&self.actor_close),depth:1,..Default::default()});}
        if let Some(value) = self.strings[self.close_string..].iter().find_map(|slot| slot.as_ref()) { return Ok(RetirementDemand { release_bytes: value.capacity(), depth: 1, ..Default::default() }); }
        if self.retirement_factory.is_some() || self.mutation_factory.is_some() || self.catalog.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.factories_close.iter().flatten().next() {
            let mut demand = owner.demands(copy).map_err(|_| diagnostic)?;
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        Ok(Default::default())
    }

    fn string_index(field_id: u16) -> Option<usize> {
        match field_id {
            1 => Some(0),
            2 => Some(1),
            9 => Some(2),
            10 => Some(3),
            11 => Some(4),
            12 => Some(5),
            _ => None,
        }
    }

    fn finish_record(&mut self) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if self.strings[1].is_some(){return Err(self.diagnostic("artifact-spr.edit-actor-unpublished-text",0));}
        let id = self.strings[0].take().ok_or_else(|| self.diagnostic("artifact-spr.edit-id-missing", 0))?;
        let forwards = self.forwards.take().ok_or_else(|| self.diagnostic("artifact-spr.edit-forwards-missing", 0))?;
        let inverse = self.inverse.take().ok_or_else(|| self.diagnostic("artifact-spr.edit-inverse-missing", 0))?;
        let started_at = self.strings[2].take().ok_or_else(|| self.diagnostic("artifact-spr.edit-started-at-missing", 0))?;
        let sequence_number = self.sequence_number.ok_or_else(|| self.diagnostic("artifact-spr.edit-sequence-missing", 0))?;
        *self.value = Some(Edit {
            id,
            actor: self.actor.take(),
            line: self.strings[5].take(),
            forwards,
            inverse: inverse.into(),
            mutation_meta: Vec::new(),
            verb: self.strings[4].take(),
            sequence_number,
            started_at,
            finished_at: self.strings[3].take(),
        });
        Ok(())
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactOwnedHistoryEntryAuthority<Edit<Mutation>> for ArtifactOwnedSprEditAuthority<P, Mutation> {
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(copy).map(|demand| demand.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.depth) }
    fn accept_token(&mut self, token: OwnedSchemaToken, _terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("artifact-spr.edit-stale", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("artifact-spr.edit-cancelled", token.start));
        }
        let (token, terminal) = match self.pending {
            Some(pending) => pending,
            None => match self.cursor.accept(token, source) {
                OwnedSchemaNestedRecordStep::Pending => return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete),
                OwnedSchemaNestedRecordStep::FieldToken { field_id, token, terminal } => {
                    if self.active.is_none() {
                        if let Some(step) = self.begin_edit_field(field_id, token, terminal, source)? {
                            return Ok(step);
                        }
                    }
                    (token, terminal)
                }
                OwnedSchemaNestedRecordStep::Complete => {
                    self.finish_record()?;
                    return Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete);
                }
                OwnedSchemaNestedRecordStep::Fault(diagnostic) => return Err(diagnostic),
            },
        };
        self.pending = Some((token, terminal));
        let outcome = self.accept_active_edit_token(token, terminal, source, cx);
        if !matches!(outcome, Ok(ArtifactEnvelopeFieldDecodeStep::Pending)) {
            self.pending = None;
        }
        outcome
    }

    fn take_value(&mut self) -> Option<Edit<Mutation>> {
        let value = self.value.take()?;
        self.terminal = true;
        Some(value)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        let diagnostic = self.diagnostic("artifact-spr.edit-close-authority", 0);
        if grant.maximum_depth < demand.depth { return Err(diagnostic); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.pending = None;
        self.terminal = true;
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(active) = self.active.as_mut() {
            let (step, terminal) = match active {
                ArtifactOwnedSprEditActive::String { authority, .. } => { authority.cancel(); (RetainedCloneStep::Complete(empty), authority.terminal_is_empty()) }
                ArtifactOwnedSprEditActive::Mutations { authority, .. } => { let step = authority.close_step(child)?; (step, authority.terminal_is_empty()) }
                ArtifactOwnedSprEditActive::EmptyMetadata(_) => (RetainedCloneStep::Complete(empty), true),
            };
            admit_retained_clone_close(child, step, terminal, "original edit active field").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if terminal { drop(self.active.take()); }
            return Ok(RetainedCloneStep::Progress(if terminal && step.progress().copied_items == 0 { RetainedCloneProgress { copied_items: 1, ..step.progress() } } else { step.progress() }));
        }
        if let Some(slot) = self.completed_arrays.iter_mut().find(|slot| slot.is_some()) {
            let owner = slot.as_mut().expect("observed original completed mutation array");
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original edit completed field").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if terminal { drop(slot.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.retirement.is_some() { return artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (diagnostic).with_native(error)); }
        if let Some(value) = self.value.take() {
            match self.retirement_factory.as_ref().expect("edit retains value factory").retire_owned(value, child) {
                Ok((owner, progress)) => { *self.retirement = Some(owner); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(diagnostic.with_progress(progress)); } return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.value = Some(value); return Err(diagnostic.with_native(error)); }
            }
        }
        if let Some(values) = self.inverse.take() {
            let factory = self.mutation_factory.as_ref().expect("edit inverse retains factory").clone();
            match admit_artifact_retirement(values, grant, |values| ArtifactStoreOperationRowsRetirement::new(values, factory)) {
                Ok((owner, progress)) => { *self.retirement = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, values)) => { *self.inverse = Some(values); return Err(diagnostic.with_native(error)); }
            }
        }
        if let Some(values) = self.forwards.take() {
            let factory = self.mutation_factory.as_ref().expect("edit forwards retains factory").clone();
            match admit_artifact_retirement(values, grant, |values| ArtifactStoreOperationRowsRetirement::new(values, factory)) {
                Ok((owner, progress)) => { *self.retirement = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, values)) => { *self.forwards = Some(values); return Err(diagnostic.with_native(error)); }
            }
        }
        if let Some(owner)=self.actor_close.as_mut(){if owner.terminal_is_empty(){self.actor_close=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}let step=owner.step(child).map_err(|error| (diagnostic).with_native(error))?;admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original decoded actor lease").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
        if let Some(original)=self.actor.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>self.actor_close=Some(owner),Err((error,original))=>{*self.actor=Some(original);return Err(diagnostic.with_native(error));}}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}
        while self.close_string < self.strings.len() {
            let index = self.close_string;
            self.close_string += 1;
            if self.strings[index].is_some() { drop(self.strings[index].take()); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty })); }
        }
        let factory = if let Some(factory) = self.retirement_factory.take() { Some((0, factory as Arc<dyn semio_framework_value::FactoryRetirement>)) }
            else if let Some(factory) = self.mutation_factory.take() { Some((1, factory as Arc<dyn semio_framework_value::FactoryRetirement>)) }
            else { self.catalog.take().map(|factory| (2, factory as Arc<dyn semio_framework_value::FactoryRetirement>)) };
        if let Some((index, factory)) = factory {
            self.factories_close[index] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(slot) = self.factories_close.iter_mut().find(|slot| slot.is_some()) {
            let owner = slot.as_mut().expect("observed original edit factory");
            let step = owner.step(child).map_err(|error| (diagnostic).with_native(error))?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original edit factory").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if terminal { drop(slot.take()); }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.actor.is_none() && self.actor_close.is_none() && self.completed_arrays.iter().all(|slot| slot.is_none()) && self.factories_close.iter().all(Option::is_none) && self.catalog.is_none() && self.mutation_factory.is_none() && self.retirement_factory.is_none() && self.active.is_none() && self.forwards.is_none() && self.inverse.is_none() && self.value.is_none() && self.retirement.is_none() && self.strings.iter().all(|value| value.is_none())
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> Drop for ArtifactOwnedSprEditAuthority<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (ArtifactOwnedHistoryEntryAuthority::terminal_is_empty(self)), "SPR edit decode reached Drop before exact publication or bounded retirement");
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(crate) struct ArtifactOwnedSprEditDecoder<P, Mutation> {
    #[factory_child]
    pub(crate) catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
    #[factory_child]
    pub(crate) mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
}

impl<P: Send + Sync + 'static, Mutation: Send + Sync + 'static> ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>> for ArtifactOwnedSprEditDecoder<P, Mutation> {
    fn begin_entry(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
    ) -> Box<dyn ArtifactOwnedHistoryEntryAuthority<Edit<Mutation>>> {
        Box::new(ArtifactOwnedSprEditAuthority::new(operation, generation, path, self.catalog.clone(), self.mutation_factory.clone(), retirement_factory))
    }
}

/// 📡️ Builds the repository-owned retained `.spr`/`.ops` edit decoder around an
/// exact domain catalog. Domain snapshots and mutations never cross this seam as serde owners.
pub fn artifact_owned_spr_edit_history_decoder<P: Send + Sync + 'static, Mutation: Send + Sync + 'static>(
    catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
    mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
) -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>>> {
    Arc::new(ArtifactOwnedSprEditDecoder { catalog, mutation_factory })
}

impl<T: FromValue + Send + Sync + 'static> ArtifactOwnedHistoryEntryDecoder<T> for ArtifactRepositoryHistoryEntryDecoder<T> {
    fn begin_entry(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,
    ) -> Box<dyn ArtifactOwnedHistoryEntryAuthority<T>> {
        Box::new(ArtifactRepositoryHistoryEntryAuthority::new(operation, generation, path, retirement_factory))
    }
}

pub(crate) struct ArtifactRepositoryHistoryEntryAuthority<T> {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: OwnedSchemaPath,
    pub(crate) raw: std::mem::ManuallyDrop<Option<Box<[u8; ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES]>>>,
    raw_len: usize,
    pub(crate) value: std::mem::ManuallyDrop<Option<T>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub(crate) retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
    factory_close: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
    terminal: bool,
}

impl<T> ArtifactRepositoryHistoryEntryAuthority<T> {
    pub(crate) fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: OwnedSchemaPath, retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>) -> Self {
        Self { operation, generation, path, raw: std::mem::ManuallyDrop::new(Some(Box::new([0; ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES]))), raw_len: 0, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None), retirement_factory: std::mem::ManuallyDrop::new(Some(retirement_factory)), factory_close: std::mem::ManuallyDrop::new(None), terminal: false }
    }

    pub(crate) fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if let Some(owner) = self.retirement.as_ref() { return artifact_retirement_box_demands(owner, copy); }
        if let Some(value) = self.value.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.retirement_factory.as_ref().expect("history value retains its factory").retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
        if self.raw_len != 0 { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if self.raw.is_some() { return Ok(RetirementDemand { release_bytes: size_of::<[u8; ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES]>(), depth: 1, ..Default::default() }); }
        if self.retirement_factory.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.factory_close.as_ref() {
            let mut demand = owner.demands(copy)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history decoder factory depth overflow"))?;
            return Ok(demand);
        }
        Ok(Default::default())
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }
}

impl<T: FromValue + Send + 'static> ArtifactOwnedHistoryEntryAuthority<T> for ArtifactRepositoryHistoryEntryAuthority<T> {
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.copy_bytes).map_err(|error| self.diagnostic("artifact-envelope.history-entry-copy-demand", 0).with_native(error)) }
    fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(copy).map(|demand| demand.capacity_bytes).map_err(|error| self.diagnostic("artifact-envelope.history-entry-capacity-demand", 0).with_native(error)) }
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.release_bytes).map_err(|error| self.diagnostic("artifact-envelope.history-entry-release-demand", 0).with_native(error)) }
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.depth).map_err(|error| self.diagnostic("artifact-envelope.history-entry-depth-demand", 0).with_native(error)) }
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("artifact-envelope.history-entry-stale", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("artifact-envelope.history-entry-cancelled", token.start));
        }
        let path = self.path;
        let diagnostic = |code| OwnedSchemaDecodeDiagnostic { code, offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
        let span = usize::try_from(token.end.saturating_sub(token.start)).unwrap_or(usize::MAX);
        let raw = self.raw.as_mut().ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-entry-closed-source", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        let Some(end) = self.raw_len.checked_add(span).filter(|end| *end <= raw.len()) else {
            return Err(diagnostic("artifact-envelope.history-entry-byte-capacity"));
        };
        if source.copy_token_bytes(token, 0, &mut raw[self.raw_len..end]) != span {
            return Err(diagnostic("artifact-envelope.history-entry-source"));
        }
        self.raw_len = end;
        cx.consume_fuel(span.max(1) as u64);
        if !terminal {
            return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete);
        }
        let text = std::str::from_utf8(&raw[..self.raw_len]).map_err(|_| diagnostic("artifact-envelope.history-entry-decode"))?;
        let value = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| diagnostic("artifact-envelope.history-entry-decode"))?;
        *self.value = Some(value);
        self.raw_len = 0;
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn take_value(&mut self) -> Option<T> {
        let value = self.value.take()?;
        self.terminal = true;
        Some(value)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress, admit_retained_clone_close};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes).map_err(|error| (self.diagnostic("artifact-envelope.history-entry-demand", 0)).with_native(error))?;
        if grant.maximum_depth < demand.depth { return Err(self.diagnostic("artifact-envelope.history-entry-depth-refused", 0)); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.terminal = true;
        if self.retirement.is_some() { return artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (self.diagnostic("artifact-envelope.history-entry-retirement-fault", 0)).with_native(error)); }
        if let Some(value) = self.value.take() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            match self.retirement_factory.as_ref().expect("history value retains its factory").retire_owned(value, child) {
                Ok((owner, progress)) => {
                    *self.retirement = Some(owner);
                    if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(self.diagnostic("artifact-envelope.history-entry-constructor-receipt", 0).with_progress(progress)); }
                    return Ok(RetainedCloneStep::Progress(progress));
                }
                Err((error, value)) => { *self.value = Some(value); return Err(self.diagnostic("artifact-envelope.history-entry-retirement-admission", 0).with_native(error)); }
            }
        }
        if self.raw_len != 0 { self.raw_len = 0; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
        if self.raw.is_some() { drop(self.raw.take()); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty })); }
        if let Some(factory) = self.retirement_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            *self.factory_close = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        let failure = self.diagnostic("artifact-envelope.history-entry-factory-close", 0);
        let owner = self.factory_close.as_mut().expect("remaining original history factory");
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = owner.step(child).map_err(|error| (failure).with_native(error))?;
        let terminal = owner.terminal_is_empty();
        admit_retained_clone_close(child, step, terminal, "original history decoder factory").map_err(|error| (self.diagnostic("artifact-envelope.history-entry-factory-receipt", 0)).with_native(error).with_progress(step.progress()))?;
        if terminal { drop(self.factory_close.take()); }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none() && self.raw_len == 0 && self.raw.is_none() && self.retirement_factory.is_none() && self.factory_close.is_none()
    }
}

impl<T> Drop for ArtifactRepositoryHistoryEntryAuthority<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal && self.value.is_none() && self.retirement.is_none() && self.raw_len == 0 && self.raw.is_none() && self.retirement_factory.is_none() && self.factory_close.is_none()), "repository history entry authority reached Drop before exact terminal ownership");
    }
}

/// 📦️ Exact owner-supplied catalog and retirement factories required to admit an
/// envelope decode. Construction is all-or-nothing so a caller cannot pair a field catalog with
/// unrelated nested-owner disposal authority.
pub struct ArtifactEnvelopeDecodeOwnerBundle<P, Mutation> {
    catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
    initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
}

impl<P, Mutation> ArtifactEnvelopeDecodeOwnerBundle<P, Mutation> {
    pub fn new(catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>, initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self {
        Self { catalog, initial_snapshot_factory, mutation_factory }
    }

    pub fn begin_fresh_decoder(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        completed: Arc<ArtifactEnvelopeCompletedRecordRegistry<P, Mutation>>,
        completion: Arc<ArtifactEnvelopeDecodeCompletion>,
    ) -> Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>
    where
        P: Send + Sync + 'static,
        Mutation: Send + 'static,
    {
        Box::new(ArtifactEnvelopeFreshFieldDecoder::new(operation, generation, Arc::clone(&self.catalog), Arc::clone(&self.initial_snapshot_factory), Arc::clone(&self.mutation_factory), completed, completion))
    }

    pub fn retire_envelope(&self, envelope: ArtifactEnvelope<P, Mutation>) -> Box<dyn ErasedSnapshotRetirement>
    where
        P: Send + Sync + 'static,
        Mutation: Send + 'static,
    {
        Box::new(ArtifactStoreEnvelopeRetirement::new(envelope, Arc::clone(&self.initial_snapshot_factory), Arc::clone(&self.mutation_factory)))
    }
}

/// 🧬️ Domain-supplied typed field owner used by envelope decode; no serde type crosses this seam.
pub trait ArtifactEnvelopeFieldDecoder<P, Mutation>: Send {
    fn accept_field_token(&mut self, field_id: u16, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn finish_record(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn maximum_close_byte_demand(&self) -> usize;
    fn maximum_retained_close_bytes(&self) -> usize;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

/// 📏️ Observes every independent demand of one field decoder's next close turn.
pub(crate) fn artifact_envelope_field_demands<P, Mutation>(owner: &dyn ArtifactEnvelopeFieldDecoder<P, Mutation>, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
    if owner.terminal_is_empty() { return Ok(Default::default()); }
    Ok(semio_framework_value::RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(maximum_copy_bytes)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? })
}

/// 🎟️ Whether one admitted item, copy, capacity and release grant covers every independent demand axis.
pub(crate) fn artifact_retirement_grant_covers(grant: RetainedCloneGrant, demand: semio_framework_value::RetirementDemand) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

/// 🎟️ Self-funds one turn from its own quoted demand for a caller no scheduler grants.
pub(crate) fn artifact_retirement_self_grant(demand: semio_framework_value::RetirementDemand, minimum_copy_bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(minimum_copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// 🧯️ Converts a decode diagnostic into the value-layer invariant refusal.
pub(crate) fn artifact_envelope_diagnostic_error(diagnostic: OwnedSchemaDecodeDiagnostic) -> ValueError {
    ValueError::literal(diagnostic.refusal_kind, diagnostic.code).with_retained_progress(diagnostic.retained_progress)
}

pub const ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeFieldDecoderTicket {
    index: u16,
    generation: u64,
}

impl ArtifactEnvelopeFieldDecoderTicket {
    pub fn index(self) -> usize {
        usize::from(self.index)
    }

    pub fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactEnvelopeFieldDecoderRegistryFault {
    Contended,
    Capacity,
    CloseByteCapacity,
    GenerationExhausted,
    Stale,
    Returned,
    FalseTerminal,
}

pub(crate) struct ArtifactEnvelopeFieldDecoderSlot<P, Mutation> {
    generation: u64,
    owner: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>>>,
}

pub(crate) struct ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation> {
    slots: Box<[ArtifactEnvelopeFieldDecoderSlot<P, Mutation>]>,
    free: [u16; ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY],
    free_len: usize,
}

impl<P, Mutation> Drop for ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.slots.iter().all(|slot| slot.owner.is_none())), "artifact envelope field registry reached Drop before every exact decoder owner was terminal-empty");
    }
}

/// 🗄️ Fixed app-owned registry that keeps nested field decoders out of stack/job destructors.
pub struct ArtifactEnvelopeFieldDecoderRegistry<P, Mutation> {
    pub(crate) state: Mutex<ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation>>,
    returned: Box<[std::sync::atomic::AtomicU64]>,
    returned_mask: std::sync::atomic::AtomicU64,
    live: std::sync::atomic::AtomicUsize,
    reclaimed: Box<[std::sync::atomic::AtomicU64]>,
}

impl<P, Mutation> ArtifactEnvelopeFieldDecoderRegistry<P, Mutation> {
    /// 🏗️ Preallocates the entire fixed decoder authority before any input owner exists.
    pub fn new() -> Arc<Self> {
        let slots = (0..ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY).map(|_| ArtifactEnvelopeFieldDecoderSlot { generation: 0, owner: std::mem::ManuallyDrop::new(None) }).collect::<Vec<_>>().into_boxed_slice();
        let returned = (0..ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY).map(|_| std::sync::atomic::AtomicU64::new(0)).collect::<Vec<_>>().into_boxed_slice();
        let reclaimed = (0..ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY).map(|_| std::sync::atomic::AtomicU64::new(0)).collect::<Vec<_>>().into_boxed_slice();
        let mut free = [0; ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY];
        for (index, value) in free.iter_mut().enumerate() {
            *value = index as u16;
        }
        Arc::new(Self {
            state: Mutex::new(ArtifactEnvelopeFieldDecoderRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY }),
            returned,
            returned_mask: std::sync::atomic::AtomicU64::new(0),
            live: std::sync::atomic::AtomicUsize::new(0),
            reclaimed,
        })
    }

    /// 🎫️ Admits one exact decoder or returns it untouched on contention/capacity/exhaustion.
    pub fn try_admit(
        self: &Arc<Self>,
        owner: Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>,
    ) -> Result<ArtifactEnvelopeFieldDecoderLease<P, Mutation>, (ArtifactEnvelopeFieldDecoderRegistryFault, Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>)> {
        let mut state = match self.state.try_lock() {
            Ok(state) => state,
            Err(_) => return Err((ArtifactEnvelopeFieldDecoderRegistryFault::Contended, owner)),
        };
        if state.free_len == 0 {
            return Err((ArtifactEnvelopeFieldDecoderRegistryFault::Capacity, owner));
        }
        state.free_len -= 1;
        let index = usize::from(state.free[state.free_len]);
        let Some(generation) = state.slots[index].generation.checked_add(1) else {
            let free_index = state.free_len;
            state.free[free_index] = index as u16;
            state.free_len += 1;
            return Err((ArtifactEnvelopeFieldDecoderRegistryFault::GenerationExhausted, owner));
        };
        state.slots[index].generation = generation;
        *state.slots[index].owner = Some(owner);
        self.returned[index].store(0, std::sync::atomic::Ordering::Release);
        self.returned_mask.fetch_and(!(1u64 << index), std::sync::atomic::Ordering::AcqRel);
        self.reclaimed[index].store(0, std::sync::atomic::Ordering::Release);
        self.live.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Ok(ArtifactEnvelopeFieldDecoderLease { registry: Arc::clone(self), ticket: ArtifactEnvelopeFieldDecoderTicket { index: index as u16, generation }, returned: false })
    }

    fn with_owner<R>(&self, ticket: ArtifactEnvelopeFieldDecoderTicket, f: impl FnOnce(&mut dyn ArtifactEnvelopeFieldDecoder<P, Mutation>) -> R) -> Result<R, ArtifactEnvelopeFieldDecoderRegistryFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeFieldDecoderRegistryFault::Contended)?;
        let index = usize::from(ticket.index);
        let Some(slot) = state.slots.get_mut(index) else { return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Stale) };
        if slot.generation != ticket.generation {
            return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Stale);
        }
        if self.returned[index].load(std::sync::atomic::Ordering::Acquire) == ticket.generation {
            return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Returned);
        }
        let owner = slot.owner.as_deref_mut().ok_or(ArtifactEnvelopeFieldDecoderRegistryFault::Stale)?;
        Ok(f(owner))
    }

    /// 🧹️ Detaches one exact returned decoder without calling it or dropping it under the registry lock.
    pub fn take_returned_ticket(&self, ticket: ArtifactEnvelopeFieldDecoderTicket) -> Result<ArtifactEnvelopeReturnedFieldDecoder<P, Mutation>, ArtifactEnvelopeFieldDecoderRegistryFault> {
        let owner = {
            let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeFieldDecoderRegistryFault::Contended)?;
            let index = usize::from(ticket.index);
            let Some(slot) = state.slots.get_mut(index) else { return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Stale) };
            if slot.generation != ticket.generation {
                return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Stale);
            }
            if self.returned[index].load(std::sync::atomic::Ordering::Acquire) != ticket.generation {
                return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Returned);
            }
            let owner = slot.owner.take().expect("validated returned decoder owner remains present");
            self.returned[index].store(0, std::sync::atomic::Ordering::Release);
            self.returned_mask.fetch_and(!(1u64 << index), std::sync::atomic::Ordering::AcqRel);
            self.reclaimed[index].store(ticket.generation, std::sync::atomic::Ordering::Release);
            self.live.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            let free_index = state.free_len;
            state.free[free_index] = ticket.index;
            state.free_len += 1;
            owner
        };
        Ok(ArtifactEnvelopeReturnedFieldDecoder { owner: std::mem::ManuallyDrop::new(Some(owner)) })
    }

    /// 🔎 Returns one exact atomically handed-back ticket without scanning the fixed registry.
    pub fn next_returned_ticket(&self) -> Option<ArtifactEnvelopeFieldDecoderTicket> {
        let mask = self.returned_mask.load(std::sync::atomic::Ordering::Acquire);
        if mask == 0 {
            return None;
        }
        let index = mask.trailing_zeros() as usize;
        let generation = self.returned[index].load(std::sync::atomic::Ordering::Acquire);
        (generation != 0).then_some(ArtifactEnvelopeFieldDecoderTicket { index: index as u16, generation })
    }

    /// ✅ Exact generation witness that the app maintenance owner detached this ticket.
    /// 🪪️ Whether THIS ticket's decoder has been handed back. Monotone by slot generation, so
    /// a later lease reusing the same slot can never make an already-reclaimed ticket read as
    /// outstanding again: an `==` test flipped a finished authority's own `Drop` witness back to
    /// false the moment a sibling authority took the next generation of the same slot, which is
    /// exactly the ABA the per-slot generation counter exists to rule out.
    pub fn ticket_reclaimed(&self, ticket: ArtifactEnvelopeFieldDecoderTicket) -> bool {
        self.reclaimed[ticket.index()].load(std::sync::atomic::Ordering::Acquire) >= ticket.generation
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.live.load(std::sync::atomic::Ordering::Acquire) == 0
    }
}

/// 🪪️ Shallow non-cloneable lease; Drop atomically hands its exact decoder back to the registry.
pub struct ArtifactEnvelopeFieldDecoderLease<P, Mutation> {
    registry: Arc<ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>>,
    ticket: ArtifactEnvelopeFieldDecoderTicket,
    returned: bool,
}

impl<P, Mutation> ArtifactEnvelopeFieldDecoderLease<P, Mutation> {
    pub(crate) fn with_owner<R>(&self, f: impl FnOnce(&mut dyn ArtifactEnvelopeFieldDecoder<P, Mutation>) -> R) -> Result<R, ArtifactEnvelopeFieldDecoderRegistryFault> {
        self.registry.with_owner(self.ticket, f)
    }

    pub fn ticket(&self) -> ArtifactEnvelopeFieldDecoderTicket {
        self.ticket
    }

    /// ↩️ Atomically returns the lease exactly once without acquiring or blocking on the owner mutex.
    pub fn return_now(&mut self) -> bool {
        if self.returned {
            return false;
        }
        let index = usize::from(self.ticket.index);
        self.returned = self.registry.returned[index].compare_exchange(0, self.ticket.generation, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok();
        if self.returned {
            self.registry.returned_mask.fetch_or(1u64 << index, std::sync::atomic::Ordering::Release);
        }
        self.returned
    }
}

impl<P, Mutation> Drop for ArtifactEnvelopeFieldDecoderLease<P, Mutation> {
    fn drop(&mut self) {
        let _ = self.return_now();
    }
}

/// 🧹️ Detached decoder owner driven outside the registry lock by the bounded maintenance lane.
pub struct ArtifactEnvelopeReturnedFieldDecoder<P, Mutation> {
    pub(crate) owner: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>>>,
}

impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeReturnedFieldDecoder<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.owner.is_none() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "returned artifact envelope field decoder exceeds admitted depth")); }
        if !artifact_retirement_grant_covers(grant, demand) { return Ok(RetainedCloneStep::Progress(empty)); }
        let owner = self.owner.as_mut().expect("returned field decoder remains present");
        if owner.terminal_is_empty() {
            drop(self.owner.take());
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = owner.close_step(child).map_err(artifact_envelope_diagnostic_error)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.terminal_is_empty(), "returned artifact envelope field decoder")?;
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none()
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(self.demands(maximum_body_bytes)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P, Mutation> ArtifactEnvelopeReturnedFieldDecoder<P, Mutation> {
    /// 📏️ Observes the detached decoder's next turn, or the separate release of its terminal shell.
    fn demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        let Some(owner) = self.owner.as_ref() else { return Ok(Default::default()) };
        if owner.terminal_is_empty() { return Ok(semio_framework_value::RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }); }
        let demand = artifact_envelope_field_demands(owner.as_ref(), maximum_copy_bytes).map_err(artifact_envelope_diagnostic_error)?;
        Ok(semio_framework_value::RetirementDemand { depth: demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "returned artifact envelope field decoder depth overflow"))?, ..demand })
    }
}

impl<P, Mutation> Drop for ArtifactEnvelopeReturnedFieldDecoder<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owner.is_none()), "returned artifact envelope field decoder reached Drop before its terminal-empty witness");
    }
}

impl<P, Mutation> ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    /// ↩️ Detaches and bounded-closes one exact returned decoder on the caller thread so a
    /// worker [`release_step`] can pass its `ticket_reclaimed` gate without waiting for the app's
    /// next maintenance visit (ticket 26/09/09/PROCEDURAL-3D-END-TO-END `env-20`).
    pub fn reclaim_returned_ticket_now(&self, ticket: ArtifactEnvelopeFieldDecoderTicket) -> Result<(), ArtifactEnvelopeFieldDecoderRegistryFault> {
        if self.ticket_reclaimed(ticket) {
            return Ok(());
        }
        let mut retired = self.take_returned_ticket(ticket)?;
        for _ in 0..4096 {
            if retired.terminal_is_empty() {
                drop(retired);
                return Ok(());
            }
            let Ok(demand) = retired.demands(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) else { return Err(ArtifactEnvelopeFieldDecoderRegistryFault::FalseTerminal) };
            if ErasedSnapshotRetirement::close_step(&mut retired, artifact_retirement_self_grant(demand, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)).is_err() { return Err(ArtifactEnvelopeFieldDecoderRegistryFault::FalseTerminal); }
        }
        Err(ArtifactEnvelopeFieldDecoderRegistryFault::FalseTerminal)
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
pub(crate) enum OwnedSchemaStringState {
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

/// 🧺️ Token-local authority for an exact empty JSON array. The opening token is
/// consumed separately from the terminal token so a surrounding record never replays either one.
pub struct OwnedSchemaEmptyArrayAuthority {
    path: OwnedSchemaPath,
    opened: bool,
    complete: bool,
}

pub const ARTIFACT_ENVELOPE_HISTORY_ITEMS: usize = 64;

pub const ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

pub(crate) enum OwnedSchemaBoundedArrayState {
    AwaitStart,
    Entries,
    Complete,
    Closing,
}

/// 📚️ Retained one-entry-at-a-time decoder for repository-owned history arrays.
/// Each semantic entry is capped at one decode page and enters the fixed generation ledger once.
pub(crate) struct OwnedSchemaBoundedArrayAuthority<T> {
    path: OwnedSchemaPath,
    pub(crate) values: std::mem::ManuallyDrop<Option<ArtifactHistoryLedger<T>>>,
    ledger_retirement: std::mem::ManuallyDrop<Option<ArtifactStoreHistoryLedgerRetirement<T>>>,
    reservation: Option<ArtifactHistoryReservation>,
    rejected: std::mem::ManuallyDrop<Option<T>>,
    pub(crate) retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
    pub(crate) decoder: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedHistoryEntryDecoder<T>>>>,
    factories_close: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
    pub(crate) active_decoder: std::mem::ManuallyDrop<Option<Box<dyn ArtifactOwnedHistoryEntryAuthority<T>>>>,
    active_retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    entry_depth: usize,
    state: OwnedSchemaBoundedArrayState,
    taken: bool,
}

impl<T: Send + 'static> OwnedSchemaBoundedArrayAuthority<T> {
    pub(crate) fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::RetirementDemand;
        let diagnostic = OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-close-demand", offset: 0, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
        if let Some(owner) = self.active_decoder.as_ref() {
            if owner.terminal_is_empty() { return Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }); }
            return Ok(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(copy)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()?.checked_add(1).ok_or(diagnostic)? });
        }
        if let Some(owner) = self.active_retirement.as_ref() { return artifact_retirement_box_demands(owner, copy).map_err(|error| diagnostic.with_native(error)); }
        if self.reservation.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(value) = self.rejected.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.retirement_factory.as_ref().expect("history entry retains its factory").retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
        if self.values.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<ArtifactHistoryLedger<T>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.ledger_retirement.as_ref() {
            return Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand().map_err(|error| diagnostic.with_native(error))?, capacity_bytes: owner.next_capacity_byte_demand(copy).map_err(|error| diagnostic.with_native(error))?, release_bytes: owner.next_release_byte_demand().map_err(|error| diagnostic.with_native(error))?, depth: owner.next_depth_demand().map_err(|error| diagnostic.with_native(error))?.checked_add(1).ok_or(diagnostic)? });
        }
        if self.retirement_factory.is_some() || self.decoder.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.factories_close.iter().flatten().next() {
            let mut demand = owner.demands(copy).map_err(|error| diagnostic.with_native(error))?;
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
    pub(crate) fn new(path: OwnedSchemaPath, retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>, decoder: Arc<dyn ArtifactOwnedHistoryEntryDecoder<T>>) -> Self {
        Self {
            path,
            values: std::mem::ManuallyDrop::new(Some(ArtifactHistoryLedger::new())),
            ledger_retirement: std::mem::ManuallyDrop::new(None),
            reservation: None,
            rejected: std::mem::ManuallyDrop::new(None),
            retirement_factory: std::mem::ManuallyDrop::new(Some(retirement_factory)),
            decoder: std::mem::ManuallyDrop::new(Some(decoder)),
            factories_close: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)),
            active_decoder: std::mem::ManuallyDrop::new(None),
            active_retirement: std::mem::ManuallyDrop::new(None),
            entry_depth: 0,
            state: OwnedSchemaBoundedArrayState::AwaitStart,
            taken: false,
        }
    }

    fn diagnostic(&self, code: &'static str, token: OwnedSchemaToken) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    fn accept(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if let Some(decoder) = self.active_decoder.as_mut() {
            let path = self.path;
            let entry_terminal = self.entry_depth == 1 && token.kind == OwnedSchemaTokenKind::ObjectEnd;
            return match decoder.accept_token(token, entry_terminal, source, cx)? {
                ArtifactEnvelopeFieldDecodeStep::Pending => Ok(ArtifactEnvelopeFieldDecodeStep::Pending),
                ArtifactEnvelopeFieldDecodeStep::TokenComplete if entry_terminal => Err(self.diagnostic("artifact-envelope.history-entry-false-terminal", token)),
                ArtifactEnvelopeFieldDecodeStep::FieldComplete if !entry_terminal => Err(self.diagnostic("artifact-envelope.history-entry-early-terminal", token)),
                ArtifactEnvelopeFieldDecodeStep::RecordComplete => Err(self.diagnostic("artifact-envelope.history-entry-record-terminal", token)),
                step => {
                    match token.kind {
                        OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart => {
                            self.entry_depth = self.entry_depth.checked_add(1).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-depth", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                        }
                        OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd => {
                            self.entry_depth = self.entry_depth.checked_sub(1).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-depth", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                        }
                        _ => {}
                    }
                    if self.entry_depth == 0 {
                        let value = decoder.take_value().ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-entry-owner-missing", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                        if !decoder.terminal_is_empty() {
                            *self.rejected = Some(value);
                            return Err(self.diagnostic("artifact-envelope.history-entry-live-after-take", token));
                        }
                        drop(self.active_decoder.take());
                        let Some(reservation) = self.reservation.take() else {
                            *self.rejected = Some(value);
                            return Err(self.diagnostic("artifact-envelope.history-reservation-missing", token));
                        };
                        let Some(values) = self.values.as_mut() else {
                            *self.rejected = Some(value);
                            return Err(self.diagnostic("artifact-envelope.history-ledger-missing", token));
                        };
                        if let Err((reservation, value)) = values.insert_reserved(reservation, value) {
                            self.reservation = Some(reservation);
                            *self.rejected = Some(value);
                            return Err(self.diagnostic("artifact-envelope.history-reservation-stale", token));
                        }
                        return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete);
                    }
                    Ok(step)
                }
            };
        }
        match self.state {
            OwnedSchemaBoundedArrayState::AwaitStart if token.kind == OwnedSchemaTokenKind::ArrayStart && !terminal => {
                self.state = OwnedSchemaBoundedArrayState::Entries;
                Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
            }
            OwnedSchemaBoundedArrayState::AwaitStart => Err(self.diagnostic("artifact-envelope.history-expected-array", token)),
            OwnedSchemaBoundedArrayState::Entries if self.entry_depth == 0 && token.kind == OwnedSchemaTokenKind::ArrayEnd && terminal => {
                self.state = OwnedSchemaBoundedArrayState::Complete;
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            OwnedSchemaBoundedArrayState::Entries if self.entry_depth == 0 && token.kind == OwnedSchemaTokenKind::Comma => Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete),
            OwnedSchemaBoundedArrayState::Entries if self.entry_depth == 0 && token.kind == OwnedSchemaTokenKind::ObjectStart => {
                let Some(values) = self.values.as_mut() else {
                    return Err(self.diagnostic("artifact-envelope.history-ledger-missing", token));
                };
                let reservation = match values.reserve_one() {
                    Ok(reservation) => reservation,
                    Err(_) => return Err(self.diagnostic("artifact-envelope.history-item-capacity", token)),
                };
                self.reservation = Some(reservation);
                *self.active_decoder = Some(self.decoder.as_ref().expect("live history decoder").begin_entry(cx.operation(), cx.generation(), self.path, self.retirement_factory.as_ref().expect("live history retirement factory").clone()));
                self.accept(token, terminal, source, cx)
            }
            OwnedSchemaBoundedArrayState::Entries if self.entry_depth == 0 => Err(self.diagnostic("artifact-envelope.history-expected-record", token)),
            OwnedSchemaBoundedArrayState::Entries => Err(self.diagnostic("artifact-envelope.history-entry-authority-missing", token)),
            OwnedSchemaBoundedArrayState::Complete => Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete),
            OwnedSchemaBoundedArrayState::Closing => Err(self.diagnostic("artifact-envelope.history-closing", token)),
        }
    }

    fn take_values(&mut self) -> Option<ArtifactHistoryLedger<T>> {
        if !matches!(self.state, OwnedSchemaBoundedArrayState::Complete) || self.taken || self.reservation.is_some() || self.active_decoder.is_some() || self.active_retirement.is_some() || self.rejected.is_some() {
            return None;
        }
        self.taken = true;
        self.values.take()
    }

    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress, admit_retained_clone_close};
        let diagnostic = OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-close-authority", offset: 0, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() };
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(diagnostic); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.state = OwnedSchemaBoundedArrayState::Closing;
        if let Some(owner) = self.active_decoder.as_mut() {
            if owner.terminal_is_empty() {
                drop(self.active_decoder.take());
                self.entry_depth = 0;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
            }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.close_step(child)?;
            admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original schema history entry decoder").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.active_retirement.is_some() { return artifact_retirement_box_close_step(&mut self.active_retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (diagnostic).with_native(error)); }
        if self.reservation.is_some() {
            let values = self.values.as_mut().ok_or(diagnostic)?;
            let reservation = self.reservation.take().expect("observed live history reservation");
            if let Err(reservation) = values.cancel_reservation(reservation) {
                self.reservation = Some(reservation);
                return Err(diagnostic);
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(value) = self.rejected.take() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            return match self.retirement_factory.as_ref().expect("history entry retains its factory").retire_owned(value, child) {
                Ok((owner, progress)) => { *self.active_retirement = Some(owner); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(diagnostic.with_progress(progress)); } Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, value)) => { *self.rejected = Some(value); Err(diagnostic.with_native(error)) }
            };
        }
        if let Some(values) = self.values.take() {
            *self.ledger_retirement = Some(ArtifactStoreHistoryLedgerRetirement::new(values, Arc::clone(self.retirement_factory.as_ref().expect("original ledger retains its entry factory"))));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(owner) = self.ledger_retirement.as_mut() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.close_step(child).map_err(|error| (diagnostic).with_native(error))?;
            admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original schema history ledger").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if owner.terminal_is_empty() { drop(self.ledger_retirement.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(factory) = self.retirement_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factories_close[0] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(factory) = self.decoder.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factories_close[1] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(slot) = self.factories_close.iter_mut().find(|slot| slot.is_some()) {
            let owner = slot.as_mut().expect("observed original schema factory");
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.step(child).map_err(|error| (diagnostic).with_native(error))?;
            admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original schema history factory").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if owner.terminal_is_empty() { drop(slot.take()); }
            self.taken = self.factories_close.iter().all(Option::is_none);
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        self.taken = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.values.is_none() && self.reservation.is_none() && self.rejected.is_none() && self.active_decoder.is_none() && self.active_retirement.is_none() && self.entry_depth == 0 && self.taken && self.ledger_retirement.is_none() && self.retirement_factory.is_none() && self.decoder.is_none() && self.factories_close.iter().all(Option::is_none)
    }
}

impl<T> Drop for OwnedSchemaBoundedArrayAuthority<T> {
    fn drop(&mut self) {
        assert!(
            self.values.is_none() && self.reservation.is_none() && self.rejected.is_none() && self.active_decoder.is_none() && self.active_retirement.is_none() && self.entry_depth == 0 && self.taken && self.ledger_retirement.is_none() && self.retirement_factory.is_none() && self.decoder.is_none() && self.factories_close.iter().all(Option::is_none),
            "bounded history array reached Drop before every exact entry owner was taken or cursor-retired"
        );
    }
}

impl OwnedSchemaEmptyArrayAuthority {
    pub fn new(path: OwnedSchemaPath) -> Self {
        Self { path, opened: false, complete: false }
    }

    pub fn accept(&mut self, token: OwnedSchemaToken, terminal: bool) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if self.complete {
            return Err(OwnedSchemaDecodeDiagnostic { code: "schema-json.trailing-empty-array-token", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        if !self.opened {
            if token.kind != OwnedSchemaTokenKind::ArrayStart || terminal {
                return Err(OwnedSchemaDecodeDiagnostic { code: "schema-json.expected-empty-array", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
            }
            self.opened = true;
            return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete);
        }
        if token.kind != OwnedSchemaTokenKind::ArrayEnd || !terminal {
            return Err(OwnedSchemaDecodeDiagnostic { code: "schema-json.expected-empty-array-end", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.complete = true;
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete
    }
}

pub const ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES: usize = 4_096;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES: usize = 262_144;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES: usize = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES / ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES * 4;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES + ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES + ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeDecodePage {
    inner: OwnedSchemaDecodePage,
}

/// 📚️ Seals exact caller-pre-admitted pages into the sole 12-field envelope cursor.
pub fn artifact_envelope_decode_record(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, pages: OwnedSchemaDecodePages) -> Result<OwnedSchemaRecordCursor, OwnedSchemaDecodePages> {
    let tokens = OwnedSchemaTokenCursor::try_new(operation, generation, pages)?;
    OwnedSchemaRecordCursor::try_new(artifact_envelope_owned_schema(), tokens).map_err(|(tokens, _)| tokens.pages)
}

impl ArtifactEnvelopeDecodePage {
    /// 📄️ Creates one definitionally shallow page from an already bounded producer buffer.
    pub fn try_from_array(bytes: [u8; ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES], len: usize) -> Result<Self, [u8; ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]> {
        if len > ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES {
            return Err(bytes);
        }
        Ok(Self { inner: OwnedSchemaDecodePage { bytes, len: len as u16 } })
    }

    /// 📄️ Constructs the fixed page inside a producer callback whose extent was already accepted.
    pub fn from_preflighted_array(bytes: [u8; ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES], len: usize) -> Self {
        debug_assert!(len <= ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
        Self { inner: OwnedSchemaDecodePage { bytes, len: len as u16 } }
    }

    /// 📥️ Moves this exact page into pre-reserved schema storage or returns it untouched.
    pub fn admit_into(self, pages: &mut OwnedSchemaDecodePages) -> Result<(), (OwnedSchemaDecodeAdmissionFault, Self)> {
        pages.admit_page(self.inner).map_err(|(fault, inner)| (fault, Self { inner }))
    }

    /// 📨️ Commits this page after the same mutable ingress authority ran its producer preflight.
    pub fn admit_preflighted_into(self, pages: &mut OwnedSchemaDecodePages) {
        pages.admit_preflighted_page(self.inner);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ArtifactEnvelopeDecodeState {
    Fields,
    ReleaseSuccess,
    ReleaseCancelled,
    ReleaseFault(OwnedSchemaDecodeDiagnostic),
    Transferred,
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

/// 🧬️ Persistent fixed-page envelope decode job; terminal outcomes are withheld until every nested owner is empty.
pub struct ArtifactEnvelopeDecodeAuthority<P, Mutation> {
    pub(crate) record: std::mem::ManuallyDrop<Option<OwnedSchemaRecordCursor>>,
    fields: std::mem::ManuallyDrop<Option<ArtifactEnvelopeFieldDecoderLease<P, Mutation>>>,
    pub(crate) field_registry: Arc<ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>>,
    pub(crate) field_ticket: ArtifactEnvelopeFieldDecoderTicket,
    field_returned: bool,
    maximum_field_close_byte_demand: usize,
    maximum_retained_field_close_bytes: usize,
    released_field_bytes: usize,
    pending_field: Option<(u16, OwnedSchemaToken, bool)>,
    pub(crate) state: ArtifactEnvelopeDecodeState,
    pub(crate) fault: semio_framework_job::RetainedPayloadBuilder,
    fault_cursor: usize,
}

impl<P, Mutation> ArtifactEnvelopeDecodeAuthority<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    /// 🏗️ Admits the field decoder before binding the shallow job lease and sealed record cursor.
    pub fn try_new(
        record: OwnedSchemaRecordCursor,
        field_registry: &Arc<ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>>,
        fields: Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>,
    ) -> Result<Self, (OwnedSchemaRecordCursor, ArtifactEnvelopeFieldDecoderRegistryFault, Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>)> {
        let maximum_field_close_byte_demand = fields.maximum_close_byte_demand();
        let maximum_retained_field_close_bytes = fields.maximum_retained_close_bytes();
        if maximum_field_close_byte_demand > ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
            || maximum_retained_field_close_bytes > ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES
            || maximum_field_close_byte_demand > maximum_retained_field_close_bytes
        {
            return Err((record, ArtifactEnvelopeFieldDecoderRegistryFault::CloseByteCapacity, fields));
        }
        let fields = match field_registry.try_admit(fields) {
            Ok(fields) => fields,
            Err((fault, fields)) => return Err((record, fault, fields)),
        };
        let field_ticket = fields.ticket();
        Ok(Self {
            record: std::mem::ManuallyDrop::new(Some(record)),
            fields: std::mem::ManuallyDrop::new(Some(fields)),
            field_registry: Arc::clone(field_registry),
            field_ticket,
            field_returned: false,
            maximum_field_close_byte_demand,
            maximum_retained_field_close_bytes,
            released_field_bytes: 0,
            pending_field: None,
            state: ArtifactEnvelopeDecodeState::Fields,
            fault: semio_framework_job::RetainedPayloadBuilder::new(semio_framework_job::JobPayloadStream::Fault),
            fault_cursor: 0,
        })
    }

    pub(crate) fn begin_release(&mut self, state: ArtifactEnvelopeDecodeState) {
        self.state = state;
    }

    fn record_release_fault(&mut self, diagnostic: OwnedSchemaDecodeDiagnostic) {
        if !matches!(self.state, ArtifactEnvelopeDecodeState::ReleaseFault(_)) {
            self.state = ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic);
        }
    }

    pub(crate) fn diagnostic(code: &'static str) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    fn checked_field_close_demand(&self, demand: semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        if demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) > self.maximum_field_close_byte_demand {
            return Err(Self::diagnostic("artifact-envelope.decode-close-demand-over-admitted-maximum"));
        }
        Ok(demand)
    }

    fn record_released_field_bytes(&mut self, released_bytes: usize) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        let released_field_bytes = self.released_field_bytes.checked_add(released_bytes).filter(|total| *total <= self.maximum_retained_field_close_bytes).ok_or_else(|| Self::diagnostic("artifact-envelope.decode-close-byte-ledger-overflow"))?;
        self.released_field_bytes = released_field_bytes;
        Ok(())
    }

    fn release_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        cx.set_stage("artifact-envelope-decode-close");
        if cx.fuel_exhausted()||cx.deadline_exceeded(){return Ok(())}
        semio_framework_job::InteractiveJob::close_step(self,cx)?;
        Ok(())
    }

    /// 🧹️ Transfers an unstarted exact decode owner into a retained close authority.
    pub fn reject(self, diagnostic: OwnedSchemaDecodeDiagnostic) -> Result<ArtifactEnvelopeDecodeRejected<P, Mutation>, Self> {
        if !matches!(self.state, ArtifactEnvelopeDecodeState::Fields) || self.record.is_none() || self.fields.is_none() || self.field_returned || self.pending_field.is_some() || !self.fault.terminal_is_empty() {
            return Err(self);
        }
        let mut source = std::mem::ManuallyDrop::new(self);
        let Some(record) = source.record.take() else { unreachable!("validated rejected record owner") };
        let Some(fields) = source.fields.take() else { unreachable!("validated rejected field owner") };
        let field_registry = Arc::clone(&source.field_registry);
        let field_ticket = source.field_ticket;
        let maximum_field_close_byte_demand = source.maximum_field_close_byte_demand;
        let maximum_retained_field_close_bytes = source.maximum_retained_field_close_bytes;
        let released_field_bytes = source.released_field_bytes;
        source.state = ArtifactEnvelopeDecodeState::Transferred;
        unsafe { std::mem::ManuallyDrop::drop(&mut source) };
        Ok(ArtifactEnvelopeDecodeRejected {
            record: std::mem::ManuallyDrop::new(Some(record)),
            fields: std::mem::ManuallyDrop::new(Some(fields)),
            field_registry,
            field_ticket,
            field_returned: false,
            maximum_field_close_byte_demand,
            maximum_retained_field_close_bytes,
            released_field_bytes,
            diagnostic,
        })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.fault.terminal_is_empty()
            && self.record.is_none()
            && self.fields.is_none()
            && self.field_returned
            && self.field_registry.ticket_reclaimed(self.field_ticket)
            && matches!(self.state, ArtifactEnvelopeDecodeState::Complete | ArtifactEnvelopeDecodeState::Cancelled | ArtifactEnvelopeDecodeState::Fault(_))
    }

    /// 📏️ Observes the next close turn: the leased field decoder as a child, then one fixed page, then the terminal handoff.
    pub fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        if !self.fault.terminal_is_empty() {
            let mut demand = self.fault.retirement_demands().map_err(|error| Self::diagnostic("artifact-envelope.fault-close-demand").with_native(error))?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| Self::diagnostic("artifact-envelope.fault-close-depth"))?;
            return Ok(demand);
        }
        if let Some(fields) = self.fields.as_ref() {
            let demand = match fields.with_owner(|owner| artifact_envelope_field_demands(owner, maximum_copy_bytes)) {
                Ok(demand) => demand?,
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }),
                Err(_) => return Err(Self::diagnostic("artifact-envelope.decode-field-lease-invalid")),
            };
            let demand = self.checked_field_close_demand(demand)?;
            return Ok(semio_framework_value::RetirementDemand { depth: demand.depth.checked_add(1).ok_or_else(|| Self::diagnostic("artifact-envelope.decode-close-depth-overflow"))?, ..demand });
        }
        if let Some(record) = self.record.as_ref().filter(|record| !record.terminal_is_empty()) {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: record.next_close_copy_byte_demand(), depth: 2, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })
    }

    pub fn released_field_bytes(&self) -> usize {
        self.released_field_bytes
    }
    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<semio_framework_job::InteractiveJobCloseStep,ValueError> {
        use semio_framework_job::InteractiveJobCloseStep as Close;
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Close::Pending { progress: empty }); }
        if matches!(self.state,ArtifactEnvelopeDecodeState::Fields){self.state=ArtifactEnvelopeDecodeState::ReleaseCancelled;}
        self.pending_field=None;
        if !self.fault.terminal_is_empty() {
            let demand = match self.close_demands(grant.maximum_copy_bytes) {
                Ok(demand) => demand,
                Err(diagnostic) => return Err(artifact_envelope_diagnostic_error(diagnostic)),
            };
            if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes || demand.depth > grant.maximum_depth { return Ok(Close::Pending { progress: empty }); }
            let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            return match self.fault.close_step_granted(child_grant) {
                Ok(step) => Ok(Close::Pending { progress: step.progress() }),
                Err(error) => Err(error),
            };
        }
        if self.fields.is_some() {
            if grant.maximum_items == 0 {
                return Ok(Close::Pending { progress: empty });
            }
            let terminal = match self.fields.as_ref().expect("closing job holds its field decoder lease").with_owner(|owner| owner.terminal_is_empty()) {
                Ok(terminal) => terminal,
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(Close::Blocked),
                Err(_) => {
                    self.record_release_fault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid"));
                    return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(empty));
                }
            };
            if terminal {
                if !self.fields.as_mut().expect("closing job holds its field decoder lease").return_now() {
                    return Ok(Close::Blocked);
                }
                drop(self.fields.take());
                self.field_returned = true;
                return Ok(Close::Pending { progress: RetainedCloneProgress { copied_items: 1, ..empty } });
            }
            let demand = match self.fields.as_ref().expect("closing job holds its field decoder lease").with_owner(|owner| artifact_envelope_field_demands(owner, grant.maximum_copy_bytes)) {
                Ok(Ok(demand)) => demand,
                Ok(Err(diagnostic)) => {
                    self.record_release_fault(diagnostic);
                    return Err(artifact_envelope_diagnostic_error(diagnostic));
                }
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(Close::Blocked),
                Err(_) => {
                    self.record_release_fault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid"));
                    return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(empty));
                }
            };
            let demand = match self.checked_field_close_demand(demand) {
                Ok(demand) => demand,
                Err(diagnostic) => {
                    self.record_release_fault(diagnostic);
                    return Err(artifact_envelope_diagnostic_error(diagnostic));
                }
            };
            if grant.maximum_depth <= demand.depth {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original decoder close refused").with_retained_progress(empty));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            if !artifact_retirement_grant_covers(child, demand) {
                return Ok(Close::Pending { progress: empty });
            }
            let close = self.fields.as_ref().expect("closing job holds its field decoder lease").with_owner(|owner| {
                let step = owner.close_step(child);
                (step, owner.terminal_is_empty())
            });
            return match close {
                Ok((Ok(step), terminal)) => {
                    let progress = step.progress();
                    if !progress.fits(child) || matches!(step, RetainedCloneStep::Complete(_)) && !terminal {
                        self.record_release_fault(Self::diagnostic("artifact-envelope.decode-close-over-budget"));
                        return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(progress));
                    }
                    if let Err(diagnostic) = self.record_released_field_bytes(progress.released_bytes) {
                        self.record_release_fault(diagnostic);
                        return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(progress));
                    }
                    Ok(Close::Pending { progress })
                }
                Ok((Err(diagnostic), _)) => {
                    let released=self.record_released_field_bytes(diagnostic.retained_progress.released_bytes);
                    self.record_release_fault(diagnostic);
                    if let Err(ledger)=released{self.record_release_fault(ledger.with_progress(diagnostic.retained_progress));}
                    Err(artifact_envelope_diagnostic_error(diagnostic))
                }
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => Ok(Close::Blocked),
                Err(_) => {
                    self.record_release_fault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid"));
                    Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(empty))
                }
            };
        }
        if !self.field_returned || !self.field_registry.ticket_reclaimed(self.field_ticket) {
            if self.field_returned {
                let _ = self.field_registry.reclaim_returned_ticket_now(self.field_ticket);
            }
            if !self.field_returned || !self.field_registry.ticket_reclaimed(self.field_ticket) {
                return Ok(Close::Blocked);
            }
        }
        if let Some(record) = self.record.as_mut() {
            let nonterminal = !record.terminal_is_empty();
            if grant.maximum_items == 0 || nonterminal && grant.maximum_copy_bytes < record.next_close_copy_byte_demand() {
                return Ok(Close::Pending { progress: empty });
            }
            if !nonterminal {
                drop(self.record.take());
                return Ok(Close::Pending { progress: RetainedCloneProgress { copied_items: 1, ..empty } });
            }
            if grant.maximum_depth < 2 {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original decoder close refused").with_retained_progress(empty));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            match record.close_step(child) {
                Ok(step) if step.progress().fits(child) => return Ok(Close::Pending { progress: step.progress() }),
                Ok(step) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(step.progress())),
                Err(error) => return Err(error),
            }
        }
        let before = self.state;
        self.state = match self.state {
            ArtifactEnvelopeDecodeState::ReleaseSuccess | ArtifactEnvelopeDecodeState::Complete => ArtifactEnvelopeDecodeState::Complete,
            ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic) | ArtifactEnvelopeDecodeState::Fault(diagnostic) => ArtifactEnvelopeDecodeState::Fault(diagnostic),
            _ => ArtifactEnvelopeDecodeState::Cancelled,
        };
        if ArtifactEnvelopeDecodeAuthority::terminal_is_empty(self) { Ok(Close::Complete { progress: RetainedCloneProgress { copied_items: usize::from(before != self.state), ..empty } }) } else { Ok(Close::Blocked) }
    }

    fn advance_decode(&mut self, cx: &mut semio_framework_job::StepContext<'_>) {
        match self.state {
            ArtifactEnvelopeDecodeState::Complete | ArtifactEnvelopeDecodeState::Cancelled | ArtifactEnvelopeDecodeState::Fault(_) => return,
            ArtifactEnvelopeDecodeState::Fields => {}
            _ => unreachable!("release states returned before decode"),
        }
        if cx.is_cancelled() {
            self.begin_release(ArtifactEnvelopeDecodeState::ReleaseCancelled);
            return ();
        }
        cx.set_stage("artifact-envelope-decode-fields");
        let Some(record) = self.record.as_mut() else {
            self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.missing-record-owner")));
            return ();
        };
        let Some(fields) = self.fields.as_ref() else {
            self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.missing-field-owner")));
            return ();
        };
        if let Some((field_id, token, terminal)) = self.pending_field {
            return match fields.with_owner(|owner| owner.accept_field_token(field_id, token, terminal, record, cx)) {
                Ok(Ok(ArtifactEnvelopeFieldDecodeStep::Pending)) => (),
                Ok(Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)) => {
                    self.pending_field = None;
                    ()
                }
                Ok(Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)) => {
                    self.pending_field = None;
                    ()
                }
                Ok(Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.field-published-record-before-outer-terminal")));
                    ()
                }
                Ok(Err(diagnostic)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic));
                    ()
                }
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => (),
                Err(_) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid")));
                    ()
                }
            };
        }
        match record.step(cx) {
            OwnedSchemaRecordStep::Pending => (),
            OwnedSchemaRecordStep::FieldToken { field_id, token, terminal } => {
                self.pending_field = Some((field_id, token, terminal));
                ()
            }
            OwnedSchemaRecordStep::Complete => match fields.with_owner(|owner| {
                let step = owner.finish_record(cx);
                let terminal = owner.terminal_is_empty();
                (step, terminal)
            }) {
                Ok((Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete), true)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseSuccess);
                    ()
                }
                Ok((Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete), false)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.record-complete-with-live-owner")));
                    ()
                }
                Ok((Ok(_), _)) => (),
                Ok((Err(diagnostic), _)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic));
                    ()
                }
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => (),
                Err(_) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid")));
                    ()
                }
            },
            OwnedSchemaRecordStep::Cancelled => {
                self.begin_release(ArtifactEnvelopeDecodeState::ReleaseCancelled);
                ()
            }
            OwnedSchemaRecordStep::Fault(diagnostic) => {
                self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic));
                ()
            }
        }
    }

}

impl<P, Mutation> semio_framework_job::InteractiveJob for ArtifactEnvelopeDecodeAuthority<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        let grant = cx.retained_grant();
        if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
        let before = cx.retained_progress();
        let state = self.state;
        if matches!(self.state,ArtifactEnvelopeDecodeState::ReleaseSuccess|ArtifactEnvelopeDecodeState::ReleaseCancelled|ArtifactEnvelopeDecodeState::ReleaseFault(_)){self.release_step(cx)?;}else{self.advance_decode(cx);}
        let outcome=match self.state {
            ArtifactEnvelopeDecodeState::Complete => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None),
            ArtifactEnvelopeDecodeState::Cancelled => semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx),
            ArtifactEnvelopeDecodeState::Fault(diagnostic) => {
                if !self.fault.advance_initialization(cx)? { return Ok(None); }
                if !self.fault.append_original(cx, diagnostic.code.as_bytes(), &mut self.fault_cursor)? { return Ok(None); }
                if !self.fault.seal(cx)? { return Ok(None); }
                semio_framework_job::JobOutcomeBorrow::admit_fault(cx, self.fault.published().expect("sealed original fault remains inside decoder"))
            }
            _ if before == cx.retained_progress() && state == self.state => Ok(None),
            _ => semio_framework_job::JobOutcomeBorrow::admit_yield(cx),
        };
        outcome.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Complete if self.state == ArtifactEnvelopeDecodeState::Complete => descriptor.complete(None, None),
            semio_framework_job::JobOutcomeKind::Cancelled if self.state == ArtifactEnvelopeDecodeState::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Fault if matches!(self.state, ArtifactEnvelopeDecodeState::Fault(_)) => descriptor.fault(self.fault.published().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original decoder fault payload is not published"))?),
            _ => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "decoder outcome descriptor lacks its original semantic result")),
        }
    }

    fn begin_close(&mut self) {
        if matches!(self.state, ArtifactEnvelopeDecodeState::Fields) {
            self.state = ArtifactEnvelopeDecodeState::ReleaseCancelled;
        }
        self.pending_field = None;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        let original=cx.retained_grant();
        let grant=RetainedCloneGrant{maximum_items:original.maximum_items.min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX)),..original};
        if cx.should_yield(){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let step=self.close_granted(grant).and_then(|step|step.admit(grant,ArtifactEnvelopeDecodeAuthority::terminal_is_empty(self))).map_err(|error|{cx.consume_fuel(error.retained_progress().copied_items as u64);semio_framework_job::OriginalTurnFailure::debit_original(error,cx)})?;
        cx.consume_fuel(step.progress().copied_items as u64);
        cx.consume_retained(step.progress()).map_err(semio_framework_job::OriginalTurnFailure::from)?;
        Ok(step)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0).map_err(artifact_envelope_diagnostic_error)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.close_demands(maximum_copy_bytes).map_err(artifact_envelope_diagnostic_error)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0).map_err(artifact_envelope_diagnostic_error)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0).map_err(artifact_envelope_diagnostic_error)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        ArtifactEnvelopeDecodeAuthority::terminal_is_empty(self)
    }
}

impl<P, Mutation> Drop for ArtifactEnvelopeDecodeAuthority<P, Mutation> {
    fn drop(&mut self) {
        if matches!(self.state, ArtifactEnvelopeDecodeState::Transferred) {
            assert!(std::thread::panicking() || (self.record.is_none() && self.fields.is_none() && self.pending_field.is_none() && !self.field_returned), "transferred artifact envelope decode source retained an owner after exact rejection handoff");
            return;
        }
        let terminal = self.fault.terminal_is_empty()
            && self.record.is_none()
            && self.fields.is_none()
            && self.field_returned
            && self.field_registry.ticket_reclaimed(self.field_ticket)
            && matches!(self.state, ArtifactEnvelopeDecodeState::Complete | ArtifactEnvelopeDecodeState::Cancelled | ArtifactEnvelopeDecodeState::Fault(_));
        assert!(std::thread::panicking() || (terminal), "artifact envelope decode authority reached Drop before terminal publication or retained close");
    }
}

pub struct ArtifactEnvelopeDecodeRejected<P, Mutation> {
    pub(crate) record: std::mem::ManuallyDrop<Option<OwnedSchemaRecordCursor>>,
    pub(crate) fields: std::mem::ManuallyDrop<Option<ArtifactEnvelopeFieldDecoderLease<P, Mutation>>>,
    pub(crate) field_registry: Arc<ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>>,
    field_ticket: ArtifactEnvelopeFieldDecoderTicket,
    field_returned: bool,
    maximum_field_close_byte_demand: usize,
    maximum_retained_field_close_bytes: usize,
    released_field_bytes: usize,
    pub(crate) diagnostic: OwnedSchemaDecodeDiagnostic,
}

impl<P, Mutation> ArtifactEnvelopeDecodeRejected<P, Mutation> {
    /// 📏️ Observes the leased field decoder as a child, then one fixed page; `None` is a contended lease.
    fn demands(&self, maximum_copy_bytes: usize) -> Result<Option<semio_framework_value::RetirementDemand>, ValueError> {
        if let Some(fields) = self.fields.as_ref() {
            let demand = match fields.with_owner(|owner| artifact_envelope_field_demands(owner, maximum_copy_bytes)) {
                Ok(demand) => demand.map_err(artifact_envelope_diagnostic_error)?,
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(None),
                Err(_) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field lease is invalid")),
            };
            if demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) > self.maximum_field_close_byte_demand {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field close demand exceeded its admitted maximum"));
            }
            return Ok(Some(semio_framework_value::RetirementDemand { depth: demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "artifact envelope rejected field depth overflow"))?, ..demand }));
        }
        if let Some(record) = self.record.as_ref().filter(|record| !record.terminal_is_empty()) {
            return Ok(Some(semio_framework_value::RetirementDemand { copy_bytes: record.next_close_copy_byte_demand(), depth: 2, ..Default::default() }));
        }
        Ok(Some(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }))
    }

    fn demand_axis(&self, maximum_copy_bytes: usize, axis: impl FnOnce(semio_framework_value::RetirementDemand) -> usize) -> Result<usize, ValueError> {
        Ok(axis(self.demands(maximum_copy_bytes)?.unwrap_or(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })))
    }
}

impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeDecodeRejected<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    /// 🧱️ A retained decode page is a GRANULE: it is released whole or not at all, so a turn
    /// that cannot pay a full page buys nothing. The empty record husk still completes under
    /// any grant — its last page has already been paid for.
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.fields.is_some() {
            let terminal = match self.fields.as_ref().expect("rejected decode retains its field lease").with_owner(|owner| owner.terminal_is_empty()) {
                Ok(terminal) => terminal,
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(RetainedCloneStep::Progress(empty)),
                Err(_) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field lease is invalid")),
            };
            if terminal {
                if !self.fields.as_mut().expect("rejected decode retains its field lease").return_now() {
                    return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field owner was returned twice"));
                }
                drop(self.fields.take());
                self.field_returned = true;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            let Some(demand) = self.demands(grant.maximum_copy_bytes)? else { return Ok(RetainedCloneStep::Progress(empty)) };
            if grant.maximum_depth < demand.depth {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "artifact envelope rejected field owner exceeds admitted depth"));
            }
            if !artifact_retirement_grant_covers(grant, demand) {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let (step, terminal) = match self.fields.as_ref().expect("rejected decode retains its field lease").with_owner(|owner| {
                let step = owner.close_step(child);
                (step, owner.terminal_is_empty())
            }) {
                Ok((Ok(step), terminal)) => (step, terminal),
                Ok((Err(diagnostic), _)) => return Err(artifact_envelope_diagnostic_error(diagnostic)),
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(RetainedCloneStep::Progress(empty)),
                Err(_) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field lease is invalid")),
            };
            let progress = step.progress();
            if !progress.fits(child) {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field owner exceeded its close grant"));
            }
            if matches!(step, RetainedCloneStep::Complete(_)) && !terminal {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field owner reported a false terminal"));
            }
            self.released_field_bytes = self
                .released_field_bytes
                .checked_add(progress.released_bytes)
                .filter(|total| *total <= self.maximum_retained_field_close_bytes)
                .ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field close ledger exceeded its admitted aggregate"))?;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.field_returned || !self.field_registry.ticket_reclaimed(self.field_ticket) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(record) = self.record.as_mut() {
            if record.terminal_is_empty() {
                drop(self.record.take());
                return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            if grant.maximum_copy_bytes < record.next_close_copy_byte_demand() {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            if grant.maximum_depth < 2 {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "artifact envelope rejected record exceeds admitted depth"));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = record.close_step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_progress(child, step.progress(), "artifact envelope rejected record")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.record.is_none() && self.fields.is_none() && self.field_returned && self.field_registry.ticket_reclaimed(self.field_ticket)
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.demand_axis(0, |demand| demand.copy_bytes) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { self.demand_axis(maximum_body_bytes, |demand| demand.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.demand_axis(0, |demand| demand.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.demand_axis(0, |demand| demand.depth) }
}

impl<P, Mutation> Drop for ArtifactEnvelopeDecodeRejected<P, Mutation> {
    fn drop(&mut self) {
        let _ = self.diagnostic;
        assert!(
            std::thread::panicking() || (self.record.is_none() && self.fields.is_none() && self.field_returned && self.field_registry.ticket_reclaimed(self.field_ticket)),
            "artifact envelope decode rejection reached Drop before every exact page owner was cursor-retired"
        );
    }
}

/// 🧹️ Exact close authority used when the shared field registry rejects admission
/// after the record pages and owner-supplied decoder have already been constructed.
pub struct ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation> {
    pub(crate) record: Option<OwnedSchemaRecordCursor>,
    pub(crate) fields: Option<Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>>,
    terminal: bool,
}

impl<P, Mutation> ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation> {
    pub fn new(record: OwnedSchemaRecordCursor, fields: Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>) -> Self {
        Self { record: Some(record), fields: Some(fields), terminal: false }
    }

    /// 📏️ Observes the unadmitted decoder as a child, the separate release of its terminal shell, then one fixed page.
    fn demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(fields) = self.fields.as_ref() {
            if fields.terminal_is_empty() { return Ok(semio_framework_value::RetirementDemand { release_bytes: size_of_val(fields.as_ref()), depth: 1, ..Default::default() }); }
            let demand = artifact_envelope_field_demands(fields.as_ref(), maximum_copy_bytes).map_err(artifact_envelope_diagnostic_error)?;
            return Ok(semio_framework_value::RetirementDemand { depth: demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "unadmitted envelope field decoder depth overflow"))?, ..demand });
        }
        if let Some(record) = self.record.as_ref().filter(|record| !record.terminal_is_empty()) {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: record.next_close_copy_byte_demand(), depth: 2, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })
    }
}

impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "unadmitted envelope rejection exceeds admitted depth"));
        }
        if !artifact_retirement_grant_covers(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if let Some(fields) = self.fields.as_mut() {
            if fields.terminal_is_empty() {
                drop(self.fields.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
            }
            let step = fields.close_step(child).map_err(artifact_envelope_diagnostic_error)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, fields.terminal_is_empty(), "unadmitted envelope field decoder")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(record) = self.record.as_mut() {
            if record.terminal_is_empty() {
                drop(self.record.take());
                self.terminal = true;
                return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            let step = record.close_step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_progress(child, step.progress(), "unadmitted envelope record")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.record.is_none() && self.fields.is_none()
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(self.demands(maximum_body_bytes)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P, Mutation> Drop for ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal && self.record.is_none() && self.fields.is_none()), "unadmitted envelope decode rejection reached Drop before every exact owner was cursor-retired");
    }
}

pub const ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeCompletedRecordTicket {
    index: u16,
    generation: u64,
}

impl ArtifactEnvelopeCompletedRecordTicket {
    pub fn index(self) -> usize {
        usize::from(self.index)
    }

    pub fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactEnvelopeCompletedRecordFault {
    Contended,
    Capacity,
    GenerationExhausted,
    Stale,
    NotReady,
    FalseTerminal,
}

/// 🎯 Atomic consumer for one completed envelope. Rejection returns the exact record to
/// its app-owned slot; no caller receives a stack-local owner on a fallible branch.
pub trait ArtifactEnvelopeCompletedRecordTarget<P, Mutation> {
    fn try_adopt_completed(&mut self, envelope: ArtifactEnvelope<P, Mutation>) -> Result<(), ArtifactEnvelope<P, Mutation>>;
}

/// 🎁️ Exact completed envelope owner retained by the app until one consumer takes it or close retires it.
pub trait ArtifactEnvelopeCompletedRecord<P, Mutation>: Send {
    fn try_publish_to(&mut self, target: &mut dyn ArtifactEnvelopeCompletedRecordTarget<P, Mutation>) -> bool;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError>;
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_depth_demand(&self) -> Result<usize, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

pub struct ArtifactEnvelopeCompletedRecordOwner<P, Mutation> {
    envelope: std::mem::ManuallyDrop<Option<ArtifactEnvelope<P, Mutation>>>,
    retirement: std::mem::ManuallyDrop<Option<ArtifactStoreEnvelopeRetirement<P, Mutation>>>,
    initial_snapshot_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    mutation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    factory_retirement: [Option<semio_framework_value::FactoryAuthority>; 2],
}

impl<P, Mutation> ArtifactEnvelopeCompletedRecordOwner<P, Mutation> {
    pub fn new(envelope: ArtifactEnvelope<P, Mutation>, initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self {
        Self { envelope: std::mem::ManuallyDrop::new(Some(envelope)), retirement: std::mem::ManuallyDrop::new(None), initial_snapshot_factory: std::mem::ManuallyDrop::new(Some(initial_snapshot_factory)), mutation_factory: std::mem::ManuallyDrop::new(Some(mutation_factory)), factory_retirement: std::array::from_fn(|_| None) }
    }

    fn ownership_is_empty(&self) -> bool {
        self.envelope.is_none() && self.retirement.is_none() && self.initial_snapshot_factory.is_none() && self.mutation_factory.is_none() && self.factory_retirement.iter().all(Option::is_none)
    }
}

impl<P, Mutation> ArtifactEnvelopeCompletedRecord<P, Mutation> for ArtifactEnvelopeCompletedRecordOwner<P, Mutation>
where
    P: Send + Sync + 'static,
    Mutation: Send + 'static,
{
    fn try_publish_to(&mut self, target: &mut dyn ArtifactEnvelopeCompletedRecordTarget<P, Mutation>) -> bool {
        if self.retirement.is_some() || self.initial_snapshot_factory.is_none() || self.mutation_factory.is_none() { return false; }
        let Some(envelope) = self.envelope.take() else { return false };
        match target.try_adopt_completed(envelope) {
            Ok(()) => true,
            Err(envelope) => { *self.envelope = Some(envelope); false }
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.ownership_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "completed record exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.retirement.is_some() {
            let step = artifact_retirement_owner_close(&mut self.retirement, grant)?;
            return Ok(if self.ownership_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        if self.envelope.is_some() {
            let snapshot = self.initial_snapshot_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "completed envelope lost its snapshot factory"))?;
            let mutation = self.mutation_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "completed envelope lost its mutation factory"))?;
            let envelope = self.envelope.take().expect("completed envelope remains retained before original retirement");
            *self.retirement = Some(ArtifactStoreEnvelopeRetirement::new(envelope, Arc::clone(snapshot), Arc::clone(mutation)));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(factory) = self.initial_snapshot_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_retirement[0] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(factory) = self.mutation_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_retirement[1] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        for index in 0..self.factory_retirement.len() {
            if let Some(factory) = self.factory_retirement[index].as_mut() {
                let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
                let step = factory.step(child)?;
                semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "completed original factory")?;
                if factory.terminal_is_empty() { self.factory_retirement[index] = None; }
                return Ok(if self.ownership_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
            }
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> { Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.ownership_is_empty() }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactEnvelopeCompletedRecordOwner<P, Mutation> {
    /// 📏️ Quotes the original inline envelope child and separately retained factory capabilities.
    fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.retirement.is_some() { return artifact_retirement_owner_demands(&self.retirement, maximum_copy_bytes); }
        if self.envelope.is_some() { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
        if self.initial_snapshot_factory.is_some() || self.mutation_factory.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        for factory in self.factory_retirement.iter().flatten() {
            let mut demand = factory.demands(maximum_copy_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "completed factory depth overflow"))?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
}

impl<P, Mutation> Drop for ArtifactEnvelopeCompletedRecordOwner<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.ownership_is_empty(), "completed envelope owner reached Drop before its original record and factories were empty");
    }
}

pub(crate) struct ArtifactEnvelopeCompletedRecordSlot<P, Mutation> {
    generation: u64,
    owner: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>>>,
}

pub(crate) struct ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation> {
    slots: Box<[ArtifactEnvelopeCompletedRecordSlot<P, Mutation>]>,
    free: [u16; ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY],
    free_len: usize,
    live: usize,
    occupied: u64,
    closing: u64,
}

impl<P, Mutation> Drop for ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation> {
    fn drop(&mut self) {
        assert!(
            std::thread::panicking() || (self.live == 0 && self.occupied == 0 && self.closing == 0 && self.slots.iter().all(|slot| slot.owner.is_none())),
            "completed envelope registry reached Drop before every exact record owner was consumed or cursor-retired"
        );
    }
}

/// 🗄️ Fixed app-owned completed-record registry; result publication never puts a deep envelope on a job stack.
pub struct ArtifactEnvelopeCompletedRecordRegistry<P, Mutation> {
    state: Mutex<ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>>,
}

impl<P, Mutation> ArtifactEnvelopeCompletedRecordRegistry<P, Mutation> {
    pub fn new() -> Arc<Self> {
        let slots = (0..ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY).map(|_| ArtifactEnvelopeCompletedRecordSlot { generation: 0, owner: std::mem::ManuallyDrop::new(None) }).collect::<Vec<_>>().into_boxed_slice();
        let mut free = [0; ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY];
        for (index, slot) in free.iter_mut().enumerate() {
            *slot = index as u16;
        }
        Arc::new(Self { state: Mutex::new(ArtifactEnvelopeCompletedRecordRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY, live: 0, occupied: 0, closing: 0 }) })
    }

    pub fn try_admit(&self, owner: Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>) -> Result<ArtifactEnvelopeCompletedRecordTicket, (ArtifactEnvelopeCompletedRecordFault, Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>)> {
        let mut state = match self.state.try_lock() {
            Ok(state) => state,
            Err(_) => return Err((ArtifactEnvelopeCompletedRecordFault::Contended, owner)),
        };
        if state.free_len == 0 {
            return Err((ArtifactEnvelopeCompletedRecordFault::Capacity, owner));
        }
        state.free_len -= 1;
        let index = usize::from(state.free[state.free_len]);
        let Some(generation) = state.slots[index].generation.checked_add(1) else {
            let free_index = state.free_len;
            state.free[free_index] = index as u16;
            state.free_len += 1;
            return Err((ArtifactEnvelopeCompletedRecordFault::GenerationExhausted, owner));
        };
        state.slots[index].generation = generation;
        *state.slots[index].owner = Some(owner);
        state.live += 1;
        state.occupied |= 1u64 << index;
        Ok(ArtifactEnvelopeCompletedRecordTicket { index: index as u16, generation })
    }

    fn validate(state: &ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>, ticket: ArtifactEnvelopeCompletedRecordTicket) -> Result<usize, ArtifactEnvelopeCompletedRecordFault> {
        let index = ticket.index();
        let Some(slot) = state.slots.get(index) else { return Err(ArtifactEnvelopeCompletedRecordFault::Stale) };
        if slot.generation != ticket.generation || slot.owner.is_none() {
            return Err(ArtifactEnvelopeCompletedRecordFault::Stale);
        }
        Ok(index)
    }

    fn release_slot(state: &mut ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>, index: usize) {
        let free_index = state.free_len;
        state.free[free_index] = index as u16;
        state.free_len += 1;
        state.live -= 1;
        state.occupied &= !(1u64 << index);
        state.closing &= !(1u64 << index);
    }

    pub fn try_publish_to(&self, ticket: ArtifactEnvelopeCompletedRecordTicket, target: &mut dyn ArtifactEnvelopeCompletedRecordTarget<P, Mutation>) -> Result<bool, ArtifactEnvelopeCompletedRecordFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        let index = Self::validate(&state, ticket)?;
        if state.closing & (1u64 << index) != 0 { return Err(ArtifactEnvelopeCompletedRecordFault::Stale); }
        let published = state.slots[index].owner.as_mut().is_some_and(|owner| owner.try_publish_to(target));
        if !published { return Ok(false); }
        state.closing |= 1u64 << index;
        Ok(true)
    }

    pub fn try_detach(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) -> Result<Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>, ArtifactEnvelopeCompletedRecordFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        let index = Self::validate(&state, ticket)?;
        let owner = state.slots[index].owner.take().expect("validated completed record remains established");
        Self::release_slot(&mut state, index);
        Ok(owner)
    }

    /// 🚫 Marks a rejected/cancelled consumer result for the app maintenance close pump.
    pub fn try_request_close(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) -> Result<(), ArtifactEnvelopeCompletedRecordFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        let index = Self::validate(&state, ticket)?;
        state.closing |= 1u64 << index;
        Ok(())
    }

    pub fn try_next_ticket(&self, cursor: &mut usize) -> Result<Option<ArtifactEnvelopeCompletedRecordTicket>, ArtifactEnvelopeCompletedRecordFault> {
        let state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        if state.live == 0 {
            *cursor = 0;
            return Ok(None);
        }
        let start = *cursor % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        let offset = state.occupied.rotate_right(start as u32).trailing_zeros() as usize;
        let index = (start + offset) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        *cursor = (index + 1) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        Ok(Some(ArtifactEnvelopeCompletedRecordTicket { index: index as u16, generation: state.slots[index].generation }))
    }

    /// 🧹 Returns only records explicitly handed back by a cancelled/rejected consumer.
    pub fn try_next_close_ticket(&self, cursor: &mut usize) -> Result<Option<ArtifactEnvelopeCompletedRecordTicket>, ArtifactEnvelopeCompletedRecordFault> {
        let state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        if state.closing == 0 {
            *cursor = 0;
            return Ok(None);
        }
        let start = *cursor % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        let offset = state.closing.rotate_right(start as u32).trailing_zeros() as usize;
        let index = (start + offset) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        *cursor = (index + 1) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        Ok(Some(ArtifactEnvelopeCompletedRecordTicket { index: index as u16, generation: state.slots[index].generation }))
    }

    pub fn try_detach_next(&self, cursor: &mut usize) -> Result<Option<Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>>, ArtifactEnvelopeCompletedRecordFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        if state.live == 0 {
            *cursor = 0;
            return Ok(None);
        }
        let start = *cursor % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        let offset = state.occupied.rotate_right(start as u32).trailing_zeros() as usize;
        let index = (start + offset) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        *cursor = (index + 1) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        let owner = state.slots[index].owner.take().expect("occupied completed-record bit retains exact owner");
        Self::release_slot(&mut state, index);
        Ok(Some(owner))
    }

    pub fn ticket_reclaimed(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) -> bool {
        self.state.try_lock().is_ok_and(|state| Self::validate(&state, ticket).is_err())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.try_lock().is_ok_and(|state| state.live == 0)
    }
}

/// 🎫️ Shallow publication cell for the exact completed-record ticket produced by one decode job.
pub struct ArtifactEnvelopeDecodeCompletion {
    index: std::sync::atomic::AtomicU16,
    generation: std::sync::atomic::AtomicU64,
}

impl ArtifactEnvelopeDecodeCompletion {
    pub fn new() -> Arc<Self> {
        Arc::new(Self { index: std::sync::atomic::AtomicU16::new(u16::MAX), generation: std::sync::atomic::AtomicU64::new(0) })
    }

    fn publish(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) -> Result<(), ArtifactEnvelopeCompletedRecordTicket> {
        if self.generation.compare_exchange(0, ticket.generation, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return Err(ticket);
        }
        self.index.store(ticket.index, std::sync::atomic::Ordering::Release);
        Ok(())
    }

    pub fn ticket(&self) -> Option<ArtifactEnvelopeCompletedRecordTicket> {
        let index = self.index.load(std::sync::atomic::Ordering::Acquire);
        let generation = self.generation.load(std::sync::atomic::Ordering::Acquire);
        (index != u16::MAX && generation != 0).then_some(ArtifactEnvelopeCompletedRecordTicket { index, generation })
    }
}

pub(crate) const ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS: &[OwnedSchemaFieldSpec] = &[
    OwnedSchemaFieldSpec { id: 1, key: "initialPack", required: true },
    OwnedSchemaFieldSpec { id: 2, key: "edits", required: true },
    OwnedSchemaFieldSpec { id: 3, key: "changes", required: true },
    OwnedSchemaFieldSpec { id: 4, key: "checkpoints", required: true },
    OwnedSchemaFieldSpec { id: 5, key: "alternatives", required: true },
];

pub(crate) struct ArtifactEnvelopeFreshSnapshotTarget<P> {
    next_generation: u64,
    reserved: Option<ArtifactEnvelopeFieldReservation>,
    value: Option<P>,
}

impl<P> ArtifactEnvelopeFreshSnapshotTarget<P> {
    pub(crate) fn new() -> Self {
        Self { next_generation: 0, reserved: None, value: None }
    }
}

impl<P: Send> ArtifactEnvelopeSnapshotFieldTarget<P> for ArtifactEnvelopeFreshSnapshotTarget<P> {
    fn reserve_snapshot(&mut self) -> Result<ArtifactEnvelopeFieldReservation, OwnedSchemaDecodeDiagnostic> {
        if self.reserved.is_some() || self.value.is_some() {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.snapshot-reservation-duplicate", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.next_generation = self.next_generation.checked_add(1).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.snapshot-reservation-generation", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        let reservation = ArtifactEnvelopeFieldReservation::new(1, self.next_generation).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.snapshot-reservation-invalid", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        self.reserved = Some(reservation);
        Ok(reservation)
    }

    fn publish_snapshot_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: P) {
        assert_eq!(self.reserved, Some(reservation), "snapshot publication requires its exact live reservation");
        assert!(self.value.is_none(), "snapshot publication cannot replace an established owner");
        self.reserved = None;
        self.value = Some(value);
    }

    fn cancel_snapshot_reservation(&mut self, reservation: ArtifactEnvelopeFieldReservation) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if self.reserved != Some(reservation) {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.snapshot-reservation-stale", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.reserved = None;
        Ok(())
    }
}

pub(crate) enum ArtifactEnvelopeFreshVcsActive<Mutation> {
    Snapshot { reservation: ArtifactEnvelopeFieldReservation, publishing: bool },
    Edits(OwnedSchemaBoundedArrayAuthority<Edit<Mutation>>),
    Changes(OwnedSchemaBoundedArrayAuthority<Change>),
    Checkpoints(OwnedSchemaBoundedArrayAuthority<Checkpoint>),
    Alternatives(OwnedSchemaBoundedArrayAuthority<Alternative>),
}

/// 🌱️ Concrete repository VCS authority for a paged envelope. It accepts one
/// owner-supplied snapshot and four fixed-capacity, one-entry-at-a-time history arrays.
pub struct ArtifactEnvelopeFreshVcsAuthority<P: Send + Sync + 'static, Mutation: Send + 'static> {
    pub(crate) cursor: OwnedSchemaNestedRecordCursor,
    pub(crate) snapshot: Option<Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>>>,
    pub(crate) snapshot_target: ArtifactEnvelopeFreshSnapshotTarget<P>,
    pub(crate) genesis_pack: Option<ArtifactGenesisPackCapture>,
    pub(crate) genesis_bytes: Option<Vec<u8>>,
    pub(crate) active: Option<ArtifactEnvelopeFreshVcsActive<Mutation>>,
    pub(crate) pending: Option<(OwnedSchemaToken, bool)>,
    pub(crate) edits: Option<ArtifactHistoryLedger<Edit<Mutation>>>,
    pub(crate) changes: Option<ArtifactHistoryLedger<Change>>,
    pub(crate) checkpoints: Option<ArtifactHistoryLedger<Checkpoint>>,
    pub(crate) alternatives: Option<ArtifactHistoryLedger<Alternative>>,
    pub(crate) value: Option<ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>>,
    pub(crate) retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub(crate) initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    pub(crate) mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
    pub(crate) edit_decoder: Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>>>,
    pub(crate) maximum_snapshot_close_byte_demand: usize,
    pub(crate) maximum_retained_snapshot_close_bytes: usize,
    pub(crate) terminal: bool,
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactEnvelopeFreshVcsAuthority<P, Mutation> {
    pub fn try_new(
        snapshot: Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>>,
        initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
        mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
        edit_decoder: Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>>>,
    ) -> Result<Self, Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>>> {
        let maximum_snapshot_close_byte_demand = snapshot.maximum_close_byte_demand();
        let maximum_retained_snapshot_close_bytes = snapshot.maximum_retained_close_bytes();
        if maximum_snapshot_close_byte_demand > ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
            || maximum_retained_snapshot_close_bytes > ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
            || maximum_snapshot_close_byte_demand > maximum_retained_snapshot_close_bytes
        {
            return Err(snapshot);
        }
        let cursor = match OwnedSchemaNestedRecordCursor::try_new(OwnedSchemaRecordSpec { fields: ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS }) {
            Ok(cursor) => cursor,
            Err(_) => return Err(snapshot),
        };
        Ok(Self {
            cursor,
            snapshot: Some(snapshot),
            snapshot_target: ArtifactEnvelopeFreshSnapshotTarget::new(),
            genesis_pack: None,
            genesis_bytes: None,
            active: None,
            pending: None,
            edits: None,
            changes: None,
            checkpoints: None,
            alternatives: None,
            value: None,
            retirement: None,
            initial_snapshot_factory,
            mutation_factory,
            edit_decoder,
            maximum_snapshot_close_byte_demand,
            maximum_retained_snapshot_close_bytes,
            terminal: false,
        })
    }

    pub(crate) fn diagnostic(code: &'static str) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: 0, line: 0, column: 0, path: OwnedSchemaPath::field("vcs").unwrap_or(OwnedSchemaPath::ROOT) , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    /// 🎯️ Routes one cursor-attributed token into the exact live nested owner. The owner is taken
    /// so the borrow checker admits the sibling-field writes, and EVERY exit restores it — an
    /// abandoned `?` here dropped a live history array and aborted the process
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn accept_active_token(&mut self, token: OwnedSchemaToken, _terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        let Some(mut active) = self.active.take() else {
            return Err(Self::diagnostic("artifact-envelope.fresh-vcs-field"));
        };
        return match &mut active {
            ArtifactEnvelopeFreshVcsActive::Snapshot { reservation, publishing } => {
                if cx.is_cancelled() { self.active = Some(active); return Err(Self::diagnostic("artifact-envelope.genesis-pack-cancelled")); }
                if cx.should_yield() { self.active = Some(active); return Ok(ArtifactEnvelopeFieldDecodeStep::Pending); }
                if !*publishing {
                    if self.genesis_pack.is_none() {
                        match ArtifactGenesisPackCapture::new(token, cx) { Ok(capture) => self.genesis_pack = Some(capture), Err(error) => { self.active = Some(active); return Err(error); } }
                    }
                    match self.genesis_pack.as_mut().expect("genesis capture remains retained").step(source, cx) {
                        Ok(true) => {},
                        Ok(false) => { self.active = Some(active); return Ok(ArtifactEnvelopeFieldDecodeStep::Pending); },
                        Err(error) => { self.active = Some(active); return Err(error); },
                    }
                }
                let Some(snapshot) = self.snapshot.as_mut() else {
                    self.active = Some(active);
                    return Err(Self::diagnostic("artifact-envelope.fresh-vcs-snapshot-owner"));
                };
                let step = if *publishing { snapshot.publish_reserved(&mut self.snapshot_target, *reservation, cx) } else { snapshot.accept_token(token, _terminal, source, cx) };
                let publishing_now = *publishing;
                match step {
                    Ok(ArtifactEnvelopeFieldDecodeStep::Pending) => {
                        self.active = Some(active);
                        Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete) if publishing_now => Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete),
                    Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete) => {
                        self.active = Some(active);
                        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) if publishing_now => Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete),
                    Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => {
                        *publishing = true;
                        self.active = Some(active);
                        Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete) => {
                        self.active = Some(active);
                        Err(Self::diagnostic("artifact-envelope.snapshot-published-record"))
                    }
                    Err(diagnostic) => {
                        self.active = Some(active);
                        Err(diagnostic)
                    }
                }
            }
            ArtifactEnvelopeFreshVcsActive::Edits(authority) => match authority.accept(token, _terminal, source, cx) {
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => match authority.take_values() {
                    Some(values) => {
                        self.edits = Some(values);
                        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    None => {
                        self.active = Some(active);
                        Err(Self::diagnostic("artifact-envelope.vcs-history-not-taken"))
                    }
                },
                Ok(step) => {
                    self.active = Some(active);
                    Ok(step)
                }
                Err(diagnostic) => {
                    self.active = Some(active);
                    Err(diagnostic)
                }
            },
            ArtifactEnvelopeFreshVcsActive::Changes(authority) => match authority.accept(token, _terminal, source, cx) {
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => match authority.take_values() {
                    Some(values) => {
                        self.changes = Some(values);
                        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    None => {
                        self.active = Some(active);
                        Err(Self::diagnostic("artifact-envelope.vcs-history-not-taken"))
                    }
                },
                Ok(step) => {
                    self.active = Some(active);
                    Ok(step)
                }
                Err(diagnostic) => {
                    self.active = Some(active);
                    Err(diagnostic)
                }
            },
            ArtifactEnvelopeFreshVcsActive::Checkpoints(authority) => match authority.accept(token, _terminal, source, cx) {
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => match authority.take_values() {
                    Some(values) => {
                        self.checkpoints = Some(values);
                        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    None => {
                        self.active = Some(active);
                        Err(Self::diagnostic("artifact-envelope.vcs-history-not-taken"))
                    }
                },
                Ok(step) => {
                    self.active = Some(active);
                    Ok(step)
                }
                Err(diagnostic) => {
                    self.active = Some(active);
                    Err(diagnostic)
                }
            },
            ArtifactEnvelopeFreshVcsActive::Alternatives(authority) => match authority.accept(token, _terminal, source, cx) {
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => match authority.take_values() {
                    Some(values) => {
                        self.alternatives = Some(values);
                        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    None => {
                        self.active = Some(active);
                        Err(Self::diagnostic("artifact-envelope.vcs-history-not-taken"))
                    }
                },
                Ok(step) => {
                    self.active = Some(active);
                    Ok(step)
                }
                Err(diagnostic) => {
                    self.active = Some(active);
                    Err(diagnostic)
                }
            },
        };
    }

    /// 🌱️ Installs the exact owner for one vcs field the moment its own nested cursor attributes a
    /// token to it. Every later token of that field is routed to this owner with the CURSOR's
    /// per-field terminal flag, never the parent record's — the parent only ever marks the vcs
    /// object's own closing brace, so an inner array could never see its terminal `]`.
    fn begin_vcs_field(&mut self, field_id: u16) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if field_id == 1 {
            let reservation = self.snapshot_target.reserve_snapshot()?;
            self.active = Some(ArtifactEnvelopeFreshVcsActive::Snapshot { reservation, publishing: false });
            return Ok(());
        }
        if !(2..=5).contains(&field_id) {
            return Err(Self::diagnostic("artifact-envelope.fresh-vcs-field"));
        }
        let path = OwnedSchemaPath::field(ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS[field_id as usize - 1].key).unwrap_or(OwnedSchemaPath::ROOT);
        self.active = Some(match field_id {
            2 => ArtifactEnvelopeFreshVcsActive::Edits(OwnedSchemaBoundedArrayAuthority::new(path, Arc::new(ArtifactStoreDecodedEditRetirementFactory { mutation_factory: self.mutation_factory.clone() }), self.edit_decoder.clone())),
            3 => ArtifactEnvelopeFreshVcsActive::Changes(OwnedSchemaBoundedArrayAuthority::new(path, Arc::new(ArtifactStoreChangeRetirementFactory), Arc::new(ArtifactRepositoryHistoryEntryDecoder::new()))),
            4 => ArtifactEnvelopeFreshVcsActive::Checkpoints(OwnedSchemaBoundedArrayAuthority::new(path, Arc::new(ArtifactStoreCheckpointRetirementFactory), Arc::new(ArtifactRepositoryHistoryEntryDecoder::new()))),
            _ => ArtifactEnvelopeFreshVcsActive::Alternatives(OwnedSchemaBoundedArrayAuthority::new(path, Arc::new(ArtifactStoreAlternativeRetirementFactory), Arc::new(ArtifactRepositoryHistoryEntryDecoder::new()))),
        });
        Ok(())
    }

    /// 🧾️ Assembles the decoded repository once the nested cursor reports the vcs record complete.
    fn finish_vcs_record(&mut self) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        let initial_snapshot = self.snapshot_target.value.take().ok_or_else(|| Self::diagnostic("artifact-envelope.fresh-vcs-missing-snapshot"))?;
        let edits = self.edits.take().ok_or_else(|| Self::diagnostic("artifact-envelope.vcs-missing-edits"))?;
        let changes = self.changes.take().ok_or_else(|| Self::diagnostic("artifact-envelope.vcs-missing-changes"))?;
        let checkpoints = self.checkpoints.take().ok_or_else(|| Self::diagnostic("artifact-envelope.vcs-missing-checkpoints"))?;
        let alternatives = self.alternatives.take().ok_or_else(|| Self::diagnostic("artifact-envelope.vcs-missing-alternatives"))?;
        let capture = self.genesis_pack.take().ok_or_else(|| Self::diagnostic("artifact-envelope.genesis-pack-missing"))?;
        let genesis = capture.admit(initial_snapshot);
        self.value = Some(ArtifactVcs { genesis, edits, changes, checkpoints, alternatives });
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn owners_terminal_empty(&self) -> bool {
        self.terminal
            && self.snapshot.is_none()
            && self.genesis_pack.is_none()
            && self.genesis_bytes.is_none()
            && self.snapshot_target.reserved.is_none()
            && self.snapshot_target.value.is_none()
            && self.active.is_none()
            && self.edits.is_none()
            && self.changes.is_none()
            && self.checkpoints.is_none()
            && self.alternatives.is_none()
            && self.value.is_none()
            && self.retirement.is_none()
    }

    /// 📏️ Observes the next close turn in the exact order `close_step` consumes owners: the unfinished snapshot,
    /// the retained child, the active history array, the four history ledgers, then the three original value owners.
    fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::RetirementDemand;
        let refusal = |_: ValueError| Self::diagnostic("artifact-envelope.fresh-vcs-close-demand");
        let deeper = |depth: usize| depth.checked_add(1).ok_or_else(|| Self::diagnostic("artifact-envelope.fresh-vcs-close-depth-overflow"));
        let capacity = |capacity_bytes: usize, depth: usize| RetirementDemand { capacity_bytes, depth, ..Default::default() };
        if let Some(snapshot) = self.snapshot.as_ref() {
            if snapshot.terminal_is_empty() {
                return Ok(RetirementDemand { release_bytes: size_of_val(snapshot.as_ref()), depth: 1, ..Default::default() });
            }
            let demand = RetirementDemand {
                copy_bytes: snapshot.next_close_copy_byte_demand()?,
                capacity_bytes: snapshot.next_close_capacity_byte_demand(maximum_copy_bytes)?,
                release_bytes: snapshot.next_close_release_byte_demand()?,
                depth: deeper(snapshot.next_close_depth_demand()?)?,
            };
            if demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) > self.maximum_snapshot_close_byte_demand {
                return Err(Self::diagnostic("artifact-envelope.snapshot-close-demand-over-admitted-maximum"));
            }
            return Ok(demand);
        }
        let demand = if let Some(retirement) = self.retirement.as_ref() {
            artifact_retirement_box_demands(retirement, maximum_copy_bytes).map_err(refusal)?
        } else if let Some(active) = self.active.as_ref() {
            if active.terminal_is_empty() {
                RetirementDemand { depth: 1, ..Default::default() }
            } else {
                let demand = active.close_demands(maximum_copy_bytes)?;
                RetirementDemand { depth: deeper(demand.depth)?, ..demand }
            }
        } else if let Some(ledger) = self.alternatives.as_ref() {
            Self::ledger_demand::<Alternative, ArtifactStoreAlternativeRetirementFactory>(ledger).map_err(refusal)?
        } else if let Some(ledger) = self.checkpoints.as_ref() {
            Self::ledger_demand::<Checkpoint, ArtifactStoreCheckpointRetirementFactory>(ledger).map_err(refusal)?
        } else if let Some(ledger) = self.changes.as_ref() {
            Self::ledger_demand::<Change, ArtifactStoreChangeRetirementFactory>(ledger).map_err(refusal)?
        } else if let Some(ledger) = self.edits.as_ref() {
            Self::ledger_demand::<Edit<Mutation>, ArtifactStoreDecodedEditRetirementFactory<Mutation>>(ledger).map_err(refusal)?
        } else if self.value.is_some() {
            capacity(size_of::<ArtifactStoreVcsRetirement<P, Mutation>>(), 2)
        } else if self.genesis_pack.is_some() {
            RetirementDemand { depth: 1, ..Default::default() }
        } else if self.genesis_bytes.is_some() {
            artifact_retirement_owned_birth_demands(&self.genesis_bytes).map_err(refusal)?
        } else if let Some(snapshot) = self.snapshot_target.value.as_ref() {
            capacity(self.initial_snapshot_factory.retirement_birth_bytes(snapshot), 2)
        } else {
            RetirementDemand::default()
        };
        if demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) > self.maximum_close_byte_demand() {
            return Err(Self::diagnostic("artifact-envelope.fresh-vcs-close-demand-over-admitted-maximum"));
        }
        Ok(demand)
    }

    /// 📚️ Prices handing one original history ledger to its retained ledger cursor; an empty ledger is merely dropped.
    fn ledger_demand<T: Send + 'static, F: semio_framework_value::FactoryPayloadRetirement>(ledger: &ArtifactHistoryLedger<T>) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if ledger.terminal_is_empty() && ledger.backing_is_empty() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand { capacity_bytes: artifact_history_ledger_birth::<T, F>()?, depth: 2, ..Default::default() })
    }

    /// 🤝️ Hands one original ledger to its retained cursor, or reports an empty ledger as one droppable item.
    fn hand_off_ledger<T: Send + 'static, F: ArtifactOwnedValueRetirementFactory<T> + semio_framework_value::FactoryPayloadRetirement>(ledger: &mut ArtifactHistoryLedger<T>, factory: F, slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if ledger.terminal_is_empty() && ledger.backing_is_empty() {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        artifact_history_ledger_handoff(ledger, factory, slot, grant)
    }
}

/// 📚️ Routes the four history-array closes through one exhaustive owner.
impl<Mutation: Send + 'static> ArtifactEnvelopeFreshVcsActive<Mutation> {
    fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Snapshot { .. } => true,
            Self::Edits(authority) => authority.terminal_is_empty(),
            Self::Changes(authority) => authority.terminal_is_empty(),
            Self::Checkpoints(authority) => authority.terminal_is_empty(),
            Self::Alternatives(authority) => authority.terminal_is_empty(),
        }
    }

    fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        match self {
            Self::Snapshot { .. } => Ok(Default::default()),
            Self::Edits(authority) => authority.close_demands(maximum_copy_bytes),
            Self::Changes(authority) => authority.close_demands(maximum_copy_bytes),
            Self::Checkpoints(authority) => authority.close_demands(maximum_copy_bytes),
            Self::Alternatives(authority) => authority.close_demands(maximum_copy_bytes),
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        match self {
            Self::Snapshot { .. } => Ok(RetainedCloneStep::Complete(Default::default())),
            Self::Edits(authority) => authority.close_step(grant),
            Self::Changes(authority) => authority.close_step(grant),
            Self::Checkpoints(authority) => authority.close_step(grant),
            Self::Alternatives(authority) => authority.close_step(grant),
        }
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactEnvelopeVcsFieldAuthority<P, Mutation> for ArtifactEnvelopeFreshVcsAuthority<P, Mutation> {
    fn accept_token(&mut self, token: OwnedSchemaToken, _terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        let (token, _terminal) = match self.pending {
            Some(pending) => pending,
            None => match self.cursor.accept(token, source) {
                OwnedSchemaNestedRecordStep::Pending => return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete),
                OwnedSchemaNestedRecordStep::FieldToken { field_id, token, terminal } => {
                    if self.active.is_none() {
                        self.begin_vcs_field(field_id)?;
                    }
                    (token, terminal)
                }
                OwnedSchemaNestedRecordStep::Complete => return self.finish_vcs_record(),
                OwnedSchemaNestedRecordStep::Fault(diagnostic) => return Err(diagnostic),
            },
        };
        self.pending = Some((token, _terminal));
        let outcome = self.accept_active_token(token, _terminal, source, cx);
        if !matches!(outcome, Ok(ArtifactEnvelopeFieldDecodeStep::Pending)) {
            self.pending = None;
        }
        outcome
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn ArtifactEnvelopeDecodedRecordTarget<P, Mutation>,
        reservation: ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if !self.snapshot.as_ref().is_some_and(|snapshot| snapshot.terminal_is_empty()) {
            return Err(Self::diagnostic("artifact-envelope.snapshot-publication-with-live-owner"));
        }
        drop(self.snapshot.take());
        let value = self.value.take().ok_or_else(|| Self::diagnostic("artifact-envelope.fresh-vcs-not-ready"))?;
        target.publish_vcs_reserved(reservation, value);
        self.terminal = true;
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        self.maximum_snapshot_close_byte_demand.max(ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES)
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        self.maximum_retained_snapshot_close_bytes + ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.depth)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        let empty = RetainedCloneProgress::default();
        let fault = |code: &'static str| Self::diagnostic(code);
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        self.pending = None;
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(fault("artifact-envelope.fresh-vcs-close-depth"));
        }
        if !artifact_retirement_grant_covers(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if let Some(snapshot) = self.snapshot.as_mut() {
            if !snapshot.terminal_is_empty() {
                let step = snapshot.close_step(child)?;
                if !step.progress().fits(child) {
                    return Err(fault("artifact-envelope.snapshot-close-over-budget").with_progress(step.progress()));
                }
                if matches!(step, RetainedCloneStep::Complete(_)) && !snapshot.terminal_is_empty() {
                    return Err(fault("artifact-envelope.snapshot-close-false-terminal").with_progress(step.progress()));
                }
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            drop(self.snapshot.take());
            if let Some(ArtifactEnvelopeFreshVcsActive::Snapshot { reservation, .. }) = self.active.as_ref() {
                let reservation = *reservation;
                self.active = None;
                self.snapshot_target.cancel_snapshot_reservation(reservation).map_err(|diagnostic|diagnostic.with_progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..empty}))?;
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
        }
        if self.retirement.is_some() {
            return artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (fault("artifact-envelope.fresh-vcs-retirement-fault")).with_native(error));
        }
        if let Some(active) = self.active.as_mut() {
            if active.terminal_is_empty() {
                drop(self.active.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            let step = active.close_step(child)?;
            if !step.progress().fits(child) || matches!(step, RetainedCloneStep::Complete(_)) && !active.terminal_is_empty() {
                return Err(fault("artifact-envelope.history-close-false-terminal").with_progress(step.progress()));
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(ledger) = self.alternatives.as_mut() {
            let step = Self::hand_off_ledger(ledger, ArtifactStoreAlternativeRetirementFactory, &mut self.retirement, grant).map_err(|error| (fault("artifact-envelope.history-ledger-handoff")).with_native(error))?;
            if step.progress().copied_items != 0 { drop(self.alternatives.take()); }
            return Ok(step);
        }
        if let Some(ledger) = self.checkpoints.as_mut() {
            let step = Self::hand_off_ledger(ledger, ArtifactStoreCheckpointRetirementFactory, &mut self.retirement, grant).map_err(|error| (fault("artifact-envelope.history-ledger-handoff")).with_native(error))?;
            if step.progress().copied_items != 0 { drop(self.checkpoints.take()); }
            return Ok(step);
        }
        if let Some(ledger) = self.changes.as_mut() {
            let step = Self::hand_off_ledger(ledger, ArtifactStoreChangeRetirementFactory, &mut self.retirement, grant).map_err(|error| (fault("artifact-envelope.history-ledger-handoff")).with_native(error))?;
            if step.progress().copied_items != 0 { drop(self.changes.take()); }
            return Ok(step);
        }
        if let Some(ledger) = self.edits.as_mut() {
            let factory = ArtifactStoreDecodedEditRetirementFactory { mutation_factory: self.mutation_factory.clone() };
            let step = Self::hand_off_ledger(ledger, factory, &mut self.retirement, grant).map_err(|error| (fault("artifact-envelope.history-ledger-handoff")).with_native(error))?;
            if step.progress().copied_items != 0 { drop(self.edits.take()); }
            return Ok(step);
        }
        if let Some(vcs) = self.value.take() {
            let (snapshots, mutations) = (self.initial_snapshot_factory.clone(), self.mutation_factory.clone());
            return match admit_artifact_retirement(vcs, child, |vcs| ArtifactStoreVcsRetirement::new(vcs, snapshots, mutations)) {
                Ok((owner, progress)) => {
                    self.retirement = Some(owner);
                    if !progress.fits(child) { return Err(fault("artifact-envelope.fresh-vcs-retirement-receipt").with_progress(progress)); }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, vcs)) => {
                    self.value = Some(vcs);
                    Err(fault("artifact-envelope.fresh-vcs-retirement-admission").with_native(error))
                }
            };
        }
        if let Some(capture) = self.genesis_pack.take() {
            self.genesis_bytes = Some(capture.bytes);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if self.genesis_bytes.is_some() {
            return artifact_retirement_admit_owned(&mut self.genesis_bytes, &mut self.retirement, grant).map_err(|error| (fault("artifact-envelope.genesis-pack-retirement")).with_native(error));
        }
        if let Some(snapshot) = self.snapshot_target.value.take() {
            return match self.initial_snapshot_factory.retire_owned(snapshot, child) {
                Ok((owner, progress)) => {
                    self.retirement = Some(owner);
                    if !progress.fits(child) { return Err(fault("artifact-envelope.fresh-vcs-retirement-receipt")); }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((_, snapshot)) => {
                    self.snapshot_target.value = Some(snapshot);
                    Err(fault("artifact-envelope.fresh-vcs-snapshot-retirement-admission"))
                }
            };
        }
        self.active = None;
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> Drop for ArtifactEnvelopeFreshVcsAuthority<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "fresh VCS authority reached Drop before every nested owner was published or cursor-retired");
    }
}

pub(crate) struct ArtifactEnvelopeFreshRecordTarget<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    next_generation: u64,
    reserved: Option<ArtifactEnvelopeFieldReservation>,
    vcs: Option<ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>>,
}

impl<P: Send, Mutation: Send> ArtifactEnvelopeFreshRecordTarget<P, Mutation> {
    fn new() -> Self {
        Self { next_generation: 0, reserved: None, vcs: None }
    }
}

impl<P: Send + Sync, Mutation: Send> ArtifactEnvelopeDecodedRecordTarget<P, Mutation> for ArtifactEnvelopeFreshRecordTarget<P, Mutation> {
    fn reserve_field(&mut self, field_id: u16) -> Result<ArtifactEnvelopeFieldReservation, OwnedSchemaDecodeDiagnostic> {
        if field_id != ARTIFACT_ENVELOPE_VCS_FIELD || self.reserved.is_some() || self.vcs.is_some() {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.fresh-field-reservation", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.next_generation = self.next_generation.checked_add(1).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.fresh-field-generation", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        let reservation = ArtifactEnvelopeFieldReservation::new(field_id, self.next_generation).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.fresh-field-reservation", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        self.reserved = Some(reservation);
        Ok(reservation)
    }

    fn publish_vcs_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>) {
        assert_eq!(self.reserved, Some(reservation), "VCS publication requires its exact live reservation");
        assert!(self.vcs.is_none(), "VCS publication cannot replace a live owner");
        self.reserved = None;
        self.vcs = Some(value);
    }

    fn publish_edit_messages_reserved(&mut self, _reservation: ArtifactEnvelopeFieldReservation, _value: ArtifactEditMessageLedger) {
        panic!("fresh envelope edit-message ledger is validated in place and never crosses the VCS target");
    }

    fn publish_conflicts_reserved(&mut self, _reservation: ArtifactEnvelopeFieldReservation, _value: Box<dyn ArtifactEnvelopeSprConflictAuthority>) {
        panic!("fresh envelope conflict ledger is validated in place and never crosses the VCS target");
    }

    fn cancel_field_reservation(&mut self, reservation: ArtifactEnvelopeFieldReservation) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if self.reserved != Some(reservation) {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.fresh-field-reservation-stale", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.reserved = None;
        Ok(())
    }
}

#[expect(clippy::large_enum_variant, reason = "The active decoder retains its admitted fixed string authority inline across bounded steps without a new allocation.")]
pub(crate) enum ArtifactEnvelopeFreshRecordActive<P, Mutation> {
    String { field_id: u16, authority: OwnedSchemaStringAuthority<256> },
    Vcs { reservation: ArtifactEnvelopeFieldReservation, authority: Box<dyn ArtifactEnvelopeVcsFieldAuthority<P, Mutation>>, publishing: bool },
    RejectedSnapshot { reservation: ArtifactEnvelopeFieldReservation, authority: Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>> },
    Empty { authority: OwnedSchemaEmptyArrayAuthority },
}

/// 📋️ Concrete 12-field catalog consumer for one fresh document envelope. All optional
/// fields must be absent or null; history/edit/conflict collections must be exact empty arrays.
pub struct ArtifactEnvelopeFreshFieldDecoder<P: Send + Sync + 'static, Mutation: Send + 'static> {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
    initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
    completed: Arc<ArtifactEnvelopeCompletedRecordRegistry<P, Mutation>>,
    completion: Arc<ArtifactEnvelopeDecodeCompletion>,
    target: ArtifactEnvelopeFreshRecordTarget<P, Mutation>,
    schema: Option<String>,
    id: Option<String>,
    active: Option<ArtifactEnvelopeFreshRecordActive<P, Mutation>>,
    pending_completed: Option<Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>>,
    pub(crate) active_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    maximum_vcs_close_byte_demand: usize,
    maximum_retained_vcs_close_bytes: usize,
    terminal: bool,
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactEnvelopeFreshFieldDecoder<P, Mutation> {
    pub fn new(
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
        initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
        mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
        completed: Arc<ArtifactEnvelopeCompletedRecordRegistry<P, Mutation>>,
        completion: Arc<ArtifactEnvelopeDecodeCompletion>,
    ) -> Self {
        let maximum_vcs_close_byte_demand = catalog.maximum_vcs_close_byte_demand();
        let maximum_retained_vcs_close_bytes = catalog.maximum_retained_vcs_close_bytes();
        Self {
            operation,
            generation,
            catalog,
            initial_snapshot_factory,
            mutation_factory,
            completed,
            completion,
            target: ArtifactEnvelopeFreshRecordTarget::new(),
            schema: None,
            id: None,
            active: None,
            pending_completed: None,
            active_retirement: None,
            maximum_vcs_close_byte_demand,
            maximum_retained_vcs_close_bytes,
            terminal: false,
        }
    }

    fn diagnostic(field: &'static str, code: &'static str) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: 0, line: 0, column: 0, path: OwnedSchemaPath::field(field).unwrap_or(OwnedSchemaPath::ROOT) , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    fn owners_terminal_empty(&self) -> bool {
        self.terminal && self.active.is_none() && self.pending_completed.is_none() && self.active_retirement.is_none() && self.target.reserved.is_none() && self.target.vcs.is_none() && self.schema.is_none() && self.id.is_none()
    }

    fn begin_field(&mut self, field_id: u16, token: OwnedSchemaToken) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        match field_id {
            ARTIFACT_ENVELOPE_SCHEMA_FIELD | ARTIFACT_ENVELOPE_ID_FIELD => {
                let field = if field_id == ARTIFACT_ENVELOPE_SCHEMA_FIELD { "schema" } else { "id" };
                let authority = OwnedSchemaStringAuthority::try_new(self.operation, self.generation, token, OwnedSchemaPath::field(field).unwrap_or(OwnedSchemaPath::ROOT)).map_err(|token| OwnedSchemaDecodeDiagnostic {
                    code: "artifact-envelope.expected-string",
                    offset: token.start,
                    line: 0,
                    column: 0,
                    path: OwnedSchemaPath::field(field).unwrap_or(OwnedSchemaPath::ROOT),
                 refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                self.active = Some(ArtifactEnvelopeFreshRecordActive::String { field_id, authority });
            }
            ARTIFACT_ENVELOPE_VCS_FIELD => {
                let reservation = self.target.reserve_field(field_id)?;
                match self.catalog.begin_vcs(self.operation, self.generation, OwnedSchemaPath::field("vcs").unwrap_or(OwnedSchemaPath::ROOT)) {
                    Ok(authority) => {
                        if authority.maximum_close_byte_demand() > self.maximum_vcs_close_byte_demand || authority.maximum_retained_close_bytes() > self.maximum_retained_vcs_close_bytes {
                            self.active = Some(ArtifactEnvelopeFreshRecordActive::Vcs { reservation, authority, publishing: false });
                            return Err(Self::diagnostic("vcs", "artifact-envelope.vcs-close-capacity-contract"));
                        }
                        self.active = Some(ArtifactEnvelopeFreshRecordActive::Vcs { reservation, authority, publishing: false });
                    }
                    Err(authority) => {
                        self.active = Some(ArtifactEnvelopeFreshRecordActive::RejectedSnapshot { reservation, authority });
                        return Err(Self::diagnostic("vcs", "artifact-envelope.vcs-construction-capacity"));
                    }
                }
            }
            ARTIFACT_ENVELOPE_EDIT_MESSAGES_FIELD => {
                self.active = Some(ArtifactEnvelopeFreshRecordActive::Empty { authority: OwnedSchemaEmptyArrayAuthority::new(OwnedSchemaPath::field("editMessages").unwrap_or(OwnedSchemaPath::ROOT)) });
            }
            ARTIFACT_ENVELOPE_CONFLICTS_FIELD => {
                self.active = Some(ArtifactEnvelopeFreshRecordActive::Empty { authority: OwnedSchemaEmptyArrayAuthority::new(OwnedSchemaPath::field("conflicts").unwrap_or(OwnedSchemaPath::ROOT)) });
            }
            _ => {
                if token.kind != OwnedSchemaTokenKind::Null {
                    return Err(Self::diagnostic("optional", "artifact-envelope.fresh-optional-field-must-be-null"));
                }
            }
        }
        Ok(())
    }

    /// 📏️ Observes the next close turn in the exact order `close_step` consumes owners: the unfinished field,
    /// the published completed record, the retained VCS child, the original VCS, then the identity strings.
    fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::RetirementDemand;
        let refusal = |_: ValueError| Self::diagnostic("$", "artifact-envelope.fresh-close-demand");
        let deeper = |depth: usize| depth.checked_add(1).ok_or_else(|| Self::diagnostic("$", "artifact-envelope.close-depth-overflow"));
        if let Some(active) = self.active.as_ref() {
            return Ok(match active {
                ArtifactEnvelopeFreshRecordActive::Vcs { authority, .. } if authority.terminal_is_empty() => RetirementDemand { release_bytes: size_of_val(authority.as_ref()), depth: 1, ..Default::default() },
                ArtifactEnvelopeFreshRecordActive::Vcs { authority, .. } => RetirementDemand {
                    copy_bytes: authority.next_close_copy_byte_demand()?,
                    capacity_bytes: authority.next_close_capacity_byte_demand(maximum_copy_bytes)?,
                    release_bytes: authority.next_close_release_byte_demand()?,
                    depth: deeper(authority.next_close_depth_demand()?)?,
                },
                ArtifactEnvelopeFreshRecordActive::RejectedSnapshot { authority, .. } if authority.terminal_is_empty() => RetirementDemand { release_bytes: size_of_val(authority.as_ref()), depth: 1, ..Default::default() },
                ArtifactEnvelopeFreshRecordActive::RejectedSnapshot { authority, .. } => RetirementDemand {
                    copy_bytes: authority.next_close_copy_byte_demand()?,
                    capacity_bytes: authority.next_close_capacity_byte_demand(maximum_copy_bytes)?,
                    release_bytes: authority.next_close_release_byte_demand()?,
                    depth: deeper(authority.next_close_depth_demand()?)?,
                },
                ArtifactEnvelopeFreshRecordActive::String { .. } | ArtifactEnvelopeFreshRecordActive::Empty { .. } => RetirementDemand { depth: 1, ..Default::default() },
            });
        }
        if let Some(owner) = self.pending_completed.as_ref() {
            if owner.terminal_is_empty() {
                return Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() });
            }
            return Ok(RetirementDemand {
                copy_bytes: owner.next_close_copy_byte_demand().map_err(refusal)?,
                capacity_bytes: owner.next_close_capacity_byte_demand(maximum_copy_bytes).map_err(refusal)?,
                release_bytes: owner.next_close_release_byte_demand().map_err(refusal)?,
                depth: deeper(owner.next_close_depth_demand().map_err(refusal)?)?,
            });
        }
        if let Some(retirement) = self.active_retirement.as_ref() {
            let demand = artifact_retirement_box_demands(retirement, maximum_copy_bytes).map_err(refusal)?;
            if demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) > self.maximum_close_byte_demand() {
                return Err(Self::diagnostic("vcs", "artifact-envelope.target-close-demand-over-admitted-maximum"));
            }
            return Ok(demand);
        }
        if self.target.vcs.is_some() {
            return Ok(RetirementDemand { capacity_bytes: size_of::<ArtifactStoreVcsRetirement<P, Mutation>>(), depth: 2, ..Default::default() });
        }
        let release_bytes = self.schema.as_ref().map_or(0, String::capacity).checked_add(self.id.as_ref().map_or(0, String::capacity)).ok_or_else(|| Self::diagnostic("$", "artifact-envelope.close-byte-demand-overflow"))?;
        Ok(RetirementDemand { release_bytes, depth: 1, ..Default::default() })
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactEnvelopeFieldDecoder<P, Mutation> for ArtifactEnvelopeFreshFieldDecoder<P, Mutation> {
    fn accept_field_token(&mut self, field_id: u16, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if self.active.is_none() {
            self.begin_field(field_id, token)?;
            if self.active.is_none() {
                return if terminal { Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) } else { Err(Self::diagnostic("optional", "artifact-envelope.fresh-null-not-terminal")) };
            }
        }
        let mut active = self.active.take().expect("fresh field authority was established");
        match &mut active {
            ArtifactEnvelopeFreshRecordActive::String { field_id, authority } => match authority.step(source, cx) {
                OwnedSchemaStringStep::Pending => {
                    self.active = Some(active);
                    Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                }
                OwnedSchemaStringStep::Complete => match authority.take_string() {
                    Some(value) => {
                        if *field_id == ARTIFACT_ENVELOPE_SCHEMA_FIELD {
                            self.schema = Some(value);
                        } else {
                            self.id = Some(value);
                        }
                        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
                    }
                    None => {
                        self.active = Some(active);
                        Err(Self::diagnostic("string", "artifact-envelope.string-publication"))
                    }
                },
                OwnedSchemaStringStep::Cancelled => {
                    self.active = Some(active);
                    Err(Self::diagnostic("string", "artifact-envelope.string-cancelled"))
                }
                OwnedSchemaStringStep::Fault(diagnostic) => {
                    self.active = Some(active);
                    Err(diagnostic)
                }
            },
            ArtifactEnvelopeFreshRecordActive::Vcs { reservation, authority, publishing } => {
                let publishing_now = *publishing;
                let step = if publishing_now { authority.publish_reserved(&mut self.target, *reservation, cx) } else { authority.accept_token(token, terminal, source, cx) };
                match step {
                    Ok(ArtifactEnvelopeFieldDecodeStep::Pending) => {
                        self.active = Some(active);
                        Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete | ArtifactEnvelopeFieldDecodeStep::TokenComplete) if publishing_now => Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete),
                    Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete) => {
                        self.active = Some(active);
                        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => {
                        *publishing = true;
                        self.active = Some(active);
                        Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete) => {
                        self.active = Some(active);
                        Err(Self::diagnostic("vcs", "artifact-envelope.vcs-published-record"))
                    }
                    Err(diagnostic) => {
                        self.active = Some(active);
                        Err(diagnostic)
                    }
                }
            }
            ArtifactEnvelopeFreshRecordActive::RejectedSnapshot { .. } => {
                self.active = Some(active);
                Err(Self::diagnostic("vcs", "artifact-envelope.vcs-construction-capacity"))
            }
            ArtifactEnvelopeFreshRecordActive::Empty { authority } => match authority.accept(token, terminal) {
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete),
                Ok(step) => {
                    self.active = Some(active);
                    Ok(step)
                }
                Err(diagnostic) => {
                    self.active = Some(active);
                    Err(diagnostic)
                }
            },
        }
    }

    fn finish_record(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if self.terminal {
            return Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete);
        }
        if self.pending_completed.is_none() {
            let schema = self.schema.take().ok_or_else(|| Self::diagnostic("schema", "artifact-envelope.missing-schema"))?;
            let id = self.id.take().ok_or_else(|| Self::diagnostic("id", "artifact-envelope.missing-id"))?;
            let vcs = self.target.vcs.take().ok_or_else(|| Self::diagnostic("vcs", "artifact-envelope.missing-vcs"))?;
            let envelope = ArtifactEnvelope::from_owners(ArtifactEnvelopeOwners {
                schema,
                id,
                vcs,
                backbone: None,
                active_alternative_id: None,
                cursor: None,
                dialect: None,
                migrated_from: None,
                owner: None,
                lanes: protocol::HistoryFoldIndex::new(),
                edit_messages: ArtifactEditMessageLedger::new(),
                conflicts: Vec::new(),
                transitions: Vec::new(),
                history_shape: crate::os_spr::HistoryShape::Document,
                open_transaction: None,
                viewer_checkpoint_id: None,
            });
            self.pending_completed = Some(Box::new(ArtifactEnvelopeCompletedRecordOwner::new(envelope, Arc::clone(&self.initial_snapshot_factory), Arc::clone(&self.mutation_factory))));
        }
        let owner = self.pending_completed.take().expect("fresh completed owner remains retained");
        match self.completed.try_admit(owner) {
            Ok(ticket) => {
                if self.completion.publish(ticket).is_err() {
                    self.completed.try_request_close(ticket).map_err(|_| Self::diagnostic("$", "artifact-envelope.completion-publication"))?;
                    return Err(Self::diagnostic("$", "artifact-envelope.completion-duplicate"));
                }
                self.terminal = true;
                Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete)
            }
            Err((_fault, owner)) => {
                self.pending_completed = Some(owner);
                Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
            }
        }
    }

    fn maximum_close_byte_demand(&self) -> usize {
        self.maximum_vcs_close_byte_demand.max(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        self.maximum_retained_vcs_close_bytes.saturating_add(ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.depth)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(Self::diagnostic("vcs", "artifact-envelope.fresh-close-depth"));
        }
        if !artifact_retirement_grant_covers(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if let Some(active) = self.active.as_mut() {
            let mut released_bytes = 0;
            match active {
                ArtifactEnvelopeFreshRecordActive::String { authority, .. } => authority.cancel(),
                ArtifactEnvelopeFreshRecordActive::Vcs { authority, .. } => {
                    if !authority.terminal_is_empty() {
                        let step = authority.close_step(child)?;
                        if !step.progress().fits(child) || matches!(step, RetainedCloneStep::Complete(_)) && !authority.terminal_is_empty() {
                            return Err(Self::diagnostic("vcs", "artifact-envelope.vcs-close-false-terminal").with_progress(step.progress()));
                        }
                        return Ok(RetainedCloneStep::Progress(step.progress()));
                    }
                    released_bytes = size_of_val(authority.as_ref());
                }
                ArtifactEnvelopeFreshRecordActive::RejectedSnapshot { authority, .. } => {
                    if !authority.terminal_is_empty() {
                        let step = authority.close_step(child)?;
                        if !step.progress().fits(child) || matches!(step, RetainedCloneStep::Complete(_)) && !authority.terminal_is_empty() {
                            return Err(Self::diagnostic("vcs", "artifact-envelope.snapshot-rejection-close-false-terminal").with_progress(step.progress()));
                        }
                        return Ok(RetainedCloneStep::Progress(step.progress()));
                    }
                    released_bytes = size_of_val(authority.as_ref());
                }
                ArtifactEnvelopeFreshRecordActive::Empty { .. } => {}
            }
            let reservation = match active {
                ArtifactEnvelopeFreshRecordActive::Vcs { reservation, .. } | ArtifactEnvelopeFreshRecordActive::RejectedSnapshot { reservation, .. } => Some(*reservation),
                _ => None,
            };
            self.active = None;
            if let Some(reservation) = reservation {
                self.target.cancel_field_reservation(reservation).map_err(|error|error.with_progress(RetainedCloneProgress{copied_items:1,released_bytes,..empty}))?;
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }));
        }
        if let Some(owner) = self.pending_completed.as_mut() {
            if owner.terminal_is_empty() {
                let released_bytes = size_of_val(owner.as_ref());
                drop(self.pending_completed.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }));
            }
            let step = owner.close_step(child).map_err(|error| (Self::diagnostic("$", "artifact-envelope.completed-close")).with_native(error))?;
            if !step.progress().fits(child) || matches!(step, RetainedCloneStep::Complete(_)) && !owner.terminal_is_empty() {
                return Err(Self::diagnostic("$", "artifact-envelope.completed-close-false-terminal").with_progress(step.progress()));
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.active_retirement.is_some() {
            return artifact_retirement_box_close_step(&mut self.active_retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (Self::diagnostic("vcs", "artifact-envelope.target-retirement")).with_native(error));
        }
        if let Some(vcs) = self.target.vcs.take() {
            let (snapshots, mutations) = (self.initial_snapshot_factory.clone(), self.mutation_factory.clone());
            return match admit_artifact_retirement(vcs, child, |vcs| ArtifactStoreVcsRetirement::new(vcs, snapshots, mutations)) {
                Ok((owner, progress)) => {
                    self.active_retirement = Some(owner);
                    if !progress.fits(child) { return Err(Self::diagnostic("vcs", "artifact-envelope.target-retirement-receipt").with_progress(progress)); }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, vcs)) => {
                    self.target.vcs = Some(vcs);
                    Err(Self::diagnostic("vcs", "artifact-envelope.target-retirement-admission").with_native(error))
                }
            };
        }
        let released_bytes = demand.release_bytes;
        self.schema = None;
        self.id = None;
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> Drop for ArtifactEnvelopeFreshFieldDecoder<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "fresh envelope field decoder reached Drop before its completed record or rejected owners reached terminal empty");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeWholeBufferIngressError;

impl std::fmt::Display for ArtifactEnvelopeWholeBufferIngressError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("artifact envelope JSON must be submitted as fixed pages to the app-owned persistent decoder")
    }
}

impl std::error::Error for ArtifactEnvelopeWholeBufferIngressError {}

/// 🚫️ Greenfield fail-closure for deleted whole-buffer envelope ingress; callers must use the app-owned paged submission handle.
pub fn reject_whole_buffer_artifact_envelope_ingress<P, Mutation>(_input: &str) -> Result<ArtifactEnvelope<P, Mutation>, ArtifactEnvelopeWholeBufferIngressError> {
    Err(ArtifactEnvelopeWholeBufferIngressError)
}
