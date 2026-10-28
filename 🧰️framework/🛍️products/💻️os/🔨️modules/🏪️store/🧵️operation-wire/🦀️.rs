//! 📦️ Prestage operation bytes from exact borrowed typed sources, with inline retained traversal.
use semio_framework_os_store_canonical_json::{ArtifactCanonicalJson, ArtifactCanonicalJsonCursor};
use semio_framework_pack_json::{ArtifactCanonicalJsonNode, ArtifactCanonicalJsonText, ArtifactCanonicalJsonTree};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_os_pack::record::{BorrowedProjectedPackCursor, BorrowedProjectedPackFailure};
use semio_framework_dsl_record::{BorrowedRecordSpec,native_encoding::FieldProjectionSource};

#[path = "📝️text/🦀️.rs"]
mod text;
pub use text::{ArtifactOperationImage, ArtifactOperationText, ArtifactOperationTextNode, ArtifactOperationTextCursor};

#[path = "📦️output/🦀️.rs"]
mod output;
pub use output::{ArtifactPreparedOperationOutput,ArtifactPreparedOperations,ArtifactPreparedOperationsOwner};
#[path="🧳️replacement/🦀️.rs"]
mod replacement;
pub use replacement::OriginalReplacementReceiving;
pub use semio_framework_os_store_one_item_publication::lifted_prepared_operation_wire::{ArtifactOperationTargets, ArtifactOriginalOperationReceiving, ArtifactCanonicalTreeSource, ArtifactOperationWire, ArtifactPreparedOperationError, ArtifactPreparedOperationSource, BatchDigestAdmission};

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

fn text_failure(error:ArtifactPreparedOperationError,header_bytes:usize)->ArtifactPreparedOperationError {
    let child=error.reason.retained_progress();
    let progress=child.checked_add(semio_framework_value::RetainedCloneProgress{copied_items:usize::from(header_bytes!=0&&child.copied_items==0),copied_bytes:header_bytes,..Default::default()}).expect("bounded original text and inline header receipts are representable");
    ArtifactPreparedOperationError{written_bytes:header_bytes.checked_add(error.written_bytes).expect("original text prefix is bounded by supplied output"),reason:error.reason.with_retained_progress(progress)}
}

enum ArtifactCodecFrontier{Json(ArtifactCanonicalJsonCursor),Text(ArtifactOperationTextCursor),Pack(BorrowedProjectedPackCursor)}

/// 🧭️ Inline traversal reborrows the exact prepared operation on every bounded copy turn.
#[derive(Default)]
pub struct ArtifactPreparedOperationCursor {
    source_identity: Option<(usize, usize, usize, usize, usize, u8)>,
    header_offset: usize,
    pack_header_length: usize,
    prefix_offset: usize,
    hex_pending: Option<u8>,
    hex_low: bool,
    pub complete: bool,
    json: ArtifactCanonicalJsonCursor,
    pub text: ArtifactOperationTextCursor,
    pub pack: BorrowedProjectedPackCursor,
    pack_started: bool,
    codec:Option<ArtifactCodecFrontier>,
    pub closed: bool,
}

impl ArtifactPreparedOperationCursor {
    fn json(&self)->&ArtifactCanonicalJsonCursor{match self.codec.as_ref(){Some(ArtifactCodecFrontier::Json(cursor))=>cursor,_=>unreachable!("original JSON codec frontier is installed")}}
    fn json_mut(&mut self)->&mut ArtifactCanonicalJsonCursor{match self.codec.as_mut(){Some(ArtifactCodecFrontier::Json(cursor))=>cursor,_=>unreachable!("original JSON codec frontier is installed")}}
    fn text(&self)->&ArtifactOperationTextCursor{match self.codec.as_ref(){Some(ArtifactCodecFrontier::Text(cursor))=>cursor,_=>unreachable!("original text codec frontier is installed")}}
    fn text_mut(&mut self)->&mut ArtifactOperationTextCursor{match self.codec.as_mut(){Some(ArtifactCodecFrontier::Text(cursor))=>cursor,_=>unreachable!("original text codec frontier is installed")}}
    fn pack(&self)->&BorrowedProjectedPackCursor{match self.codec.as_ref(){Some(ArtifactCodecFrontier::Pack(cursor))=>cursor,_=>unreachable!("original Pack codec frontier is installed")}}
    fn pack_mut(&mut self)->&mut BorrowedProjectedPackCursor{match self.codec.as_mut(){Some(ArtifactCodecFrontier::Pack(cursor))=>cursor,_=>unreachable!("original Pack codec frontier is installed")}}
    /// 🌱️ Quotes the one selected native codec, instead of constructing three unrelated traversal stacks.
    pub fn codec_birth_copy_bytes(&self)->usize{if self.codec.is_none(){size_of::<Option<ArtifactCodecFrontier>>()}else{0}}
    fn admit_codec(&mut self,source:ArtifactPreparedOperationSource<'_>,grant:RetainedCloneGrant)->Result<Option<ArtifactPreparedOperationProgress>,ArtifactPreparedOperationError>{
        let expected=match source{ArtifactPreparedOperationSource::Pack{..}=>2,ArtifactPreparedOperationSource::Text{..}=>1,_=>0};
        if let Some(codec)=&self.codec{let actual=match codec{ArtifactCodecFrontier::Pack(_)=>2,ArtifactCodecFrontier::Text(_)=>1,_=>0};if actual!=expected{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original codec kind changed").into())}return Ok(None)}
        let copied_bytes=self.codec_birth_copy_bytes();if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(Some(Default::default()))}
        self.codec=Some(match expected{2=>ArtifactCodecFrontier::Pack(Default::default()),1=>ArtifactCodecFrontier::Text(Default::default()),_=>ArtifactCodecFrontier::Json(Default::default())});Ok(Some(ArtifactPreparedOperationProgress{processed_items:1,copied_bytes,..Default::default()}))
    }
    /// ✍️ Writes at most sixty-four bytes into caller-funded storage without copying the source owner.
    pub fn advance(&mut self, source: ArtifactPreparedOperationSource<'_>, output: &mut [u8], grant: RetainedCloneGrant) -> Result<ArtifactPreparedOperationProgress, ArtifactPreparedOperationError> {
        if self.closed { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "prepared operation cursor is closed").into()); }
        if let Some(step)=self.admit_codec(source,grant)?{return Ok(step)}
        if let ArtifactPreparedOperationSource::Pack { tag, body, spec } = source {
            return self.advance_pack(tag,body,spec,output,grant);
        }
        if let ArtifactPreparedOperationSource::CanonicalJson { header, body } = source { return self.advance_json(header, &[], body, false, output, grant); }
        if let ArtifactPreparedOperationSource::HexJson { header, prefix, body } = source { return self.advance_json(header, prefix, body, true, output, grant); }
        let maximum = output.len().min(grant.maximum_copy_bytes).min(64);
        if grant.maximum_items == 0 || maximum == 0 { return Ok(ArtifactPreparedOperationProgress::default()); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "prepared operation wrapper requires its original parent depth").into()); }
        let (header, prefix, body_pointer, mode) = match source {
            ArtifactPreparedOperationSource::Pack { .. } => unreachable!(),
            ArtifactPreparedOperationSource::Text { header, body } => (header, &b""[..], body as *const dyn ArtifactOperationText as *const () as usize, 2),
            ArtifactPreparedOperationSource::CanonicalJson { header, body } => (header, &b""[..], body as *const dyn ArtifactCanonicalJson as *const () as usize, 0),
            ArtifactPreparedOperationSource::HexJson { header, prefix, body } => (header, prefix, body as *const dyn ArtifactCanonicalJson as *const () as usize, 1),
        };
        if header.len() > 11 || prefix.len() > 32 { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "prepared operation header exceeds declared inline capacity").into()); }
        let identity = (body_pointer, header.as_ptr() as usize, header.len(), prefix.as_ptr() as usize, prefix.len(), mode);
        if self.source_identity.is_some_and(|retained| retained != identity) { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "prepared operation source changed before completion").into()); }
        self.source_identity = Some(identity);
        let head = maximum.min(header.len() - self.header_offset);
        output[..head].copy_from_slice(&header[self.header_offset..self.header_offset + head]);
        self.header_offset += head;
        let prefix_count = (maximum - head).min(prefix.len() - self.prefix_offset);
        output[head..head + prefix_count].copy_from_slice(&prefix[self.prefix_offset..self.prefix_offset + prefix_count]);
        self.prefix_offset += prefix_count;
        let mut written = head + prefix_count;
        let mut copied = written;
        if mode == 2 && written < maximum {
            let ArtifactPreparedOperationSource::Text { body, .. } = source else { unreachable!() };
            let child = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - written, maximum_depth: grant.maximum_depth - 1, ..grant };
            written += match self.text_mut().advance(body, &mut output[written..maximum], child) { Ok(count) => count, Err(error) => { self.closed = true; return Err(text_failure(error,written)); } };
        }
        self.complete = self.header_offset == header.len() && self.prefix_offset == prefix.len() && self.hex_pending.is_none() && if mode == 2 { self.text().is_complete() } else { self.json().is_complete() };
        if mode != 1 { copied = written; }
        Ok(ArtifactPreparedOperationProgress { processed_items: 1, written_bytes: written, copied_bytes:copied, complete: self.complete, ..Default::default() })
    }

    fn advance_json(&mut self, header: &'static [u8], prefix: &'static [u8], body: &dyn ArtifactCanonicalJson, hex: bool, output: &mut [u8], grant: RetainedCloneGrant) -> Result<ArtifactPreparedOperationProgress, ArtifactPreparedOperationError> {
        if grant.maximum_items == 0 || output.is_empty() || grant.maximum_copy_bytes == 0 { return Ok(Default::default()); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "prepared JSON wrapper requires its original parent depth").into()); }
        if header.len() > 11 || prefix.len() > 32 { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "prepared JSON header exceeds declared inline capacity").into()); }
        let original = body.canonical_json_original_owner().unwrap_or(body);
        let identity = (original as *const dyn ArtifactCanonicalJson as *const () as usize, header.as_ptr() as usize, header.len(), prefix.as_ptr() as usize, prefix.len(), u8::from(hex));
        if self.source_identity.is_some_and(|original| original != identity) { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "prepared JSON original source changed").into()); }
        if self.source_identity.is_none() {
            let copied_bytes = size_of_val(&self.source_identity);
            if grant.maximum_copy_bytes < copied_bytes { return Ok(Default::default()); }
            self.source_identity = Some(identity);
            return Ok(ArtifactPreparedOperationProgress { processed_items: 1, copied_bytes, ..Default::default() });
        }
        if self.header_offset < header.len() || self.prefix_offset < prefix.len() {
            let (bytes, offset) = if self.header_offset < header.len() { (header, &mut self.header_offset) } else { (prefix, &mut self.prefix_offset) };
            let count = (bytes.len() - *offset).min(output.len()).min(grant.maximum_copy_bytes).min(64);
            output[..count].copy_from_slice(&bytes[*offset..*offset + count]); *offset += count;
            return Ok(ArtifactPreparedOperationProgress { processed_items: 1, written_bytes: count, copied_bytes: count, ..Default::default() });
        }
        if let Some(byte) = self.hex_pending {
            output[0] = b"0123456789abcdef"[usize::from(if self.hex_low { byte & 15 } else { byte >> 4 })];
            if self.hex_low { self.hex_pending = None; } else { self.hex_low = true; }
            self.complete = self.hex_pending.is_none() && self.json().is_complete();
            return Ok(ArtifactPreparedOperationProgress { processed_items: 1, written_bytes: 1, copied_bytes: 1, complete: self.complete, ..Default::default() });
        }
        let pending_copy = usize::from(hex && self.json().next_encode_output_bound() != 0);
        if grant.maximum_copy_bytes < pending_copy { return Ok(Default::default()); }
        let child = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - pending_copy, maximum_depth: grant.maximum_depth - 1, ..grant };
        let mut original_byte = [0];
        let step = if hex { self.json_mut().encode_chunk_admitted(body, &mut original_byte, child) } else { self.json_mut().encode_chunk_admitted(body, output, child) }.map_err(|error| ArtifactPreparedOperationError { written_bytes: error.written_bytes, reason: error.reason })?;
        let progress = step.ownership.progress();
        if hex && step.written_bytes != 0 { self.hex_pending = Some(original_byte[0]); self.hex_low = false; }
        self.complete = self.hex_pending.is_none() && self.json().is_complete();
        Ok(ArtifactPreparedOperationProgress { processed_items: progress.copied_items, written_bytes: if hex { 0 } else { step.written_bytes }, copied_bytes: progress.copied_bytes + usize::from(hex && step.written_bytes != 0), retained_capacity_bytes: progress.retained_capacity_bytes, released_bytes: progress.released_bytes, complete: self.complete })
    }

    fn advance_pack(&mut self,tag:u64,body:&dyn FieldProjectionSource,spec:BorrowedRecordSpec,output:&mut[u8],grant:RetainedCloneGrant)->Result<ArtifactPreparedOperationProgress,ArtifactPreparedOperationError> {
        self.pack_started=true;
        if grant.maximum_items==0 { return Ok(Default::default()); }
        let identity=(body as*const dyn FieldProjectionSource as*const() as usize,tag as u32 as usize,(tag>>32) as usize,0,0,3);
        if self.source_identity.is_some_and(|original|original!=identity) { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"prepared Pack original source/tag changed").into()); }
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
        let step=self.pack_mut().advance(body,spec,output,grant)?;self.complete=step.complete;
        Ok(ArtifactPreparedOperationProgress {processed_items:step.progress.copied_items,written_bytes:step.written_bytes,copied_bytes:step.progress.copied_bytes,retained_capacity_bytes:step.progress.retained_capacity_bytes,released_bytes:step.progress.released_bytes,complete:step.complete})
    }

    /// 🪙️ The same retained phase exposes whole symbol backing before physical allocation/release.
    pub fn next_capacity_byte_demand(&self) -> Result<usize,ValueError> { if self.source_identity.is_some_and(|identity|identity.5==3){self.pack().next_capacity_byte_demand()}else{Ok(0)} }
    pub fn next_close_byte_demand(&self) -> Result<usize,ValueError> { if self.source_identity.is_some_and(|identity|identity.5==3){self.pack().next_close_byte_demand()}else{Ok(0)} }
    /// 🛌️ Only a Pack source ever births symbol or frame scaffolds, so every other source leaves its pack cursor unstarted.
    fn pack_idle(&self) -> bool { !self.pack_started }
    pub fn next_minimum_copy_bytes(&self) -> usize {
        if self.closed || self.complete { return 0; }
        if self.codec.is_none(){return self.codec_birth_copy_bytes();}
        if self.source_identity.is_some_and(|identity|identity.5==3) && self.header_offset==self.pack_header_length { self.pack().next_minimum_copy_bytes() } else { 1 }
    }

    /// 📏️ Prices only the retained codec frontier, keeping payload encoding separate.
    pub fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.closed { return Ok(Default::default()); }
        if !self.source_identity.is_some_and(|identity|identity.5==3){return Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
        let mut demand = self.pack().retirement_demands()?;
        demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "operation codec retirement depth overflow"))?;
        Ok(demand)
    }

    /// 🍂️ Retires exact funded symbol scaffolds; the original operation stays in its publication.
    pub fn close(&mut self, grant: RetainedCloneGrant) -> Result<ArtifactPreparedOperationProgress,ArtifactPreparedOperationError> {
        if self.closed { return Ok(ArtifactPreparedOperationProgress { complete: true, ..Default::default() }); }
        if self.pack_idle() { self.source_identity = None; self.closed = true; return Ok(ArtifactPreparedOperationProgress { complete: true, ..Default::default() }); }
        let demand = self.retirement_demands()?;
        if grant.maximum_items == 0 || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(ArtifactPreparedOperationProgress::default()); }
        if !self.source_identity.is_some_and(|identity|identity.5==3){self.source_identity=None;self.closed=true;return Ok(ArtifactPreparedOperationProgress{processed_items:1,complete:true,..Default::default()})}
        let step=self.pack_mut().close(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?;
        if step.complete { self.source_identity=None;self.closed=true; }
        Ok(ArtifactPreparedOperationProgress {processed_items:step.progress.copied_items,retained_capacity_bytes:step.progress.retained_capacity_bytes,released_bytes:step.progress.released_bytes,copied_bytes:step.progress.copied_bytes,complete:step.complete,..Default::default()})
    }

}

/// 🌐️ Prices the exact wire wrapper and its original semantic factory constructor tree.
pub fn operation_wire_preparation_factory_birth_bytes<P: 'static, M: 'static>(inner_birth: usize) -> usize {
    semio_framework_value::factory_constructor_birth_bytes::<OperationWirePreparationFactory<P, M>>(inner_birth)
}

/// 🌳️ Quotes only the original wire and semantic factory Arcs before source construction.
pub fn operation_wire_preparation_factory_source_birth_demand<P: 'static, M: 'static>(inner: semio_framework_value::retained_clone::RetainedCloneBirthDemand) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError> {
    let capacity_bytes = semio_framework_value::factory_arc_birth_bytes::<OperationWirePreparationFactory<P, M>>().checked_add(inner.capacity_bytes).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "wire source capacity overflow"))?;
    let depth = inner.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "wire source depth overflow"))?;
    Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth })
}

/// 🧵️ Constructs the funded wire authority around the original semantic preparation factory.
pub fn operation_wire_preparation_factory<P: 'static, M: 'static>(factory: std::sync::Arc<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<P, M>>, source: for<'a> fn(&'a M) -> Option<ArtifactPreparedOperationSource<'a>>, schema: for<'a> fn(&'a M) -> Option<(&'a str, &'a str)>) -> std::sync::Arc<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<P, M>> {
    std::sync::Arc::new(OperationWirePreparationFactory { factory, source, schema })
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct OperationWirePreparationFactory<P, M> {
    #[factory_child]
    factory: std::sync::Arc<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<P, M>>,
    source: for<'a> fn(&'a M) -> Option<ArtifactPreparedOperationSource<'a>>,
    schema: for<'a> fn(&'a M) -> Option<(&'a str, &'a str)>,
}

impl<P: 'static, M: 'static> semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory<P, M> for OperationWirePreparationFactory<P, M> {
    fn begin_batch_digest(&self,edit:&mut Option<Box<semio_framework_os_spr::Edit<M>>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn semio_framework_os_store_one_item_publication::batch_digest::ArtifactStoreBatchDigest<M>>,semio_framework_value::RetainedCloneProgress)>,ValueError>{self.factory.begin_batch_digest(edit,grant)}
    fn operation_wire_source<'a>(&self, mutation: &'a M) -> Option<ArtifactPreparedOperationSource<'a>> { (self.source)(mutation) }
    fn operation_schema_parts<'a>(&'a self, mutation: &'a M) -> Option<(&'a str, &'a str)> { (self.schema)(mutation) }
    fn preflight(&self, mutation: &M, lane: semio_framework_os_store_command::HistoryLane) -> Result<semio_framework_os_store_one_item::ArtifactStoreOneItemFootprint, String> { self.factory.preflight(mutation, lane) }
    fn begin_copy_byte_demand(&self,mutation:&M,lane:semio_framework_os_store_command::HistoryLane)->Result<usize,ValueError>{self.factory.begin_copy_byte_demand(mutation,lane)}
    fn begin_demand(&self, mutation: &M, lane: semio_framework_os_store_command::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, ValueError> { self.factory.begin_demand(mutation, lane) }
    fn begin_copy_byte_demand(&self, mutation: &M, lane: semio_framework_os_store_command::HistoryLane) -> Result<usize, ValueError> { self.factory.begin_copy_byte_demand(mutation, lane) }
    fn begin(&self, request: semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationRequest<P, M, M>, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<(Box<dyn semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparation<P, M>>, semio_framework_value::retained_clone::RetainedCloneProgress), (ValueError, semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationRequest<P, M, M>)> { self.factory.begin(request, grant) }
    fn stamped_clock(&self) -> Option<semio_framework_os_spr::HybridLogicalTimestamp> { self.factory.stamped_clock() }
    fn stamped_mutation_id(&self) -> Option<&semio_framework_os_spr::MutationId> { self.factory.stamped_mutation_id() }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;
#[path="🧳️original/🦀️.rs"]
pub mod original_bytes;
pub use original_bytes::ArtifactOriginalOperationBytes;

#[path="📤️edit/🦀️.rs"]
pub mod original_edit;

#[path="📜️history/🦀️.rs"]
mod original_history;
pub use original_history::OriginalHistoryTransitionPayload;

#[path="🎯️common/🦀️.rs"]
mod original_common_target;
pub use original_common_target::OriginalCommonTarget;

#[path="📜️history/✉️envelope/🦀️.rs"]
mod original_history_envelope;
pub use original_history_envelope::OriginalHistoryEnvelopeHeader;
#[path="🧳️replacement/⚖️canonical/🦀️.rs"]
mod original_wire_comparison;
pub use original_wire_comparison::OriginalOperationWireComparison;

#[path="📜️history/📤️batch/🦀️.rs"]
mod original_history_batch_message;
pub use original_history_batch_message::OriginalHistoryBatchMessage;
