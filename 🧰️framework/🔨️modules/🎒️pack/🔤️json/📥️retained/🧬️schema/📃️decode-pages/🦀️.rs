//! 📃️ Decode pages: the fixed decode page constants, the page and its faults, the credits, the admission faults and the page ring. See [`super`].

use super::*;

pub const OWNED_SCHEMA_DECODE_PAGE_BYTES: usize = 4_096;

pub const OWNED_SCHEMA_DECODE_MAXIMUM_PAGES: usize = 4_096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnedSchemaDecodePage {
    pub(super) bytes: [u8; OWNED_SCHEMA_DECODE_PAGE_BYTES],
    pub(super) len: u16,
}

impl OwnedSchemaDecodePage {
    /// 🫙️ Fixed inline storage admits no heap allocation before the first source copy.
    pub const fn empty()->Self{Self{bytes:[0;OWNED_SCHEMA_DECODE_PAGE_BYTES],len:0}}
    /// 📄️ Wraps one bounded producer buffer or returns it untouched when the extent exceeds the inline capacity.
    pub fn try_from_array(bytes:[u8;OWNED_SCHEMA_DECODE_PAGE_BYTES],len:usize)->Result<Self,[u8;OWNED_SCHEMA_DECODE_PAGE_BYTES]>{if len>OWNED_SCHEMA_DECODE_PAGE_BYTES{return Err(bytes)}Ok(Self{bytes,len:len as u16})}
    /// 📄️ Wraps one producer buffer whose extent the caller already admitted against the inline capacity.
    pub fn from_preflighted_array(bytes:[u8;OWNED_SCHEMA_DECODE_PAGE_BYTES],len:usize)->Self{debug_assert!(len<=OWNED_SCHEMA_DECODE_PAGE_BYTES);Self{bytes,len:len as u16}}
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
    pub(super) page_count: usize,
    pub(super) byte_count: usize,
    pub(super) maximum_bytes: usize,
    sealed: bool,
}

impl OwnedSchemaDecodePages {
    /// 📐️ Names the indivisible boxed slot allocation independently of logical page bytes.
    pub fn allocation_byte_demand(&self)->usize{self.slots.len().checked_mul(size_of::<OwnedSchemaDecodePage>()).expect("admitted decode slots have a representable allocation")}
    /// 🪪️ Names the fixed slot allocation by its address, the stable identity of this page owner.
    pub fn slot_address(&self)->usize{self.slots.as_ptr() as usize}
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
