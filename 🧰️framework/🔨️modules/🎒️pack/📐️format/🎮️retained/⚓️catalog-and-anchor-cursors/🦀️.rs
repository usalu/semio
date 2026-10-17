//! ⚓️ Catalog and anchor cursors of the retained Pack source: the catalog cause and fault, the paged kind helper and the anchor cursor with its default and drop guard. See [`super`].

use super::*;

//#region 🔖️RetainedCanonicalSource
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// 🗂️ Retains the actual borrowed producer cause independently of Catalog context.
pub enum RetainedPackCatalogCause{
    Value{kind:ValueRefusalKind,reason:&'static str},
    Paged(PagedListError),
    Allocation(PagedListAllocationError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// 🧭️ Carries source metadata alongside the Catalog context and byte offset.
pub struct RetainedPackCatalogFault {
    pub cause:RetainedPackCatalogCause,
    pub code: &'static str,
    pub offset: u64,
}

impl RetainedPackCatalogFault{
    /// 🪪️ Requires the producer to author the machine kind at its precise refusal site.
    pub const fn refusal(kind:ValueRefusalKind,code:&'static str,offset:u64)->Self{Self{cause:RetainedPackCatalogCause::Value{kind,reason:code},code,offset}}
    /// 📋️ Keeps the lower borrowed kind and reason without classifying its prose.
    pub const fn from_paged_refusal(error:PagedListError,code:&'static str,offset:u64)->Self{Self{cause:RetainedPackCatalogCause::Paged(error),code,offset}}
    /// 🧱️ Keeps the actual lower allocation witness with its original cause.
    pub const fn from_paged_allocation(error:PagedListAllocationError,code:&'static str,offset:u64)->Self{Self{cause:RetainedPackCatalogCause::Allocation(error),code,offset}}
    /// 🔎️ Projects the closed source cause without consulting its Catalog code.
    pub const fn kind(&self)->ValueRefusalKind{match self.cause{RetainedPackCatalogCause::Value{kind,..}=>kind,RetainedPackCatalogCause::Paged(error)=>retained_paged_kind(error.kind),RetainedPackCatalogCause::Allocation(error)=>retained_paged_kind(error.kind)}}
    /// 📤️ Projects into the canonical Pack owner while retaining source metadata.
    pub const fn into_pack_refusal(self,what:&'static str)->PackRefusal{match self.cause{
        RetainedPackCatalogCause::Value{kind,reason}=>PackRefusal::RetainedMalformed{kind,what,offset:self.offset,detail:reason},
        RetainedPackCatalogCause::Paged(error)=>PackRefusal::from_paged_refusal(error,what,self.offset),
        RetainedPackCatalogCause::Allocation(error)=>PackRefusal::from_paged_allocation(error,what,self.offset),
    }}
}

pub(super) const fn retained_paged_kind(kind:PagedListRefusalKind)->ValueRefusalKind{match kind{PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,PagedListRefusalKind::InvariantViolated=>ValueRefusalKind::InvariantViolated}}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedPackAnchorPhase {
    Collect,
    VerifyHeader(usize),
    VerifyFooter(usize),
    Ready,
    Closed,
}

pub struct RetainedPackAnchorCursor {
    pub(super) header: [u8; HEADER_SIZE],
    header_len: usize,
    footer_ring: [u8; FOOTER_SIZE],
    footer_len: usize,
    footer_head: usize,
    pub(super) total: u64,
    ordered_footer: [u8; FOOTER_SIZE],
    header_crc: semio_framework_replication_codec::Crc32cCursor,
    footer_crc: semio_framework_replication_codec::Crc32cCursor,
    pub(super) phase: RetainedPackAnchorPhase,
    pub(super) value: Option<Superblock>,
    handed_back: bool,
    pub(super) fault: Option<PackRefusal>,
}

impl RetainedPackAnchorCursor {
    pub fn new() -> Self {
        Self {
            header: [0; HEADER_SIZE],
            header_len: 0,
            footer_ring: [0; FOOTER_SIZE],
            footer_len: 0,
            footer_head: 0,
            total: 0,
            ordered_footer: [0; FOOTER_SIZE],
            header_crc: semio_framework_replication_codec::Crc32cCursor::new(),
            footer_crc: semio_framework_replication_codec::Crc32cCursor::new(),
            phase: RetainedPackAnchorPhase::Collect,
            value: None,
            handed_back: false,
            fault: None,
        }
    }

    pub fn grant(&mut self, event: Option<RetainedPackSourceEvent>) -> Result<bool, PackRefusal> {
        if let Some(fault) = &self.fault {
            return Err(fault.clone());
        }
        let result = self.grant_inner(event);
        match result {
            Ok(step) => Ok(step),
            Err(fault) => {
                self.fault = Some(fault);
                Err(self.fault.as_ref().expect("retained anchor first fault").clone())
            }
        }
    }

    pub(super) fn grant_inner(&mut self, event: Option<RetainedPackSourceEvent>) -> Result<bool, PackRefusal> {
        match self.phase {
            RetainedPackAnchorPhase::Collect => match event {
                Some(RetainedPackSourceEvent::Byte { offset, value }) => {
                    if offset != self.total {
                        return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated, what: "retained-anchor", offset, detail: "non-contiguous source event" });
                    }
                    if self.header_len < HEADER_SIZE {
                        self.header[self.header_len] = value;
                        self.header_len += 1;
                    }
                    self.footer_ring[self.footer_head] = value;
                    self.footer_head = (self.footer_head + 1) % FOOTER_SIZE;
                    self.footer_len = (self.footer_len + 1).min(FOOTER_SIZE);
                    self.total += 1;
                    Ok(false)
                }
                Some(RetainedPackSourceEvent::Complete { bytes, .. }) => {
                    if bytes != self.total || self.header_len != HEADER_SIZE || self.footer_len != FOOTER_SIZE {
                        return Err(PackRefusal::Truncated(self.total));
                    }
                    for index in 0..FOOTER_SIZE {
                        self.ordered_footer[index] = self.footer_ring[(self.footer_head + index) % FOOTER_SIZE];
                    }
                    self.phase = RetainedPackAnchorPhase::VerifyHeader(0);
                    Ok(false)
                }
                None => Ok(false),
            },
            RetainedPackAnchorPhase::VerifyHeader(index) => {
                if event.is_some() {
                    return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated, what: "retained-anchor", offset: self.total, detail: "source replay after completion" });
                }
                if index < 20 {
                    self.header_crc.update_page(&self.header[index..index + 1]);
                    self.phase = RetainedPackAnchorPhase::VerifyHeader(index + 1);
                    return Ok(false);
                }
                let stored = u32::from_le_bytes(self.header[20..24].try_into().expect("fixed header crc"));
                if stored != self.header_crc.finish() {
                    return Err(PackRefusal::ChecksumMismatch { segment: "header", offset: 20 });
                }
                if self.header[..8] != MAGIC {
                    return Err(PackRefusal::BadMagic);
                }
                if self.header[24..32] != [0; 8] {
                    return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "header", offset: 24, detail: "reserved header bytes are nonzero" });
                }
                self.phase = RetainedPackAnchorPhase::VerifyFooter(0);
                Ok(false)
            }
            RetainedPackAnchorPhase::VerifyFooter(index) => {
                if event.is_some() {
                    return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated, what: "retained-anchor", offset: self.total, detail: "source replay after completion" });
                }
                if index < 80 {
                    self.footer_crc.update_page(&self.ordered_footer[index..index + 1]);
                    self.phase = RetainedPackAnchorPhase::VerifyFooter(index + 1);
                    return Ok(false);
                }
                let stored = u32::from_le_bytes(self.ordered_footer[80..84].try_into().expect("fixed footer crc"));
                if stored != self.footer_crc.finish() {
                    return Err(PackRefusal::ChecksumMismatch { segment: "footer", offset: self.total - 4 });
                }
                if self.ordered_footer[..8] != FOOTER_MAGIC {
                    return Err(PackRefusal::BadMagic);
                }
                let version_major = u16::from_le_bytes(self.header[8..10].try_into().expect("header major"));
                let version_minor = u16::from_le_bytes(self.header[10..12].try_into().expect("header minor"));
                let required_flags = u32::from_le_bytes(self.header[12..16].try_into().expect("header flags"));
                let optional_flags = u32::from_le_bytes(self.header[16..20].try_into().expect("header optional flags"));
                if version_major != FORMAT_VERSION_MAJOR {
                    return Err(PackRefusal::UnsupportedVersion { major: version_major, minor: version_minor });
                }
                let unknown = required_flags & !REQUIRED_KNOWN_MASK;
                if unknown != 0 {
                    return Err(PackRefusal::UnknownRequiredFlags(unknown));
                }
                let footer_major = u16::from_le_bytes(self.ordered_footer[8..10].try_into().expect("footer major"));
                let footer_minor = u16::from_le_bytes(self.ordered_footer[10..12].try_into().expect("footer minor"));
                let footer_flags = u32::from_le_bytes(self.ordered_footer[12..16].try_into().expect("footer flags"));
                let file_len = u64::from_le_bytes(self.ordered_footer[32..40].try_into().expect("footer length"));
                let manifest_offset = u64::from_le_bytes(self.ordered_footer[16..24].try_into().expect("manifest offset"));
                let manifest_len = u64::from_le_bytes(self.ordered_footer[24..32].try_into().expect("manifest length"));
                let manifest_end = manifest_offset.checked_add(manifest_len).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"manifest span overflow"})?;
                if footer_major != version_major || footer_minor != version_minor || footer_flags != required_flags || file_len != self.total {
                    return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "footer", offset: self.total - FOOTER_SIZE as u64, detail: "anchor identity mismatch" });
                }
                if manifest_offset < HEADER_SIZE as u64 || manifest_len == 0 || manifest_end > self.total - FOOTER_SIZE as u64 {
                    return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvalidValue, what: "footer", offset: self.total - FOOTER_SIZE as u64, detail: "manifest span is outside the segment area" });
                }
                let mut content_hash = [0; 32];
                content_hash.copy_from_slice(&self.ordered_footer[40..72]);
                self.value = Some(Superblock {
                    header: Header { version_major, version_minor, required_flags, optional_flags },
                    footer: Footer {
                        version_major: footer_major,
                        version_minor: footer_minor,
                        required_flags: footer_flags,
                        manifest_offset,
                        manifest_len,
                        file_len,
                        content_hash: ContentHash(content_hash),
                        prev_footer_offset: u64::from_le_bytes(self.ordered_footer[72..80].try_into().expect("previous footer")),
                    },
                });
                self.phase = RetainedPackAnchorPhase::Ready;
                Ok(true)
            }
            RetainedPackAnchorPhase::Ready => Ok(true),
            RetainedPackAnchorPhase::Closed => Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated, what: "retained-anchor", offset: self.total, detail: "anchor is closed" }),
        }
    }

    pub fn take(&mut self) -> Option<Superblock> {
        if self.phase != RetainedPackAnchorPhase::Ready || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.value.take()
    }

    pub fn close_step(&mut self) -> RetainedPackCloseStep {
        self.value = None;
        self.phase = RetainedPackAnchorPhase::Closed;
        self.handed_back = true;
        RetainedPackCloseStep::Complete
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.value.is_none() && matches!(self.phase, RetainedPackAnchorPhase::Ready | RetainedPackAnchorPhase::Closed)
    }
}

impl Default for RetainedPackAnchorCursor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RetainedPackAnchorCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained pack anchors reached Drop before handback or close");
    }
}
//#endregion 🔖️RetainedCanonicalSource
