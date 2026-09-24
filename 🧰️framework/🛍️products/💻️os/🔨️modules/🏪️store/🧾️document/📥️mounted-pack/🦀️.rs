//! 📥️ Retained typed pack session: the artifact-neutral mounted ingress for one canonical `.spk`
//! document. It gates the artifact's own header before any semantic allocation, pages the canonical
//! stream into the framework retained cursors (source → segment → catalog → value) and hands every
//! value token to an artifact-owned [`RetainedTypedPackOwner`], one bounded unit of work per grant.

use super::mounted_pack_rt as mounted;

/// @emoji 🧬️ The artifact-specific half of a mounted pack session: consumes catalog/value events
/// straight into typed domain owners, never into a schema-erased record tree.
pub trait RetainedTypedPackOwner {
    type Value;
    fn accept(&mut self, token: mounted::RetainedValueToken, catalog: &mounted::RetainedPackCatalogCursor) -> Result<(), &'static str>;
    fn grant_symbol(&mut self, catalog: &mounted::RetainedPackCatalogCursor) -> Result<bool, &'static str>;
    fn take(&mut self) -> Option<Self::Value>;
    fn close_step(&mut self) -> bool;
    fn terminal_is_empty(&self) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedTypedPackPhase {
    Header,
    Ingress,
    Drive,
    Ready,
    Published,
    Closing,
    Closed,
}

/// @emoji 🧾️ One close grant's outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedTypedPackCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Complete,
}

/// @emoji 🧵️ Worker-owned mounted session over one exact byte count: `header` is matched byte by
/// byte and rejected before the owner or any cursor is allocated; every byte after it is handed
/// unchanged to the canonical retained page source.
pub struct RetainedTypedPackSession<O: RetainedTypedPackOwner> {
    phase: RetainedTypedPackPhase,
    header: Vec<u8>,
    header_len: usize,
    expected_bytes: usize,
    maximum_items: usize,
    maximum_depth: u16,
    owner_factory: fn() -> Result<O, &'static str>,
    page: [u8; mounted::RETAINED_PACK_PAGE_BYTES],
    page_len: usize,
    admitted: usize,
    source: std::mem::ManuallyDrop<Option<mounted::RetainedPackSourceCursor>>,
    anchor: std::mem::ManuallyDrop<Option<mounted::RetainedPackAnchorCursor>>,
    segment: std::mem::ManuallyDrop<Option<mounted::RetainedPackSegmentCursor>>,
    catalog: std::mem::ManuallyDrop<Option<mounted::RetainedPackCatalogCursor>>,
    value: std::mem::ManuallyDrop<Option<mounted::RetainedValueCursor>>,
    typed: std::mem::ManuallyDrop<Option<O>>,
    catalog_value: std::mem::ManuallyDrop<Option<mounted::RetainedPackCatalog>>,
    document_byte: Option<(u64, u8)>,
    source_complete: bool,
    segment_complete: bool,
    anchor_ready: bool,
    catalog_complete: bool,
    value_sealed: bool,
    value_complete: bool,
}

impl<O: RetainedTypedPackOwner> RetainedTypedPackSession<O> {
    /// @emoji 🚪️ Preflights exact credits only; nothing semantic exists until `header` has passed.
    pub fn new(header: Vec<u8>, expected_bytes: usize, maximum_items: usize, maximum_depth: u16, owner_factory: fn() -> Result<O, &'static str>) -> Result<Self, &'static str> {
        if expected_bytes <= header.len() || expected_bytes > super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES || maximum_items == 0 || maximum_depth == 0 {
            return Err("mounted-pack.exact-credits");
        }
        Ok(Self {
            phase: if header.is_empty() { RetainedTypedPackPhase::Ingress } else { RetainedTypedPackPhase::Header },
            header,
            header_len: 0,
            expected_bytes,
            maximum_items,
            maximum_depth,
            owner_factory,
            page: [0; mounted::RETAINED_PACK_PAGE_BYTES],
            page_len: 0,
            admitted: 0,
            source: std::mem::ManuallyDrop::new(None),
            anchor: std::mem::ManuallyDrop::new(None),
            segment: std::mem::ManuallyDrop::new(None),
            catalog: std::mem::ManuallyDrop::new(None),
            value: std::mem::ManuallyDrop::new(None),
            typed: std::mem::ManuallyDrop::new(None),
            catalog_value: std::mem::ManuallyDrop::new(None),
            document_byte: None,
            source_complete: false,
            segment_complete: false,
            anchor_ready: false,
            catalog_complete: false,
            value_sealed: false,
            value_complete: false,
        })
    }

    fn allocate_after_header(&mut self) -> Result<(), &'static str> {
        let canonical = self.expected_bytes - self.header.len();
        let pages = canonical.div_ceil(mounted::RETAINED_PACK_PAGE_BYTES);
        let maximum_symbols = self.maximum_items.min(u32::MAX as usize) as u32;
        let decoded = super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES as u64;
        let limits = || mounted::PackLimits { max_file_len: decoded, max_segment_len: decoded, max_symbols: maximum_symbols, max_depth: self.maximum_depth, max_items: self.maximum_items as u64, max_total_alloc: decoded };
        let maximum_source_allocation_bytes = super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES.checked_mul(4).ok_or("mounted-pack.source-allocation-credits")?;
        *self.source = Some(mounted::RetainedPackSourceCursor::try_new(pages, canonical, maximum_source_allocation_bytes)?);
        *self.anchor = Some(mounted::RetainedPackAnchorCursor::new());
        *self.segment = Some(mounted::RetainedPackSegmentCursor::try_new(limits(), super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES).map_err(|_| "mounted-pack.segment-preflight")?);
        *self.catalog = Some(
            mounted::RetainedPackCatalogCursor::try_new(limits(), maximum_symbols as usize, decoded as usize, decoded as usize, self.maximum_items, super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES)
                .map_err(|_| "mounted-pack.catalog-preflight")?,
        );
        *self.value = Some(mounted::RetainedValueCursor::try_new(limits(), super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES).map_err(|_| "mounted-pack.value-preflight")?);
        *self.typed = Some((self.owner_factory)()?);
        self.phase = RetainedTypedPackPhase::Ingress;
        Ok(())
    }

    /// @emoji 📥️ Admits one byte; a refused byte is handed back unchanged.
    pub fn admit_byte(&mut self, value: u8) -> Result<(), u8> {
        if !matches!(self.phase, RetainedTypedPackPhase::Header | RetainedTypedPackPhase::Ingress) || self.admitted == self.expected_bytes {
            return Err(value);
        }
        if self.header_len < self.header.len() {
            if value != self.header[self.header_len] {
                return Err(value);
            }
            self.header_len += 1;
            self.admitted += 1;
            if self.header_len == self.header.len() && self.allocate_after_header().is_err() {
                self.admitted -= 1;
                self.header_len -= 1;
                return Err(value);
            }
            return Ok(());
        }
        if self.source.is_none() && self.allocate_after_header().is_err() {
            return Err(value);
        }
        if !self.source.as_ref().is_some_and(mounted::RetainedPackSourceCursor::has_reserved_page) {
            return Err(value);
        }
        self.page[self.page_len] = value;
        self.page_len += 1;
        self.admitted += 1;
        if self.page_len == mounted::RETAINED_PACK_PAGE_BYTES && self.flush_page().is_err() {
            self.admitted -= 1;
            return Err(value);
        }
        Ok(())
    }

    fn flush_page(&mut self) -> Result<(), &'static str> {
        if self.page_len == 0 {
            return Ok(());
        }
        let len = self.page_len;
        let source = self.source.as_mut().ok_or("mounted-pack.source-owner")?;
        source.preflight_page(len)?;
        let page = mounted::RetainedPackPage::try_from_array(std::mem::replace(&mut self.page, [0; mounted::RETAINED_PACK_PAGE_BYTES]), len).map_err(|_| "mounted-pack.page-owner")?;
        self.page_len = 0;
        if let Err(page) = source.admit_page(page) {
            (self.page, self.page_len) = page.into_array();
            return Err("mounted-pack.producer-handback");
        }
        Ok(())
    }

    /// @emoji 🔏️ Seals ingress after exactly `expected_bytes`.
    pub fn seal(&mut self) -> Result<(), &'static str> {
        if self.admitted != self.expected_bytes || self.header_len != self.header.len() {
            return Err("mounted-pack.exact-byte-seal");
        }
        self.flush_page()?;
        self.source.as_mut().ok_or("mounted-pack.source-owner")?.seal()?;
        self.phase = RetainedTypedPackPhase::Drive;
        Ok(())
    }

    /// @emoji ⏱️ One bounded unit of retained decode work; `true` once the typed value is ready.
    pub fn grant(&mut self) -> Result<bool, &'static str> {
        if matches!(self.phase, RetainedTypedPackPhase::Ready | RetainedTypedPackPhase::Published) {
            return Ok(true);
        }
        if self.phase != RetainedTypedPackPhase::Drive {
            return Err("mounted-pack.missing-seal");
        }
        if self.next_retained_allocation_bytes()?.is_some() {
            return Ok(false);
        }
        if self.typed.as_mut().ok_or("mounted-pack.typed-owner")?.grant_symbol(self.catalog.as_ref().ok_or("mounted-pack.catalog-owner")?)? {
            return Ok(false);
        }
        if let Some((index, byte)) = self.document_byte {
            if self.value.as_ref().ok_or("mounted-pack.value-owner")?.ingress_ready() {
                self.document_byte = None;
                self.value.as_mut().expect("mounted value cursor retained").admit_byte(index, byte).map_err(|_| "mounted-pack.value-backpressure")?;
                return Ok(false);
            }
        }
        if !self.value_complete {
            if let Some(token) = self.value.as_mut().ok_or("mounted-pack.value-owner")?.grant().map_err(|_| "mounted-pack.value-malformed")? {
                self.value_complete = matches!(token, mounted::RetainedValueToken::Complete { .. });
                self.typed.as_mut().expect("mounted typed owner retained").accept(token, self.catalog.as_ref().expect("mounted catalog retained"))?;
                return Ok(false);
            }
        }
        if self.catalog_complete && !self.value_sealed {
            let bytes = self.catalog.as_ref().expect("mounted catalog retained").document_bytes();
            self.value.as_mut().expect("mounted value cursor retained").seal(bytes).map_err(|_| "mounted-pack.value-seal")?;
            self.value_sealed = true;
            return Ok(false);
        }
        if self.catalog.as_ref().is_some_and(mounted::RetainedPackCatalogCursor::has_pending_input) {
            let event = self.catalog.as_mut().expect("mounted catalog retained").grant().map_err(|_| "mounted-pack.catalog-malformed")?;
            self.catalog_event(event);
            return Ok(false);
        }
        if self.document_byte.is_none() && !self.segment_complete && (self.segment.as_ref().ok_or("mounted-pack.segment-owner")?.preflight().is_err() || self.source_complete) {
            if let Some(event) = self.segment.as_mut().expect("mounted segment retained").grant().map_err(|_| "mounted-pack.segment-malformed")? {
                self.segment_complete = matches!(event, mounted::RetainedPackSegmentEvent::PackComplete { .. });
                let catalog = self.catalog.as_mut().expect("mounted catalog retained");
                catalog.admit(event).map_err(|_| "mounted-pack.catalog-backpressure")?;
                let event = catalog.grant().map_err(|_| "mounted-pack.catalog-malformed")?;
                self.catalog_event(event);
                return Ok(false);
            }
        }
        if !self.source_complete && self.segment.as_ref().expect("mounted segment retained").preflight().is_ok() {
            if let Some(event) = self.source.as_mut().ok_or("mounted-pack.source-owner")?.grant()? {
                self.source_complete = matches!(event, mounted::RetainedPackSourceEvent::Complete { .. });
                self.anchor.as_mut().expect("mounted anchor retained").grant(Some(event)).map_err(|_| "mounted-pack.anchor-malformed")?;
                self.segment.as_mut().expect("mounted segment retained").admit(event).map_err(|_| "mounted-pack.segment-handback")?;
                return Ok(false);
            }
        }
        if self.source_complete && !self.anchor_ready {
            self.anchor_ready = self.anchor.as_mut().expect("mounted anchor retained").grant(None).map_err(|_| "mounted-pack.anchor-malformed")?;
            return Ok(false);
        }
        if self.anchor_ready && self.catalog_complete && self.value_complete && self.catalog_value.is_none() {
            let superblock = self.anchor.as_mut().expect("mounted anchor retained").take().ok_or("mounted-pack.anchor-handoff")?;
            *self.catalog_value = self.catalog.as_mut().expect("mounted catalog retained").take(superblock).map_err(|_| "mounted-pack.catalog-validation")?;
            self.phase = RetainedTypedPackPhase::Ready;
            return Ok(true);
        }
        Ok(false)
    }

    fn catalog_event(&mut self, event: Option<mounted::RetainedPackCatalogEvent>) {
        match event {
            Some(mounted::RetainedPackCatalogEvent::DocumentByte { index, value, .. }) => self.document_byte = Some((index, value)),
            Some(mounted::RetainedPackCatalogEvent::Complete) => self.catalog_complete = true,
            _ => {}
        }
    }

    /// @emoji 🤝️ Hands the typed value over exactly once.
    pub fn take(&mut self) -> Option<O::Value> {
        if self.phase != RetainedTypedPackPhase::Ready {
            return None;
        }
        let value = self.typed.as_mut()?.take()?;
        self.phase = RetainedTypedPackPhase::Published;
        Some(value)
    }

    pub fn progress(&self) -> Option<mounted::RetainedPackSourceProgress> {
        self.source.as_ref().map(mounted::RetainedPackSourceCursor::progress)
    }

    pub fn semantic_allocated(&self) -> bool {
        self.typed.is_some()
    }

    /// @emoji 📏️ The exact allocation the next grant needs, bounded by the envelope close budget.
    pub fn next_retained_allocation_bytes(&mut self) -> Result<Option<usize>, &'static str> {
        let requested = match self.phase {
            RetainedTypedPackPhase::Ingress => match self.source.as_ref() {
                Some(source) if !source.has_reserved_page() => Some(source.next_allocation_bytes()?),
                _ => None,
            },
            RetainedTypedPackPhase::Drive => match self.segment.as_ref().ok_or("mounted-pack.segment-owner")?.next_allocation_bytes() {
                Some(requested) => Some(requested),
                None => match self.value.as_mut().ok_or("mounted-pack.value-owner")?.next_allocation_bytes().map_err(|_| "mounted-pack.value-allocation")? {
                    Some(requested) => Some(requested),
                    None => self.catalog.as_mut().ok_or("mounted-pack.catalog-owner")?.next_allocation_bytes().map_err(|fault| fault.code)?,
                },
            },
            _ => None,
        };
        if let Some(requested) = requested {
            self.retained_allocated_bytes().checked_add(requested).filter(|total| *total <= super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES).ok_or("mounted-pack.retained-allocation-credits")?;
        }
        Ok(requested)
    }

    /// @emoji 🧱️ Reserves at most `maximum_bytes` of the pending allocation.
    pub fn reserve_retained_allocation(&mut self, maximum_bytes: usize) -> Result<mounted::RetainedPackSourceAllocationStep, mounted::RetainedPackSourceAllocationError> {
        let remaining = super::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES.saturating_sub(self.retained_allocated_bytes());
        let fault = |reason: &'static str| mounted::RetainedPackSourceAllocationError { allocated_bytes: 0, reason };
        match self.phase {
            RetainedTypedPackPhase::Ingress => self.source.as_mut().ok_or(fault("mounted-pack.source-owner"))?.reserve_page(maximum_bytes.min(remaining)),
            RetainedTypedPackPhase::Drive => {
                let segment = self.segment.as_mut().ok_or(fault("mounted-pack.segment-owner"))?;
                if segment.next_allocation_bytes().is_some() {
                    return segment.reserve_allocation(maximum_bytes.min(remaining));
                }
                let value = self.value.as_mut().ok_or(fault("mounted-pack.value-owner"))?;
                if value.next_allocation_bytes().map_err(|_| fault("mounted-pack.value-allocation"))?.is_some() {
                    return value
                        .reserve_allocation(maximum_bytes.min(remaining))
                        .map(|step| mounted::RetainedPackSourceAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
                        .map_err(|error| mounted::RetainedPackSourceAllocationError { allocated_bytes: error.allocated_bytes, reason: "mounted-pack.value-allocation" });
                }
                self.catalog
                    .as_mut()
                    .ok_or(fault("mounted-pack.catalog-owner"))?
                    .reserve_allocation(maximum_bytes.min(remaining))
                    .map(|step| mounted::RetainedPackSourceAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
                    .map_err(|error| mounted::RetainedPackSourceAllocationError { allocated_bytes: error.allocated_bytes, reason: error.fault.code })
            }
            _ => Ok(mounted::RetainedPackSourceAllocationStep::default()),
        }
    }

    pub fn retained_allocated_bytes(&self) -> usize {
        self.source.as_ref().map_or(0, mounted::RetainedPackSourceCursor::allocated_bytes)
            + self.segment.as_ref().map_or(0, mounted::RetainedPackSegmentCursor::allocated_bytes)
            + self.catalog.as_ref().map_or(0, mounted::RetainedPackCatalogCursor::allocated_bytes)
            + self.value.as_ref().map_or(0, mounted::RetainedValueCursor::allocated_bytes)
    }

    pub fn request_cancel(&mut self) {
        if let Some(source) = self.source.as_mut() {
            source.request_cancel();
        }
        self.phase = RetainedTypedPackPhase::Closing;
    }

    /// @emoji 🧹️ Releases one owner per grant, typed owner first, source last.
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<RetainedTypedPackCloseStep, &'static str> {
        let one = RetainedTypedPackCloseStep::Pending { released_items: 1, released_bytes: 0 };
        let retained = |step: mounted::RetainedPackCloseStep| match step {
            mounted::RetainedPackCloseStep::Pending { released_items, released_bytes } => Some(RetainedTypedPackCloseStep::Pending { released_items, released_bytes }),
            mounted::RetainedPackCloseStep::Complete => None,
        };
        if maximum_items == 0 {
            return Ok(RetainedTypedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.phase = RetainedTypedPackPhase::Closing;
        if self.document_byte.take().is_some() {
            return Ok(one);
        }
        if self.page_len != 0 {
            self.page.fill(0);
            self.page_len = 0;
            return Ok(one);
        }
        if self.catalog_value.take().is_some() {
            return Ok(one);
        }
        if let Some(typed) = self.typed.as_mut() {
            if typed.close_step() {
                drop(self.typed.take());
            }
            return Ok(one);
        }
        if let Some(value) = self.value.as_mut() {
            if let Some(step) = retained(value.close_step(1, maximum_bytes)?) {
                return Ok(step);
            }
            drop(self.value.take());
            return Ok(one);
        }
        if let Some(catalog) = self.catalog.as_mut() {
            if let Some(step) = retained(catalog.close_step(1, maximum_bytes)?) {
                return Ok(step);
            }
            drop(self.catalog.take());
            return Ok(one);
        }
        if let Some(segment) = self.segment.as_mut() {
            if let Some(step) = retained(segment.close_step(1, maximum_bytes)) {
                return Ok(step);
            }
            drop(self.segment.take());
            return Ok(one);
        }
        if let Some(anchor) = self.anchor.as_mut() {
            anchor.close_step();
            drop(self.anchor.take());
            return Ok(one);
        }
        if let Some(source) = self.source.as_mut() {
            if let Some(step) = retained(source.close_step(1, maximum_bytes)?) {
                return Ok(step);
            }
            drop(self.source.take());
            return Ok(one);
        }
        self.phase = RetainedTypedPackPhase::Closed;
        Ok(RetainedTypedPackCloseStep::Complete)
    }

    pub fn next_retained_release_allocation_bytes(&self) -> Option<usize> {
        if self.document_byte.is_some() || self.page_len != 0 || self.catalog_value.is_some() || self.typed.is_some() {
            return None;
        }
        if let Some(value) = self.value.as_ref() {
            return value.next_release_allocation_bytes();
        }
        if let Some(catalog) = self.catalog.as_ref() {
            return catalog.next_release_allocation_bytes().ok().flatten();
        }
        if let Some(segment) = self.segment.as_ref() {
            return segment.next_release_allocation_bytes();
        }
        self.source.as_ref()?.next_release_allocation_bytes().ok()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == RetainedTypedPackPhase::Closed
            && self.page_len == 0
            && self.source.is_none()
            && self.anchor.is_none()
            && self.segment.is_none()
            && self.catalog.is_none()
            && self.value.is_none()
            && self.typed.is_none()
            && self.catalog_value.is_none()
            && self.document_byte.is_none()
    }
}

impl<O: RetainedTypedPackOwner> Drop for RetainedTypedPackSession<O> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained typed pack session reached Drop before exact terminal-empty close");
    }
}
