//! 📦️ Prestage operation bytes from exact borrowed typed sources, with inline retained traversal.
use super::{ArtifactCanonicalJson, ArtifactCanonicalJsonCursor};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::os_pack::record::BorrowedProjectedPackCursor;
use semio_framework_dsl_record::{BorrowedRecordSpec,native_encoding::FieldProjectionSource};

#[path = "📝️text/🦀️.rs"]
mod text;
pub use text::{ArtifactOperationText, ArtifactOperationTextNode, ArtifactOperationTextCursor};

/// 🫳️ Installed domain codec authority over one original immutable typed operation.
#[derive(Clone, Copy)]
pub enum ArtifactPreparedOperationSource<'a> {
    Pack { tag: u64, body: &'a dyn FieldProjectionSource, spec: BorrowedRecordSpec },
    Text { header: &'static [u8], body: &'a dyn ArtifactOperationText },
    CanonicalJson { header: &'static [u8], body: &'a dyn ArtifactCanonicalJson },
    HexJson { header: &'static [u8], prefix: &'static [u8], body: &'a dyn ArtifactCanonicalJson },
}

/// 📏️ One admitted operation-wire turn, independent of allocation and release axes.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct ArtifactPreparedOperationProgress {
    pub processed_items: usize,
    pub written_bytes: usize,
    pub copied_bytes: usize,
    pub retained_capacity_bytes: usize,
    pub released_bytes: usize,
    pub complete: bool,
}

/// ⚠️ Exact initialized output prefix remains visible when a domain source refuses.
#[derive(Debug)]
pub struct ArtifactPreparedOperationError {
    pub written_bytes: usize,
    pub reason: ValueError,
}

impl From<ValueError> for ArtifactPreparedOperationError {
    fn from(reason: ValueError) -> Self { Self { written_bytes: 0, reason } }
}

/// 🧭️ Inline traversal reborrows the exact prepared operation on every bounded copy turn.
#[derive(Default)]
pub struct ArtifactPreparedOperationCursor {
    source_identity: Option<(usize, usize, usize, usize, usize, u8)>,
    header_offset: usize,
    pack_header_length: usize,
    prefix_offset: usize,
    hex_pending: Option<u8>,
    hex_low: bool,
    complete: bool,
    json: ArtifactCanonicalJsonCursor,
    text: ArtifactOperationTextCursor,
    pack: BorrowedProjectedPackCursor,
    closed: bool,
}

impl ArtifactPreparedOperationCursor {
    /// ✍️ Writes at most sixty-four bytes into caller-funded storage without copying the source owner.
    pub fn advance(&mut self, source: ArtifactPreparedOperationSource<'_>, output: &mut [u8], grant: RetainedCloneGrant) -> Result<ArtifactPreparedOperationProgress, ArtifactPreparedOperationError> {
        if self.closed { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "prepared operation cursor is closed").into()); }
        if let ArtifactPreparedOperationSource::Pack { tag, body, spec } = source {
            return self.advance_pack(tag,body,spec,output,grant);
        }
        let maximum = output.len().min(grant.maximum_copy_bytes).min(64);
        if grant.maximum_items == 0 || maximum == 0 { return Ok(ArtifactPreparedOperationProgress::default()); }
        if grant.maximum_depth < super::ARTIFACT_CANONICAL_JSON_DEPTH { return Err(ValueError::new(ValueRefusalKind::DepthLimit, "prepared operation traversal requires its declared inline depth").into()); }
        let (header, prefix, body_pointer, mode) = match source {
            ArtifactPreparedOperationSource::Pack { .. } => unreachable!(),
            ArtifactPreparedOperationSource::Text { header, body } => (header, &b""[..], body as *const dyn ArtifactOperationText as *const () as usize, 2),
            ArtifactPreparedOperationSource::CanonicalJson { header, body } => (header, &b""[..], body as *const dyn ArtifactCanonicalJson as *const () as usize, 0),
            ArtifactPreparedOperationSource::HexJson { header, prefix, body } => (header, prefix, body as *const dyn ArtifactCanonicalJson as *const () as usize, 1),
        };
        if header.len() > 11 || prefix.len() > 32 { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "prepared operation header exceeds declared inline capacity").into()); }
        let identity = (body_pointer, header.as_ptr() as usize, header.len(), prefix.as_ptr() as usize, prefix.len(), mode);
        if self.source_identity.is_some_and(|retained| retained != identity) { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "prepared operation source changed before completion").into()); }
        self.source_identity = Some(identity);
        let head = maximum.min(header.len() - self.header_offset);
        output[..head].copy_from_slice(&header[self.header_offset..self.header_offset + head]);
        self.header_offset += head;
        let prefix_count = (maximum - head).min(prefix.len() - self.prefix_offset);
        output[head..head + prefix_count].copy_from_slice(&prefix[self.prefix_offset..self.prefix_offset + prefix_count]);
        self.prefix_offset += prefix_count;
        let mut written = head + prefix_count;
        let mut copied = written;
        let json_body = match source { ArtifactPreparedOperationSource::CanonicalJson { body, .. } | ArtifactPreparedOperationSource::HexJson { body, .. } => Some(body), _ => None };
        if mode == 1 {
            while written < maximum && copied < grant.maximum_copy_bytes.min(64) {
                if self.hex_pending.is_none() {
                    let mut byte = [0];
                    let count = self.json.encode_chunk(json_body.unwrap(), &mut byte).map_err(|error| ArtifactPreparedOperationError { written_bytes: written, reason: ValueError::new(ValueRefusalKind::InvariantViolated, error.reason) })?;
                    if count == 0 { break; }
                    copied += count;
                    self.hex_pending = Some(byte[0]);
                    self.hex_low = false;
                }
                if copied == grant.maximum_copy_bytes.min(64) { break; }
                let byte = self.hex_pending.unwrap();
                output[written] = b"0123456789abcdef"[usize::from(if self.hex_low { byte & 15 } else { byte >> 4 })];
                written += 1;
                copied += 1;
                if self.hex_low { self.hex_pending = None; } else { self.hex_low = true; }
            }
        } else if mode == 2 && written < maximum {
            let ArtifactPreparedOperationSource::Text { body, .. } = source else { unreachable!() };
            written += self.text.advance(body, &mut output[written..maximum]).map_err(|reason| ArtifactPreparedOperationError { written_bytes: written, reason })?;
        } else if mode == 0 && written < maximum {
            written += self.json.encode_chunk(json_body.unwrap(), &mut output[written..maximum]).map_err(|error| ArtifactPreparedOperationError { written_bytes: written + error.written_bytes, reason: ValueError::new(ValueRefusalKind::InvariantViolated, error.reason) })?;
        }
        self.complete = self.header_offset == header.len() && self.prefix_offset == prefix.len() && self.hex_pending.is_none() && if mode == 2 { self.text.is_complete() } else { self.json.is_complete() };
        if mode != 1 { copied = written; }
        Ok(ArtifactPreparedOperationProgress { processed_items: 1, written_bytes: written, copied_bytes:copied, complete: self.complete, ..Default::default() })
    }

    fn advance_pack(&mut self,tag:u64,body:&dyn FieldProjectionSource,spec:BorrowedRecordSpec,output:&mut[u8],grant:RetainedCloneGrant)->Result<ArtifactPreparedOperationProgress,ArtifactPreparedOperationError> {
        if grant.maximum_items==0 { return Ok(Default::default()); }
        let identity=(body as*const dyn FieldProjectionSource as*const() as usize,tag as u32 as usize,(tag>>32) as usize,0,0,3);
        if self.source_identity.is_some_and(|original|original!=identity) { return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"prepared Pack original source/tag changed").into()); }
        let mut header=[0;11];header[0]=1;let mut length=1;let mut word=tag;
        loop { let byte=(word&127) as u8;word>>=7;header[length]=byte|if word==0{0}else{128};length+=1;if word==0{break} }
        self.pack_header_length=length;
        if self.header_offset<length {
            let count=output.len().min(grant.maximum_copy_bytes).min(64).min(length-self.header_offset);
            if count==0 { return Ok(Default::default()); }
            self.source_identity=Some(identity);output[..count].copy_from_slice(&header[self.header_offset..self.header_offset+count]);self.header_offset+=count;
            return Ok(ArtifactPreparedOperationProgress {processed_items:1,written_bytes:count,copied_bytes:count,..Default::default()});
        }
        self.source_identity=Some(identity);
        let step=self.pack.advance(body,spec,output,grant)?;self.complete=step.complete;
        Ok(ArtifactPreparedOperationProgress {processed_items:step.progress.copied_items,written_bytes:step.written_bytes,copied_bytes:step.progress.copied_bytes,retained_capacity_bytes:step.progress.retained_capacity_bytes,released_bytes:step.progress.released_bytes,complete:step.complete})
    }

    /// 🪙️ The same retained phase exposes whole symbol backing before physical allocation/release.
    pub fn next_capacity_byte_demand(&self) -> Result<usize,ValueError> { self.pack.next_capacity_byte_demand() }
    pub fn next_close_byte_demand(&self) -> Result<usize,ValueError> { self.pack.next_close_byte_demand() }
    pub fn next_minimum_copy_bytes(&self) -> usize {
        if self.closed || self.complete { return 0; }
        if self.source_identity.is_some_and(|identity|identity.5==3) && self.header_offset==self.pack_header_length { self.pack.next_minimum_copy_bytes() } else { 1 }
    }

    /// 📏️ Prices only the retained codec frontier, keeping payload encoding separate.
    pub fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.closed { return Ok(Default::default()); }
        let mut demand = self.pack.retirement_demands()?;
        demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "operation codec retirement depth overflow"))?;
        Ok(demand)
    }

    /// 🍂️ Retires exact funded symbol scaffolds; the original operation stays in its publication.
    pub fn close(&mut self, grant: RetainedCloneGrant) -> Result<ArtifactPreparedOperationProgress,ArtifactPreparedOperationError> {
        if self.closed { return Ok(ArtifactPreparedOperationProgress { complete: true, ..Default::default() }); }
        let demand = self.retirement_demands()?;
        if grant.maximum_items == 0 || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(ArtifactPreparedOperationProgress::default()); }
        let step=self.pack.close(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?;
        if step.complete { self.source_identity=None;self.closed=true; }
        Ok(ArtifactPreparedOperationProgress {processed_items:step.progress.copied_items,retained_capacity_bytes:step.progress.retained_capacity_bytes,released_bytes:step.progress.released_bytes,copied_bytes:step.progress.copied_bytes,complete:step.complete,..Default::default()})
    }

}

/// 🌐️ Prices the exact wire wrapper and its original semantic factory constructor tree.
pub fn operation_wire_preparation_factory_birth_bytes<P: 'static, M: 'static>(inner_birth: usize) -> usize {
    semio_framework_value::factory_constructor_birth_bytes::<OperationWirePreparationFactory<P, M>>(inner_birth)
}

/// 🌳️ Quotes only the original wire and semantic factory Arcs before source construction.
pub fn operation_wire_preparation_factory_source_birth_demand<P: 'static, M: 'static>(inner: semio_framework_value::retained_clone::RetainedCloneBirthDemand) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
    let capacity_bytes = semio_framework_value::factory_arc_birth_bytes::<OperationWirePreparationFactory<P, M>>().checked_add(inner.capacity_bytes).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "wire source capacity overflow"))?;
    let depth = inner.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "wire source depth overflow"))?;
    Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth })
}

/// 🧵️ Constructs the funded wire authority around the original semantic preparation factory.
pub fn operation_wire_preparation_factory<P: 'static, M: 'static>(factory: std::sync::Arc<dyn super::ArtifactStoreOneItemPreparationFactory<P, M>>, source: for<'a> fn(&'a M) -> Option<ArtifactPreparedOperationSource<'a>>, schema: for<'a> fn(&'a M) -> Option<(&'a str, &'a str)>) -> std::sync::Arc<dyn super::ArtifactStoreOneItemPreparationFactory<P, M>> {
    std::sync::Arc::new(OperationWirePreparationFactory { factory, source, schema })
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct OperationWirePreparationFactory<P, M> {
    #[factory_child]
    factory: std::sync::Arc<dyn super::ArtifactStoreOneItemPreparationFactory<P, M>>,
    source: for<'a> fn(&'a M) -> Option<ArtifactPreparedOperationSource<'a>>,
    schema: for<'a> fn(&'a M) -> Option<(&'a str, &'a str)>,
}

impl<P: 'static, M: 'static> super::ArtifactStoreOneItemPreparationFactory<P, M> for OperationWirePreparationFactory<P, M> {
    fn begin_batch_digest(&self,edit:&mut Option<Box<crate::os_spr::Edit<M>>>,grant:semio_framework_value::RetainedCloneGrant)->Result<Option<(Box<dyn super::ArtifactStoreBatchDigest<M>>,semio_framework_value::RetainedCloneProgress)>,semio_framework_value::ValueError>{self.factory.begin_batch_digest(edit,grant)}
    fn operation_wire_source<'a>(&self, mutation: &'a M) -> Option<ArtifactPreparedOperationSource<'a>> { (self.source)(mutation) }
    fn operation_schema_parts<'a>(&'a self, mutation: &'a M) -> Option<(&'a str, &'a str)> { (self.schema)(mutation) }
    fn preflight(&self, mutation: &M, lane: super::HistoryLane) -> Result<super::ArtifactStoreOneItemFootprint, String> { self.factory.preflight(mutation, lane) }
    fn begin_demand(&self, mutation: &M, lane: super::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> { self.factory.begin_demand(mutation, lane) }
    fn begin(&self, request: super::ArtifactStoreOneItemPreparationRequest<P, M>, grant: super::ArtifactStoreOneItemGrant) -> Result<(Box<dyn super::ArtifactStoreOneItemPreparation<P, M>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, super::ArtifactStoreOneItemPreparationRequest<P, M>)> { self.factory.begin(request, grant) }
    fn stamped_clock(&self) -> Option<super::HybridLogicalTimestamp> { self.factory.stamped_clock() }
    fn stamped_mutation_id(&self) -> Option<&super::MutationId> { self.factory.stamped_mutation_id() }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
