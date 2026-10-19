//! 📥️ Items of `canonical_edit` that belong to `os-store-one-item-publication` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧵️canonical-edit/🦀️.rs`.

use std::sync::Arc;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, DslValue, ErasedSnapshotRetirement, FromValue, SnapshotRetirementFactory, ToValue, ValueError};
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement;
use semio_framework_os_store_one_item::{ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep};
use crate::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPrepared};
use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use serde::{Deserialize, Serialize};
use semio_framework_os_spr::Edit;
use semio_framework_value::retained_clone::RetainedCloneBirthDemand;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_value_snapshot_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands};
use semio_framework_value::{RetirementDemand, FactoryAuthority, retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress}};
use crate::native_source::{ArtifactStoreCanonicalOwner, ArtifactStoreCanonicalSource};
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
use semio_framework_os_store_canonical_json::ARTIFACT_CANONICAL_JSON_CHUNK_BYTES;
use semio_framework_os_store_canonical_json::CANONICAL_EDIT_MAXIMUM_BYTES;
use semio_framework_os_store_canonical_json::CANONICAL_EDIT_MAXIMUM_OVERHEAD_BYTES;
//#region 🔣️ByteEncoder


pub struct ArtifactStoreOneItemAuthorityRetirement {
    pub owner: semio_framework_value::retirement::arc_custody::SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>,
}

impl ArtifactStoreOneItemAuthorityRetirement {
    pub fn new(authority: Arc<ArtifactStoreOneItemLiveAuthority>) -> Self {
        Self { owner: semio_framework_value::retirement::arc_custody::SharedControlledRetirement::lease(authority) }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreOneItemAuthorityRetirement {
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.owner.next_demand(body) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { self.owner.next_capacity_byte_demand(maximum_body_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.owner.next_depth_demand() }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.owner.step(grant) }
    fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
}

/// 📍️ Portable replay witness. Restoration re-executes each prior byte and verifies this prefix;
/// no supplied hash state or digest can directly create publication authority.
/// 🔮️ serde stays TEST-ONLY: this file's own round-trip test (`serde_json::to_vec`/
/// `from_slice` against `checkpoint()`) uses it as an independent differential oracle. Production
/// never serializes this type through serde.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[cfg_attr(any(test, feature = "testkit"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactStoreOneItemSealCheckpoint {
    pub version: u8,
    pub operation: u64,
    pub generation: u64,
    pub base_revision: [u8; 32],
    pub authority_digest: [u8; 32],
    pub phase: u8,
    pub completed_bytes: u64,
    pub canonical_bytes: u64,
    pub prefix_digest: [u8; 32],
}

/// 🔏️ Exact Store-owned edit, post-root, encoding state, and retirement lifecycle.
pub struct ArtifactStoreOneItemSealer<P, M> {
    pub native: Option<Box<dyn ArtifactStoreCanonicalOwner<M>>>,
    native_frame_bytes: usize,
    native_identities: Option<[semio_framework_value::paged::PagedUtf8<{usize::MAX}>;3]>,
    native_identities_close: Option<semio_framework_value::retirement::controlled::ControlledRetirement<(semio_framework_value::paged::PagedUtf8<{usize::MAX}>,semio_framework_value::paged::PagedUtf8<{usize::MAX}>,semio_framework_value::paged::PagedUtf8<{usize::MAX}>)>>,
    prepared_digest: Option<[u8;32]>,
    pub ownership: RetainedCloneProgress,
    pub authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    pub edit: Option<Box<Edit<M>>>,
    unboxed_edit: Option<Edit<M>>,
    pub post: Option<Arc<P>>,
    pub prepared: Option<ArtifactStoreOneItemPrepared<P, M>>,
    pub hash: semio_framework_hash::Sha256,
    transcript: semio_framework_hash::Sha256,
    pub phase: u8,
    pub header_offset: usize,
    pub canonical_bytes: u64,
    hashed_canonical_bytes: u64,
    pub completed_bytes: u64,
    pub turns: u32,
    last_chunk: [u8; ARTIFACT_CANONICAL_JSON_CHUNK_BYTES],
    last_length: usize,
    last_canonical: bool,
    pub replay: Option<ArtifactStoreOneItemSealCheckpoint>,
    pub mutation_retirement: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,
    pub snapshot_retirement: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    pub active_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    retirement_strings: [Option<String>; 3],
    pub factory_close: [Option<FactoryAuthority>; 2],
    pub identities: [Vec<u8>; 3],
    identity_index: usize,
    pub cancelled: bool,
    pub closing: bool,
}

impl<P, M> ArtifactStoreOneItemSealer<P, M> {
    /// 🎟️ Measures the original edit/source frames before native ownership transfer.
    pub fn constructor_demand() -> RetainedCloneBirthDemand where M: semio_framework_value::retirement::RetireOwned+Sync+ArtifactCanonicalJsonTree {
        RetainedCloneBirthDemand {
            capacity_bytes: ArtifactStoreCanonicalSource::<M>::source_constructor_demand().capacity_bytes,
            depth: 1,
        }
    }
    pub fn constructor_copy_byte_demand()->usize where M:semio_framework_value::retirement::RetireOwned+Sync+ArtifactCanonicalJsonTree{size_of::<Edit<M>>()+ArtifactStoreCanonicalSource::<M>::source_constructor_demand().copy_bytes+size_of::<Self>()}

    /// 🧳️ Retains every exact original input when constructor capacity or depth is refused.
    pub fn admit(
        authority: Arc<ArtifactStoreOneItemLiveAuthority>,
        edit: Edit<M>,
        post: Arc<P>,
        mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
        snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>,
        grant: RetainedCloneGrant,
    ) -> Result<(Self, RetainedCloneProgress), (ValueError, Arc<ArtifactStoreOneItemLiveAuthority>, Edit<M>, Arc<P>, Arc<dyn ArtifactOwnedValueRetirementFactory<M>>, Arc<dyn SnapshotRetirementFactory<P>>)> where M: semio_framework_value::retirement::RetireOwned+Sync+ArtifactCanonicalJsonTree {
        if grant.maximum_copy_bytes<Self::constructor_copy_byte_demand(){return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"canonical original source constructor lacks input transfer copy grant"),authority,edit,post,mutation_retirement,snapshot_retirement));}
        let progress = match Self::constructor_demand().admit(grant) {
            Ok(progress) => progress,
            Err(error) => return Err((error, authority, edit, post, mutation_retirement, snapshot_retirement)),
        };
        Ok((Self::new(authority, edit, post, mutation_retirement, snapshot_retirement), RetainedCloneProgress{copied_bytes:Self::constructor_copy_byte_demand(),..progress}))
    }

    pub(super) fn new(authority: Arc<ArtifactStoreOneItemLiveAuthority>, edit: Edit<M>, post: Arc<P>, mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>, snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>) -> Self where M: semio_framework_value::retirement::RetireOwned+Sync+ArtifactCanonicalJsonTree {
        Self {
            native: Some(Box::new(ArtifactStoreCanonicalSource::new(Arc::clone(&authority),Box::new(edit)))),
            native_frame_bytes:size_of::<ArtifactStoreCanonicalSource<M>>(),
            native_identities:None,
            native_identities_close:None,
            prepared_digest:None,
            ownership:Default::default(),
            authority: Some(authority),
            edit: None,
            unboxed_edit: None,
            post: Some(post),
            prepared: None,
            hash: semio_framework_hash::Sha256::new(),
            transcript: semio_framework_hash::Sha256::new(),
            phase: 0,
            header_offset: 0,
            canonical_bytes: 0,
            hashed_canonical_bytes: 0,
            completed_bytes: 0,
            turns: 0,
            last_chunk: [0; ARTIFACT_CANONICAL_JSON_CHUNK_BYTES],
            last_length: 0,
            last_canonical: false,
            replay: None,
            mutation_retirement: Some(mutation_retirement),
            snapshot_retirement: Some(snapshot_retirement),
            active_retirement: None,
            retirement_strings: Default::default(),
            factory_close: Default::default(),
            identities: std::array::from_fn(|_| Vec::new()),
            identity_index: 0,
            cancelled: false,
            closing: false,
        }
    }

    /// 🌐️ The original completed domain cursor funds this scalar declaration after sealing.
    pub fn record_foreign_step_presence(&mut self,presence:bool)->bool{if let Some(prepared)=self.prepared.as_mut(){prepared.record_foreign_step_presence(presence);true}else{false}}

    pub fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.as_ref()
    }
    /// 📥️ Installs the already funded ready diagnostic owner before exposing the sealed candidate witness.
    pub fn record_prepared_outcome_messages(&mut self, messages: semio_framework_os_spr::command::MutationMessagePublication) {
        self.prepared.as_mut().expect("ready diagnostic handoff requires a sealed candidate").record_outcome_messages(messages);
    }
    pub fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.take()
    }
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
        self.replay = None;
        if let Some(owner)=self.native.as_mut(){owner.begin_close();}
    }
    pub fn canonical_chunk(&self) -> &[u8] {
        if self.last_canonical { &self.last_chunk[..self.last_length] } else { &[] }
    }

    pub fn checkpoint(&self) -> ArtifactStoreOneItemSealCheckpoint {
        let authority = self.authority.as_ref().expect("nonterminal sealer retains its authority");
        ArtifactStoreOneItemSealCheckpoint {
            version: 1,
            operation: authority.operation().0,
            generation: authority.generation().0,
            base_revision: authority.base_revision(),
            authority_digest: self.authority_digest(),
            phase: self.phase,
            completed_bytes: self.completed_bytes,
            canonical_bytes: self.canonical_bytes,
            prefix_digest: self.transcript.clone().finalize(),
        }
    }

    pub fn restore_checkpoint(&mut self, checkpoint: ArtifactStoreOneItemSealCheckpoint) -> Result<(), String> {
        let authority = self.authority.as_ref().ok_or_else(|| "canonical-edit.authority-missing".to_string())?;
        if self.phase != 0
            || self.completed_bytes != 0
            || self.cancelled
            || self.closing
            || checkpoint.version != 1
            || checkpoint.phase > 6
            || checkpoint.operation != authority.operation().0
            || checkpoint.generation != authority.generation().0
            || checkpoint.base_revision != authority.base_revision()
            || checkpoint.authority_digest != self.authority_digest()
            || checkpoint.canonical_bytes > CANONICAL_EDIT_MAXIMUM_BYTES
            || checkpoint.completed_bytes > CANONICAL_EDIT_MAXIMUM_BYTES * 2 + CANONICAL_EDIT_MAXIMUM_OVERHEAD_BYTES
        {
            return Err("canonical-edit.checkpoint-authority".into());
        }
        self.replay = Some(checkpoint);
        Ok(())
    }

    pub fn progress(&self) -> ArtifactStoreOneItemCheckpoint {
        ArtifactStoreOneItemCheckpoint { cursor: self.turns, completed_items: self.turns, completed_bytes: self.completed_bytes, digest: self.prepared.as_ref().map(ArtifactStoreOneItemPrepared::edit_digest).or(self.prepared_digest).unwrap_or_else(||self.transcript.clone().finalize()) }
    }

    fn verify_replay(&mut self) -> Result<(), String> {
        let Some(target) = self.replay else {
            return Ok(());
        };
        if self.completed_bytes == target.completed_bytes && self.phase == target.phase {
            if self.canonical_bytes != target.canonical_bytes || self.transcript.clone().finalize() != target.prefix_digest {
                return Err("canonical-edit.checkpoint-prefix".into());
            }
            self.replay = None;
        } else if self.completed_bytes > target.completed_bytes || self.phase > target.phase || self.phase == 6 {
            return Err("canonical-edit.checkpoint-position".into());
        }
        Ok(())
    }

    fn header_byte(&self, offset: usize) -> Option<u8> {
        let id = self.edit.as_ref()?.id.as_bytes();
        let fixed = b"semio.artifact.cursor.v2";
        let mut at = offset;
        for segment in [fixed.as_slice(), &4u64.to_be_bytes(), b"edit".as_slice(), &(id.len() as u64).to_be_bytes(), id, &self.canonical_bytes.to_be_bytes()] {
            if at < segment.len() {
                return Some(segment[at]);
            }
            at -= segment.len();
        }
        None
    }

    pub fn authority_digest(&self) -> [u8; 32] {
        let authority = self.authority.as_ref().expect("nonterminal sealer retains authority");
        CursorRevisionAccumulator::hash_record(
            b"one-item-authority",
            &[
                &authority.operation().0.to_be_bytes(),
                &authority.generation().0.to_be_bytes(),
                &authority.base_revision(),
                &authority.next_sequence_number().to_be_bytes(),
                &authority.next_clock().actor.to_be_bytes(),
                &authority.next_clock().physical_ms.to_be_bytes(),
                &authority.next_clock().logical.to_be_bytes(),
                authority.actor().as_bytes(),
                &[u8::from(authority.line_id().is_some())],
                authority.line_id().unwrap_or("").as_bytes(),
                &[u8::from(authority.group_id().is_some())],
                authority.group_id().unwrap_or("").as_bytes(),
            ],
        )
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.native.is_none()
            && self.native_identities.is_none()
            && self.native_identities_close.is_none()
            && self.authority.is_none()
            && self.edit.is_none()
            && self.unboxed_edit.is_none()
            && self.post.is_none()
            && self.prepared.is_none()
            && self.active_retirement.is_none()
            && self.retirement_strings.iter().all(Option::is_none)
            && self.mutation_retirement.is_none()
            && self.snapshot_retirement.is_none()
            && self.identities.iter().all(|value| value.capacity() == 0)
            && self.factory_close.iter().all(Option::is_none)
    }
}

impl<P: Send + Sync + 'static, M: Send + 'static> ArtifactStoreOneItemSealer<P, M> {
    /// 📐️ Quotes the actual selected native child or the separately retained source-frame/publication handoff.
    pub fn preparation_demands(&self) -> Result<RetirementDemand,ValueError> {
        if let Some(owner)=self.native.as_ref(){
            if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<dyn ArtifactStoreCanonicalOwner<M>>>>(),release_bytes:self.native_frame_bytes,depth:1,..Default::default()});}
            let mut demand=owner.next_demand()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"canonical preparation child depth overflow"))?;return Ok(demand);
        }
        if self.prepared.is_some(){return Ok(Default::default());}
        Ok(RetirementDemand{copy_bytes:size_of::<ArtifactStoreOneItemPrepared<P,M>>()+size_of::<Box<Edit<M>>>()+size_of::<Arc<P>>()+size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>()+size_of::<[semio_framework_value::paged::PagedUtf8<{usize::MAX}>;3]>(),depth:1,..Default::default()})
    }
    /// 🧾️ Returns every actual currency from the preceding canonical preparation turn.
    pub fn ownership_progress(&self)->RetainedCloneProgress{self.ownership}
    pub fn advance(&mut self, grant: ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        self.ownership=Default::default();self.last_length=0;self.last_canonical=false;
        if self.cancelled||self.closing{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
        if self.prepared.is_some(){return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.progress(),self.ownership));}
        if !grant.permits_one(){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
        if self.replay.is_some(){self.cancelled=true;return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical-edit.native-checkpoint-unsupported"));}
        let demand=match self.preparation_demands(){Ok(demand)=>demand,Err(error)=>{self.cancelled=true;self.ownership=error.retained_progress();return Err(error)}};
        if demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes||demand.depth>grant.maximum_depth{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
        let retained=grant.retained_grant();
        let result=(||->Result<RetainedCloneProgress,ValueError>{
            if let Some(owner)=self.native.as_mut(){
                if owner.terminal_is_empty(){self.native=None;self.phase=5;return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()});}
                let child=RetainedCloneGrant{maximum_items:1,maximum_depth:retained.maximum_depth-1,..retained};
                if owner.ready(){
                    let(edit,identities,digest,receipt)=owner.take(child)?.ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical-edit.original-return-refused"))?;
                    self.edit=Some(edit);self.native_identities=Some(identities);self.prepared_digest=Some(digest);return Ok(receipt);
                }
                return Ok(owner.advance(child)?.progress());
            }
            if self.authority.is_none()||self.edit.is_none()||self.post.is_none()||self.native_identities.is_none()||self.prepared_digest.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical-edit.original-prepared-owners-missing"));}
            let authority=self.authority.as_ref().expect("validated original publication authority");
            let edit=self.edit.take().expect("validated original edit");
            let post=self.post.take().expect("validated original post");
            let identities=self.native_identities.take().expect("validated original identities");
            let digest=self.prepared_digest.take().expect("validated original digest");
            self.prepared=Some(authority.seal_prepared_owned(edit,post,digest,identities));self.phase=6;
            Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
        })();
        let (progress,error)=match result{Ok(progress)=>(progress,None),Err(error)=>(error.retained_progress(),Some(error))};
        self.ownership=progress;
        if error.is_some()||!progress.fits(retained){self.cancelled=true;}
        let bytes=progress.copied_bytes.checked_add(progress.retained_capacity_bytes).and_then(|bytes|bytes.checked_add(progress.released_bytes)).and_then(|bytes|u64::try_from(bytes).ok());
        if progress!=RetainedCloneProgress::default(){
            let Some((completed,turns))=bytes.and_then(|bytes|self.completed_bytes.checked_add(bytes)).zip(self.turns.checked_add(1))else{self.cancelled=true;return Err(error.unwrap_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"canonical-edit.work-overflow").with_retained_progress(progress)));};
            self.completed_bytes=completed;self.turns=turns;
        }
        if let Some(error)=error{return Err(error);}
        if !progress.fits(retained){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"canonical-edit.original-child-receipt-exceeds-grant").with_retained_progress(progress));}
        Ok(if self.prepared.is_some(){ArtifactStoreOneItemPreparationStep::Prepared(self.progress(),progress)}else if progress==RetainedCloneProgress::default(){ArtifactStoreOneItemPreparationStep::Blocked}else{ArtifactStoreOneItemPreparationStep::Progress(self.progress(),progress)})
    }
}

impl<P: Send + Sync + 'static, M: Send + 'static> ArtifactStoreOneItemSealer<P, M> {
    pub fn retirement_demands(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
        let depth_error = || ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "canonical sealer depth overflow");
        if let Some(owner)=self.native.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<dyn ArtifactStoreCanonicalOwner<M>>>>(),release_bytes:self.native_frame_bytes,depth:1,..Default::default()});}let mut demand=owner.next_demand()?;demand.depth=demand.depth.checked_add(1).ok_or_else(depth_error)?;return Ok(demand);}
        if self.native_identities.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<[semio_framework_value::paged::PagedUtf8<{usize::MAX}>;3]>()+size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<(semio_framework_value::paged::PagedUtf8<{usize::MAX}>,semio_framework_value::paged::PagedUtf8<{usize::MAX}>,semio_framework_value::paged::PagedUtf8<{usize::MAX}>)>>(),depth:1,..Default::default()});}
        if let Some(owner)=self.native_identities_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of_val(&self.native_identities_close),depth:1,..Default::default()});}let body=owner.next_copy_byte_demand()?;return Ok(RetirementDemand{copy_bytes:body,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(depth_error)?});}
        if let Some(active) = self.active_retirement.as_ref() { let mut demand = artifact_retirement_box_demands(active, maximum_body_bytes)?; demand.depth = demand.depth.checked_add(1).ok_or_else(depth_error)?; return Ok(demand); }
        if let Some(bytes) = self.identities.iter().find(|bytes| bytes.capacity() != 0) { return Ok(RetirementDemand { release_bytes: bytes.capacity(), depth: 1, ..Default::default() }); }
        if self.prepared.is_some() { let birth=ArtifactStoreOneItemPrepared::<P,M>::retirement_birth_demand();return Ok(RetirementDemand { capacity_bytes:birth.capacity_bytes,depth:birth.depth+1, ..Default::default() }); }
        if self.edit.is_some() { return Ok(RetirementDemand { release_bytes: size_of::<Edit<M>>(), depth: 1, ..Default::default() }); }
        if self.unboxed_edit.is_some() { return Ok(RetirementDemand { capacity_bytes: size_of::<ArtifactStoreDecodedEditRetirement<M>>(), depth: 2, ..Default::default() }); }
        if let Some(post) = self.post.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.snapshot_retirement.as_ref().expect("original sealer snapshot factory").retirement_birth_bytes(post), depth: 2, ..Default::default() }); }
        if let Some(value) = self.retirement_strings.iter().find_map(Option::as_ref) { return Ok(RetirementDemand { release_bytes: value.capacity(), depth: 1, ..Default::default() }); }
        if let Some(authority) = self.authority.as_ref() { let birth = authority.retirement_birth_demand(); return Ok(RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth.checked_add(1).ok_or_else(depth_error)?, ..Default::default() }); }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(factory) = self.factory_close.iter().find_map(Option::as_ref) { let mut demand = factory.demands(maximum_body_bytes)?; demand.depth = demand.depth.checked_add(1).ok_or_else(depth_error)?; return Ok(demand); }
        Ok(Default::default())
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "canonical sealer exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(owner)=self.native.as_mut(){if owner.terminal_is_empty(){self.native=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}));}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};return owner.advance(child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some([a,b,c])=self.native_identities.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new((a,b,c)){Ok(owner)=>self.native_identities_close=Some(owner),Err((error,(a,b,c)))=>{self.native_identities=Some([a,b,c]);return Err(error);}}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if let Some(owner)=self.native_identities_close.as_mut(){if owner.terminal_is_empty(){self.native_identities_close=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};return owner.step(child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active_retirement.is_some() { return artifact_retirement_box_close_step(&mut self.active_retirement, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(bytes) = self.identities.iter_mut().find(|bytes| bytes.capacity() != 0) {
            let released_bytes = bytes.capacity();
            *bytes = Vec::new();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }));
        }
        if let Some(prepared) = self.prepared.take() {
            let mutation_factory=self.mutation_retirement.take().expect("original sealer mutation issuer");let snapshot_factory=self.snapshot_retirement.take().expect("original sealer root issuer");
            return match prepared.admit_retirement(mutation_factory,snapshot_factory,child){Ok((owner,receipt))=>{self.active_retirement=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original,mutations,snapshots))=>{self.prepared=Some(original);self.mutation_retirement=Some(mutations);self.snapshot_retirement=Some(snapshots);Err(error)}};
        }
        if let Some(edit) = self.edit.take() {
            self.unboxed_edit = Some(*edit);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
        }
        if let Some(edit) = self.unboxed_edit.take() {
            let factory = self.mutation_retirement.as_ref().expect("original mutation factory");
            return match admit_artifact_retirement(edit, child, |original| ArtifactStoreDecodedEditRetirement::new(original, Arc::clone(factory))) {
                Ok((active, progress)) => { self.active_retirement = Some(active); Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original)) => { self.unboxed_edit = Some(original); Err(error) },
            };
        }
        if let Some(post) = self.post.take() {
            return match self.snapshot_retirement.as_ref().expect("original snapshot factory").retire(post, child) {
                Ok((active, progress)) => { self.active_retirement = Some(active); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical sealer constructor changed its receipt")); } Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original)) => { self.post = Some(original); Err(error) },
            };
        }
        if let Some(value) = self.retirement_strings.iter_mut().find_map(Option::take) {
            let released_bytes = value.capacity();
            drop(value);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }));
        }
        if let Some(authority) = self.authority.take() {
            return match authority.retire(child) {
                Ok((active, progress)) => { self.active_retirement = Some(active); Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original)) => { self.authority = Some(original); Err(error) },
            };
        }
        if let Some(factory) = self.mutation_retirement.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_close[0] = Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(factory) = self.snapshot_retirement.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_close[1] = Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(slot) = self.factory_close.iter_mut().find(|slot| slot.is_some()) {
            let factory = slot.as_mut().unwrap();
            let step = factory.step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "canonical sealer factory")?;
            if factory.terminal_is_empty() { *slot = None; }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}

impl<P, M> Drop for ArtifactStoreOneItemSealer<P, M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "canonical edit sealer dropped before exact owners were transferred or retired");
    }
}
//#endregion 🔣️ByteEncoder
