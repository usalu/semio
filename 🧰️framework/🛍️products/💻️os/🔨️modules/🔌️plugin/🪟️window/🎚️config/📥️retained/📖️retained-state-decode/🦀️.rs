//! 📖️ Retained state decode: the retained state phase, the retained window config state decode, its stepping, retirement and drop. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[doc(hidden)]
pub enum RetainedStatePhase {
    Envelope,
    Ingress,
    Replay,
    Ready,
    Closing,
    Closed,
}

#[doc(hidden)]
pub struct RetainedWindowConfigStateDecode<O: WindowConfigOwner> {
    pub(super) active:ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    #[doc(hidden)]
    pub phase: RetainedStatePhase,
    inner_start: usize,
    #[doc(hidden)]
    pub admitted: usize,
    document_byte: Option<(u64, u8)>,
    #[doc(hidden)]
    pub source: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackSourceCursor>>,
    anchor: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackAnchorCursor>>,
    segment: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackSegmentCursor>>,
    pub(super) catalog: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackCatalogCursor>>,
    pub(super) value: ManuallyDrop<Option<store::mounted_pack_rt::RetainedValueCursor>>,
    pub(super) typed: ManuallyDrop<Option<RetainedWindowConfigTypedState<O>>>,
    catalog_value: ManuallyDrop<Option<store::mounted_pack_rt::RetainedPackCatalog>>,
    source_complete: bool,
    anchor_ready: bool,
    segment_complete: bool,
    catalog_complete: bool,
    value_sealed: bool,
    value_complete: bool,
    pub(super) state: ManuallyDrop<Option<O::State>>,
    state_retirement: Option<ControlledRetirement<O::State>>,
    pub(super) digest: Option<[u8; 32]>,
    hasher: semio_framework_hash::Hasher,
    pub(super) native: Option<semio_framework_value_native_owned::native_decoding::NativeDecodeContinuation>,
}

impl<O: WindowConfigOwner> RetainedWindowConfigStateDecode<O> {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self {
            active:ManuallyDrop::new(None),
            phase: RetainedStatePhase::Envelope,
            inner_start: 0,
            admitted: 0,
            document_byte: None,
            source: ManuallyDrop::new(None),
            anchor: ManuallyDrop::new(None),
            segment: ManuallyDrop::new(None),
            catalog: ManuallyDrop::new(None),
            value: ManuallyDrop::new(None),
            typed: ManuallyDrop::new(None),
            catalog_value: ManuallyDrop::new(None),
            source_complete: false,
            anchor_ready: false,
            segment_complete: false,
            catalog_complete: false,
            value_sealed: false,
            value_complete: false,
            state: ManuallyDrop::new(None),
            state_retirement: None,
            digest: None,
            hasher: semio_framework_hash::Hasher::new(),
            native: None,
        }
    }

    fn validate_envelope(&mut self, pack: &[u8], grant: RetainedCloneGrant) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        let envelope_id = <O::State as pack::snapshot::ArtifactDsl>::envelope_id();
        let expected_token_bytes = envelope_id.len().checked_add(".pack v1".len()).ok_or(WindowConfigPackLoadDiagnostic::EnvelopeIdentity)?;
        let header_bytes = 12usize.checked_add(expected_token_bytes).ok_or(WindowConfigPackLoadDiagnostic::EnvelopeIdentity)?;
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < header_bytes.saturating_add(size_of::<RetainedWindowConfigTypedState<O>>()) {
            return Ok(false);
        }
        if pack.len() <= header_bytes || pack.get(..8) != Some(store::semio_format::BINARY_MAGIC.as_slice()) || pack.get(8..12).and_then(|bytes| <[u8; 4]>::try_from(bytes).ok()).map(u32::from_le_bytes) != Some(expected_token_bytes as u32) {
            return Err(WindowConfigPackLoadDiagnostic::EnvelopeIdentity);
        }
        let token = pack.get(12..header_bytes).ok_or(WindowConfigPackLoadDiagnostic::EnvelopeIdentity)?;
        let id = envelope_id.as_bytes();
        if token.get(..id.len()) != Some(id) || token.get(id.len()..) != Some(b".pack v1") {
            return Err(WindowConfigPackLoadDiagnostic::EnvelopeIdentity);
        }
        let inner_len = pack.len() - header_bytes;
        if inner_len > store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES {
            return Err(WindowConfigPackLoadDiagnostic::Capacity);
        }
        let pages = inner_len.div_ceil(store::mounted_pack_rt::RETAINED_PACK_PAGE_BYTES);
        if pages == 0 || pages > store::mounted_pack_rt::RETAINED_PACK_MAXIMUM_PAGES {
            return Err(WindowConfigPackLoadDiagnostic::Capacity);
        }
        let maximum_items = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
        let maximum_allocation = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES;
        let limits =
            || store::mounted_pack_rt::PackLimits { max_file_len: inner_len as u64, max_segment_len: inner_len as u64, max_symbols: maximum_items as u32, max_depth: 64, max_items: maximum_items as u64, max_total_alloc: maximum_allocation as u64 };
        *self.source = Some(store::mounted_pack_rt::RetainedPackSourceCursor::try_new(pages, inner_len, maximum_allocation).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?);
        *self.anchor = Some(store::mounted_pack_rt::RetainedPackAnchorCursor::new());
        *self.segment = Some(store::mounted_pack_rt::RetainedPackSegmentCursor::try_new(limits(), maximum_allocation).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?);
        *self.catalog = Some(store::mounted_pack_rt::RetainedPackCatalogCursor::try_new(limits(), maximum_items, inner_len, inner_len, maximum_items, maximum_allocation).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?);
        *self.value = Some(store::mounted_pack_rt::RetainedValueCursor::try_new(limits(), maximum_allocation).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?);
        let mut accepted = |progress: semio_framework_value_native_owned::native_decoding::NativeDecodeProgress| progress.completed <= grant.maximum_items;
        let mut native = match self.native.take() {
            Some(receipt) => semio_framework_value_native_owned::native_decoding::NativeDecodeControl::resume(receipt, &mut accepted).map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?,
            None => semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut accepted),
        };
        native.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
        let typed = native.scoped_depth(grant.maximum_depth, |native| RetainedWindowConfigTypedState::<O>::new(native).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "Window config requires its actual authored metadata producer")));
        self.native = Some(native.pause().map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?);
        *self.typed = Some(typed.map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?);
        self.hasher.update(&pack[..header_bytes]);
        self.inner_start = header_bytes;
        self.phase = RetainedStatePhase::Ingress;
        Ok(true)
    }

    fn retained_allocated_bytes(&self) -> usize {
        self.source.as_ref().map_or(0, store::mounted_pack_rt::RetainedPackSourceCursor::allocated_bytes)
            + self.segment.as_ref().map_or(0, store::mounted_pack_rt::RetainedPackSegmentCursor::allocated_bytes)
            + self.catalog.as_ref().map_or(0, store::mounted_pack_rt::RetainedPackCatalogCursor::allocated_bytes)
            + self.value.as_ref().map_or(0, store::mounted_pack_rt::RetainedValueCursor::allocated_bytes)
    }

    fn reserve_next(&mut self, maximum_capacity_bytes: usize) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        let allocated = self.retained_allocated_bytes();
        let remaining = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES.saturating_sub(allocated);
        if self.phase == RetainedStatePhase::Ingress {
            let source = self.source.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
            if source.has_reserved_page() {
                return Ok(false);
            }
            let requested = source.next_allocation_bytes().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            if requested > maximum_capacity_bytes || requested > remaining {
                return Ok(false);
            }
            let step = source.reserve_page(maximum_capacity_bytes.min(remaining)).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
            return Ok(step.progressed);
        }
        if self.phase != RetainedStatePhase::Replay {
            return Ok(false);
        }
        let segment = self.segment.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        if let Some(requested) = segment.next_allocation_bytes() {
            if requested > maximum_capacity_bytes || requested > remaining {
                return Ok(false);
            }
            return segment.reserve_allocation(maximum_capacity_bytes.min(remaining)).map(|step| step.progressed).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity);
        }
        let value = self.value.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        if let Some(requested) = value.next_allocation_bytes().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
            if requested > maximum_capacity_bytes || requested > remaining {
                return Ok(false);
            }
            return value.reserve_allocation(maximum_capacity_bytes.min(remaining)).map(|step| step.progressed).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity);
        }
        let catalog = self.catalog.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        if let Some(requested) = catalog.next_allocation_bytes().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
            if requested > maximum_capacity_bytes || requested > remaining {
                return Ok(false);
            }
            return catalog.reserve_allocation(maximum_capacity_bytes.min(remaining)).map(|step| step.progressed).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity);
        }
        Ok(false)
    }

    fn ingress(&mut self, pack: &[u8], grant: RetainedCloneGrant) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        let start = self.inner_start + self.admitted;
        let remaining = pack.len().saturating_sub(start);
        if remaining == 0 {
            let source = self.source.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
            source.seal().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            self.phase = RetainedStatePhase::Replay;
            return Ok(true);
        }
        if self.reserve_next(grant.maximum_capacity_bytes)? {
            return Ok(true);
        }
        let source = self.source.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        if !source.has_reserved_page() {
            return Ok(false);
        }
        let len = remaining.min(store::mounted_pack_rt::RETAINED_PACK_PAGE_BYTES).min(grant.maximum_copy_bytes);
        if len == 0 {
            return Ok(false);
        }
        source.preflight_page(len).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
        let mut bytes = [0; store::mounted_pack_rt::RETAINED_PACK_PAGE_BYTES];
        bytes[..len].copy_from_slice(&pack[start..start + len]);
        self.hasher.update(&pack[start..start + len]);
        let page = store::mounted_pack_rt::RetainedPackPage::try_from_array(bytes, len).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
        source.admit_page(page).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
        self.admitted += len;
        Ok(true)
    }

    fn replay(&mut self, grant: RetainedCloneGrant) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        if self.reserve_next(grant.maximum_capacity_bytes)? {
            return Ok(true);
        }
        if self.typed.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?.grant_symbol(self.catalog.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?)? {
            return Ok(true);
        }
        if let Some((index, byte)) = self.document_byte {
            if self.value.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.ingress_ready() {
                self.document_byte = None;
                self.value.as_mut().expect("retained config value owner remains").admit_byte(index, byte).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
                return Ok(true);
            }
        }
        if !self.value_complete {
            if let Some(token) = self.value.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                self.value_complete = matches!(token, store::mounted_pack_rt::RetainedValueToken::Complete { .. });
                self.typed.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?.accept(token, self.catalog.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?)?;
                return Ok(true);
            }
        }
        if self.catalog_complete && !self.value_sealed {
            let bytes = self.catalog.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.document_bytes();
            self.value.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.seal(bytes).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            self.value_sealed = true;
            return Ok(true);
        }
        if self.catalog.as_ref().is_some_and(store::mounted_pack_rt::RetainedPackCatalogCursor::has_pending_input) {
            if let Some(event) = self.catalog.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                match event {
                    store::mounted_pack_rt::RetainedPackCatalogEvent::DocumentByte { index, value, .. } => self.document_byte = Some((index, value)),
                    store::mounted_pack_rt::RetainedPackCatalogEvent::Complete => self.catalog_complete = true,
                    _ => {}
                }
            }
            return Ok(true);
        }
        let segment_can_admit = self.segment.as_ref().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.preflight().is_ok();
        if self.document_byte.is_none() && !self.segment_complete && (!segment_can_admit || self.source_complete) {
            if let Some(event) = self.segment.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                self.segment_complete = matches!(event, store::mounted_pack_rt::RetainedPackSegmentEvent::PackComplete { .. });
                let catalog = self.catalog.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
                catalog.admit(event).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
                if let Some(event) = catalog.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                    match event {
                        store::mounted_pack_rt::RetainedPackCatalogEvent::DocumentByte { index, value, .. } => self.document_byte = Some((index, value)),
                        store::mounted_pack_rt::RetainedPackCatalogEvent::Complete => self.catalog_complete = true,
                        _ => {}
                    }
                }
                return Ok(true);
            }
        }
        if !self.source_complete && segment_can_admit {
            if let Some(event) = self.source.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant().map_err(|_| WindowConfigPackLoadDiagnostic::Pack)? {
                self.source_complete = matches!(event, store::mounted_pack_rt::RetainedPackSourceEvent::Complete { .. });
                self.anchor.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant(Some(event)).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
                self.segment.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.admit(event).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
                return Ok(true);
            }
        }
        if self.source_complete && !self.anchor_ready {
            self.anchor_ready = self.anchor.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.grant(None).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            return Ok(true);
        }
        if self.anchor_ready && self.catalog_complete && self.value_complete && self.catalog_value.is_none() {
            let superblock = self.anchor.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.take().ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
            *self.catalog_value = self.catalog.as_mut().ok_or(WindowConfigPackLoadDiagnostic::Pack)?.take(superblock).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
            *self.state = self.typed.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?.take();
            if self.state.is_none() {
                return Err(WindowConfigPackLoadDiagnostic::TypedState);
            }
            self.digest = Some(*self.hasher.finalize().as_bytes());
            self.phase = RetainedStatePhase::Ready;
            return Ok(true);
        }
        Ok(false)
    }

    #[doc(hidden)]
    pub fn advance(&mut self, pack: &[u8], grant: RetainedCloneGrant) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        if grant.maximum_items == 0 {
            return Ok(false);
        }
        match self.phase {
            RetainedStatePhase::Envelope => self.validate_envelope(pack, grant),
            RetainedStatePhase::Ingress => self.ingress(pack, grant),
            RetainedStatePhase::Replay => self.replay(grant),
            RetainedStatePhase::Ready => Ok(true),
            RetainedStatePhase::Closing | RetainedStatePhase::Closed => Ok(false),
        }
    }

    pub(super) fn take_ready(&mut self) -> Option<(O::State, [u8; 32])> {
        if self.phase != RetainedStatePhase::Ready {
            return None;
        }
        Some((self.state.take()?, self.digest.take()?))
    }

    pub(super) fn request_cancel(&mut self) {
        if let Some(source) = self.source.as_mut() {
            source.request_cancel();
        }
        self.phase = RetainedStatePhase::Closing;
    }

    #[doc(hidden)]
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let frame = |copy_bytes| Ok(RetirementDemand { copy_bytes, depth: 1, ..Default::default() });
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if self.document_byte.is_some() { return frame(size_of::<(u64, u8)>()); }
        if self.catalog_value.is_some() { return frame(size_of::<store::mounted_pack_rt::RetainedPackCatalog>()); }
        if self.state.is_some() { return frame(size_of::<O::State>()); }
        if let Some(owner) = self.state_retirement.as_ref() { return controlled_demands(owner, body); }
        if let Some(typed) = self.typed.as_ref() { return if typed.terminal_is_empty() { frame(size_of::<RetainedWindowConfigTypedState<O>>()) } else { typed.retirement_demands(body) }; }
        if let Some(owner) = self.value.as_ref() { return if owner.terminal_is_empty() { frame(size_of::<store::mounted_pack_rt::RetainedValueCursor>()) } else { Ok(owner.retirement_demands()) }; }
        if let Some(owner) = self.catalog.as_ref() { return if owner.terminal_is_empty() { frame(size_of::<store::mounted_pack_rt::RetainedPackCatalogCursor>()) } else { owner.retirement_demands() }; }
        if let Some(owner) = self.segment.as_ref() { return if owner.terminal_is_empty() { frame(size_of::<store::mounted_pack_rt::RetainedPackSegmentCursor>()) } else { Ok(owner.retirement_demands()) }; }
        if self.anchor.is_some() { return frame(size_of::<store::mounted_pack_rt::RetainedPackAnchorCursor>()); }
        if let Some(owner) = self.source.as_ref() { return if owner.terminal_is_empty() { frame(size_of::<store::mounted_pack_rt::RetainedPackSourceCursor>()) } else { owner.retirement_demands() }; }
        frame(0)
    }

    #[doc(hidden)]
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, String> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_message)?;
        if !super::super::registry::fits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.request_cancel();
        let moved = || RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() });
        if self.document_byte.take().is_some() || self.catalog_value.take().is_some() { return Ok(moved()); }
        if let Some(state) = self.state.take() {
            return match ControlledRetirement::new(state) {
                Ok(owner) => { self.state_retirement = Some(owner); Ok(moved()) },
                Err((error, state)) => { *self.state = Some(state); Err(error.into_message()) },
            };
        }
        if let Some(owner) = self.state_retirement.as_mut() {
            let step = owner.step(grant).map_err(ValueError::into_message)?;
            if owner.terminal_is_empty() { self.state_retirement = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(typed) = self.typed.as_mut() {
            if typed.terminal_is_empty() { drop(self.typed.take()); return Ok(moved()); }
            return typed.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(ValueError::into_message);
        }
        if let Some(value) = self.value.as_mut() {
            if value.terminal_is_empty() { drop(self.value.take()); return Ok(moved()); }
            let step = value.close_step(grant.maximum_items.min(1), grant.maximum_release_bytes).map_err(|error| error.to_string())?;
            return Ok(pack_close_receipt(step, demand));
        }
        if let Some(catalog) = self.catalog.as_mut() {
            if catalog.terminal_is_empty() { drop(self.catalog.take()); return Ok(moved()); }
            let step = catalog.close_step(grant.maximum_items.min(1), grant.maximum_release_bytes).map_err(|error| error.reason.to_string())?;
            return Ok(pack_close_receipt(step, demand));
        }
        if let Some(segment) = self.segment.as_mut() {
            if segment.terminal_is_empty() { drop(self.segment.take()); return Ok(moved()); }
            return Ok(pack_close_receipt(segment.close_step(grant.maximum_items.min(1), grant.maximum_release_bytes), demand));
        }
        if let Some(anchor) = self.anchor.as_mut() {
            anchor.close_step();
            drop(self.anchor.take());
            return Ok(moved());
        }
        if let Some(source) = self.source.as_mut() {
            if source.terminal_is_empty() { drop(self.source.take()); return Ok(moved()); }
            let step = source.close_step(grant.maximum_items.min(1), grant.maximum_release_bytes).map_err(|error| error.to_string())?;
            return Ok(pack_close_receipt(step, demand));
        }
        self.phase = RetainedStatePhase::Closed;
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    fn close_retained(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"window state decoder depth refused"));}
        self.request_cancel();
        if self.active.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active,grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.state.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.state,&mut self.active,grant);}
        if self.typed.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.typed,&mut self.active,grant);}
        if self.document_byte.take().is_some()||self.catalog_value.take().is_some(){return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        let convert=|step|match step{store::mounted_pack_rt::RetainedPackCloseStep::Pending{released_items,released_bytes}=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()}),store::mounted_pack_rt::RetainedPackCloseStep::Complete=>RetainedCloneStep::Progress(Default::default())};
        if let Some(value)=self.value.as_mut(){let step=value.close_step(1,grant.maximum_release_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"window value retirement refused"))?;if matches!(step,store::mounted_pack_rt::RetainedPackCloseStep::Complete){if !value.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window value false terminal"));}self.value.take();}return Ok(convert(step));}
        if let Some(catalog)=self.catalog.as_mut(){let step=catalog.close_step(1,grant.maximum_release_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"window catalog retirement refused"))?;if matches!(step,store::mounted_pack_rt::RetainedPackCloseStep::Complete){if !catalog.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window catalog false terminal"));}self.catalog.take();}return Ok(convert(step));}
        if let Some(segment)=self.segment.as_mut(){let step=segment.close_step(1,grant.maximum_release_bytes);if matches!(step,store::mounted_pack_rt::RetainedPackCloseStep::Complete){if !segment.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window segment false terminal"));}self.segment.take();}return Ok(convert(step));}
        if let Some(anchor)=self.anchor.as_mut(){anchor.close_step();self.anchor.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        if let Some(source)=self.source.as_mut(){let step=source.close_step(1,grant.maximum_release_bytes).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"window source retirement refused"))?;if matches!(step,store::mounted_pack_rt::RetainedPackCloseStep::Complete){if !source.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"window source false terminal"));}self.source.take();}return Ok(convert(step));}
        self.phase=RetainedStatePhase::Closed;Ok(RetainedCloneStep::Complete(Default::default()))
    }
    #[doc(hidden)]
    pub fn terminal_is_empty(&self) -> bool {
        self.phase == RetainedStatePhase::Closed
            && self.active.is_none()
            && self.document_byte.is_none()
            && self.source.is_none()
            && self.anchor.is_none()
            && self.segment.is_none()
            && self.catalog.is_none()
            && self.value.is_none()
            && self.typed.is_none()
            && self.catalog_value.is_none()
            && self.state.is_none()
            && self.state_retirement.is_none()
    }
}

impl<O:WindowConfigOwner> store::ErasedSnapshotRetirement for RetainedWindowConfigStateDecode<O>{
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.retirement_demands(maximum_body_bytes)}

 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.retirement_demands(0).map(|demand|demand.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.retirement_demands(body).map(|demand|demand.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.retirement_demands(0).map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.retirement_demands(0).map(|demand|demand.depth)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.close_retained(grant)}
 fn terminal_is_empty(&self)->bool{RetainedWindowConfigStateDecode::terminal_is_empty(self)}
}

impl<O: WindowConfigOwner> Drop for RetainedWindowConfigStateDecode<O> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained window config state decoder reached Drop before terminal-empty retirement");
    }
}
