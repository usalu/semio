//! 📄️ Pages and source state of the retained Pack source: the close demands, the page record, the source events, progress, faults and the source cursor record. See [`super`].

use super::*;

pub fn retained_page_close_demands<T, const N: usize>(pages: &semio_framework_value::list::PagedList<T, N>) -> Result<RetirementDemand, ValueError> {
    let error = |error: PagedListError| ValueError::literal(ValueRefusalKind::OwnershipLimit, error.reason);
    if !pages.is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<T>(), depth: pages.next_pop_depth_demand().map_err(error)?, ..Default::default() }); }
    Ok(RetirementDemand { release_bytes: pages.next_release_allocation_bytes().map_err(error)?, depth: pages.next_release_depth_demand().map_err(error)?.max(1), ..Default::default() })
}

//#region 🔖️RetainedCanonicalSource
pub const RETAINED_PACK_PAGE_BYTES: usize = 4_096;

#[derive(Debug)]
pub struct RetainedPackPage {
    pub(super) bytes: [u8; RETAINED_PACK_PAGE_BYTES],
    pub(super) len: usize,
}

pub const RETAINED_PACK_MAXIMUM_PAGES: usize = isize::MAX as usize / size_of::<RetainedPackPage>();

pub(super) type RetainedPackPages = semio_framework_value::list::PagedList<RetainedPackPage, RETAINED_PACK_MAXIMUM_PAGES>;

impl RetainedPackPage {
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[expect(clippy::result_large_err, reason = "Invalid page length returns the caller-owned fixed byte array without allocating on rejection.")]
    pub fn try_from_array(bytes: [u8; RETAINED_PACK_PAGE_BYTES], len: usize) -> Result<Self, [u8; RETAINED_PACK_PAGE_BYTES]> {
        if len == 0 || len > RETAINED_PACK_PAGE_BYTES {
            return Err(bytes);
        }
        Ok(Self { bytes, len })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn into_array(self) -> ([u8; RETAINED_PACK_PAGE_BYTES], usize) {
        (self.bytes, self.len)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedPackSourceEvent {
    Byte { offset: u64, value: u8 },
    Complete { bytes: u64, pages: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackSourceProgress {
    pub admitted_pages: usize,
    pub admitted_bytes: usize,
    pub reserved_pages: usize,
    pub allocated_bytes: usize,
    pub consumed_bytes: u64,
    pub sealed: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedPackSourceAllocationStep {
    pub progressed: bool,
    pub allocated_bytes: usize,
}

/// 📥️ Holds a source-owned borrowed kind, reason and optional actual allocation witness.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackSourceFault { pub kind:ValueRefusalKind, pub reason:&'static str, pub allocated_bytes:Option<usize> }

impl RetainedPackSourceFault {
    /// 🪪️ Requires explicit producer authority at the refusal site.
    pub const fn refusal(kind:ValueRefusalKind,reason:&'static str)->Self{Self{kind,reason,allocated_bytes:None}}
    /// 📋️ Keeps the actual lower kind and static reason.
    pub const fn from_paged_refusal(error:PagedListError)->Self{Self::refusal(retained_paged_kind(error.kind),error.reason)}
    /// 🧱️ Keeps zero or nonzero actual lower allocation metadata.
    pub const fn from_paged_allocation(error:PagedListAllocationError)->Self{Self::allocation(retained_paged_kind(error.kind),error.reason,error.allocated_bytes)}
    /// 📏️ Retains the provider's byte witness independently of the machine kind.
    pub const fn allocation(kind:ValueRefusalKind,reason:&'static str,allocated_bytes:usize)->Self{Self{kind,reason,allocated_bytes:Some(allocated_bytes)}}
    /// 📤️ Projects into the canonical Pack owner without allocating.
    pub const fn into_pack_refusal(self,what:&'static str,offset:u64)->PackRefusal{match self.allocated_bytes{None=>PackRefusal::RetainedMalformed{kind:self.kind,what,offset,detail:self.reason},Some(allocated_bytes)=>PackRefusal::RetainedAllocation{kind:self.kind,allocated_bytes,what,offset,detail:self.reason}}}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackSourceAllocationError { pub allocated_bytes:usize, pub fault:RetainedPackSourceFault }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedPackCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Complete,
}

pub struct RetainedPackSourceCursor {
    pub(super) pages: RetainedPackPages,
    pub(super) maximum_pages: usize,
    pub(super) maximum_payload_bytes: usize,
    pub(super) maximum_allocation_bytes: usize,
    pub(super) admitted_bytes: usize,
    pub(super) page: usize,
    pub(super) byte: usize,
    pub(super) consumed: u64,
    pub(super) sealed: bool,
    pub(super) cancelled: bool,
    pub(super) completed: bool,
    pub(super) allocation_fault: Option<RetainedPackSourceFault>,
    pub(super) closed: bool,
}
//#endregion 🔖️RetainedCanonicalSource
