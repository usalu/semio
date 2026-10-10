//! 📬️ Retained one-item Store preparation shared by Semio native editors.

use crate::protocol;
use semio_framework_plugin::plugin_app_close_prelude::store as app_store;
use std::{sync::Arc,mem::ManuallyDrop};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

pub(crate) const PAGE_BYTES: usize = 4_096;

pub(crate) enum StructuralCopyStep {
    Progress { items: usize, bytes: usize },
    Complete,
}

pub(crate) trait StructuralMutationCopy<S, M>: Send {
    fn advance(&mut self, base: &S, mutation: &M, maximum_items: usize, maximum_bytes: usize) -> Result<StructuralCopyStep, String>;
    fn take_result(&mut self) -> Option<(S, M)>;
    fn begin_close(&mut self);
    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError>;
    fn retirement_demands(&self, maximum_body_bytes:usize) -> Result<RetirementDemand,ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(crate) struct StructuralPreparationFactory<S, M> {
    prefix: &'static str,
    recognizes: fn(&M) -> bool,
    preflight: fn(&M) -> Result<usize, String>,
    copy_birth: fn() -> semio_framework_value::retained_clone::RetainedCloneBirthDemand,
    copy: fn() -> Box<dyn StructuralMutationCopy<S, M>>,
    #[factory_child]
    mutation_retirement: Arc<dyn app_store::ArtifactOwnedValueRetirementFactory<M>>,
    #[factory_child]
    snapshot_retirement: Arc<dyn app_store::SnapshotRetirementFactory<S>>,
}

impl<S, M> StructuralPreparationFactory<S, M> {
    /// 🌱️ Prices the concrete structural factory and both original child constructors.
    pub(crate) fn constructor_birth_bytes(mutation_birth: usize, snapshot_birth: usize) -> usize
    where Self: semio_framework_value::FactoryPayloadRetirement,
    {
        semio_framework_value::factory_constructor_birth_bytes::<Self>(mutation_birth.checked_add(snapshot_birth).expect("structural child constructor layout"))
    }

    pub(crate) fn new(
        prefix: &'static str,
        recognizes: fn(&M) -> bool,
        preflight: fn(&M) -> Result<usize, String>,
        copy_birth: fn() -> semio_framework_value::retained_clone::RetainedCloneBirthDemand,
    copy: fn() -> Box<dyn StructuralMutationCopy<S, M>>,
        mutation_retirement: Arc<dyn app_store::ArtifactOwnedValueRetirementFactory<M>>,
        snapshot_retirement: Arc<dyn app_store::SnapshotRetirementFactory<S>>,
    ) -> Self {
        Self { prefix, recognizes, preflight, copy_birth, copy, mutation_retirement, snapshot_retirement }
    }
}

impl<S, M> app_store::ArtifactStoreOneItemPreparationFactory<S, M> for StructuralPreparationFactory<S, M>
where
    S: semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: app_store::ArtifactCanonicalJson + protocol::Mutation<S> + Send + Sync + 'static,
{
    fn preflight(&self, mutation: &M, lane: app_store::HistoryLane) -> Result<app_store::ArtifactStoreOneItemFootprint, String> {
        if !(self.recognizes)(mutation) || lane != app_store::HistoryLane::Document {
            return Err(format!("{}-admission", self.prefix));
        }
        let retained_bytes = (self.preflight)(mutation)?;
        if retained_bytes > app_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES {
            return Err(format!("{}-payload", self.prefix));
        }
        Ok(app_store::ArtifactStoreOneItemFootprint::for_leaf::<S, M>(mutation, retained_bytes.max(1)))
    }

    fn begin_demand(&self, mutation:&M, lane:app_store::HistoryLane)->Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::RetainedCloneBirthDemand};
        if lane!=app_store::HistoryLane::Document || !(self.recognizes)(mutation){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Semio structural mutation has no original preparation issuer"));}
        let child=(self.copy_birth)();
        Ok(RetainedCloneBirthDemand{capacity_bytes:std::mem::size_of::<StructuralPreparation<S,M>>().checked_add(child.capacity_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Semio preparation source layout overflow"))?,depth:child.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"Semio preparation source depth overflow"))?})
    }

    fn begin(&self, request:app_store::ArtifactStoreOneItemPreparationRequest<S,M>,grant:app_store::ArtifactStoreOneItemGrant)->Result<(Box<dyn app_store::ArtifactStoreOneItemPreparation<S,M>>,semio_framework_value::retained_clone::RetainedCloneProgress),(semio_framework_value::ValueError,app_store::ArtifactStoreOneItemPreparationRequest<S,M>)>{
        use semio_framework_value::{ValueError,ValueRefusalKind};
        let demand=match self.begin_demand(&request.mutation,request.lane){Ok(value)=>value,Err(error)=>return Err((error,request))};
        let receipt=match demand.admit(grant.retained_grant()){Ok(value)=>value,Err(error)=>return Err((error,request))};
        let admitted=request.operation==request.authority.operation() && request.generation==request.authority.generation() && request.base_revision==request.authority.base_revision() && request.authority.actor().len()<=app_store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES;
        if !admitted{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"Semio structural request differs from its original authority"),request));}
        Ok((Box::new(StructuralPreparation{
            prefix:self.prefix,base:ManuallyDrop::new(Some(request.base)),mutation:ManuallyDrop::new(Some(request.mutation)),authority:ManuallyDrop::new(Some(request.authority)),copy:ManuallyDrop::new(Some((self.copy)())),mutation_retirement:ManuallyDrop::new(Some(Arc::clone(&self.mutation_retirement))),snapshot_retirement:ManuallyDrop::new(Some(Arc::clone(&self.snapshot_retirement))),factory_close:ManuallyDrop::new(Default::default()),sealer:ManuallyDrop::new(None),external_retirement:ManuallyDrop::new(None),checkpoint:Default::default(),seal_base_checkpoint:None,phase:0,cancelled:false,closing:false,
        }),receipt))
    }
}

struct StructuralPreparation<S, M> {
    prefix: &'static str,
    base: ManuallyDrop<Option<app_store::SnapshotRead<S>>>,
    mutation: ManuallyDrop<Option<M>>,
    authority: ManuallyDrop<Option<Arc<app_store::ArtifactStoreOneItemLiveAuthority>>>,
    copy: ManuallyDrop<Option<Box<dyn StructuralMutationCopy<S, M>>>>,
    mutation_retirement: ManuallyDrop<Option<Arc<dyn app_store::ArtifactOwnedValueRetirementFactory<M>>>>,
    snapshot_retirement: ManuallyDrop<Option<Arc<dyn app_store::SnapshotRetirementFactory<S>>>>,
    factory_close: ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>;2]>,
    sealer: ManuallyDrop<Option<app_store::ArtifactStoreOneItemSealer<S, M>>>,
    external_retirement: ManuallyDrop<Option<Box<dyn app_store::ErasedSnapshotRetirement>>>,
    checkpoint: app_store::ArtifactStoreOneItemCheckpoint,
    seal_base_checkpoint: Option<app_store::ArtifactStoreOneItemCheckpoint>,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

impl<S, M> StructuralPreparation<S, M> {
    fn progress(&mut self, items: usize, bytes: usize) -> app_store::ArtifactStoreOneItemPreparationStep {
        self.checkpoint.cursor = self.checkpoint.cursor.saturating_add(1);
        self.checkpoint.completed_items = self.checkpoint.completed_items.saturating_add(items as u32);
        self.checkpoint.completed_bytes = self.checkpoint.completed_bytes.saturating_add(bytes as u64);
        app_store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint)
    }

    fn is_terminal_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.authority.is_none() && self.copy.is_none() && self.sealer.is_none() && self.external_retirement.is_none() && self.mutation_retirement.is_none() && self.snapshot_retirement.is_none() && self.factory_close.iter().all(Option::is_none)
    }
}

impl<S, M> app_store::ArtifactStoreOneItemPreparation<S, M> for StructuralPreparation<S, M>
where
    S: semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    M: app_store::ArtifactCanonicalJson + Send + 'static,
{
    fn advance(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(app_store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if let Some(sealer) = self.sealer.as_mut() {
            let step = sealer.advance(grant)?;
            let checkpoint = match step {
                app_store::ArtifactStoreOneItemPreparationStep::Progress(checkpoint) | app_store::ArtifactStoreOneItemPreparationStep::Prepared(checkpoint) => checkpoint,
                app_store::ArtifactStoreOneItemPreparationStep::Blocked => return Ok(app_store::ArtifactStoreOneItemPreparationStep::Blocked),
            };
            let base = self.seal_base_checkpoint.ok_or_else(|| format!("{}-seal-checkpoint-owner", self.prefix))?;
            self.checkpoint = app_store::ArtifactStoreOneItemCheckpoint {
                cursor: base.cursor.saturating_add(checkpoint.cursor),
                completed_items: base.completed_items.saturating_add(checkpoint.completed_items),
                completed_bytes: base.completed_bytes.saturating_add(checkpoint.completed_bytes),
                digest: checkpoint.digest,
            };
            if matches!(step, app_store::ArtifactStoreOneItemPreparationStep::Prepared(_)) {
                self.checkpoint.digest = sealer.prepared().ok_or_else(|| format!("{}-prepared-owner", self.prefix))?.edit_digest();
                return Ok(app_store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
            }
            return Ok(app_store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        match self.phase {
            0 => {
                let step = self.copy.as_mut().ok_or_else(|| format!("{}-copy-owner", self.prefix))?.advance(
                    self.base.as_ref().ok_or_else(|| format!("{}-base-owner", self.prefix))?.get(),
                    self.mutation.as_ref().ok_or_else(|| format!("{}-mutation-owner", self.prefix))?,
                    grant.maximum_items,
                    grant.maximum_copy_bytes.min(PAGE_BYTES),
                )?;
                match step {
                    StructuralCopyStep::Progress { items, bytes } => Ok(self.progress(items, bytes)),
                    StructuralCopyStep::Complete => {
                        self.phase = 1;
                        Ok(self.progress(1, 0))
                    }
                }
            }
            1 => {
                if app_store::ArtifactStoreOneItemSealer::<S, M>::constructor_demand().admit(grant.retained_grant()).is_err() {
                    return Ok(app_store::ArtifactStoreOneItemPreparationStep::Blocked);
                }
                let (post, inverse) = self.copy.as_mut().ok_or_else(|| format!("{}-copy-owner", self.prefix))?.take_result().ok_or_else(|| format!("{}-copy-result", self.prefix))?;
                let mutation = self.mutation.take().ok_or_else(|| format!("{}-mutation-owner", self.prefix))?;
                let authority = self.authority.as_ref().ok_or_else(|| format!("{}-authority-owner", self.prefix))?;
                let edit = authority.next_edit(mutation, vec![inverse]);
                *self.sealer = Some(Arc::clone(authority).begin_one_item_seal(edit, Arc::new(post), Arc::clone(self.mutation_retirement.as_ref().expect("original mutation issuer")), Arc::clone(self.snapshot_retirement.as_ref().expect("original snapshot issuer")), grant.retained_grant()).unwrap_or_else(|_| unreachable!("pre-admitted exact Semio sealer birth")).0);
                self.seal_base_checkpoint = Some(self.checkpoint);
                self.phase = 2;
                Ok(self.progress(1, 0))
            }
            _ => Err(format!("{}-preparation-state", self.prefix)),
        }
    }

    fn checkpoint(&self) -> app_store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&app_store::ArtifactStoreOneItemPrepared<S, M>> {
        self.sealer.as_ref().and_then(app_store::ArtifactStoreOneItemSealer::prepared)
    }

    fn take_prepared(&mut self) -> Option<app_store::ArtifactStoreOneItemPrepared<S, M>> {
        self.sealer.as_mut().and_then(app_store::ArtifactStoreOneItemSealer::take_prepared)
    }

    fn cancel(&mut self) {
        self.cancelled = true;
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.cancel();
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.begin_close();
        }
        if let Some(copy) = self.copy.as_mut() {
            copy.begin_close();
        }
    }

    fn close_step(&mut self,grant:app_store::ArtifactStoreOneItemGrant)->Result<RetainedCloneStep,ValueError>{
        if self.is_terminal_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if !self.closing||!grant.permits_one(){return Ok(RetainedCloneStep::Progress(Default::default()));}
        let funded=grant.retained_grant();let demand=self.close_demands(funded.maximum_copy_bytes)?;
        if funded.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"Semio preparation exceeds original nested depth"));}
        if funded.maximum_copy_bytes<demand.copy_bytes||funded.maximum_capacity_bytes<demand.capacity_bytes||funded.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let child=RetainedCloneGrant{maximum_items:1,maximum_depth:funded.maximum_depth.saturating_sub(1),..funded};
        if self.external_retirement.is_some(){return app_store::artifact_retirement_box_close_step(&mut self.external_retirement,child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some(sealer)=self.sealer.as_mut(){if !sealer.terminal_is_empty(){return sealer.close_step(child).map(|step|RetainedCloneStep::Progress(step.progress()));}self.sealer.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        if let Some(copy)=self.copy.as_mut(){if !copy.terminal_is_empty(){let nested=app_store::ArtifactStoreOneItemGrant{maximum_items:child.maximum_items,maximum_copy_bytes:child.maximum_copy_bytes,maximum_capacity_bytes:child.maximum_capacity_bytes,maximum_release_bytes:child.maximum_release_bytes,maximum_depth:child.maximum_depth};return copy.close_step(nested).map(|step|RetainedCloneStep::Progress(step.progress()));}drop(self.copy.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}));}
        if let Some(original)=self.mutation.take(){return match self.mutation_retirement.as_ref().expect("original mutation issuer").retire_owned(original,child){Ok((owner,progress))=>{*self.external_retirement=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{*self.mutation=Some(original);Err(error)}};}
        if self.base.is_some(){return app_store::artifact_retirement_admit_owned(&mut self.base,&mut self.external_retirement,child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some(original)=self.authority.take(){return match original.retire(child){Ok((owner,progress))=>{*self.external_retirement=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Err((error,original))=>{*self.authority=Some(original);Err(error)}};}
        if let Some(original)=self.mutation_retirement.take(){self.factory_close[0]=Some(semio_framework_value::FactoryAuthority::new(original));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if let Some(original)=self.snapshot_retirement.take(){self.factory_close[1]=Some(semio_framework_value::FactoryAuthority::new(original));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if let Some(slot)=self.factory_close.iter_mut().find(|slot|slot.is_some()){let owner=slot.as_mut().unwrap();if !owner.terminal_is_empty(){return owner.step(child).map(|step|RetainedCloneStep::Progress(step.progress()));}*slot=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn terminal_is_empty(&self) -> bool {
        self.is_terminal_empty()
    }
}

impl<S:semio_framework_value::retirement::RetireOwned+Send+Sync+'static,M:Send+'static> StructuralPreparation<S,M>{
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let nested=|mut value:RetirementDemand|->Result<RetirementDemand,ValueError>{value.depth=value.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"Semio original preparation depth overflow"))?;Ok(value)};
        if let Some(owner)=self.external_retirement.as_ref(){return nested(app_store::artifact_retirement_box_demands(owner,body)?);}
        if let Some(owner)=self.sealer.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{nested(owner.retirement_demands(body)?)};}
        if let Some(owner)=self.copy.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:std::mem::size_of_val(owner.as_ref()),depth:1,..Default::default()})}else{nested(owner.retirement_demands(body)?)};}
        if let Some(original)=self.mutation.as_ref(){return nested(RetirementDemand{capacity_bytes:self.mutation_retirement.as_ref().expect("original mutation issuer").retirement_birth_bytes(original),depth:1,..Default::default()});}
        if self.base.is_some(){return nested(app_store::artifact_retirement_owned_birth_demands(&self.base)?);}
        if let Some(original)=self.authority.as_ref(){let birth=original.retirement_birth_demand();return nested(RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()});}
        if self.mutation_retirement.is_some()||self.snapshot_retirement.is_some(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),depth:1,..Default::default()});}
        if let Some(original)=self.factory_close.iter().find_map(Option::as_ref){return if original.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{nested(original.demands(body)?)};}
        Ok(Default::default())
    }
}
impl<S, M> Drop for StructuralPreparation<S, M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.is_terminal_empty(), "Semio structural preparation dropped with live owners");
    }
}
