use super::*;
use std::collections::VecDeque;

type MemberMutationTriple = (KernelMutation, MutationId, InverseMutation);

pub(crate) struct MountedMemberReceiptSource<'a> {
    pub(crate) publication: &'a dyn store::ErasedMemberStoreOneItemPublication,
    pub(crate) document: ArtifactHandle,
    pub(crate) invocation_id: &'a str,
    pub(crate) group_id: &'a str,
    pub(crate) fallback_author: &'a str,
}
impl MountedMemberReceiptSource<'_> {
    fn identity(&self) -> [(usize, usize); 5] {
        let edit = self.publication.prepared_edit_id().unwrap_or("");
        [(self.publication as *const dyn store::ErasedMemberStoreOneItemPublication as *const () as usize, 0), (edit.as_ptr() as usize, edit.len()), (self.invocation_id.as_ptr() as usize, self.invocation_id.len()), (self.group_id.as_ptr() as usize, self.group_id.len()), (self.fallback_author.as_ptr() as usize, self.fallback_author.len())]
    }
    fn kernel<'a>(&'a self, index: usize, inverse: &'a [u8]) -> Result<MountedKernelMutationReceiptSource<'a>, ValueError> {
        let (schema, kind) = self.publication.prepared_operation_schema_parts(index).ok_or_else(|| MountedMemberReceipt::refusal("member receipt requires its original operation semantic schema"))?;
        let meta = self.publication.prepared_operation_metadata(index).ok_or_else(|| MountedMemberReceipt::refusal("member receipt requires original prepared operation metadata"))?;
        let id = meta.mutation_id.as_ref().ok_or_else(|| MountedMemberReceipt::refusal("member receipt requires original prepared mutation identity"))?;
        if meta.group_id.as_ref().is_none_or(|group| group.len() != self.group_id.len()) { return Err(MountedMemberReceipt::refusal("member receipt requires its exact original common group identity")); }
        Ok(MountedKernelMutationReceiptSource { document: self.document, mutation_id: id.0.as_str(), invocation_id: self.invocation_id, schema, schema_separator: ".", schema_suffix: kind, author: meta.author_id.as_ref().map_or(self.fallback_author, |author| author.0.as_str()), dependencies: &meta.dependencies, inverse, base_version: ArtifactVersion(meta.base_version), timestamp: meta.timestamp, undo_policy: meta.undo_policy })
    }
}

pub(crate) struct MountedMemberReceiptOutput {
    pub(crate) document: ArtifactHandle,
    pub(crate) edit_id: String,
    pub(crate) triples: VecDeque<MemberMutationTriple>,
}

pub(crate) struct MountedMemberReceipt {
    identity: [(usize, usize); 5],
    document: ArtifactHandle,
    operation_count: usize,
    inverse_count: usize,
    validated_operations: usize,
    group_offset: usize,
    index: usize,
    phase: usize,
    edit_offset: usize,
    edit: MountedReceiptBytes,
    inverse_encoder: Option<MountedPreparedOperationsBytes>,
    inverse: ManuallyDrop<Vec<u8>>,
    forward: Option<MountedPreparedOperationBytes>,
    kernel: Option<MountedKernelMutationReceipt>,
    triples: ManuallyDrop<VecDeque<MemberMutationTriple>>,
    close_field: usize,
    operation_schema: Option<[(usize, usize); 2]>,
    closing: bool,
    transferred: bool,
}

impl MountedMemberReceipt {
    fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated, message) }
    pub(crate) fn frame_birth_bytes() -> usize { size_of::<Self>() }
    /// 🧬️ Seals the exact retained publication and original borrowed identities without allocating.
    pub(crate) fn new(source: &MountedMemberReceiptSource<'_>) -> Result<Self, ValueError> {
        source.publication.prepared_edit_id().ok_or_else(|| Self::refusal("member receipt requires its original prepared edit identity"))?;
        let count = source.publication.prepared_operation_count(false).ok_or_else(|| Self::refusal("member receipt requires original forward operations"))?;
        let inverse_count = source.publication.prepared_operation_count(true).ok_or_else(|| Self::refusal("member receipt requires original inverse operations"))?;
        if count.checked_mul(size_of::<MemberMutationTriple>()).is_none_or(|bytes| bytes > MOUNTED_RECEIPT_MAXIMUM_BYTES) { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "member receipt typed backing exceeds original admission")); }
        Ok(Self { identity: source.identity(), document: source.document, operation_count: count, inverse_count, validated_operations: 0, group_offset: 0, index: 0, phase: 0, edit_offset: 0, edit: MountedReceiptBytes::new(), inverse_encoder: Some(MountedPreparedOperationsBytes::new(inverse_count)), inverse: ManuallyDrop::new(Vec::new()), forward: None, kernel: None, triples: ManuallyDrop::new(VecDeque::new()), close_field: 0, operation_schema: None, closing: false, transferred: false })
    }
    fn validate(&self, source: &MountedMemberReceiptSource<'_>) -> Result<(), ValueError> {
        if self.closing || self.transferred || self.identity != source.identity() || self.document != source.document || source.publication.prepared_operation_count(false) != Some(self.operation_count) || source.publication.prepared_operation_count(true) != Some(self.inverse_count) { return Err(Self::refusal("member receipt lost its original prepared publication authority")); }
        if let Some(identity) = self.operation_schema {
            let (entity, kind) = source.publication.prepared_operation_schema_parts(self.index).ok_or_else(|| Self::refusal("member receipt lost its current original operation schema"))?;
            if identity != [(entity.as_ptr() as usize, entity.len()), (kind.as_ptr() as usize, kind.len())] { return Err(Self::refusal("member receipt changed its current original operation schema")); }
        }
        if self.index < self.operation_count { source.kernel(self.index, &self.inverse)?; }
        Ok(())
    }
    /// 🔢️ Exposes the current original forward ordinal so its exact schema can be borrowed per turn.
    pub(crate) fn operation_index(&self) -> Option<usize> { (!self.closing && !self.transferred && self.index < self.operation_count).then_some(self.index) }
    pub(crate) fn next_capacity_byte_demand(&self, source: &MountedMemberReceiptSource<'_>) -> Result<usize, ValueError> {
        self.validate(source)?;
        match self.phase {
            0 => { if self.validated_operations < self.operation_count { source.kernel(self.validated_operations, &self.inverse)?; Ok(0) } else { Ok(self.operation_count * size_of::<MemberMutationTriple>()) } },
            1 => self.edit.next_capacity_byte_demand((source.publication.prepared_edit_id().unwrap().len() - self.edit_offset).min(64)),
            2 => self.inverse_encoder.as_ref().unwrap().next_capacity_byte_demand(),
            3 => self.forward.as_ref().unwrap().next_capacity_byte_demand(),
            4 => self.kernel.as_ref().unwrap().next_capacity_byte_demand(&source.kernel(self.index, &self.inverse)?),
            _ => Ok(0),
        }
    }
    pub(crate) fn next_release_byte_demand(&self, source: &MountedMemberReceiptSource<'_>) -> Result<usize, ValueError> {
        self.validate(source)?;
        match self.phase { 1 => Ok(self.edit.next_growth_release_byte_demand()), 2 => self.inverse_encoder.as_ref().unwrap().next_release_byte_demand(), 3 => self.forward.as_ref().unwrap().next_growth_release_byte_demand(), 4 => Ok(self.kernel.as_ref().unwrap().next_release_byte_demand()), 5 => Ok(self.inverse.capacity()), _ => Ok(0) }
    }
    fn progress(birth: usize, release: usize) -> RetainedCloneStep { MountedGroupReceipt::progress(birth, release) }
    /// 🧾️ Prebuilds all original forward, framed inverse, and causal owners before publication acknowledgment.
    pub(crate) fn advance(&mut self, source: &MountedMemberReceiptSource<'_>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.validate(source)?;
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        match self.phase {
            0 => { if self.validated_operations < self.operation_count {
                source.kernel(self.validated_operations, &self.inverse)?;
                let group = source.publication.prepared_operation_metadata(self.validated_operations).unwrap().group_id.as_deref().unwrap().as_bytes();
                let count = (group.len() - self.group_offset).min(grant.maximum_copy_bytes / 2).min(32);
                if count == 0 && self.group_offset < group.len() { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let end = self.group_offset + count;
                if group[self.group_offset..end] != source.group_id.as_bytes()[self.group_offset..end] { return Err(Self::refusal("member receipt requires its exact original common group identity")); }
                self.group_offset = end;
                if end == group.len() { self.validated_operations += 1; self.group_offset = 0; }
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: count * 2, ..Default::default() }));
            } let birth = self.next_capacity_byte_demand(source)?; if grant.maximum_capacity_bytes < birth { return Ok(RetainedCloneStep::Progress(Default::default())); } *self.triples = VecDeque::with_capacity(self.operation_count); self.phase = 1; Ok(Self::progress(birth, 0)) }
            1 => { let text = source.publication.prepared_edit_id().unwrap().as_bytes(); let additional = (text.len() - self.edit_offset).min(64); if additional > 0 { if !self.edit.ready_for(additional) { return self.edit.reserve_step(additional, grant); } let step = self.edit.append_step(&text[self.edit_offset..self.edit_offset + additional], grant)?; self.edit_offset += step.progress().copied_bytes; return Ok(step); } self.phase = 2; Ok(Self::progress(0, 0)) }
            2 => { let encoder = self.inverse_encoder.as_mut().unwrap(); if !encoder.is_complete() { let original = match encoder.operation_index() { Some(index) => Some(source.publication.prepared_operation_wire_source(true, index).ok_or_else(|| Self::refusal("member receipt requires original inverse codec authority"))?), None => None }; return encoder.advance(original, grant); } *self.inverse = encoder.take(grant).unwrap(); self.inverse_encoder = None; self.phase = if self.operation_count == 0 { 5 } else { self.forward = Some(MountedPreparedOperationBytes::new()); 3 }; Ok(Self::progress(0, 0)) }
            3 => { if self.operation_schema.is_none() { let (entity, kind) = source.publication.prepared_operation_schema_parts(self.index).ok_or_else(|| Self::refusal("member receipt requires its current original operation schema"))?; self.operation_schema = Some([(entity.as_ptr() as usize, entity.len()), (kind.as_ptr() as usize, kind.len())]); } let forward = self.forward.as_mut().unwrap(); if !forward.is_complete() { let original = source.publication.prepared_operation_wire_source(false, self.index).ok_or_else(|| Self::refusal("member receipt requires original forward codec authority"))?; return forward.advance(original, grant); } let causal = source.kernel(self.index, &self.inverse)?; self.kernel = Some(MountedKernelMutationReceipt::new(&causal, forward.take(grant).unwrap())); self.forward = None; self.phase = 4; Ok(Self::progress(0, 0)) }
            4 => { let causal = source.kernel(self.index, &self.inverse)?; let kernel = self.kernel.as_mut().unwrap(); if !kernel.is_complete() { return kernel.advance(&causal, grant); } self.triples.push_back(kernel.take(grant).unwrap()); self.kernel = None; self.operation_schema = None; self.index += 1; self.phase = if self.index == self.operation_count { 5 } else { self.forward = Some(MountedPreparedOperationBytes::new()); 3 }; Ok(Self::progress(0, 0)) }
            5 => { let bytes = self.inverse.capacity(); if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); } drop(std::mem::take(&mut *self.inverse)); self.phase = 6; Ok(Self::progress(0, bytes)) }
            _ => Ok(RetainedCloneStep::Complete(Default::default())),
        }
    }
    pub(crate) fn is_complete(&self) -> bool { !self.closing && !self.transferred && self.phase == 6 && self.index == self.operation_count }
    pub(crate) fn ready_edit_id(&self) -> Option<&str> { self.is_complete().then(|| unsafe { std::str::from_utf8_unchecked(self.edit.as_slice()) }) }
    pub(crate) fn ready_document(&self) -> Option<ArtifactHandle> { self.is_complete().then_some(self.document) }
    pub(crate) fn operation_count(&self) -> usize { self.operation_count }
    /// 📤️ Moves one complete original causal triple in source order while retaining the remaining row owners.
    pub(crate) fn take_next_triple(&mut self, grant: RetainedCloneGrant) -> Option<MemberMutationTriple> {
        if !self.is_complete() || grant.maximum_items == 0 { return None; }
        self.triples.pop_front()
    }
    /// 🧳️ Transfers the original complete row backing and exact edit identity through constant moves.
    pub(crate) fn take(&mut self, grant: RetainedCloneGrant) -> Option<MountedMemberReceiptOutput> {
        if !self.is_complete() || grant.maximum_items == 0 { return None; }
        let edit_id = unsafe { String::from_utf8_unchecked(self.edit.take(grant)?) }; self.transferred = true;
        Some(MountedMemberReceiptOutput { document: self.document, edit_id, triples: std::mem::take(&mut *self.triples) })
    }
    pub(crate) fn next_close_byte_demand(&self) -> Result<usize, ValueError> {
        if self.transferred { return Ok(0); }
        if let Some(owner) = self.kernel.as_ref() { return Ok(owner.next_close_byte_demand()); }
        if let Some(owner) = self.forward.as_ref() { return owner.next_close_byte_demand(); }
        if let Some(owner) = self.inverse_encoder.as_ref() { return owner.next_close_byte_demand(); }
        if self.inverse.capacity() > 0 { return Ok(self.inverse.capacity()); }
        if !self.edit.terminal_is_empty() { return Ok(self.edit.next_close_byte_demand()); }
        Ok(self.triples.back().map_or(self.triples.capacity() * size_of::<MemberMutationTriple>(), |triple| match self.close_field { 0..=9 => MountedGroupReceipt::mutation_demand(&triple.0, self.close_field), 10 => triple.1.0.capacity(), 11..=14 => MountedGroupReceipt::inverse_demand(&triple.2, self.close_field - 11), _ => 0 }))
    }
    /// 📏️ Prices retained encoder paths separately from fixed causal release fields.
    pub(crate) fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if let Some(owner) = self.kernel.as_ref() { return mounted_nested_retirement_demand(owner.retirement_demands()?, 1); }
        if let Some(owner) = self.forward.as_ref() { return mounted_nested_retirement_demand(owner.retirement_demands()?, 1); }
        if let Some(owner) = self.inverse_encoder.as_ref() { return mounted_nested_retirement_demand(owner.retirement_demands()?, 1); }
        if self.inverse.capacity() > 0 { return Ok(semio_framework_value::RetirementDemand { release_bytes: self.inverse.capacity(), depth: 1, ..Default::default() }); }
        if !self.edit.terminal_is_empty() { return mounted_nested_retirement_demand(self.edit.retirement_demands()?, 1); }
        let depth = if self.triples.back().is_some() { match self.close_field { 7 | 8 | 14 => 4, _ => 3 } } else { 1 };
        Ok(semio_framework_value::RetirementDemand { release_bytes: self.next_close_byte_demand()?, depth, ..Default::default() })
    }
    /// 🍂️ Retires one exact original physical owner per turn without fractional release credit.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let bytes = self.next_close_byte_demand()?;
        if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !mounted_retirement_grant_funds(self.retirement_demands()?, grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.closing = true;
        if let Some(owner) = self.kernel.as_mut() { let step = owner.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }); if owner.terminal_is_empty() { self.kernel = None; } return Ok(step); }
        if let Some(owner) = self.forward.as_mut() { let step = owner.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?; if owner.terminal_is_empty() { self.forward = None; } return Ok(step); }
        if let Some(owner) = self.inverse_encoder.as_mut() { let step = owner.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?; if owner.terminal_is_empty() { self.inverse_encoder = None; } return Ok(step); }
        if self.inverse.capacity() > 0 { drop(std::mem::take(&mut *self.inverse)); return Ok(Self::progress(0, bytes)); }
        if !self.edit.terminal_is_empty() { return Ok(self.edit.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })); }
        if let Some(triple) = self.triples.back_mut() {
            let ready = match self.close_field { 0..=9 => MountedGroupReceipt::clear_mutation(&mut triple.0, self.close_field), 10 => { MountedGroupReceipt::clear_text(&mut triple.1.0); true }, 11..=14 => MountedGroupReceipt::clear_inverse(&mut triple.2, self.close_field - 11), _ => { self.triples.pop_back(); self.close_field = 0; return Ok(Self::progress(0, 0)); } };
            if ready { self.close_field += 1; }
        } else { drop(std::mem::take(&mut *self.triples)); }
        Ok(Self::progress(0, bytes))
    }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.transferred || (self.closing && self.kernel.is_none() && self.forward.is_none() && self.inverse_encoder.is_none() && self.inverse.capacity() == 0 && self.edit.terminal_is_empty() && self.triples.is_empty() && self.triples.capacity() == 0) }
}
impl Drop for MountedMemberReceipt { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "member receipt retains every original owner until complete transfer or funded close"); } }

#[cfg(test)]
include!("🧪️tests/🦀️.rs");
