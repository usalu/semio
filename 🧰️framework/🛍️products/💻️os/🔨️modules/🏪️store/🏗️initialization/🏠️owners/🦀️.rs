//! 🏠️ The Store's owner catalog: the document and member owner catalogs an `ArtifactStore` installs (retirement
//! factories, preparation authorities and the store disposer), their unscheduled completion, and the framework's bounded
//! catalog constructors. The catalog carries the engine's disposer, so it lives with the engine, above the vocabulary.
//!
//! Layering: `📓️2026-10-13-wave-e1b-kernel-seams.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
#[path = "📥️lifted/🏭️operation/🦀️.rs"]
pub mod lifted_operation;
use semio_framework_os_store_read::retire_document_envelope;
use semio_framework_os_store_read::retirement_owner_demands;
use semio_framework_os_store_read::retirement_owner_close;
use semio_framework_os_store_read::ArtifactStoreEnvelopeRetirement;
use crate::lifted_store::ArtifactStoreCursorDisposer;
use semio_framework_os_store_read::ArtifactGenesisRetirement;
use crate::lifted_operation::MemberSnapshotOpenOperation;
use semio_framework_os_store_read::history_read_admission;
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirementFactory;
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement;
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreChangeRetirementFactory;
use semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_store_replay::replay_preparation::{ArtifactReplayPreparation, ArtifactReplayPreparationFactory, ArtifactReplayPreparationProgress};
use semio_framework_os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::MutationId;
use semio_framework_os_spr::{DiffRegions, Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_value_snapshot_read::SnapshotReadRegistryHandle;
use semio_framework_value_snapshot_read::{SnapshotReadRegistryAliasRetirement,snapshot_registry_alias_demands,snapshot_registry_alias_close_step};
use semio_framework_os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_vcs::{Alternative, ArtifactHistoryLedger, ArtifactVcs, Change, Checkpoint, Identified, Patchable};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_pack::pack_rt;
use semio_framework_os_store_ledger::{ArtifactEditMessageLedgerRetirement, CursorRevisionAccumulator};
use semio_framework_os_store_canonical_json::{ArtifactCanonicalJson, ArtifactCanonicalJsonTree};
use semio_framework_os_store_operation_wire::{ArtifactOperationWire, ArtifactOperationText};
use semio_framework_os_store_one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
use semio_framework_os_store_space_schema::{ArtifactBackboneRef, ArtifactCursor, ArtifactEnvelope, ArtifactEnvelopeOwners, HistoryLane, MigrationProvenance, OpenToolTransaction};
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPreparationFactory;
use crate::lifted_store::ArtifactStoreOwnedDisposer;
use semio_framework_os_store_replay::{EditReplay, EditReplayResult, EffectiveSupersessions, ReplayProgress, retire_shared_projection};
use semio_framework_os_store_one_item_publication::lifted_publication::MemberStoreOneItemWirePreparationFactory;

#[path="🏭️producer/🧬️schema/🦀️.rs"]
pub mod producer;
pub use producer::{DocumentStoreOwnerProducer,BoundedDocumentStoreOwnerProducer};

pub struct DocumentCatalogOwner<T>(pub(crate) Option<T>);

impl<T> DocumentCatalogOwner<T> {
    pub fn take(&mut self) -> Option<T> { self.0.take() }
    pub fn is_some(&self) -> bool { self.0.is_some() }
    pub fn as_ref(&self) -> Option<&T> { self.0.as_ref() }
}

impl<T> std::ops::Deref for DocumentCatalogOwner<T> {
    type Target = T;
    fn deref(&self) -> &T { self.0.as_ref().expect("catalog owner remains retained before handoff") }
}

impl<T> std::ops::DerefMut for DocumentCatalogOwner<T> {
    fn deref_mut(&mut self) -> &mut T { self.0.as_mut().expect("catalog owner remains retained before handoff") }
}

/// 🏪️ Prices the exact disposer and three complete factory constructor trees without constructing owners.
pub fn document_store_owners_constructor_birth_bytes<D>(factory_births: [usize; 3]) -> usize {
    factory_births.into_iter().try_fold(size_of::<D>(), usize::checked_add).expect("Store owners constructor layout")
}

pub struct DocumentStoreOwners<P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + FromValue + self::Mutation<P>,
{
    pub snapshot_retirement: DocumentCatalogOwner<Arc<dyn SnapshotRetirementFactory<P>>>,
    pub initial_snapshot_retirement: DocumentCatalogOwner<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>,
    pub mutation_retirement: DocumentCatalogOwner<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>,
    pub store_disposer: DocumentCatalogOwner<Box<dyn ArtifactStoreOwnedDisposer<P, Mutation>>>,
    pub one_item_preparation: Option<Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
    pub replay_preparation: Option<Arc<dyn ArtifactReplayPreparationFactory<P, Mutation>>>,
    pub one_item_wire_preparation: Option<Arc<dyn MemberStoreOneItemWirePreparationFactory<P, Mutation>>>,
    pub operation_wire: ArtifactOperationWire<Mutation>,
    pub factory_retirement_tickets: semio_framework_value::FactoryChildTickets<6>,
}

/// 🛡️ Retains original typed catalog inputs and any actual completed allocation on admission refusal.
pub struct DocumentStoreOwnersAdmissionError<P, Mutation>
where P: ToValue, Mutation: ToValue + self::Mutation<P>,
{
    pub error: ValueError,
    pub owners: Option<DocumentStoreOwners<P, Mutation>>,
    pub progress: RetainedCloneProgress,
}

impl<P, Mutation> DocumentStoreOwners<P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + FromValue + self::Mutation<P>,
{
    /// 🧮️ Prices original factory Arc shells and the original disposer frame before ingress.
    pub fn source_birth_bytes<F1, F2, F3, D>() -> Result<usize, ValueError>
    where F1: semio_framework_value::FactoryPayloadRetirement, F2: semio_framework_value::FactoryPayloadRetirement, F3: semio_framework_value::FactoryPayloadRetirement,
    {
        [semio_framework_value::factory_arc_birth_bytes::<F1>(), semio_framework_value::factory_arc_birth_bytes::<F2>(), semio_framework_value::factory_arc_birth_bytes::<F3>()]
            .into_iter().try_fold(size_of::<D>(), |bytes, source| bytes.checked_add(source))
            .ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "catalog source allocation extent overflow"))
    }

    /// 🧮️ Quotes the existing decoded Edit issuer with its original mutation factory.
    pub fn history_edit_read_issuer_demands(&self)->semio_framework_value::RetirementDemand where Mutation:Send+'static {crate::os_store::edit_retirement::history_edit_read_issuer_demands::<Mutation>()}
    /// 🎟️ Captures the installed mutation issuer through the shared decoded Edit factory.
    pub fn admit_history_edit_read_issuer(&self,slot:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> where Mutation:Send+'static {
        let original=self.mutation_retirement.0.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"history read lost its installed original mutation issuer"))?;
        crate::os_store::edit_retirement::admit_history_edit_read_issuer(original,slot,grant)
    }

    /// 🎟️ Funds the original semantic producer before transferring its retained request from initialization custody.
    pub fn admit_native_replay_preparation(&self,request:&mut Option<super::ArtifactReplayPreparationRequest<P,Mutation>>,task:&mut Option<super::ArtifactReplayPreparationTask<P,Mutation>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,ValueError>
    where Mutation:Sync,
    {
        if task.is_some()||request.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native replay admission requires its original request and empty producer slot"));}
        let context=request.as_ref().unwrap().context;if context.generation!=cx.generation().0{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native replay request belongs to another original Job generation"));}
        let factory=self.replay_preparation.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native replay requires its installed semantic producer"))?;
        let birth=factory.preparation_birth_demand();let overhead=size_of::<Option<super::ArtifactReplayPreparationRequest<P,Mutation>>>()+size_of::<Option<super::ArtifactReplayPreparationTask<P,Mutation>>>();let mut demand=birth;demand.copy_bytes=demand.copy_bytes.checked_add(overhead).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"native replay request movement overflow"))?;
        let grant=cx.retained_grant();if cx.should_yield()||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant};
        let result=cx.with_native_receiving(|control|{control.admit_turn_capacity(grant.maximum_capacity_bytes)?;if birth.capacity_bytes!=0{control.charge(birth.capacity_bytes)?;}let original=request.take().unwrap();match factory.begin(original,child){Ok((cursor,mut p))=>{*task=Some(super::ArtifactReplayPreparationTask::new(context,cursor));p.copied_bytes=p.copied_bytes.checked_add(overhead).expect("native replay preflight movement");if !p.fits(grant)||p.retained_capacity_bytes!=birth.capacity_bytes{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native replay issuer changed its original birth receipt").with_retained_progress(p));}Ok(RetainedCloneStep::Progress(p))},Err((error,original))=>{*request=Some(original);Err(error)}}});
        let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_receiving_retained(grant,progress)?;cx.consume_fuel(progress.copied_items as u64);result
    }
    /// ⏯️ Loans only the actual initializer context to its installed original semantic task.
    pub fn advance_native_replay_preparation(&self,task:&mut super::ArtifactReplayPreparationTask<P,Mutation>,generation:u64,revision:[u8;32],cx:&mut semio_framework_job::StepContext<'_>)->Result<super::ArtifactReplayPreparationStep,ValueError>{
        if generation!=cx.generation().0{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native replay continuation belongs to another original Job generation"));}
        let body=cx.retained_grant().maximum_copy_bytes;let grant=cx.retained_grant();if cx.should_yield()||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,task.demands(generation,revision,body)?){return Ok(super::ArtifactReplayPreparationStep::Blocked);}
        let clock=cx.clock_source();let deadline=cx.deadline_us();let result=cx.with_native_receiving(|control|task.step(generation,revision,body,super::ArtifactStoreOneItemGrant{maximum_items:grant.maximum_items,maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes,maximum_depth:grant.maximum_depth},control,&mut||clock().is_none_or(|now|now>=deadline)));
        let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_receiving_retained(grant,progress)?;cx.consume_fuel(progress.copied_items as u64);result
    }
    /// ♻️ Closes the same native producer through the initializer's original cleanup authority.
    pub fn close_native_replay_preparation(&self,owner:&mut super::ArtifactReplayPreparationRetirement<P,Mutation>,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,ValueError>{
        let grant=cx.retained_grant();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,owner.next_demand(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        let result=cx.with_native_retirement(|control|owner.close_step(grant,control));let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_retained(progress)?;cx.consume_fuel(progress.copied_items as u64);result
    }
    /// 📏️ Quotes the installed canonical operation receiver before retained replacement ingress.
    pub fn original_operation_receiving_birth_demand(&self)->semio_framework_value::RetirementDemand where Mutation:ArtifactCanonicalJsonTree+semio_framework_value::native_decoding::NativeReceive<semio_framework_value::DslValue>+Sync{self.operation_wire.original_receiving_birth_demand()}
    /// 🧳️ Keeps the catalog's genuine mutation issuer and installed wire authority during typed replacement decoding.
    pub fn begin_original_operation_receiving(&self,wire:semio_framework_value::RetainedOwnedProjection<Vec<u8>>,control:&semio_framework_value::NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<(semio_framework_pack_json::OriginalJsonReceiving<Mutation>,RetainedCloneProgress),(ValueError,semio_framework_value::RetainedOwnedProjection<Vec<u8>>)> where Mutation:ArtifactCanonicalJsonTree+semio_framework_value::native_decoding::NativeReceive<semio_framework_value::DslValue>+Sync{
        let Some(factory)=self.mutation_retirement.0.as_ref()else{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original operation receiver lost its installed mutation issuer"),wire));};self.operation_wire.begin_original_receiving(wire,factory,control,grant)
    }

    /// 🎟️ Captures original replacement bytes only after the caller funds all owner and source-slot movements.
    pub fn admit_original_operation_receiving(&self,wire:&mut Option<semio_framework_value::RetainedOwnedProjection<Vec<u8>>>,owner:&mut Option<semio_framework_pack_json::OriginalJsonReceiving<Mutation>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,ValueError> where Mutation:ArtifactCanonicalJsonTree+semio_framework_value::native_decoding::NativeReceive<semio_framework_value::DslValue>+Sync{
        if owner.is_some()||wire.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original operation admission requires its original wire and empty receiver slot"));}
        let mut demand=self.original_operation_receiving_birth_demand();let overhead=2*size_of::<Option<semio_framework_pack_json::OriginalJsonReceiving<Mutation>>>()+2*size_of::<Option<semio_framework_value::RetainedOwnedProjection<Vec<u8>>>>();demand.copy_bytes=demand.copy_bytes.checked_add(overhead).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original operation admission movement overflow"))?;
        let grant=cx.retained_grant();if cx.should_yield()||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
        let result=cx.with_native_receiving(|control|{let original=wire.take().expect("funded original wire slot");match self.begin_original_operation_receiving(original,control,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant}){Ok((received,mut progress))=>{*owner=Some(received);progress.copied_bytes=progress.copied_bytes.checked_add(overhead).expect("preflight checked original admission movement");Ok(progress)},Err((error,original))=>{*wire=Some(original);Err(error)}}});
        let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};cx.consume_receiving_retained(grant,progress)?;cx.consume_fuel(progress.copied_items as u64);result.map(RetainedCloneStep::Progress)
    }
    /// ⏯️ Advances original replacement bytes under the caller's actual decoder, fuel and physical recipient.
    pub fn advance_original_operation_receiving(&self,owner:&mut semio_framework_pack_json::OriginalJsonReceiving<Mutation>,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,ValueError> where Mutation:ArtifactCanonicalJsonTree+semio_framework_value::native_decoding::NativeReceive<semio_framework_value::DslValue>+Sync{
        let grant=cx.retained_grant();if cx.should_yield()||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,owner.demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        let result=cx.with_native_receiving(|control|owner.advance(control,grant.maximum_copy_bytes,grant));let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_receiving_retained(grant,progress)?;cx.consume_fuel(progress.copied_items as u64);result
    }
    /// ♻️ Keeps the original decoder ledger while closing every partial replacement and producer source.
    pub fn close_original_operation_receiving(&self,owner:&mut semio_framework_pack_json::OriginalJsonReceiving<Mutation>,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,ValueError> where Mutation:ArtifactCanonicalJsonTree+semio_framework_value::native_decoding::NativeReceive<semio_framework_value::DslValue>+Sync{
        let grant=cx.retained_grant();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,owner.close_demands(grant.maximum_copy_bytes)?){return Ok(RetainedCloneStep::Progress(Default::default()));}
        let result=cx.with_native_retirement(|control|owner.close_step(control,grant));let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_retained(progress)?;cx.consume_fuel(progress.copied_items as u64);result
    }
    /// 🏗️ Admits original typed sources while retaining unfunded factory tickets inline.
    pub fn admit_sources<F1, F2, F3, D>(snapshots: F1, initial: F2, mutations: F3, disposer: D, grant: RetainedCloneGrant) -> Result<(Self, RetainedCloneProgress), (ValueError, F1, F2, F3, D)>
    where
        Mutation: ArtifactCanonicalJsonTree,
        F1: SnapshotRetirementFactory<P> + semio_framework_value::FactoryPayloadRetirement,
        F2: ArtifactOwnedValueRetirementFactory<P> + semio_framework_value::FactoryPayloadRetirement,
        F3: ArtifactOwnedValueRetirementFactory<Mutation> + semio_framework_value::FactoryPayloadRetirement,
        D: ArtifactStoreOwnedDisposer<P, Mutation> + 'static,
    {
        let capacity = match Self::source_birth_bytes::<F1, F2, F3, D>() { Ok(capacity) => capacity, Err(error) => return Err((error, snapshots, initial, mutations, disposer)) };
        if grant.maximum_items == 0 { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "catalog sources require an admitted ingress item"), snapshots, initial, mutations, disposer)); }
        if grant.maximum_depth == 0 { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "catalog sources require admitted original depth"), snapshots, initial, mutations, disposer)); }
        let copied_bytes=0;
        if grant.maximum_copy_bytes<copied_bytes{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"catalog sources exceed original copy authority"),snapshots,initial,mutations,disposer))}
        if grant.maximum_capacity_bytes < capacity { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "catalog sources exceed supplied capacity"), snapshots, initial, mutations, disposer)); }
        Ok((Self::from_sources(snapshots, initial, mutations, disposer), RetainedCloneProgress { copied_items: 1, copied_bytes, retained_capacity_bytes: capacity, ..Default::default() }))
    }

    /// 🎟️ Funds exact native source ingress before constructing original typed factories or disposer.
    pub fn admit_source_constructor<F1, F2, F3, D>(grant: RetainedCloneGrant, construct: impl FnOnce() -> (F1, F2, F3, D)) -> Result<(Self, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, Mutation>>
    where
        Mutation: ArtifactCanonicalJsonTree,
        F1: SnapshotRetirementFactory<P> + semio_framework_value::FactoryPayloadRetirement,
        F2: ArtifactOwnedValueRetirementFactory<P> + semio_framework_value::FactoryPayloadRetirement,
        F3: ArtifactOwnedValueRetirementFactory<Mutation> + semio_framework_value::FactoryPayloadRetirement,
        D: ArtifactStoreOwnedDisposer<P, Mutation> + 'static,
    {
        let capacity = Self::source_constructor_capacity::<F1, F2, F3, D>(grant, RetainedCloneBirthDemand { capacity_bytes: 0, depth: 0 })?;
        let (snapshots, initial, mutations, disposer) = construct();
        Ok((Self::from_sources(snapshots, initial, mutations, disposer), RetainedCloneProgress { copied_items: 1, copied_bytes:0, retained_capacity_bytes: capacity, ..Default::default() }))
    }

    /// 🏗️🧷️ Funds original semantic preparation source Arcs with the catalog ingress before invoking either source constructor.
    pub fn admit_source_constructor_with_one_item_preparation<F1, F2, F3, D>(grant: RetainedCloneGrant, preparation_birth: RetainedCloneBirthDemand, construct: impl FnOnce() -> (F1, F2, F3, D, Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>)) -> Result<(Self, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, Mutation>>
    where
        Mutation: ArtifactCanonicalJsonTree,
        F1: SnapshotRetirementFactory<P> + semio_framework_value::FactoryPayloadRetirement,
        F2: ArtifactOwnedValueRetirementFactory<P> + semio_framework_value::FactoryPayloadRetirement,
        F3: ArtifactOwnedValueRetirementFactory<Mutation> + semio_framework_value::FactoryPayloadRetirement,
        D: ArtifactStoreOwnedDisposer<P, Mutation> + 'static,
    {
        let capacity = Self::source_constructor_capacity::<F1, F2, F3, D>(grant, preparation_birth)?;
        let (snapshots, initial, mutations, disposer, preparation) = construct();
        let mut owners = Self::from_sources(snapshots, initial, mutations, disposer);
        owners.one_item_preparation = Some(preparation);
        Ok((owners, RetainedCloneProgress { copied_items: 1, copied_bytes:0, retained_capacity_bytes: capacity, ..Default::default() }))
    }

    fn source_constructor_capacity<F1, F2, F3, D>(grant: RetainedCloneGrant, additional_birth: RetainedCloneBirthDemand) -> Result<usize, DocumentStoreOwnersAdmissionError<P, Mutation>>
    where F1: semio_framework_value::FactoryPayloadRetirement, F2: semio_framework_value::FactoryPayloadRetirement, F3: semio_framework_value::FactoryPayloadRetirement,
    {
        let capacity = Self::source_birth_bytes::<F1, F2, F3, D>().and_then(|capacity| capacity.checked_add(additional_birth.capacity_bytes).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "catalog original source tree extent overflow"))).map_err(|error| DocumentStoreOwnersAdmissionError { error, owners: None, progress: Default::default() })?;
        let depth = additional_birth.depth.max(1);
        let refusal = if grant.maximum_items == 0 { Some((semio_framework_value::ValueRefusalKind::WorkLimit, "catalog source constructor requires one admitted item")) }
            else if grant.maximum_depth < depth { Some((semio_framework_value::ValueRefusalKind::DepthLimit, "catalog source constructor requires its original source tree depth")) }
            else if grant.maximum_capacity_bytes < capacity { Some((semio_framework_value::ValueRefusalKind::OwnershipLimit, "catalog source constructor exceeds supplied capacity")) }
            else { None };
        if let Some((kind, message)) = refusal { return Err(DocumentStoreOwnersAdmissionError { error: ValueError::literal(kind, message), owners: None, progress: Default::default() }); }
        Ok(capacity)
    }

    /// 🏭️ Installs publication and replay authorities from one original granted source construction.
    pub fn admit_source_constructor_with_semantic_preparations<F1,F2,F3,D>(grant:RetainedCloneGrant,preparation_birth:RetainedCloneBirthDemand,construct:impl FnOnce()->(F1,F2,F3,D,Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>,Arc<dyn ArtifactReplayPreparationFactory<P,Mutation>>))->Result<(Self,RetainedCloneProgress),DocumentStoreOwnersAdmissionError<P,Mutation>>
    where Mutation:ArtifactCanonicalJsonTree,F1:SnapshotRetirementFactory<P>+semio_framework_value::FactoryPayloadRetirement,F2:ArtifactOwnedValueRetirementFactory<P>+semio_framework_value::FactoryPayloadRetirement,F3:ArtifactOwnedValueRetirementFactory<Mutation>+semio_framework_value::FactoryPayloadRetirement,D:ArtifactStoreOwnedDisposer<P,Mutation>+'static{
        let capacity=Self::source_constructor_capacity::<F1,F2,F3,D>(grant,preparation_birth)?;
        let(snapshots,initial,mutations,disposer,publication,replay)=construct();let mut owners=Self::from_sources(snapshots,initial,mutations,disposer);owners.one_item_preparation=Some(publication);owners.replay_preparation=Some(replay);
        Ok((owners,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()}))
    }

    /// 📏️ Quotes one concrete semantic allocation and its two interface aliases before source construction.
    pub fn shared_semantic_source_demands<F1, F2, F3, D, F>(children: RetainedCloneBirthDemand, child_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where F1: semio_framework_value::FactoryPayloadRetirement, F2: semio_framework_value::FactoryPayloadRetirement, F3: semio_framework_value::FactoryPayloadRetirement, F: semio_framework_value::FactoryPayloadRetirement,
    {
        let overflow = || ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "shared semantic source extent overflow");
        let capacity_bytes = Self::source_birth_bytes::<F1, F2, F3, D>()?.checked_add(semio_framework_value::factory_arc_birth_bytes::<F>()).and_then(|bytes| bytes.checked_add(children.capacity_bytes)).ok_or_else(overflow)?;
        let copy_bytes = [size_of::<F>(), size_of::<Arc<F>>(), size_of::<Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>(), size_of::<Arc<dyn ArtifactReplayPreparationFactory<P, Mutation>>>()].into_iter().try_fold(child_copy_bytes, usize::checked_add).ok_or_else(overflow)?;
        let depth = children.depth.checked_add(1).ok_or_else(overflow)?;
        Ok(semio_framework_value::RetirementDemand { copy_bytes, capacity_bytes, depth, ..Default::default() })
    }

    /// 🏭️ Constructs one funded concrete issuer and installs publication and replay aliases in its original catalog.
    pub fn admit_source_constructor_with_shared_semantic_preparation<F1, F2, F3, D, F>(grant: RetainedCloneGrant, children: RetainedCloneBirthDemand, child_copy_bytes: usize, construct: impl FnOnce() -> (F1, F2, F3, D, F)) -> Result<(Self, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, Mutation>>
    where
        Mutation: ArtifactCanonicalJsonTree + Sync,
        F1: SnapshotRetirementFactory<P> + semio_framework_value::FactoryPayloadRetirement,
        F2: ArtifactOwnedValueRetirementFactory<P> + semio_framework_value::FactoryPayloadRetirement,
        F3: ArtifactOwnedValueRetirementFactory<Mutation> + semio_framework_value::FactoryPayloadRetirement,
        D: ArtifactStoreOwnedDisposer<P, Mutation> + 'static,
        F: ArtifactStoreOneItemPreparationFactory<P, Mutation> + ArtifactReplayPreparationFactory<P, Mutation> + semio_framework_value::FactoryPayloadRetirement + 'static,
    {
        let demand = Self::shared_semantic_source_demands::<F1, F2, F3, D, F>(children, child_copy_bytes).map_err(|error| DocumentStoreOwnersAdmissionError { error, owners: None, progress: Default::default() })?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) {
            let kind = if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes { semio_framework_value::ValueRefusalKind::WorkLimit } else if grant.maximum_depth < demand.depth { semio_framework_value::ValueRefusalKind::DepthLimit } else { semio_framework_value::ValueRefusalKind::OwnershipLimit };
            return Err(DocumentStoreOwnersAdmissionError { error: ValueError::literal(kind, "shared semantic source requires its original caller authority"), owners: None, progress: Default::default() });
        }
        let (snapshots, initial, mutations, disposer, preparation) = construct();
        let shared = Arc::new(preparation);
        let publication: Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>> = Arc::clone(&shared) as Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>;
        let replay: Arc<dyn ArtifactReplayPreparationFactory<P, Mutation>> = shared;
        let mut owners = Self::from_sources(snapshots, initial, mutations, disposer);
        owners.one_item_preparation = Some(publication);
        owners.replay_preparation = Some(replay);
        Ok((owners, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() }))
    }

    /// 🪞️ Borrows both original semantic ports without constructing another alias or producer.
    pub fn original_semantic_issuer_is_shared(&self) -> bool {
        match (&self.one_item_preparation, &self.replay_preparation) {
            (Some(publication), Some(replay)) => Arc::as_ptr(publication) as *const () == Arc::as_ptr(replay) as *const (),
            _ => false,
        }
    }

    fn from_sources<F1, F2, F3, D>(snapshots: F1, initial: F2, mutations: F3, disposer: D) -> Self
    where
        Mutation: ArtifactCanonicalJsonTree,
        F1: SnapshotRetirementFactory<P> + semio_framework_value::FactoryPayloadRetirement,
        F2: ArtifactOwnedValueRetirementFactory<P> + semio_framework_value::FactoryPayloadRetirement,
        F3: ArtifactOwnedValueRetirementFactory<Mutation> + semio_framework_value::FactoryPayloadRetirement,
        D: ArtifactStoreOwnedDisposer<P, Mutation> + 'static,
    {
        Self {
            snapshot_retirement: DocumentCatalogOwner(Some(Arc::new(snapshots))),
            initial_snapshot_retirement: DocumentCatalogOwner(Some(Arc::new(initial))),
            mutation_retirement: DocumentCatalogOwner(Some(Arc::new(mutations))),
            store_disposer: DocumentCatalogOwner(Some(Box::new(disposer))),
            one_item_preparation: None, replay_preparation: None, one_item_wire_preparation: None,
            operation_wire: ArtifactOperationWire::canonical(),
            factory_retirement_tickets: semio_framework_value::FactoryChildTickets { slots: std::array::from_fn(|_| semio_framework_value::FactoryChildSlot::empty()), prepared: 0 },
        }
    }

    pub fn new(
        snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>,
        initial_snapshot_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
        mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
        store_disposer: Box<dyn ArtifactStoreOwnedDisposer<P, Mutation>>,
        grant: RetainedCloneGrant,
    ) -> Result<(Self, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, Mutation>>
    where
        Mutation: ArtifactCanonicalJsonTree,
    {
        let mut owners = Self { snapshot_retirement: DocumentCatalogOwner(Some(snapshot_retirement)), initial_snapshot_retirement: DocumentCatalogOwner(Some(initial_snapshot_retirement)), mutation_retirement: DocumentCatalogOwner(Some(mutation_retirement)), store_disposer: DocumentCatalogOwner(Some(store_disposer)), one_item_preparation: None, replay_preparation: None, one_item_wire_preparation: None, operation_wire: ArtifactOperationWire::canonical(), factory_retirement_tickets: semio_framework_value::FactoryChildTickets { slots: std::array::from_fn(|_| semio_framework_value::FactoryChildSlot::empty()), prepared: 0 } };
        match owners.admit_constructor(grant) {
            Ok(progress) => Ok((owners, progress)),
            Err((error, progress)) => Err(DocumentStoreOwnersAdmissionError { error, owners: Some(owners), progress }),
        }
    }

    /// 🏗️ One original factory ticket is admitted under the complete caller grant.
    pub fn admit_constructor(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, (ValueError, RetainedCloneProgress)> {
        let empty=RetainedCloneProgress::default();
        if self.constructor_is_complete(){return Ok(empty)}
        let demand=self.constructor_demands(grant.maximum_copy_bytes).map_err(|error|(error,empty))?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"catalog constructor exceeds original full authority"),empty))}
        let index=self.constructor_factories().iter().enumerate().find(|(index,factory)|factory.is_some()&&!self.factory_retirement_tickets.slots[*index].preparation_is_complete()).map(|(index,_)|index).expect("pending original catalog factory");
        let parent=Self::constructor_header_copy_bytes();
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,maximum_depth:grant.maximum_depth-1,..grant};
        let slot=&mut self.factory_retirement_tickets.slots[index];
        macro_rules! prepare {($source:expr)=>{{let source=$source;slot.prepare_step(source.as_ref(),||Arc::clone(source) as Arc<dyn semio_framework_value::FactoryRetirement>,child)}};}
        let step=match index {
            0=>prepare!(self.snapshot_retirement.0.as_ref().unwrap()),
            1=>prepare!(self.initial_snapshot_retirement.0.as_ref().unwrap()),
            2=>prepare!(self.mutation_retirement.0.as_ref().unwrap()),
            3=>prepare!(self.one_item_preparation.as_ref().unwrap()),
            4=>prepare!(self.replay_preparation.as_ref().unwrap()),
            _=>prepare!(self.one_item_wire_preparation.as_ref().unwrap()),
        };
        let mut receipt=match step {Ok(step)=>step.progress(),Err(error)=>{let mut receipt=error.retained_progress();receipt.copied_bytes+=parent;return Err((error,receipt));}};
        if !receipt.fits(child){return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"catalog factory changed original constructor receipt"),receipt));}
        if slot.preparation_is_complete(){self.factory_retirement_tickets.prepared=index+1;}
        receipt.copied_bytes+=parent;
        Ok(receipt)
    }
    fn constructor_header_copy_bytes()->usize{0}
    pub fn constructor_is_complete(&self) -> bool {
        self.constructor_factories().iter().enumerate().all(|(index,factory)|factory.is_none()||self.factory_retirement_tickets.slots[index].preparation_is_complete())
    }
    pub fn installation_copy_bytes(&self)->usize{0}
    pub fn constructor_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        for(index,factory)in self.constructor_factories().into_iter().enumerate(){
            let slot=&self.factory_retirement_tickets.slots[index];
            if slot.preparation_is_complete(){continue}
            if let Some(factory)=factory{
                let mut demand=slot.preparation_demands(factory,body.saturating_sub(Self::constructor_header_copy_bytes()))?;
                demand.copy_bytes=demand.copy_bytes.checked_add(Self::constructor_header_copy_bytes()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"catalog constructor original copy overflow"))?;
                demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"catalog constructor original depth overflow"))?;
                return Ok(demand);
            }
        }
        Ok(Default::default())
    }
    fn constructor_factories(&self)->[Option<&dyn semio_framework_value::FactoryRetirement>;6]{[
        self.snapshot_retirement.0.as_ref().map(|factory|factory.as_ref() as &dyn semio_framework_value::FactoryRetirement),
        self.initial_snapshot_retirement.0.as_ref().map(|factory|factory.as_ref() as &dyn semio_framework_value::FactoryRetirement),
        self.mutation_retirement.0.as_ref().map(|factory|factory.as_ref() as &dyn semio_framework_value::FactoryRetirement),
        self.one_item_preparation.as_ref().map(|factory|factory.as_ref() as &dyn semio_framework_value::FactoryRetirement),
        self.replay_preparation.as_ref().map(|factory|factory.as_ref() as &dyn semio_framework_value::FactoryRetirement),
        self.one_item_wire_preparation.as_ref().map(|factory|factory.as_ref() as &dyn semio_framework_value::FactoryRetirement),
    ]}

    /// 🏭️ Prices original catalog factory ticket births and nested state before constructor admission.
    pub fn factory_retirement_birth_bytes(&self) -> usize {
        let mut bytes = self.snapshot_retirement.factory_retirement_birth_bytes();
        for demand in [self.initial_snapshot_retirement.factory_retirement_birth_bytes(), self.mutation_retirement.factory_retirement_birth_bytes(), self.one_item_preparation.as_ref().map_or(0, |factory| factory.factory_retirement_birth_bytes()), self.replay_preparation.as_ref().map_or(0, |factory| factory.factory_retirement_birth_bytes()), self.one_item_wire_preparation.as_ref().map_or(0, |factory| factory.factory_retirement_birth_bytes())] {
            bytes = bytes.checked_add(demand).expect("Store factory ticket constructor layout");
        }
        bytes
    }

    pub fn take_factory_retirement_tickets(&mut self) -> semio_framework_value::FactoryChildTickets<6> {
        std::mem::replace(&mut self.factory_retirement_tickets, semio_framework_value::FactoryChildTickets { slots: std::array::from_fn(|_| semio_framework_value::FactoryChildSlot::empty()), prepared: 0 })
    }

    pub fn with_replay_preparation(mut self, factory: Arc<dyn ArtifactReplayPreparationFactory<P, Mutation>>, grant: RetainedCloneGrant) -> Result<(Self, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, Mutation>> {
        assert!(self.replay_preparation.is_none(), "catalog installs each original preparation authority exactly once");
        self.replay_preparation = Some(factory);
        match self.admit_constructor(grant) {
            Ok(progress) => Ok((self, progress)),
            Err((error, progress)) => Err(DocumentStoreOwnersAdmissionError { error, owners: Some(self), progress }),
        }
    }

    pub fn with_one_item_preparation(mut self, factory: Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>, grant: RetainedCloneGrant) -> Result<(Self, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, Mutation>> {
        assert!(self.one_item_preparation.is_none(), "catalog installs each original preparation authority exactly once");
        self.one_item_preparation = Some(factory);
        match self.admit_constructor(grant) {
            Ok(progress) => Ok((self, progress)),
            Err((error, progress)) => Err(DocumentStoreOwnersAdmissionError { error, owners: Some(self), progress }),
        }
    }

    pub fn with_one_item_wire_preparation(mut self, factory: Arc<dyn MemberStoreOneItemWirePreparationFactory<P, Mutation>>, grant: RetainedCloneGrant) -> Result<(Self, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, Mutation>> {
        assert!(self.one_item_wire_preparation.is_none(), "catalog installs each original preparation authority exactly once");
        self.one_item_wire_preparation = Some(factory);
        match self.admit_constructor(grant) {
            Ok(progress) => Ok((self, progress)),
            Err((error, progress)) => Err(DocumentStoreOwnersAdmissionError { error, owners: Some(self), progress }),
        }
    }

    /// 🧹️ Admits the exact whole-envelope frame while retaining original catalog authorities.
    pub fn retire_envelope(&self, envelope: ArtifactEnvelope<P, Mutation>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, ArtifactEnvelope<P, Mutation>)>
    where P: Send + Sync + 'static, Mutation: Send + 'static,
    {
        retire_document_envelope(envelope, Arc::clone(&self.initial_snapshot_retirement), Arc::clone(&self.mutation_retirement), grant)
            .map_err(|(error, envelope, snapshots, mutations)| { drop((snapshots, mutations)); (error, envelope) })
    }

    /// 📐️ Quotes the exact original envelope and uninstalled catalog frame before transfer.
    pub fn uninstalled_envelope_retirement_demands(&self, _envelope: &ArtifactEnvelope<P, Mutation>) -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { copy_bytes:size_of::<(Self,ArtifactEnvelope<P,Mutation>)>()+size_of::<UninstalledDocumentEnvelopeRetirement<P,Mutation>>()+2*size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>(),capacity_bytes:size_of::<UninstalledDocumentEnvelopeRetirement<P,Mutation>>(),depth:1,..Default::default() }
    }

    /// 📦️ Admits one whole frame retaining both original envelope and uninstalled catalog.
    pub fn retire_envelope_uninstalled(self, envelope: ArtifactEnvelope<P, Mutation>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Self, ArtifactEnvelope<P, Mutation>)>
    where P: Send + Sync + 'static, Mutation: Send + 'static,
    {
        let demand=self.uninstalled_envelope_retirement_demands(&envelope);
        if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original uninstalled envelope frame exceeds supplied authority"),self,envelope));}
        admit_artifact_retirement((self, envelope), grant, |(owners, envelope)| {
            let retirement = ArtifactStoreEnvelopeRetirement::new(envelope, Arc::clone(&owners.initial_snapshot_retirement), Arc::clone(&owners.mutation_retirement));
            UninstalledDocumentEnvelopeRetirement { envelope: Some(retirement), owners: Some(owners) }
        }).map(|(owner,mut progress)|{progress.copied_bytes=demand.copy_bytes;(owner,progress)}).map_err(|(error, (owners, envelope))| (error, owners, envelope))
    }

    pub fn retire_decoded_edit(&self, edit: Edit<Mutation>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Edit<Mutation>)>
    where Mutation: Send + 'static,
    {
        admit_artifact_retirement((edit, self.mutation_retirement.clone()), grant, |(edit, factory)| ArtifactStoreDecodedEditRetirement::new(edit, factory))
            .map_err(|(error, (edit, factory))| { drop(factory); (error, edit) })
    }

    pub fn retire_decoded_envelope(&self, envelope: ArtifactEnvelope<P, Mutation>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, ArtifactEnvelope<P, Mutation>)>
    where P: Send + Sync + 'static, Mutation: Send + 'static,
    {
        self.retire_envelope(envelope, grant)
    }

    /// 🪪️ The catalog's original initial-snapshot issuer, shared with an initialization runtime that retires its current projection.
    pub fn initial_snapshot_factory(&self) -> Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>> {
        self.initial_snapshot_retirement.0.clone()
    }

    /// 📏️ Quotes the constructor frame of [`Self::retire_initial_snapshot_owned`] before ownership transfers.
    pub fn initial_snapshot_retirement_demand(&self, snapshot: &P) -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { capacity_bytes: self.initial_snapshot_retirement.retirement_birth_bytes(snapshot), depth: 2, ..Default::default() }
    }

    pub fn retire_initial_snapshot_owned(&self, snapshot: P, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, P)>
    where P: Send + Sync + 'static,
    {
        self.initial_snapshot_retirement.retire_owned(snapshot, grant)
    }

    pub fn retire_genesis_owned(&self, genesis: AdmittedArtifactGenesis<P>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, AdmittedArtifactGenesis<P>)>
    where P: Send + Sync + 'static,
    {
        admit_artifact_retirement((genesis, self.initial_snapshot_retirement.clone()), grant, |(genesis, factory)| ArtifactGenesisRetirement::new(genesis, factory))
            .map_err(|(error, (genesis, factory))| { drop(factory); (error, genesis) })
    }

    /// 📏️ Borrows every current child currency and the exact final disposer frame.
    pub fn uninstalled_owners_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if !self.constructor_is_complete() { return self.constructor_demands(body); }
        if self.store_disposer.is_some() && !self.store_disposer.uninstalled_terminal_is_empty() {
            let mut demand = self.store_disposer.uninstalled_demands(body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "catalog disposer depth overflow"))?;
            return Ok(demand);
        }
        if self.factory_aliases_present() { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
        if !self.factory_retirement_tickets.terminal_is_empty() {
            let mut demand = self.factory_retirement_tickets.demands(body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "catalog factory depth overflow"))?;
            return Ok(demand);
        }
        Ok(semio_framework_value::RetirementDemand { release_bytes: self.store_disposer.0.as_ref().map_or(0, |owner| size_of_val(owner.as_ref())), depth: usize::from(self.store_disposer.is_some()), ..Default::default() })
    }

    pub fn factory_aliases_present(&self) -> bool {
        self.snapshot_retirement.is_some() || self.initial_snapshot_retirement.is_some() || self.mutation_retirement.is_some() || self.one_item_preparation.is_some() || self.replay_preparation.is_some() || self.one_item_wire_preparation.is_some()
    }

    /// 🧹️ Keeps original aliases, factory tickets and the disposer until their admitted physical turn.
    pub fn close_uninstalled_owners_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.uninstalled_owners_terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.uninstalled_owners_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "catalog retirement exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if !self.constructor_is_complete() { return self.admit_constructor(RetainedCloneGrant { maximum_items: 1, ..grant }).map(RetainedCloneStep::Progress).map_err(|(error, progress)| error.with_retained_progress(progress)); }
        if self.store_disposer.is_some() && !self.store_disposer.uninstalled_terminal_is_empty() {
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = self.store_disposer.close_uninstalled_step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, self.store_disposer.uninstalled_terminal_is_empty(), "uninstalled catalog disposer")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        macro_rules! release_alias {
            ($field:ident) => { if self.$field.is_some() { drop(self.$field.take()); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); } };
        }
        release_alias!(snapshot_retirement);
        release_alias!(initial_snapshot_retirement);
        release_alias!(mutation_retirement);
        release_alias!(one_item_preparation);
        release_alias!(replay_preparation);
        release_alias!(one_item_wire_preparation);
        if !self.factory_retirement_tickets.terminal_is_empty() {
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = self.factory_retirement_tickets.close_step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, self.factory_retirement_tickets.terminal_is_empty(), "uninstalled catalog factories")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(disposer) = self.store_disposer.0.as_ref() {
            let extent = size_of_val(disposer.as_ref());
            if grant.maximum_release_bytes < extent { return Ok(RetainedCloneStep::Progress(empty)); }
            drop(self.store_disposer.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..empty }));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub fn uninstalled_owners_terminal_is_empty(&self) -> bool {
        !self.factory_aliases_present() && self.factory_retirement_tickets.terminal_is_empty() && !self.store_disposer.is_some()
    }
}

impl<P, M> Drop for DocumentStoreOwners<P, M>
where P: ToValue, M: ToValue + Mutation<P>,
{
    fn drop(&mut self) { assert!(std::thread::panicking() || self.uninstalled_owners_terminal_is_empty(), "document catalog reached Drop before exact handoff or controlled retirement"); }
}

pub struct UninstalledDocumentEnvelopeRetirement<P, M>
where P: ToValue, M: ToValue + Mutation<P>,
{
    pub envelope: Option<ArtifactStoreEnvelopeRetirement<P, M>>,
    pub owners: Option<DocumentStoreOwners<P, M>>,
}

impl<P, M> ErasedSnapshotRetirement for UninstalledDocumentEnvelopeRetirement<P, M>
where P: ToValue + Send + Sync + 'static, M: ToValue + Mutation<P> + Send + 'static,
{
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.envelope.is_some() {
            let step = retirement_owner_close(&mut self.envelope, grant)?;
            return Ok(if self.owners.is_some() { RetainedCloneStep::Progress(step.progress()) } else { step });
        }
        if self.owners.is_some() {
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
            if grant.maximum_depth < self.demands(grant.maximum_copy_bytes)?.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "uninstalled envelope catalog requires original child depth")); }
            let owners = self.owners.as_mut().expect("uninstalled envelope retains the observed original catalog");
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owners.close_uninstalled_owners_step(child)?;
            let terminal = owners.uninstalled_owners_terminal_is_empty();
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, terminal, "uninstalled envelope original catalog")?;
            if terminal { self.owners.take(); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
    fn terminal_is_empty(&self) -> bool { self.envelope.is_none() && self.owners.is_none() }
}

impl<P, M> UninstalledDocumentEnvelopeRetirement<P, M>
where P: ToValue + Send + Sync + 'static, M: ToValue + Mutation<P> + Send + 'static,
{
    pub fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.envelope.is_some() { return retirement_owner_demands(&self.envelope, body); }
        if let Some(owners) = self.owners.as_ref() {
            let mut demand = owners.uninstalled_owners_demands(body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "uninstalled catalog logical depth overflow"))?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
}

#[path="🏭️member/🧬️schema/🦀️.rs"]
pub mod member_producer;
pub use member_producer::MemberStoreProducer;

pub const UNSCHEDULED_CATALOG_STEPS: usize = 1 << 12;

/// 🏗️ Funds one complete member catalog from its own quoted demands for async construction that no scheduler grants.
pub fn funded_member_store_owners<P, Mutation, C>(producer:&C) -> Result<DocumentStoreOwners<P, Mutation>, ValueError>
where
    P: ToValue,
    Mutation: ToValue + FromValue + self::Mutation<P>,
    C:MemberStoreProducer<P,Mutation>,
{
    let birth = producer.member_store_owners_birth_demand()?;
    complete_unscheduled_catalog(producer.member_store_owners(RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: birth.capacity_bytes, maximum_depth: birth.depth.max(1), ..Default::default() }))
}

/// 🎟️ Completes every pending factory ticket of an admitted catalog from its own quoted constructor demands.
pub fn complete_unscheduled_catalog<P, Mutation>(admitted: Result<(DocumentStoreOwners<P, Mutation>, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, Mutation>>) -> Result<DocumentStoreOwners<P, Mutation>, ValueError>
where
    P: ToValue,
    Mutation: ToValue + FromValue + self::Mutation<P>,
{
    let mut owners = match admitted {
        Ok((owners, _)) => owners,
        Err(refused) => {
            let error = refused.error;
            if let Some(owners) = refused.owners { close_unscheduled_catalog(owners); }
            return Err(error);
        }
    };
    for _ in 0..UNSCHEDULED_CATALOG_STEPS {
        if owners.constructor_is_complete() { return Ok(owners); }
        let outcome = owners.constructor_demands(0).map_err(|error| (error, RetainedCloneProgress::default())).and_then(|demand| owners.admit_constructor(RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: demand.capacity_bytes, maximum_depth: demand.depth.max(1), ..Default::default() }));
        if let Err((error, _)) = outcome {
            close_unscheduled_catalog(owners);
            return Err(error);
        }
    }
    close_unscheduled_catalog(owners);
    Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "catalog constructor did not complete within its bounded ticket admission"))
}

pub fn close_unscheduled_catalog<P, Mutation>(mut owners: DocumentStoreOwners<P, Mutation>)
where
    P: ToValue,
    Mutation: ToValue + FromValue + self::Mutation<P>,
{
    for _ in 0..UNSCHEDULED_CATALOG_STEPS {
        if owners.uninstalled_owners_terminal_is_empty() { return; }
        let Ok(demand) = owners.uninstalled_owners_demands(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES) else { break };
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        if owners.close_uninstalled_owners_step(grant).is_err() { break; }
    }
    std::mem::forget(owners);
}

impl<P, Mutation> DocumentStoreOwners<P, Mutation>
where
    P: ToValue,
    Mutation: ToValue + FromValue + self::Mutation<P> + ArtifactCanonicalJsonTree + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
{
    /// 🔏️ Grants the catalog's operation wire its aggregate-edit digest authority, so the Store commits batches without an app factory.
    pub fn with_batch_digest(mut self) -> Self { self.operation_wire = self.operation_wire.with_batch_digest(); self }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct BoundedArtifactRetirementFactory<T>(PhantomData<fn() -> T>);

impl<T> BoundedArtifactRetirementFactory<T> {
    pub fn new() -> Self { Self(PhantomData) }
}

impl<T: semio_framework_value::retirement::RetireOwned> ArtifactOwnedValueRetirementFactory<T> for BoundedArtifactRetirementFactory<T> {
    fn retirement_birth_bytes(&self, _: &T) -> usize { semio_framework_value::retirement::controlled::controlled_retirement_birth_bytes::<T>() }
    fn retire_owned(&self, value: T, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, T)> {
        semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(value, grant).map(|(owner, progress)| (owner as Box<dyn ErasedSnapshotRetirement>, progress))
    }
}

impl<T: semio_framework_value::retirement::RetireOwned + Sync> SnapshotRetirementFactory<T> for BoundedArtifactRetirementFactory<T> {
    fn retirement_birth_bytes(&self, _: &Arc<T>) -> usize { semio_framework_value::retirement::arc_custody::shared_retirement_birth_bytes::<T>() }
    fn retire(&self, snapshot: Arc<T>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<T>)> {
        semio_framework_value::retirement::arc_custody::admit_shared_retirement(snapshot, grant, true)
    }
}

/// 🧮️ Prices the exact bounded catalog constructor without constructing factory owners.
pub fn bounded_artifact_store_owners_birth_bytes<P, Mutation>() -> usize
where
    P: Clone + ToValue + FromValue + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + semio_framework_value::retirement::RetireOwned,
{
    DocumentStoreOwners::<P, Mutation>::source_birth_bytes::<BoundedArtifactRetirementFactory<P>, BoundedArtifactRetirementFactory<P>, BoundedArtifactRetirementFactory<Mutation>, ArtifactStoreCursorDisposer<P, Mutation>>().expect("bounded catalog native source layout")
}

/// 📏️ Quotes original leaf factory Arcs and the inline disposer before any retirement ticket birth.
pub fn bounded_artifact_store_owners_birth_demand<P, Mutation>() -> Result<RetainedCloneBirthDemand, ValueError>
where
    P: Clone + ToValue + FromValue + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + semio_framework_value::retirement::RetireOwned,
{
    Ok(RetainedCloneBirthDemand { capacity_bytes: DocumentStoreOwners::<P, Mutation>::source_birth_bytes::<BoundedArtifactRetirementFactory<P>, BoundedArtifactRetirementFactory<P>, BoundedArtifactRetirementFactory<Mutation>, ArtifactStoreCursorDisposer<P, Mutation>>()?, depth: 1 })
}

/// 📐️ Borrows the native source headers and birth before invoking their original constructor.
pub fn bounded_artifact_store_owners_source_demands<P,Mutation>()->Result<semio_framework_value::RetirementDemand,ValueError>
where P:Clone+ToValue+FromValue+semio_framework_value::retirement::RetireOwned+Sync,
Mutation:Clone+ToValue+FromValue+self::Mutation<P>+OpBinary+OpText+semio_framework_value::retirement::RetireOwned{
 let birth=bounded_artifact_store_owners_birth_demand::<P,Mutation>()?;
 Ok(semio_framework_value::RetirementDemand{copy_bytes:0,capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()})
}

/// 🗃️ Admits the original typed catalog source ingress while retaining separate factory-ticket births.
pub fn bounded_artifact_store_owners<P, Mutation>(grant: RetainedCloneGrant) -> Result<(DocumentStoreOwners<P, Mutation>, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, Mutation>>
where
    P: Clone + ToValue + FromValue + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + semio_framework_value::retirement::RetireOwned + ArtifactCanonicalJsonTree + Send + Sync + 'static,
{
    let demand = bounded_artifact_store_owners_birth_demand::<P, Mutation>().map_err(|error| DocumentStoreOwnersAdmissionError { error, owners: None, progress: Default::default() })?;
    if grant.maximum_depth < demand.depth { return Err(DocumentStoreOwnersAdmissionError { error: ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "bounded catalog source exceeds declared owner depth"), owners: None, progress: Default::default() }); }
    DocumentStoreOwners::admit_source_constructor(grant, || (BoundedArtifactRetirementFactory::<P>::new(), BoundedArtifactRetirementFactory::<P>::new(), BoundedArtifactRetirementFactory::<Mutation>::new(), ArtifactStoreCursorDisposer::<P, Mutation>::new())).map(|(owners, progress)| (owners.with_batch_digest(), progress))
}

/// 🌳️ Admits the original bounded catalog and preparation source tree in one funded ingress.
pub fn bounded_artifact_store_owners_with_one_item_preparation<P, Mutation>(grant: RetainedCloneGrant, preparation_birth: RetainedCloneBirthDemand, construct: impl FnOnce() -> Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>) -> Result<(DocumentStoreOwners<P, Mutation>, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, Mutation>>
where
    P: Clone + ToValue + FromValue + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + semio_framework_value::retirement::RetireOwned + ArtifactCanonicalJsonTree + Send + Sync + 'static,
{
    DocumentStoreOwners::admit_source_constructor_with_one_item_preparation(grant, preparation_birth, || (BoundedArtifactRetirementFactory::<P>::new(), BoundedArtifactRetirementFactory::<P>::new(), BoundedArtifactRetirementFactory::<Mutation>::new(), ArtifactStoreCursorDisposer::<P, Mutation>::new(), construct())).map(|(owners, progress)| (owners.with_batch_digest(), progress))
}

/// 🌳️ Admits both native semantic producers before the owner catalog prepares their exact retirement tickets.
pub fn bounded_artifact_store_owners_with_semantic_preparations<P,Mutation>(grant:RetainedCloneGrant,preparation_birth:RetainedCloneBirthDemand,construct:impl FnOnce()->(Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>,Arc<dyn ArtifactReplayPreparationFactory<P,Mutation>>))->Result<(DocumentStoreOwners<P,Mutation>,RetainedCloneProgress),DocumentStoreOwnersAdmissionError<P,Mutation>>
where P:ToValue+ArtifactPack+semio_framework_value::retirement::RetireOwned+Sync,Mutation:ToValue+FromValue+self::Mutation<P>+OpBinary+OpText+semio_framework_value::retirement::RetireOwned+ArtifactCanonicalJsonTree+Send+Sync+'static{
    DocumentStoreOwners::admit_source_constructor_with_semantic_preparations(grant,preparation_birth,||{let(publication,replay)=construct();(BoundedArtifactRetirementFactory::<P>::new(),BoundedArtifactRetirementFactory::<P>::new(),BoundedArtifactRetirementFactory::<Mutation>::new(),ArtifactStoreCursorDisposer::<P,Mutation>::new(),publication,replay)}).map(|(owners,progress)|(owners.with_batch_digest(),progress))
}

/// 🧮️ Quotes one bounded catalog and its shared concrete semantic source before constructing any factory.
pub fn bounded_artifact_store_owners_shared_semantic_source_demands<P, Mutation, F>(children: RetainedCloneBirthDemand, child_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>
where
    P: ToValue + ArtifactPack + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + semio_framework_value::retirement::RetireOwned,
    F: semio_framework_value::FactoryPayloadRetirement,
{
    DocumentStoreOwners::<P, Mutation>::shared_semantic_source_demands::<BoundedArtifactRetirementFactory<P>, BoundedArtifactRetirementFactory<P>, BoundedArtifactRetirementFactory<Mutation>, ArtifactStoreCursorDisposer<P, Mutation>, F>(children, child_copy_bytes)
}

/// 🌱️ Funds the single semantic issuer at the same original bounded catalog source boundary.
pub fn bounded_artifact_store_owners_with_shared_semantic_preparation<P, Mutation, F>(grant: RetainedCloneGrant, children: RetainedCloneBirthDemand, child_copy_bytes: usize, construct: impl FnOnce() -> F) -> Result<(DocumentStoreOwners<P, Mutation>, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, Mutation>>
where
    P: ToValue + ArtifactPack + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + semio_framework_value::retirement::RetireOwned + ArtifactCanonicalJsonTree + Send + Sync + 'static,
    F: ArtifactStoreOneItemPreparationFactory<P, Mutation> + ArtifactReplayPreparationFactory<P, Mutation> + semio_framework_value::FactoryPayloadRetirement + 'static,
{
    DocumentStoreOwners::admit_source_constructor_with_shared_semantic_preparation(grant, children, child_copy_bytes, || (BoundedArtifactRetirementFactory::<P>::new(), BoundedArtifactRetirementFactory::<P>::new(), BoundedArtifactRetirementFactory::<Mutation>::new(), ArtifactStoreCursorDisposer::<P, Mutation>::new(), construct())).map(|(owners, progress)| (owners.with_batch_digest(), progress))
}

/// 🧬️ Prices a shared concrete native issuer once, with distinct publication and replay retirement authorities.
pub fn funded_bounded_artifact_store_owners_with_semantic_preparation<P,Mutation,F>(preparation:Arc<F>)->Result<DocumentStoreOwners<P,Mutation>,ValueError>
where P:ToValue+ArtifactPack+semio_framework_value::retirement::RetireOwned+Sync,Mutation:ToValue+FromValue+self::Mutation<P>+OpBinary+OpText+semio_framework_value::retirement::RetireOwned+ArtifactCanonicalJsonTree+Send+Sync+'static,F:ArtifactStoreOneItemPreparationFactory<P,Mutation>+ArtifactReplayPreparationFactory<P,Mutation>+semio_framework_value::FactoryPayloadRetirement+'static{
    let overflow=||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"semantic issuer source extent overflow");let arc=semio_framework_value::factory_arc_birth_bytes::<F>();let tickets=preparation.factory_retirement_birth_bytes().checked_mul(2).ok_or_else(overflow)?;let extra=RetainedCloneBirthDemand{capacity_bytes:arc.checked_add(tickets).ok_or_else(overflow)?,depth:preparation.factory_retirement_depth_demand()};let base=bounded_artifact_store_owners_birth_demand::<P,Mutation>()?;let birth=RetainedCloneBirthDemand{capacity_bytes:base.capacity_bytes.checked_add(extra.capacity_bytes).ok_or_else(overflow)?,depth:base.depth.max(extra.depth)};
    fund_document_store_owners(birth,move|grant|bounded_artifact_store_owners_with_semantic_preparations(grant,extra,move||{let replay:Arc<dyn ArtifactReplayPreparationFactory<P,Mutation>>=Arc::clone(&preparation)as Arc<dyn ArtifactReplayPreparationFactory<P,Mutation>>;(preparation,replay)}))
}

/// 🏗️ Funds one complete catalog from its own quoted demands for synchronous construction that no scheduler grants.
pub fn fund_document_store_owners<P, Mutation>(birth: RetainedCloneBirthDemand, source: impl FnOnce(RetainedCloneGrant) -> Result<(DocumentStoreOwners<P, Mutation>, RetainedCloneProgress), DocumentStoreOwnersAdmissionError<P, Mutation>>) -> Result<DocumentStoreOwners<P, Mutation>, ValueError>
where
    P: ToValue,
    Mutation: ToValue + FromValue + self::Mutation<P>,
{
    let ingress = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: birth.capacity_bytes, maximum_depth: birth.depth.max(1), ..Default::default() };
    let mut owners = match source(ingress) {
        Ok((owners, _)) => owners,
        Err(refused) => {
            let error = refused.error;
            if let Some(owners) = refused.owners { close_unscheduled_catalog(owners); }
            return Err(error);
        }
    };
    for _ in 0..UNSCHEDULED_CATALOG_STEPS {
        if owners.constructor_is_complete() { return Ok(owners); }
        let outcome = owners.constructor_demands(0).map_err(|error| (error, RetainedCloneProgress::default())).and_then(|demand| owners.admit_constructor(RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: demand.capacity_bytes, maximum_depth: demand.depth.max(1), ..Default::default() }));
        if let Err((error, _)) = outcome {
            close_unscheduled_catalog(owners);
            return Err(error);
        }
    }
    close_unscheduled_catalog(owners);
    Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "catalog constructor did not complete within its bounded ticket admission"))
}

/// 🗃️ Funds the framework's bounded catalog for a throwaway reduction store that no scheduler grants.
pub fn funded_bounded_artifact_store_owners<P, Mutation>() -> Result<DocumentStoreOwners<P, Mutation>, ValueError>
where
    P: Clone + ToValue + FromValue + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + semio_framework_value::retirement::RetireOwned + ArtifactCanonicalJsonTree + Send + Sync + 'static,
{
    fund_document_store_owners(bounded_artifact_store_owners_birth_demand::<P, Mutation>()?, bounded_artifact_store_owners::<P, Mutation>)
}

/// 🗃️🧷️ Funds the bounded catalog together with one original preparation authority for a store that no scheduler grants: the authority's source extent is priced from its own constructed Arc and its quoted retirement ticket tree, so the catalog retires it like every other original factory.
pub fn funded_bounded_artifact_store_owners_with_one_item_preparation<P, Mutation>(preparation: Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>) -> Result<DocumentStoreOwners<P, Mutation>, ValueError>
where
    P: Clone + ToValue + FromValue + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + semio_framework_value::retirement::RetireOwned + ArtifactCanonicalJsonTree + Send + Sync + 'static,
{
    let overflow = || ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "preparation source extent overflow");
    let arc = std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::for_value(&*preparation)).map_err(|_| overflow())?.0.pad_to_align().size();
    let additional = RetainedCloneBirthDemand { capacity_bytes: arc.checked_add(preparation.factory_retirement_birth_bytes()).ok_or_else(overflow)?, depth: preparation.factory_retirement_depth_demand() };
    let base = bounded_artifact_store_owners_birth_demand::<P, Mutation>()?;
    let birth = RetainedCloneBirthDemand { capacity_bytes: base.capacity_bytes.checked_add(additional.capacity_bytes).ok_or_else(overflow)?, depth: base.depth.max(additional.depth) };
    fund_document_store_owners(birth, move |grant| bounded_artifact_store_owners_with_one_item_preparation(grant, additional, move || preparation))
}
