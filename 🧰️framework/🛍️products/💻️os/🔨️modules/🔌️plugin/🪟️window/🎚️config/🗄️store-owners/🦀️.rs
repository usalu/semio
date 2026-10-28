//! 🗄️ Store owners of the window config: the store and preview owner types, the partition, the erased store owner trait, the direct ingress and the typed store owner. See [`super`].

use super::*;
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use crate::owner_contract_and_mutation::WindowConfigAuthorityIdentity;
use crate::owner_contract_and_mutation::WindowConfigBorrow;

pub(super) type WindowConfigStore<O> = store::ConfigStore<<O as WindowConfigOwner>::State, <O as WindowConfigOwner>::Mutation>;

pub(super) type WindowConfigStoreBox<O> = semio_framework_plugin_ownership::original_store_genesis::OriginalStoreSlot<<O as WindowConfigOwner>::State,<O as WindowConfigOwner>::Mutation>;

type WindowPreviewOwners = (Option<WindowConfigSnapshot>, Option<Vec<store::os_store::snapshot_read::ErasedSnapshotRead>>, Option<store::os_store::snapshot_read::ErasedSnapshotRead>);

pub(super) struct WindowConfigPartition<O: WindowConfigOwner> {
    pub(super) catalog:Option<store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>>,
    pub(super) address:Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
    pub(super) semantic_preview:Option<WindowSemanticPreview<O>>,
    pub(super) semantic_epoch:u64,
    pub(super) published_preview:Option<WindowConfigSnapshot>,
    pub(super) published_epoch:u64,
    pub(super) prepared_preview:Option<WindowConfigSnapshot>,
    pub(super) prepared_epoch:u64,
    pub(super) finish_preview_base:Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>,
    pub(super) finish_preview_read:Option<store::os_store::snapshot_read::ErasedSnapshotRead>,
    pub(super) authority: WindowConfigAuthorityIdentity,
    pub(super) store: WindowConfigStoreBox<O>,
    pub(super) disposer: Option<Box<dyn ArtifactOwnedDisposer<WindowConfigStore<O>>>>,
    pub(super) pending_preview: Option<WindowConfigSnapshot>,
    pub(super) pending_preview_reads: Option<Vec<store::os_store::snapshot_read::ErasedSnapshotRead>>,
    pub(super) pending_preview_projection: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>,
    pub(super) pending_preview_projection_read: Option<store::os_store::snapshot_read::ErasedSnapshotRead>,
    pub(super) preview_retirement: Option<ControlledRetirement<WindowPreviewOwners>>,
    pub(super) returned_read_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
}

impl<O:WindowConfigOwner> WindowConfigPartition<O>{
    pub(crate) fn original_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
            if let Some(catalog)=self.catalog.as_ref(){return if catalog.uninstalled_owners_terminal_is_empty(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>>>(),depth:1,..Default::default()})}else{catalog.uninstalled_owners_demands(body)}}
            if self.preview_close_pending() { return self.preview_close_demand(body); }
            if self.returned_read_pending() { return self.returned_read_demands(body); }
            if let Some(address)=self.address.as_ref(){return if address.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<semio_framework_value::SharedUtf8>>>(),depth:1,..Default::default()})}else{address.next_demand(body)};}
            return self.disposer.as_ref().map_or(Ok(if self.store.terminal_is_empty(){RetirementDemand { copy_bytes: size_of::<usize>(), depth: 1, ..Default::default() }}else{self.store.box_close_demands()}), |disposer| {
                if disposer.terminal_is_empty(&self.store) { Ok(RetirementDemand { release_bytes: size_of_val(disposer.as_ref()), depth: 1, ..Default::default() }) }
                else { disposer.retirement_demands(&self.store, body) }
            });
    }
    pub(crate) fn original_close_step(&mut self,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,Fault>{let demand=self.original_close_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;if !registry::fits(grant,demand){return Ok(PluginLifecycleStep::Progress(Default::default()));}
            if let Some(catalog)=self.catalog.as_mut(){if catalog.uninstalled_owners_terminal_is_empty(){self.catalog=None;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}let step=catalog.close_uninstalled_owners_step(grant).map_err(ValueError::into_fault)?;return Ok(PluginLifecycleStep::retained(step,false));}
            if self.preview_close_pending() { return self.preview_close_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
            if self.returned_read_pending() { return self.returned_read_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
            if let Some(address)=self.address.as_mut(){if address.terminal_is_empty(){self.address=None;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}return address.step(grant).map(|step|PluginLifecycleStep::retained(step,false)).map_err(ValueError::into_fault);}
            if let Some(disposer) = self.disposer.as_mut() {
                if disposer.terminal_is_empty(&self.store) {
                    drop(self.disposer.take());
                    return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
                }
                let step = disposer.close_step(&mut self.store, grant)?;
                return Ok(match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other });
            }
        self.store.box_close_step(grant).map(|step|PluginLifecycleStep::retained(step,false)).map_err(ValueError::into_fault)
    }
    pub(crate) fn original_terminal(&self)->bool{self.catalog.is_none()&&!self.preview_close_pending()&&!self.returned_read_pending()&&self.address.is_none()&&self.disposer.is_none()&&self.store.terminal_is_empty()}
    pub(super) fn returned_read_pending(&self)->bool {self.returned_read_retirement.is_some()||(!self.store.terminal_is_empty()&&self.store.returned_snapshot_read_retirement_demand().map_or(true,|demand|demand!=RetirementDemand::default()))}
    pub(super) fn returned_read_demands(&self,body:usize)->Result<RetirementDemand,ValueError> {match self.returned_read_retirement.as_ref(){Some(owner)=>store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body),None=>self.store.returned_snapshot_read_retirement_demand()}}
    pub(super) fn returned_read_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.returned_read_retirement.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.returned_read_retirement,grant);}
        let(owner,progress)=self.store.take_returned_snapshot_read_retirement(grant)?;
        self.returned_read_retirement=owner;
        Ok(RetainedCloneStep::Progress(progress))
    }
    pub(super) fn catalog_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.store.owned_disposer_installed(){return Ok(Default::default());}
        if let Some(catalog)=self.catalog.as_ref(){return if catalog.constructor_is_complete(){Ok(RetirementDemand{copy_bytes:2*size_of::<store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>>(),depth:1,..Default::default()})}else{catalog.constructor_demands(body)};}
        let mut d=store::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<O::State,O::Mutation>()?;let extra=O::preparation_source_birth();d.capacity_bytes=d.capacity_bytes.checked_add(extra.capacity_bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Window source catalog birth overflow"))?;d.copy_bytes=2*size_of::<store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>>();d.depth=d.depth.max(extra.depth);Ok(d)
    }
    pub(super) fn catalog_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.store.owned_disposer_installed(){return Ok(RetainedCloneStep::Complete(Default::default()));}let d=self.catalog_demands(grant.maximum_copy_bytes)?;if !registry::fits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.catalog.is_none(){return match O::build_store_owners(grant){Ok((owners,p))=>{self.catalog=Some(owners);Ok(RetainedCloneStep::Progress(p))},Err(error)=>{self.catalog=error.owners;Err(error.error.with_retained_progress(error.progress))}};}
        let owners=self.catalog.as_mut().unwrap();if !owners.constructor_is_complete(){return owners.admit_constructor(grant).map(RetainedCloneStep::Progress).map_err(|(error,p)|error.with_retained_progress(p));}
        let original=self.catalog.take().unwrap();match self.store.install_document_store_owners_exact(original){Ok(())=>Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),Err((error,owners))=>{self.catalog=Some(owners);Err(error)}}
    }
    pub(super) fn preview_finish_demands(&self,epoch:u64,body:usize)->Result<RetirementDemand,ValueError>{
        if self.prepared_epoch==epoch{return Ok(RetirementDemand{depth:1,..Default::default()})}
        if self.pending_preview.is_some()||self.preview_retirement.is_some(){return self.preview_retirement_demand(body);}
        if self.semantic_epoch!=epoch||self.semantic_preview.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<WindowConfigSnapshot>>()+size_of::<u64>(),depth:1,..Default::default()});}
        if let Some(base)=self.finish_preview_base.as_ref(){if self.finish_preview_read.is_none(){return self.store.retained_derived_read_demands(base);}return Ok(RetirementDemand{copy_bytes:size_of::<WindowConfigSnapshot>()+size_of::<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>()+size_of::<u64>(),depth:1,..Default::default()});}
        if self.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::leaf_ready){return Ok(RetirementDemand{copy_bytes:size_of::<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>(),depth:1,..Default::default()});}
        self.preview_retirement_demand(body)
    }
    pub(super) fn preview_finish_advance(&mut self,epoch:u64,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(bool,RetainedCloneProgress),ValueError>{
        if let Some(owner)=self.semantic_preview.as_mut(){if !owner.leaf_ready()&&!owner.terminal_is_empty(){return owner.advance(&self.store,body,grant,receiving).map(|step|(false,step.progress()));}}
        self.preview_finish_step(epoch,grant)
    }
    pub(super) fn preview_finish_step(&mut self,epoch:u64,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{let d=self.preview_finish_demands(epoch,grant.maximum_copy_bytes)?;if !registry::fits(grant,d){return Ok((false,Default::default()));}
        if self.prepared_epoch==epoch{return Ok((true,Default::default()))}
        if self.prepared_preview.is_some()||self.prepared_epoch!=0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Window candidate requires prior epoch closure"))}
        if self.pending_preview.is_some()||self.preview_retirement.is_some(){return self.preview_retirement_step(grant).map(|step|(false,step.progress()));}
        if self.semantic_epoch!=epoch||self.semantic_preview.is_none(){self.prepared_epoch=epoch;return Ok((true,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
        if let Some(base)=self.finish_preview_base.as_ref(){if self.finish_preview_read.is_none(){return self.store.admit_retained_derived_read(base,&mut self.finish_preview_read,grant).map(|step|(false,step.progress()));}
            let read=self.finish_preview_read.take().unwrap();let snapshot=WindowConfigSnapshot{authority:self.authority,window_id:self.address.as_ref().and_then(ControlledRetirement::original).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window preview lost its address"))?.clone(),window_kind_id:O::WINDOW_KIND_ID,generation:self.store.generation(),revision:self.store.content_revision_now(),snapshot:read};self.prepared_preview=Some(snapshot);self.prepared_epoch=epoch;drop(self.finish_preview_base.take());return Ok((true,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
        let owner=self.semantic_preview.as_mut().unwrap();if owner.leaf_ready(){if let Some((base,p))=owner.take_base(grant)?{self.finish_preview_base=Some(base);return Ok((false,p));}}self.preview_retirement_step(grant).map(|step|(false,step.progress()))
    }
    pub(super)  fn preview_retirement_pending(&self)->bool{self.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::work_pending)||self.pending_preview.is_some()||self.pending_preview_reads.is_some()||self.pending_preview_projection.is_some()||self.pending_preview_projection_read.is_some()||self.preview_retirement.is_some()}
    pub(super) fn preview_close_pending(&self)->bool{self.semantic_preview.is_some()||self.published_preview.is_some()||self.prepared_preview.is_some()||self.prepared_epoch!=0||self.finish_preview_base.is_some()||self.finish_preview_read.is_some()||self.preview_retirement_pending()}
    pub(super) fn preview_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.preview_cancel_demand(body,true)}
    pub(super) fn preview_cancel_demand(&self,body:usize,clear_published:bool)->Result<RetirementDemand,ValueError>{
        if self.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::needs_cancellation){return Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()});}
        if self.pending_preview.is_some()||self.pending_preview_projection_read.is_some()||self.pending_preview_projection.is_some()||self.preview_retirement.is_some(){return self.preview_retirement_demand(body);}
        if self.prepared_preview.is_some()||self.prepared_epoch!=0{return Ok(RetirementDemand{copy_bytes:size_of::<Option<WindowConfigSnapshot>>()+size_of::<u64>(),depth:1,..Default::default()})}
        if clear_published&&self.published_preview.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<WindowConfigSnapshot>>(),depth:1,..Default::default()});}
        if self.finish_preview_read.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<store::os_store::snapshot_read::ErasedSnapshotRead>>(),depth:1,..Default::default()});}
        if self.finish_preview_base.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>>(),depth:1,..Default::default()});}
        if self.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::needs_cancellation){return Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()});}self.preview_retirement_demand(body)
    }
    pub(super) fn preview_close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.preview_cancel_step(grant,true)}
    pub(super) fn preview_cancel_step(&mut self,grant:RetainedCloneGrant,clear_published:bool)->Result<RetainedCloneStep,ValueError>{let d=self.preview_cancel_demand(grant.maximum_copy_bytes,clear_published)?;if !registry::fits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::needs_cancellation){self.semantic_preview.as_mut().unwrap().cancel();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}
        if self.pending_preview.is_some()||self.pending_preview_projection_read.is_some()||self.pending_preview_projection.is_some()||self.preview_retirement.is_some(){return self.preview_retirement_step(grant);}
        let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
        if self.prepared_preview.is_some()||self.prepared_epoch!=0{self.pending_preview=self.prepared_preview.take();self.prepared_epoch=0;return Ok(RetainedCloneStep::Progress(p))}
        if clear_published&&self.published_preview.is_some(){self.pending_preview=self.published_preview.take();return Ok(RetainedCloneStep::Progress(p));}
        if self.finish_preview_read.is_some(){self.pending_preview_projection_read=self.finish_preview_read.take();return Ok(RetainedCloneStep::Progress(p));}
        if self.finish_preview_base.is_some(){self.pending_preview_projection=self.finish_preview_base.take();return Ok(RetainedCloneStep::Progress(p));}
        if self.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::needs_cancellation){self.semantic_preview.as_mut().unwrap().cancel();return Ok(RetainedCloneStep::Progress(p));}self.preview_retirement_step(grant,None)
    }
    pub(super)  fn preview_retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.semantic_preview.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<WindowSemanticPreview<O>>>(),depth:1,..Default::default()})}else{owner.demand(&self.store,body)};}
        if let Some(projection)=self.pending_preview_projection.as_ref(){let mut d=self.store.retained_derived_read_demands(projection)?;d.copy_bytes+=size_of::<Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>>();return Ok(d);}
        if let Some(owner)=self.preview_retirement.as_ref(){
            if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<WindowPreviewOwners>>>(),depth:1,..Default::default()});}
            return owner.next_demand(body);
        }
        Ok(RetirementDemand{copy_bytes:size_of::<WindowPreviewOwners>(),depth:1,..Default::default()})
    }
    pub(super) fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.preview_retirement_step_original(grant.maximum_copy_bytes,grant,None)}
    fn preview_retirement_step_original(&mut self,body:usize,grant:RetainedCloneGrant,mut control:Option<&mut dyn semio_framework_value::OriginalRetirementControl>)->Result<RetainedCloneStep,ValueError>{
        if !self.preview_retirement_pending(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        let demand=self.preview_retirement_demand(body)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(owner)=self.semantic_preview.as_mut(){if owner.terminal_is_empty(){self.semantic_preview=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}return match control.as_deref_mut(){Some(control)=>owner.close_controlled(&self.store,body,grant,control),None=>owner.step(&self.store,grant)};}
        if let Some(projection)=self.pending_preview_projection.as_ref(){
            if let Some(control)=control.as_deref_mut(){control.charge(demand.capacity_bytes)?;}
            let overhead=size_of::<Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>>();
            let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant};
            let step=self.store.admit_retained_derived_read(projection,&mut self.pending_preview_projection_read,child)?;
            let mut progress=step.progress();
            if matches!(step,RetainedCloneStep::Complete(_)){drop(self.pending_preview_projection.take());progress.copied_bytes+=overhead;}
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if let Some(owner)=self.preview_retirement.as_mut(){
            if owner.terminal_is_empty(){drop(self.preview_retirement.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
            return match control.as_deref_mut(){Some(control)=>owner.select(body)?.step_controlled(grant,control),None=>owner.step(grant)};
        }
        let originals=(self.pending_preview.take(),self.pending_preview_reads.take(),self.pending_preview_projection_read.take());
        match ControlledRetirement::new(originals){
            Ok(owner)=>self.preview_retirement=Some(owner),
            Err((error,(preview,reads,read)))=>{self.pending_preview=preview;self.pending_preview_reads=reads;self.pending_preview_projection_read=read;return Err(error);}
        }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
    }
}

pub(super) trait ErasedWindowConfigStoreOwner: Send {
    fn admit_retained_load(&mut self,lifetime:u64,pack:&mut Option<WindowConfigPack>,actor:&protocol::ActorId,cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<WindowConfigPackLoad>)->Result<bool,semio_framework_job::OriginalTurnFailure>;
    fn retained_load_commit_demands(&self,load:&dyn retained::ErasedWindowConfigPackLoad)->Result<RetirementDemand,ValueError>;
    fn retained_load_insert_demands(&self,load:&dyn retained::ErasedWindowConfigPackLoad)->Result<Option<RetirementDemand>,ValueError>;
    fn cancel_retained_load_insert(&mut self,load:&mut dyn retained::ErasedWindowConfigPackLoad,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError>;
    fn ensure_partition(&mut self,window:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>;
    fn close_step_context(&mut self,grant:RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>;
    fn admit_catalog(&mut self,authority:&WindowConfigAuthority,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>;
    fn borrow<'a>(&'a self, window_id: &'a str) -> Option<WindowConfigBorrow<'a>>;
    fn borrow_preview<'a>(&'a self,window:&'a str,epoch:u64)->Option<WindowConfigBorrow<'a>>;
    fn capture<'a>(&'a mut self, window_id: &'a str) -> Pin<Box<dyn Future<Output = Result<WindowConfigAuthority, Fault>> + 'a>>;
    fn capture_original_header(&self,window_id:&str,cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<WindowConfigAuthority>)->Result<bool,semio_framework_job::OriginalTurnFailure>;

    fn dispatch<'a, 'b>(&'a mut self, actor: &'a str, mutation: WindowConfigMutation, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> Pin<Box<dyn Future<Output = Result<(), Fault>> + 'a>>;
    fn admit_begin(&mut self, operation: semio_framework_job::OperationId, actor: &mut Option<ControlledRetirement<semio_framework_value::SharedUtf8>>, mutations: &mut Vec<WindowConfigMutation>, authority: &WindowConfigAuthority, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedWindowConfigPublication>,RetainedCloneProgress)>,ValueError>;
    fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, body:usize, grant: store::os_store::one_item::ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault>;
    fn refresh_demands(&self,authority:&WindowConfigAuthority,body:usize)->Result<RetirementDemand,ValueError>;
    fn refresh(&mut self,authority:&mut WindowConfigAuthority,grant:RetainedCloneGrant)->Result<crate::window_mutation::WindowAuthorityRefreshStep,Fault>;
    fn packs<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<WindowConfigPack>, Fault>> + 'a>>;
    fn commit_retained_load(&mut self, registry_lifetime: u64, load: &mut dyn retained::ErasedWindowConfigPackLoad,grant:RetainedCloneGrant) -> Result<WindowConfigPackLoadStep, WindowConfigPackLoadDiagnostic>;
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>;
    fn direct_ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
    fn close_direct_ingress(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
    fn direct_ingress_terminal_is_empty(&self)->bool;
    fn terminal_is_empty(&self) -> bool;
    fn maintenance_retirements_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault>;
    fn maintenance_retirements_terminal_is_empty(&self) -> bool;
    fn maintenance_retirements_under_pressure(&self) -> bool;
    fn pointer_values(&self, pointers: &[String]) -> Vec<(String, Vec<Option<semio_framework_value::DslValue>>)>;
    fn snapshot(&self, window_id: &str) -> Option<WindowConfigSnapshot>;
    fn preview_input_demands(&self,window:&str,epoch:u64,body:usize)->Result<RetirementDemand,ValueError>;
    fn preview_input(&mut self,input:&semio_framework_value::retained_clone::RetainedOwnedProjection<WindowConfigMutation>,epoch:u64,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>;
    fn preview_leaf_demands(&self,window:&str,epoch:u64,body:usize)->Result<RetirementDemand,ValueError>;
    fn preview_leaf_step(&mut self,window:&str,epoch:u64,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(bool,RetainedCloneProgress),ValueError>;
    fn preview_leaf_ready(&self,window:&str,epoch:u64)->bool;
    fn preview_input_held(&self,window:&str,epoch:u64)->bool;
    fn preview_partition_count(&self)->usize;
    fn preview_epoch_close_demands(&self,index:usize,epoch:u64,published:bool,body:usize)->Result<RetirementDemand,ValueError>;
    fn preview_epoch_close_step(&mut self,index:usize,epoch:u64,published:bool,body:usize,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<(bool,RetainedCloneProgress),ValueError>;
    fn preview_partition_demands(&self,index:usize,epoch:u64,closing:bool,body:usize)->Result<RetirementDemand,ValueError>;
    fn preview_partition_step(&mut self,index:usize,epoch:u64,closing:bool,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(bool,RetainedCloneProgress),ValueError>;
    fn preview_or<'a>(&'a self,committed:&'a WindowConfigSnapshot,epoch:u64)->&'a WindowConfigSnapshot;
    fn preview_partition_index(&self,window:&str)->Option<usize>;
    fn preview_generation(&self,window:&str)->Option<u64>;
    fn preview_available(&self,window_id:&str)->bool;
    fn can_retire_preview(&self,preview:&WindowConfigSnapshot)->bool;
    fn retire_preview(&mut self,preview:&mut Option<WindowConfigSnapshot>)->bool;
    fn preview_retirement_pending(&self)->bool;
    fn preview_retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
    fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
}

#[derive(semio_framework_value::RetireOwned)]
#[doc(hidden)]
pub struct DirectIngress<M:Send+'static>{#[doc(hidden)] pub mutations:Vec<WindowConfigMutation>,#[doc(hidden)] pub allocations:Vec<FactoryBoxedPublication<M>>,#[doc(hidden)] pub addresses:Vec<String>}

#[doc(hidden)]
pub struct TypedWindowConfigStoreOwner<O: WindowConfigOwner> {
    #[doc(hidden)] pub initializing:Option<Box<WindowConfigPartitionBirth<O>>>,
    #[doc(hidden)] pub initialized_window:Option<String>,
    #[doc(hidden)] pub initialized_store:Option<Box<WindowConfigStore<O>>>,
    #[doc(hidden)] pub initialized_address:Option<semio_framework_value::SharedUtf8>,
    #[doc(hidden)] pub registry_address:Option<semio_framework_value::SharedUtf8>,
    #[doc(hidden)] pub pending_partition:Option<WindowConfigPartition<O>>,
    #[doc(hidden)] pub partition_inserting:bool,
    #[doc(hidden)] pub initialization_residue:Option<ControlledRetirement<(Option<String>,Option<semio_framework_value::SharedUtf8>,Option<semio_framework_value::SharedUtf8>)>>,
    #[doc(hidden)]
    pub direct_ingress:ControlledRetirement<DirectIngress<O::Mutation>>,
    #[doc(hidden)]
    pub partitions: WindowRegistry<semio_framework_value::SharedUtf8, WindowConfigPartition<O>>,
    #[doc(hidden)]
    pub partition_close_cursor: usize,
    #[doc(hidden)]
    pub partition_address_retirement: Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
    #[doc(hidden)]
    pub actor: Option<protocol::ActorId>,
    #[doc(hidden)]
    pub actor_retirement: Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
}

impl<O: WindowConfigOwner> TypedWindowConfigStoreOwner<O> {
    /// 🏠️ Registers only the source owner; concrete partitions are born by their mounted caller.
    pub fn new(actor:protocol::ActorId)->Self{Self{initializing:None,initialized_window:None,initialized_store:None,initialized_address:None,registry_address:None,pending_partition:None,partition_inserting:false,initialization_residue:None,direct_ingress:ControlledRetirement::new(DirectIngress{mutations:Vec::new(),allocations:Vec::new(),addresses:Vec::new()}).map_err(|(error,_)|error).expect("direct ingress declares controlled retirement"),partitions:WindowRegistry::new(),partition_close_cursor:0,partition_address_retirement:None,actor:Some(actor),actor_retirement:None}}
    #[doc(hidden)]
    pub async fn partition(&mut self,window_id:&str)->Result<&mut WindowConfigPartition<O>,Fault>{self.partitions.get_mut(window_id).ok_or_else(||Fault::new(FaultOrigin::Framework,FaultCode::new("window-config.owned-initialization"),"window partition requires its original mounted initialization"))}
    fn initialization_pending(&self)->bool{self.initializing.is_some()||self.initialized_window.is_some()||self.initialized_store.is_some()||self.initialized_address.is_some()||self.registry_address.is_some()||self.pending_partition.is_some()||self.partition_inserting||self.initialization_residue.is_some()}
    fn initialization_close_demands(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        if let Some(owner)=self.initializing.as_ref(){return Ok(Some(if owner.terminal_is_empty(){WindowConfigPartitionBirth::<O>::boxed_release_demands()}else{owner.retirement_demands(body)?}));}
        if self.partition_inserting{let mut demand=self.partitions.cancel_insert_original_demands()?;demand.copy_bytes+=size_of::<bool>();return Ok(Some(demand));}
        if let Some(partition)=self.pending_partition.as_ref(){return Ok(Some(if partition.original_terminal(){RetirementDemand{copy_bytes:size_of::<Option<WindowConfigPartition<O>>>(),depth:1,..Default::default()}}else{partition.original_close_demands(body)?}));}
        if let Some(store)=self.initialized_store.as_ref(){return Ok(Some(if store.close_owned_store_terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<Option<Box<WindowConfigStore<O>>>>(),release_bytes:size_of::<WindowConfigStore<O>>(),depth:1,..Default::default()}}else{store.close_owned_demands(body)?}));}
        if self.initialized_window.is_some()||self.initialized_address.is_some()||self.registry_address.is_some(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<ControlledRetirement<(Option<String>,Option<semio_framework_value::SharedUtf8>,Option<semio_framework_value::SharedUtf8>)>>()+2*size_of::<(Option<String>,Option<semio_framework_value::SharedUtf8>,Option<semio_framework_value::SharedUtf8>)>(),depth:1,..Default::default()}));}
        if let Some(owner)=self.initialization_residue.as_ref(){return Ok(Some(if owner.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<(Option<String>,Option<semio_framework_value::SharedUtf8>,Option<semio_framework_value::SharedUtf8>)>>>(),depth:1,..Default::default()}}else{owner.next_demand(body)?}));}
        Ok(None)
    }
    fn ensure_original_partition(&mut self,window:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(false);}
        if !self.initialization_pending()&&self.partitions.contains_key(window){return Ok(true);}
        if let Some(owner)=self.initializing.as_mut(){
            if owner.terminal_is_empty(){let d=WindowConfigPartitionBirth::<O>::boxed_release_demands();semio_framework_job::close_original_retained_step(cx,d,|_|{self.initializing=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))})?;return Ok(false);}
            if !owner.window_id().is_empty()&&owner.window_id()!=window{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Window initialization retains another mounted address").into());}
            if let Some((address,store))=owner.step(cx)?{self.initialized_window=Some(address);self.initialized_store=Some(store);}return Ok(false);
        }
        if self.initialized_window.is_some(){let d=semio_framework_value::SharedUtf8::admission_demand();let grant=cx.retained_grant();let mut admitted=None;semio_framework_job::admit_original_construction(cx,&mut admitted,window_construction(d),|_|{let original=self.initialized_window.take().unwrap();match semio_framework_value::SharedUtf8::admit(original,grant){Ok((address,receipt))=>{self.initialized_address=Some(address);Ok(((),receipt))},Err((error,original))=>{self.initialized_window=Some(original);Err(error)}}})?;return Ok(false);}
        if self.registry_address.is_none()&&self.initialized_address.is_some(){let d=semio_framework_value::SharedUtf8::lease_demand();let grant=cx.retained_grant();let mut admitted=None;semio_framework_job::admit_original_construction(cx,&mut admitted,window_construction(d),|_|{let(address,receipt)=self.initialized_address.as_ref().unwrap().admit_clone(grant)?;self.registry_address=Some(address);Ok(((),receipt))})?;return Ok(false);}
        if self.pending_partition.is_none()&&self.initialized_store.is_some(){let mut d=O::store_disposer_birth()?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<WindowConfigPartition<O>>()+size_of::<Box<WindowConfigStore<O>>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Window partition birth extent overflow"))?;d.depth=d.depth.max(1);let mut admitted=None;semio_framework_job::admit_original_construction(cx,&mut admitted,window_construction(d),|_|{let authority=WindowConfigAuthorityIdentity::issue().ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Window partition authority exhausted"))?;self.pending_partition=Some(WindowConfigPartition{catalog:None,address:Some(ControlledRetirement::new(self.initialized_address.take().unwrap()).map_err(|(error,_)|error).expect("original Window address supports controlled closure")),semantic_preview:None,semantic_epoch:0,published_preview:None,published_epoch:0,prepared_preview:None,prepared_epoch:0,finish_preview_base:None,finish_preview_read:None,authority,store:WindowConfigStoreBox::<O>::from_original(self.initialized_store.take().unwrap()),disposer:Some(O::build_store_disposer()),pending_preview:None,pending_preview_reads:None,pending_preview_projection:None,pending_preview_projection_read:None,preview_retirement:None,returned_read_retirement:None});self.partition_inserting=true;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))})?;return Ok(false);}
        if self.partition_inserting{let d=self.partitions.insert_original_demands()?;let grant=cx.retained_grant();let mut ready=None;semio_framework_job::admit_original_construction(cx,&mut ready,window_construction(d),|_|self.partitions.insert_original(&mut self.registry_address,&mut self.pending_partition,grant))?;if ready==Some(true){self.partition_inserting=false;return Ok(true);}return Ok(false);}
        self.initializing=WindowConfigPartitionBirth::<O>::admit(window,self.actor.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window session actor absent"))?,cx)?;Ok(false)
    }

}

impl<O: WindowConfigOwner> ErasedWindowConfigStoreOwner for TypedWindowConfigStoreOwner<O> {
    fn admit_retained_load(&mut self,lifetime:u64,pack:&mut Option<WindowConfigPack>,actor:&protocol::ActorId,cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<WindowConfigPackLoad>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let generation=pack.as_ref().and_then(|pack|self.partitions.get(&pack.window_id)).map(|partition|partition.store.generation());retained::admit_typed_window_config_pack_load::<O>(lifetime,generation,pack,actor,cx,output)}
    fn retained_load_commit_demands(&self,load:&dyn retained::ErasedWindowConfigPackLoad)->Result<RetirementDemand,ValueError>{retained::typed_window_config_pack_load_commit_demands::<O>(&self.partitions,load)}
    fn retained_load_insert_demands(&self,load:&dyn retained::ErasedWindowConfigPackLoad)->Result<Option<RetirementDemand>,ValueError>{if !load.insertion_pending(){return Ok(None);}let mut demand=self.partitions.cancel_insert_original_demands()?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<bool>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Window load cancellation extent overflow"))?;demand.depth=demand.depth.max(1);Ok(Some(demand))}
    fn cancel_retained_load_insert(&mut self,load:&mut dyn retained::ErasedWindowConfigPackLoad,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError>{retained::cancel_typed_window_config_pack_load::<O>(&mut self.partitions,load,grant)}
    fn ensure_partition(&mut self,window:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.ensure_original_partition(window,cx)}
    fn close_step_context(&mut self,grant:RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        if let Some(owner)=self.initializing.as_mut(){if !owner.terminal_is_empty(){return owner.close_step(cx).map(|step|PluginLifecycleStep::retained(step,false)).map_err(Into::into);}let d=WindowConfigPartitionBirth::<O>::boxed_release_demands();return semio_framework_job::close_original_retained_step(cx,d,|_|{self.initializing=None;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(Into::into);}
        if self.partition_inserting{let mut d=self.partitions.cancel_insert_original_demands()?;d.copy_bytes+=size_of::<bool>();let before=cx.retained_progress();let mut done=None;semio_framework_job::admit_original_construction(cx,&mut done,window_construction(d),|_|{let(ready,mut p)=self.partitions.cancel_insert_original(&mut self.registry_address,&mut self.pending_partition,grant)?;if ready{self.partition_inserting=false;}p.copied_bytes+=size_of::<bool>();Ok((ready,p))})?;return Ok(PluginLifecycleStep::Progress(semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?));}
        if let Some(partition)=self.pending_partition.as_ref(){let terminal=partition.original_terminal();let d=if terminal{RetirementDemand{copy_bytes:size_of::<Option<WindowConfigPartition<O>>>(),depth:1,..Default::default()}}else{partition.original_close_demands(grant.maximum_copy_bytes)?};return semio_framework_job::close_original_retained_turn(cx,d,|g|{if terminal{self.pending_partition=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}self.pending_partition.as_mut().unwrap().original_close_step(g).map(|step|RetainedCloneStep::Progress(step.progress().unwrap_or_default())).map_err(Into::into)}).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(Into::into);}
        if let Some(store)=self.initialized_store.as_ref(){let terminal=store.close_owned_store_terminal_is_empty();let d=if terminal{RetirementDemand{copy_bytes:size_of::<Option<Box<WindowConfigStore<O>>>>(),release_bytes:size_of::<WindowConfigStore<O>>(),depth:1,..Default::default()}}else{store.close_owned_demands(grant.maximum_copy_bytes)?};return semio_framework_job::close_original_retained_step(cx,d,|g|{if terminal{self.initialized_store=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}));}self.initialized_store.as_mut().unwrap().close_owned_store_step(g)}).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(Into::into);}
        if self.initialized_window.is_some()||self.initialized_address.is_some()||self.registry_address.is_some(){let d=RetirementDemand{copy_bytes:size_of::<ControlledRetirement<(Option<String>,Option<semio_framework_value::SharedUtf8>,Option<semio_framework_value::SharedUtf8>)>>()+2*size_of::<(Option<String>,Option<semio_framework_value::SharedUtf8>,Option<semio_framework_value::SharedUtf8>)>(),depth:1,..Default::default()};return semio_framework_job::close_original_retained_step(cx,d,|_|{let original=(self.initialized_window.take(),self.initialized_address.take(),self.registry_address.take());self.initialization_residue=Some(ControlledRetirement::new(original).map_err(|(error,_)|error).expect("Window initialization text declares controlled retirement"));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(Into::into);}
        if let Some(owner)=self.initialization_residue.as_ref(){let terminal=owner.terminal_is_empty();let d=if terminal{RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<(Option<String>,Option<semio_framework_value::SharedUtf8>,Option<semio_framework_value::SharedUtf8>)>>>(),depth:1,..Default::default()}}else{owner.next_demand(grant.maximum_copy_bytes)?};return semio_framework_job::close_original_retained_step(cx,d,|g|{if terminal{self.initialization_residue=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}self.initialization_residue.as_mut().unwrap().step(g)}).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(Into::into);}
        let d=self.retirement_demands(grant.maximum_copy_bytes)?;
        semio_framework_job::close_original_retained_turn(cx,d,|grant|self.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress().unwrap_or_default())).map_err(Into::into)).map(|step|PluginLifecycleStep::retained(RetainedCloneStep::Progress(step.progress()),self.terminal_is_empty())).map_err(Into::into)
    }
    fn borrow<'a>(&'a self, window_id: &'a str) -> Option<WindowConfigBorrow<'a>> {
        let partition = self.partitions.get(window_id)?;
        Some(WindowConfigBorrow { authority: partition.authority, window_id, window_kind_id: O::WINDOW_KIND_ID, generation: partition.store.generation(), revision: partition.store.content_revision_now(), state: partition.store.snapshot_ref() })
    }
    fn borrow_preview<'a>(&'a self,window:&'a str,epoch:u64)->Option<WindowConfigBorrow<'a>>{let partition=self.partitions.get(window)?;let preview=if partition.prepared_epoch==epoch{partition.prepared_preview.as_ref()}else if partition.published_epoch==epoch{partition.published_preview.as_ref()}else{None};if let Some(preview)=preview.filter(|preview|preview.generation()==partition.store.generation()){return Some(WindowConfigBorrow{authority:preview.authority,window_id:window,window_kind_id:O::WINDOW_KIND_ID,generation:preview.generation,revision:preview.revision,state:preview.get::<O>()?})}self.borrow(window)}
    fn capture_original_header(&self,window_id:&str,cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<WindowConfigAuthority>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let partition=self.partitions.get(window_id).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window capture requires its existing native partition"))?;
        let copy=2*size_of::<WindowConfigAuthority>()+size_of::<WindowConfigSnapshot>();
        semio_framework_job::admit_original_construction(cx,output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:copy,capacity_bytes:0,release_bytes:0,depth:1},|_|{
            let address=partition.address.as_ref().and_then(ControlledRetirement::original).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window capture lost its registered address"))?;
            let snapshot=partition.store.snapshot_read_original()?.into_erased();
            Ok((WindowConfigAuthority{window_id:String::new(),window_kind_id:String::new(),generation:partition.store.generation_now(),revision:partition.store.content_revision_now(),snapshot:WindowConfigSnapshot{authority:partition.authority,window_id:address.clone(),window_kind_id:O::WINDOW_KIND_ID,generation:partition.store.generation_now(),revision:partition.store.content_revision_now(),snapshot},pending_snapshot:None,retired_snapshots:Default::default()},RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))
        })
    }
    fn capture<'a>(&'a mut self, window_id: &'a str) -> Pin<Box<dyn Future<Output = Result<WindowConfigAuthority, Fault>> + 'a>> {
        Box::pin(async move {
            let partition = self.partition(window_id).await?;
            Ok(WindowConfigAuthority {
                window_id: window_id.to_string(),
                window_kind_id: O::WINDOW_KIND_ID.to_string(),
                generation: partition.store.generation(),
                revision: partition.store.content_revision_now(),
                pending_snapshot:None,retired_snapshots:Default::default(),
                snapshot: WindowConfigSnapshot {
                    authority: partition.authority,
                    window_id: partition.address.as_ref().and_then(ControlledRetirement::original).expect("original Window address remains registered").clone(),
                    window_kind_id: O::WINDOW_KIND_ID,
                    generation: partition.store.generation(),
                    revision: partition.store.content_revision_now(),
                    snapshot: partition.store.snapshot_read().map_err(|error|error.into_fault())?.into_erased(),
                },
            })
        })
    }

    fn dispatch<'a, 'b>(&'a mut self,_actor:&'a str,mutation:WindowConfigMutation,_identity:&'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>)->Pin<Box<dyn Future<Output=Result<(),Fault>>+'a>>{
        Box::pin(async move{self.direct_ingress.original_mut().expect("live direct ingress remains original").mutations.push(mutation);Err(Fault::new(FaultOrigin::Framework,FaultCode::new("window-config.owned-operation"),"window configuration publication requires its original mounted operation"))})
    }
    fn admit_catalog(&mut self,authority:&WindowConfigAuthority,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{let partition=self.partitions.get_mut(&authority.window_id).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window catalog requires its captured partition"))?;if partition.store.owned_disposer_installed(){return Ok(None);}partition.catalog_step(grant).map(|step|Some(step.progress()))}
    /// 🎟️ Admits one original typed frame and stops before the separately paid Store constructor.
    fn admit_begin(
        &mut self, operation: semio_framework_job::OperationId,
        actor: &mut Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
        mutations: &mut Vec<WindowConfigMutation>, authority: &WindowConfigAuthority, grant: RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn ErasedWindowConfigPublication>,RetainedCloneProgress)>,ValueError> {
        let original=mutations.last().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original window publication requires its retained input"))?;
        if original.window_id!=authority.window_id||original.window_kind_id!=O::WINDOW_KIND_ID||authority.window_kind_id!=O::WINDOW_KIND_ID||!original.mutation.as_any().is::<O::Mutation>(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original window input differs from its captured owner"));}
        let captured=actor.as_ref().filter(|owner|owner.original_is_untouched()).and_then(ControlledRetirement::original).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original window publication requires its paid actor capture"))?;
        if self.actor.as_ref().is_none_or(|opened|opened.0.as_str()!=captured.as_str()){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original window actor differs from its opened session"));}
        let partition=self.partitions.get(&original.window_id).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original captured window partition is absent"))?;
        if partition.store.generation()!=authority.generation||partition.store.content_revision_now()!=authority.revision{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original window authority is stale"));}
        let frame=size_of::<TypedWindowConfigPublication<O>>();
        let copied_bytes=[frame,size_of::<WindowConfigMutation>(),size_of::<FactoryBoxedValue<O::Mutation>>(),size_of::<O::Mutation>()].into_iter().try_fold(0usize,usize::checked_add).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original window constructor copy overflow"))?;
        let retained_capacity_bytes=frame.checked_add(size_of::<O::Mutation>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original window constructor capacity overflow"))?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_capacity_bytes<retained_capacity_bytes||grant.maximum_depth==0{return Ok(None);}
        let mut pending=Vec::new();pending.try_reserve_exact(1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original window typed input backing allocation failed"))?;
        let WindowConfigMutation{window_id,window_kind_id:_,mutation}=mutations.pop().expect("quoted original window input remains");
        let typed=mutation.into_any().downcast::<FactoryBoxedValue<O::Mutation>>().expect("quoted original native mutation carrier remains");
        let(original,ingress)=typed.take_for_publication();pending.push(original);
        let publication=TypedWindowConfigPublication::<O>{window_id:ControlledRetirement::new(window_id).expect("native window address supports controlled retirement"),operation,generation:authority.generation,revision:authority.revision,actor:actor.take(),pending:std::mem::ManuallyDrop::new(pending),transaction:None,publication:std::mem::ManuallyDrop::new(None),ingress:ControlledRetirement::new(ingress).map_err(|(error,_)|error).expect("native carrier has genuine controlled facets"),cancelled:None,closing:false};
        Ok(Some((Box::new(publication),RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes,released_bytes:0})))
    }

    fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, body:usize, grant: store::os_store::one_item::ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault> {
        let publication = publication
            .as_any_mut()
            .downcast_mut::<TypedWindowConfigPublication<O>>()
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.publication-type"), "window config publication did not match its registered window owner"))?;
        if publication.closing{return Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Blocked);}
        let address=publication.window_id.original().ok_or_else(||Fault::from("original window publication lost its captured address"))?;
        let partition=self.partitions.get_mut(address).ok_or_else(||Fault::from("original window publication lost its exact partition"))?;
        if publication.publication.is_none(){
            return match partition.store.admit_member_apply_batch(publication.operation,publication.generation,publication.revision,&mut publication.actor,&mut publication.pending,&mut publication.transaction,grant.retained_grant()).map_err(|error|error.into_fault())?{
                Some((owner,progress))=>{*publication.publication=Some(owner);Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(Default::default(),progress))},
                None=>Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Blocked),
            };
        }
        let result=partition.store.advance_apply_batch(publication.publication.as_mut().expect("admitted original Store publication remains"),body,grant,receiving).map_err(|error|error.into_fault());
        if publication.publication.as_ref().is_some_and(|owner|owner.phase()==store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Closing){publication.closing=true;}
        result
    }

    fn refresh_demands(&self,authority:&WindowConfigAuthority,_body:usize)->Result<RetirementDemand,ValueError>{crate::window_mutation::refresh::refresh_demands(&authority.pending_snapshot,&authority.retired_snapshots)}
    fn refresh(&mut self,authority:&mut WindowConfigAuthority,grant:RetainedCloneGrant)->Result<crate::window_mutation::WindowAuthorityRefreshStep,Fault>{
        let demand=self.refresh_demands(authority,grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
        if grant.maximum_depth<demand.depth{return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("window-config.depth"),"window config refresh exceeds its admitted depth"));}
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(crate::window_mutation::WindowAuthorityRefreshStep::Pending(Default::default()));}
        if authority.pending_snapshot.is_none(){
            let partition=self.partitions.get(&authority.window_id).ok_or_else(||Fault::new(FaultOrigin::Framework,FaultCode::new("window-config.partition"),"window config authority lost its exact partition"))?;
            let snapshot=partition.store.snapshot_read().map_err(|error|error.into_fault())?.into_erased();
            authority.pending_snapshot=Some(WindowConfigSnapshot{authority:partition.authority,window_id:semio_framework_value::SharedUtf8::default(),window_kind_id:O::WINDOW_KIND_ID,generation:partition.store.generation(),revision:partition.store.content_revision_now(),snapshot});
            return Ok(crate::window_mutation::WindowAuthorityRefreshStep::Pending(RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        let step=crate::window_mutation::refresh::refresh_transfer(&mut authority.snapshot,&mut authority.pending_snapshot,&mut authority.retired_snapshots,grant).map_err(ValueError::into_fault)?;
        if matches!(step,crate::window_mutation::WindowAuthorityRefreshStep::Ready(_)){authority.generation=authority.snapshot.generation;authority.revision=authority.snapshot.revision;}
        Ok(step)
    }
    fn packs<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<WindowConfigPack>, Fault>> + 'a>> {
        Box::pin(async move {
            let mut packs = Vec::with_capacity(self.partitions.len());
            for (window_id, partition) in self.partitions.iter() {
                let files = store::os_store::text_format::print_document_pack(partition.store.envelope()).await.map_err(|error| error.into_fault())?;
                packs.push(WindowConfigPack { window_id: window_id.to_string(), window_kind_id: O::WINDOW_KIND_ID.to_string(), source:crate::WindowConfigPackSource::Pair(files) });
            }
            Ok(packs)
        })
    }

    fn commit_retained_load(&mut self, registry_lifetime: u64, load: &mut dyn retained::ErasedWindowConfigPackLoad,grant:RetainedCloneGrant) -> Result<WindowConfigPackLoadStep, WindowConfigPackLoadDiagnostic> {
        retained::commit_typed_window_config_pack_load::<O>(registry_lifetime, &mut self.partitions, load,grant)
    }

    fn snapshot(&self, window_id: &str) -> Option<WindowConfigSnapshot> {
        let partition = self.partitions.get(window_id)?;
        Some(WindowConfigSnapshot { authority: partition.authority, window_id: partition.address.as_ref().and_then(ControlledRetirement::original).expect("original Window address remains registered").clone(), window_kind_id: O::WINDOW_KIND_ID, generation: partition.store.generation(), revision: partition.store.content_revision_now(), snapshot: partition.store.snapshot_read().ok()?.into_erased() })
    }

    fn preview_input_demands(&self,window:&str,epoch:u64,body:usize)->Result<RetirementDemand,ValueError>{let p=self.partitions.get(window).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window preview partition is absent"))?;if !p.store.owned_disposer_installed(){return p.catalog_demands(body);}if let Some(owner)=p.semantic_preview.as_ref(){if p.semantic_epoch!=epoch||owner.work_pending(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Window preview waits for its previous source to close"));}return Ok(WindowSemanticPreview::<O>::input_demand());}let mut d=WindowSemanticPreview::<O>::birth_demand();d.copy_bytes+=size_of::<u64>();Ok(d)}
    fn preview_input(&mut self,input:&semio_framework_value::retained_clone::RetainedOwnedProjection<WindowConfigMutation>,epoch:u64,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{let original=input.borrow()?;let address=original.get().window_id();let demand=self.preview_input_demands(address,epoch,grant.maximum_copy_bytes)?;if !registry::fits(grant,demand){return Ok(Some(Default::default()));}let p=self.partitions.get_mut(address).unwrap();if !p.store.owned_disposer_installed(){return p.catalog_step(grant).map(|step|Some(step.progress()));}if p.semantic_preview.is_none(){let overhead=size_of::<u64>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant};let Some((owner,mut receipt))=WindowSemanticPreview::begin(&p.store,child)?else{return Ok(Some(Default::default()));};p.semantic_preview=Some(owner);p.semantic_epoch=epoch;receipt.copied_bytes+=overhead;return Ok(Some(receipt));}p.semantic_preview.as_mut().unwrap().admit_input(input,grant)}
    fn preview_leaf_demands(&self,window:&str,epoch:u64,body:usize)->Result<RetirementDemand,ValueError>{let partition=self.partitions.get(window).filter(|partition|partition.semantic_epoch==epoch).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Window preview leaf changed its partition epoch"))?;partition.semantic_preview.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window preview leaf source absent"))?.demand(&partition.store,body)}
    fn preview_leaf_step(&mut self,window:&str,epoch:u64,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(bool,RetainedCloneProgress),ValueError>{let partition=self.partitions.get_mut(window).filter(|partition|partition.semantic_epoch==epoch).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Window preview leaf changed its partition epoch"))?;let owner=partition.semantic_preview.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window preview leaf source absent"))?;if owner.leaf_ready(){return Ok((true,Default::default()))}let step=owner.advance(&partition.store,body,grant,receiving)?;Ok((owner.leaf_ready(),step.progress()))}
    fn preview_leaf_ready(&self,window:&str,epoch:u64)->bool{self.partitions.get(window).is_some_and(|p|p.semantic_epoch==epoch&&p.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::leaf_ready))}
    fn preview_input_held(&self,window:&str,epoch:u64)->bool{self.partitions.get(window).is_some_and(|p|p.semantic_epoch==epoch&&p.semantic_preview.as_ref().is_some_and(WindowSemanticPreview::work_pending))}
    fn preview_partition_count(&self)->usize{self.partitions.len()}
    fn preview_epoch_close_demands(&self,index:usize,epoch:u64,published:bool,body:usize)->Result<RetirementDemand,ValueError>{self.partitions.get_index(index).map_or(Ok(Default::default()),|partition|partition.epoch_close_demands(epoch,published,body))}
    fn preview_epoch_close_step(&mut self,index:usize,epoch:u64,published:bool,body:usize,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<(bool,RetainedCloneProgress),ValueError>{self.partitions.get_index_mut(index).map_or(Ok((true,Default::default())),|partition|partition.epoch_close_step(epoch,published,body,grant,control))}
    fn preview_partition_demands(&self,index:usize,epoch:u64,closing:bool,body:usize)->Result<RetirementDemand,ValueError>{let Some(p)=self.partitions.get_index(index)else{return Ok(Default::default());};if closing{p.preview_cancel_demand(body,false)}else{p.preview_finish_demands(epoch,body)}}
    fn preview_partition_step(&mut self,index:usize,epoch:u64,closing:bool,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(bool,RetainedCloneProgress),ValueError>{let Some(p)=self.partitions.get_index_mut(index)else{return Ok((true,Default::default()));};if closing{let pending=p.semantic_preview.is_some()||p.finish_preview_base.is_some()||p.finish_preview_read.is_some()||p.preview_retirement_pending();if !pending{return Ok((true,Default::default()));}let step=p.preview_cancel_step(grant,false)?;let pending=p.semantic_preview.is_some()||p.finish_preview_base.is_some()||p.finish_preview_read.is_some()||p.preview_retirement_pending();return Ok((!pending,step.progress()));}p.preview_finish_advance(epoch,body,grant,receiving)}
    fn preview_or<'a>(&'a self,committed:&'a WindowConfigSnapshot,epoch:u64)->&'a WindowConfigSnapshot{self.partitions.get(committed.window_id()).and_then(|p|if p.prepared_epoch==epoch{p.prepared_preview.as_ref()}else if p.published_epoch==epoch{p.published_preview.as_ref()}else{None}).filter(|preview|preview.generation()==committed.generation()).unwrap_or(committed)}
    fn preview_partition_index(&self,window:&str)->Option<usize>{self.partitions.iter().position(|(key,_)|key.as_str()==window)}
    fn preview_generation(&self,window:&str)->Option<u64>{self.partitions.get(window)?.published_preview.as_ref().map(WindowConfigSnapshot::generation)}
    fn preview_available(&self,window_id:&str)->bool{self.partitions.get(window_id).is_some_and(|partition|!partition.preview_retirement_pending())}
    fn can_retire_preview(&self,preview:&WindowConfigSnapshot)->bool{preview.window_kind_id==O::WINDOW_KIND_ID&&preview.snapshot.get::<O::State>().is_some()&&self.partitions.get(&preview.window_id).is_some_and(|partition|partition.pending_preview.is_none()&&partition.pending_preview_projection.is_none()&&partition.pending_preview_projection_read.is_none())}
    fn retire_preview(&mut self,preview:&mut Option<WindowConfigSnapshot>)->bool{
        let Some(original)=preview.as_ref()else{return true;};
        if !self.can_retire_preview(original){return false;}
        let partition=self.partitions.get_mut(&original.window_id).expect("checked original window partition remains");partition.pending_preview=preview.take();true
    }
    fn preview_retirement_pending(&self)->bool{self.partitions.values().any(WindowConfigPartition::preview_retirement_pending)}
    fn preview_retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.partitions.values().find(|partition|partition.preview_retirement_pending()).map_or(Ok(Default::default()),|partition|partition.preview_retirement_demand(body))}
    fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}self.partitions.values_mut().find(|partition|partition.preview_retirement_pending()).map_or(Ok(RetainedCloneStep::Complete(Default::default())),|partition|partition.preview_retirement_step(grant,None))}

    fn pointer_values(&self, pointers: &[String]) -> Vec<(String, Vec<Option<semio_framework_value::DslValue>>)> {
        self.partitions
            .iter()
            .map(|(window_id, partition)| {
                let document = semio_framework_value::ToValue::to_value(partition.store.snapshot_owner().as_ref());
                (window_id.to_string(), pointers.iter().map(|pointer| semio_framework_tool_run::tool_run_pointer_value(&document, pointer).cloned()).collect())
            })
            .collect()
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(demand)=self.initialization_close_demands(body)?{return Ok(demand);}
        if !self.direct_ingress.terminal_is_empty() { return self.direct_ingress_demands(body); }
        if let Some(owner) = self.partition_address_retirement.as_ref() { return controlled_owner_demands(owner, body); }
        if let Some(partition)=self.partitions.get_index(self.partition_close_cursor){return partition.original_close_demands(body);}
        if !self.partitions.is_empty() { return self.partitions.pop_demand(); }
        if !self.partitions.terminal_is_empty() { return self.partitions.backing_demand(); }
        window_actor_demands(&self.actor, &self.actor_retirement, body)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if self.initialization_pending(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Window initialization retirement requires its original mounted context").into_fault());}

        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
        if !registry::fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if !self.direct_ingress.terminal_is_empty() { return self.direct_ingress.step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
        if let Some(owner) = self.partition_address_retirement.as_mut() {
            let step = owner.step(grant).map_err(ValueError::into_fault)?;
            if owner.terminal_is_empty() { self.partition_address_retirement = None; }
            return Ok(PluginLifecycleStep::retained(step, false));
        }
        if let Some(partition)=self.partitions.get_index_mut(self.partition_close_cursor){if !partition.original_terminal(){return partition.original_close_step(grant);}self.partition_close_cursor+=1;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if !self.partitions.is_empty() {
            let Some(((address, partition), progress)) = self.partitions.pop_original(grant).map_err(ValueError::into_fault)? else { return Ok(PluginLifecycleStep::Progress(Default::default())); };
            assert!(partition.disposer.is_none() && !partition.preview_retirement_pending() && !partition.returned_read_pending());
            drop(partition);
            self.partition_address_retirement = Some(ControlledRetirement::new(address).map_err(|(error, _)| error).expect("original address has controlled retirement"));
            return Ok(PluginLifecycleStep::Progress(progress));
        }
        if !self.partitions.terminal_is_empty() { return self.partitions.close_backing_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
        close_window_actor(&mut self.actor, &mut self.actor_retirement, grant)
    }

    fn direct_ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(self.direct_ingress.next_demand(body)?)}
    fn close_direct_ingress(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.direct_ingress.step(grant)}
    fn direct_ingress_terminal_is_empty(&self)->bool{self.direct_ingress.terminal_is_empty()}
    fn terminal_is_empty(&self) -> bool {
        !self.initialization_pending() && self.partitions.terminal_is_empty() && self.partition_address_retirement.is_none() && self.direct_ingress.terminal_is_empty() && self.actor.is_none() && self.actor_retirement.is_none()
    }

    /// 🧹️ Retires the displaced owners a LIVE partition accumulates — every coalesced amend (a
    /// playback tick, a gumball flag) displaces the previous snapshot, edit id and envelope into the
    /// partition store's fixed 1 024-slot retirement queue, and only a maintenance pump gives those
    /// slots back. One partition per step, the first that still owes work.
    fn maintenance_retirements_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.partitions.values().find(|partition| partition.returned_read_pending() || !partition.store.maintenance_retirements_terminal_is_empty()).map_or(Ok(Default::default()), |partition| if partition.returned_read_pending() { partition.returned_read_demands(body) } else { partition.store.maintenance_retirements_demands(body) })
    }
    fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let Some(partition) = self.partitions.values_mut().find(|partition| partition.returned_read_pending() || !partition.store.maintenance_retirements_terminal_is_empty()) else { return Ok(RetainedCloneStep::Complete(Default::default())) };
        if partition.returned_read_pending() { return partition.returned_read_step(grant).map_err(ValueError::into_fault); }
        partition.store.maintenance_retirements_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|message| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.maintenance-retirement"), message.into_message()))
    }

    fn maintenance_retirements_terminal_is_empty(&self) -> bool {
        self.partitions.values().all(|partition| !partition.returned_read_pending() && partition.store.maintenance_retirements_terminal_is_empty())
    }

    fn maintenance_retirements_under_pressure(&self) -> bool {
        self.partitions.values().any(|partition| partition.store.maintenance_retirements_under_pressure())
    }
}

#[path="🌱️initialization/🦀️.rs"]
mod initialization;
pub use initialization::{WindowConfigInitialization,WindowConfigInitializationOwners,WindowConfigPartitionBirth,begin_window_config_initialization};

fn window_construction(demand:RetirementDemand)->semio_framework_job::OriginalConstructionDemand{semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth}}

#[path="🔒️epoch/🦀️.rs"]
mod original_epoch;
