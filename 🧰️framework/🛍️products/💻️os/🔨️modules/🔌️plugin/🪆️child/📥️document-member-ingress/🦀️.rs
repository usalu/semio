//! 📥️ Document member ingress of the child members: the owned document member ingress, its page slots, the ingress registry and their drops. See [`super`].

use super::*;

/// 📥️ Retains one exact recursive member identity with its admitted bounded open request.
pub struct OwnedDocumentMemberIngress {
    pub identity: ManuallyDrop<Option<(usize, ArtifactRef, store::OwnerRef)>>,
    pub request: ManuallyDrop<Option<store::MemberOpenRequest>>,
    pub identity_field: u8,
}

impl OwnedDocumentMemberIngress {
    /// 🪪️ Binds one closure ordinal and recursive owner to the request's sealed identity.
    #[expect(clippy::result_large_err, reason = "Rejected ingress returns its exact retained page and identity owners unchanged.")]
    pub fn try_new(ordinal: usize, reference: ArtifactRef, owner: store::OwnerRef, request: store::MemberOpenRequest) -> Result<Self, (Fault, Self)> {
        let ingress = Self { identity: ManuallyDrop::new(Some((ordinal, reference, owner))), request: ManuallyDrop::new(Some(request)), identity_field: 0 };
        let (_, reference, owner) = ingress.identity.as_ref().expect("fresh member ingress retains identity");
        let request = ingress.request.as_ref().expect("fresh member ingress retains request");
        let valid = ordinal < store::OWNED_DOCUMENT_MAXIMUM_MEMBERS && request.admitted_expected().is_ok() && request.expected() == reference && request.owner() == Some(owner);
        if valid {
            Ok(ingress)
        } else {
            Err((plugin_sdk_fault("owned document member ingress identity, owner, ordinal, or admitted request does not match"), ingress))
        }
    }

    pub fn ordinal(&self) -> usize {
        self.identity.as_ref().expect("live member ingress retains identity").0
    }

    pub fn operation(&self) -> semio_framework_job::OperationId {
        self.request.as_ref().expect("live member ingress retains request").operation()
    }

    pub fn generation(&self) -> semio_framework_job::Generation {
        self.request.as_ref().expect("live member ingress retains request").generation()
    }

    pub fn take_request(&mut self) -> Option<store::MemberOpenRequest> {
        self.request.take()
    }

    pub(crate) fn has_request(&self) -> bool {
        self.request.is_some()
    }

    fn return_request(&mut self, request: store::MemberOpenRequest) {
        assert!(self.request.is_none(), "member ingress request can only return to its exact empty slot");
        *self.request = Some(request);
    }

    pub fn take_identity(&mut self) -> Option<(usize, ArtifactRef, store::OwnerRef)> {
        self.identity.take()
    }

    pub fn has_identity(&self) -> bool {
        self.identity.is_some()
    }

    pub fn identity_string_mut(&mut self) -> Option<&mut String> {
        let (_, reference, owner) = self.identity.as_mut()?;
        match self.identity_field {
            0 => Some(&mut reference.artifact_id),
            1 => Some(&mut reference.dialect.artifact_kind),
            2 => Some(&mut reference.dialect.standard),
            3 => Some(&mut reference.dialect.subset),
            4 => Some(&mut owner.parent.artifact_id),
            5 => Some(&mut owner.parent.dialect.artifact_kind),
            6 => Some(&mut owner.parent.dialect.standard),
            7 => Some(&mut owner.parent.dialect.subset),
            8 => Some(&mut owner.slot),
            9 => Some(&mut owner.child_id),
            _ => None,
        }
    }

    /// 📏️ Quotes the next request step, identity string release, or identity handoff.
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(request) = self.request.as_ref() {
            if request.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(&*self.request), depth: 1, ..Default::default() }); }
            return Ok(RetirementDemand { copy_bytes: request.next_copy_byte_demand()?, capacity_bytes: request.next_capacity_byte_demand(body)?, release_bytes: request.next_release_byte_demand()?, depth: request.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original member request parent depth overflow"))? });
        }
        let Some((_, reference, owner)) = self.identity.as_ref() else { return Ok(RetirementDemand::default()) };
        let fields = [&reference.artifact_id, &reference.dialect.artifact_kind, &reference.dialect.standard, &reference.dialect.subset, &owner.parent.artifact_id, &owner.parent.dialect.artifact_kind, &owner.parent.dialect.standard, &owner.parent.dialect.subset, &owner.slot, &owner.child_id];
        Ok(match fields.get(usize::from(self.identity_field)) { Some(field)=>RetirementDemand { copy_bytes: size_of::<String>() + size_of::<u8>(), release_bytes: field.capacity(), depth: 1, ..Default::default() }, None=>RetirementDemand { copy_bytes: size_of_val(&*self.identity), depth: 1, ..Default::default() } })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if let Some(request) = self.request.as_mut() {
            if request.terminal_is_empty() {
                drop(self.request.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = request.close_step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, request.terminal_is_empty(), "member ingress request")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(field) = self.identity_string_mut() {
            let bytes = field.capacity();
            drop(std::mem::take(field));
            self.identity_field += 1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: bytes, ..empty }));
        }
        drop(self.identity.take());
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.identity.is_none() && self.request.is_none()
    }
}

impl Drop for OwnedDocumentMemberIngress {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "owned document member ingress reached Drop before exact handoff or bounded close");
    }
}

#[doc(hidden)]
pub const OWNED_DOCUMENT_INGRESS_PAGE_SLOTS: usize = 8;

#[doc(hidden)]
pub struct OwnedDocumentMemberIngressRegistry {
    pub pages: Vec<Option<Box<[std::mem::MaybeUninit<OwnedDocumentMemberIngress>]>>>,
    pub occupied: [u64; CHILD_CONTENT_SLOTS / 64],
    pub expected: usize,
    pub len: usize,
    pub generation: u64,
    pub sealed: bool,
    pub close_cursor: usize,
}

impl OwnedDocumentMemberIngressRegistry {
    pub fn try_new(expected: usize) -> Result<Self, Fault> {
        if expected > store::OWNED_DOCUMENT_MAXIMUM_MEMBERS {
            return Err(plugin_sdk_fault("owned document member ingress exceeds the recursive closure member limit"));
        }
        let count = expected.div_ceil(OWNED_DOCUMENT_INGRESS_PAGE_SLOTS);
        let mut pages = Vec::new();
        pages.try_reserve_exact(count).map_err(|_| plugin_sdk_fault("owned document member ingress page directory was not admitted"))?;
        for index in 0..count {
            let length = (expected - index * OWNED_DOCUMENT_INGRESS_PAGE_SLOTS).min(OWNED_DOCUMENT_INGRESS_PAGE_SLOTS);
            let mut slots = Vec::new();
            slots.try_reserve_exact(length).map_err(|_| plugin_sdk_fault("owned document member ingress page allocation was not admitted"))?;
            slots.resize_with(length, std::mem::MaybeUninit::uninit);
            pages.push(Some(slots.into_boxed_slice()));
        }
        Ok(Self { pages, occupied: [0; CHILD_CONTENT_SLOTS / 64], expected, len: 0, generation: 0, sealed: false, close_cursor: 0 })
    }

    pub fn entry(&self, ordinal: usize) -> Option<&OwnedDocumentMemberIngress> {
        if ordinal >= self.expected || self.occupied[ordinal / 64] & (1 << (ordinal % 64)) == 0 { return None; }
        let page = self.pages.get(ordinal / OWNED_DOCUMENT_INGRESS_PAGE_SLOTS)?.as_ref()?;
        Some(unsafe { page[ordinal % OWNED_DOCUMENT_INGRESS_PAGE_SLOTS].assume_init_ref() })
    }

    pub fn entry_mut(&mut self, ordinal: usize) -> Option<&mut OwnedDocumentMemberIngress> {
        if ordinal >= self.expected || self.occupied[ordinal / 64] & (1 << (ordinal % 64)) == 0 { return None; }
        let page = self.pages.get_mut(ordinal / OWNED_DOCUMENT_INGRESS_PAGE_SLOTS)?.as_mut()?;
        Some(unsafe { page[ordinal % OWNED_DOCUMENT_INGRESS_PAGE_SLOTS].assume_init_mut() })
    }

    fn next_occupied(&self) -> Option<usize> {
        let first = self.close_cursor / 64;
        for index in first..self.occupied.len() {
            let word = self.occupied[index] & if index == first { u64::MAX << (self.close_cursor % 64) } else { u64::MAX };
            if word != 0 { return Some(index * 64 + word.trailing_zeros() as usize); }
        }
        None
    }

    pub fn empty_backing_byte_demand(&self) -> usize {
        if let Some(page) = self.pages.iter().find_map(Option::as_ref) { return size_of_val(page.as_ref()); }
        self.pages.capacity() * size_of::<Option<Box<[std::mem::MaybeUninit<OwnedDocumentMemberIngress>]>>>()
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact retained member ingress owner unchanged.")]
    pub fn admit(&mut self, ingress: OwnedDocumentMemberIngress) -> Result<(), (Fault, OwnedDocumentMemberIngress)> {
        let ordinal = ingress.ordinal();
        if self.sealed || ordinal >= self.expected || self.occupied[ordinal / 64] & (1 << (ordinal % 64)) != 0 {
            return Err((plugin_sdk_fault("owned document member ingress is sealed, out of range, or duplicates an ordinal"), ingress));
        }
        let Some(generation) = self.generation.checked_add(1) else {
            return Err((plugin_sdk_fault("owned document member ingress generation exhausted"), ingress));
        };
        self.pages[ordinal / OWNED_DOCUMENT_INGRESS_PAGE_SLOTS].as_mut().expect("open ingress ordinal retains its original page")[ordinal % OWNED_DOCUMENT_INGRESS_PAGE_SLOTS].write(ingress);
        self.occupied[ordinal / 64] |= 1 << (ordinal % 64);
        self.len += 1;
        self.generation = generation;
        Ok(())
    }

    pub fn seal(&mut self) -> Result<(), Fault> {
        if self.sealed || self.len != self.expected {
            return Err(plugin_sdk_fault("owned document member ingress is duplicate-sealed or incomplete"));
        }
        self.sealed = true;
        Ok(())
    }

    pub fn take(&mut self, ordinal: usize) -> Option<OwnedDocumentMemberIngress> {
        if ordinal >= self.expected || self.occupied[ordinal / 64] & (1 << (ordinal % 64)) == 0 {
            return None;
        }
        self.occupied[ordinal / 64] &= !(1 << (ordinal % 64));
        self.len -= 1;
        Some(unsafe { self.pages[ordinal / OWNED_DOCUMENT_INGRESS_PAGE_SLOTS].as_mut().expect("occupied ingress ordinal retains its original page")[ordinal % OWNED_DOCUMENT_INGRESS_PAGE_SLOTS].assume_init_read() })
    }

    /// ♻️ Retires original occupied slots, indivisible pages, and the separate directory.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(|error| plugin_sdk_fault(error.to_string()))?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if !self.sealed { self.sealed = true; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(ordinal) = self.next_occupied() {
            let active = self.entry_mut(ordinal).expect("occupied ingress keeps its original page");
            if active.terminal_is_empty() {
                drop(self.take(ordinal).ok_or_else(|| plugin_sdk_fault("member ingress close lost its exact occupied ordinal"))?);
                self.close_cursor = ordinal + 1;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = active.close_step(child).map_err(|error| plugin_sdk_fault(error.to_string()))?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, active.terminal_is_empty(), "member ingress").map_err(|error| plugin_sdk_fault(error.to_string()))?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(page) = self.pages.iter_mut().find(|page| page.is_some()) { drop(page.take()); } else { drop(std::mem::take(&mut self.pages)); }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..empty };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(RetirementDemand::default()); }
        if !self.sealed { return Ok(RetirementDemand { copy_bytes: size_of::<bool>(), depth: 1, ..Default::default() }); }
        if let Some(active) = self.next_occupied().and_then(|ordinal| self.entry(ordinal)) {
            if active.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<OwnedDocumentMemberIngress>() + size_of::<u64>() + 2 * size_of::<usize>(), depth: 1, ..Default::default() }); }
            let child = active.retirement_demands(body)?;
            return Ok(RetirementDemand { depth: child.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "member ingress registry depth overflow"))?, ..child });
        }
        let copy_bytes=if self.pages.iter().any(Option::is_some) { size_of::<Option<Box<[std::mem::MaybeUninit<OwnedDocumentMemberIngress>]>>>() } else { size_of_val(&self.pages) };
        Ok(RetirementDemand { copy_bytes, release_bytes: self.empty_backing_byte_demand(), depth: 1, ..Default::default() })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.pages.is_empty() && self.pages.capacity() == 0
    }
}

impl Drop for OwnedDocumentMemberIngressRegistry {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "owned document member ingress registry reached Drop before every request owner was handed off or bounded-closed");
    }
}
