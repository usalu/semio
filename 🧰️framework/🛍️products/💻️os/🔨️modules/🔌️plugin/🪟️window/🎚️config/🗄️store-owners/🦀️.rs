//! 🗄️ Store owners of the window config: the store and preview owner types, the partition, the erased store owner trait, the direct ingress and the typed store owner. See [`super`].

use super::*;

type WindowConfigStore<O> = store::ConfigStore<<O as WindowConfigOwner>::State, <O as WindowConfigOwner>::Mutation>;

type WindowPreviewOwners = (Option<WindowConfigSnapshot>, Option<Vec<store::os_store::snapshot_read::ErasedSnapshotRead>>, Option<store::os_store::snapshot_read::ErasedSnapshotRead>);

pub(super) struct WindowConfigPartition<O: WindowConfigOwner> {
    pub(super) store: WindowConfigStore<O>,
    pub(super) disposer: Option<Box<dyn ArtifactOwnedDisposer<WindowConfigStore<O>>>>,
    pub(super) pending_preview: Option<WindowConfigSnapshot>,
    pub(super) pending_preview_reads: Option<Vec<store::os_store::snapshot_read::ErasedSnapshotRead>>,
    pub(super) pending_preview_projection: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<O::State>>,
    pub(super) pending_preview_projection_read: Option<store::os_store::snapshot_read::ErasedSnapshotRead>,
    pub(super) preview_retirement: Option<ControlledRetirement<WindowPreviewOwners>>,
    pub(super) returned_read_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
}

impl<O:WindowConfigOwner> WindowConfigPartition<O>{
    pub(super) fn returned_read_pending(&self)->bool {self.returned_read_retirement.is_some()||self.store.returned_snapshot_read_retirement_demand().map_or(true,|demand|demand!=RetirementDemand::default())}
    pub(super) fn returned_read_demands(&self,body:usize)->Result<RetirementDemand,ValueError> {match self.returned_read_retirement.as_ref(){Some(owner)=>store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body),None=>self.store.returned_snapshot_read_retirement_demand()}}
    pub(super) fn returned_read_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.returned_read_retirement.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.returned_read_retirement,grant);}
        let(owner,progress)=self.store.take_returned_snapshot_read_retirement(grant)?;
        self.returned_read_retirement=owner;
        Ok(RetainedCloneStep::Progress(progress))
    }
    pub(super) fn preview_retirement_pending(&self)->bool{self.pending_preview.is_some()||self.pending_preview_reads.is_some()||self.pending_preview_projection.is_some()||self.pending_preview_projection_read.is_some()||self.preview_retirement.is_some()}
    pub(super) fn preview_retirement_demand(&self)->Result<RetirementDemand,ValueError>{
        if self.pending_preview_projection.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=self.preview_retirement.as_ref(){
            if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<WindowPreviewOwners>>>(),depth:1,..Default::default()});}
            let copy=owner.next_copy_byte_demand()?;let release=owner.next_release_byte_demand()?;
            return Ok(RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release})?,release_bytes:release,depth:owner.next_depth_demand()?});
        }
        Ok(RetirementDemand{copy_bytes:size_of::<WindowPreviewOwners>(),depth:1,..Default::default()})
    }
    pub(super) fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if !self.preview_retirement_pending(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        let demand=self.preview_retirement_demand()?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(projection)=self.pending_preview_projection.as_ref(){
            let read=match self.store.retained_derived_read(projection){Ok(read)=>read,Err(_)=>return Ok(RetainedCloneStep::Progress(Default::default()))};
            self.pending_preview_projection_read=Some(read);drop(self.pending_preview_projection.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        if let Some(owner)=self.preview_retirement.as_mut(){
            if owner.terminal_is_empty(){drop(self.preview_retirement.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
            return owner.step(grant);
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
    fn capture<'a>(&'a mut self, window_id: &'a str) -> Pin<Box<dyn Future<Output = Result<WindowConfigAuthority, Fault>> + 'a>>;
    fn dispatch<'a, 'b>(&'a mut self, actor: &'a str, mutation: WindowConfigMutation, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> Pin<Box<dyn Future<Output = Result<(), Fault>> + 'a>>;
    fn admit_begin(&mut self, operation: semio_framework_job::OperationId, actor: &mut Option<ControlledRetirement<semio_framework_value::SharedUtf8>>, mutations: &mut Vec<WindowConfigMutation>, authority: &WindowConfigAuthority, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedWindowConfigPublication>,RetainedCloneProgress)>,ValueError>;
    fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault>;
    fn refresh_demands(&self,authority:&WindowConfigAuthority,body:usize)->Result<RetirementDemand,ValueError>;
    fn refresh(&mut self,authority:&mut WindowConfigAuthority,grant:RetainedCloneGrant)->Result<crate::window_mutation::WindowAuthorityRefreshStep,Fault>;
    fn packs<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Vec<WindowConfigPack>, Fault>> + 'a>>;
    fn begin_retained_load(&mut self, registry_lifetime: u64, pack: WindowConfigPack) -> WindowConfigPackLoad;
    fn commit_retained_load(&mut self, registry_lifetime: u64, load: &mut dyn retained::ErasedWindowConfigPackLoad) -> Result<WindowConfigPackLoadStep, WindowConfigPackLoadDiagnostic>;
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
    fn preview(&mut self, window_id: &str, mutations: &[&WindowConfigMutation]) -> Option<WindowConfigSnapshot>;
    fn preview_available(&self,window_id:&str)->bool;
    fn can_retire_preview(&self,preview:&WindowConfigSnapshot)->bool;
    fn retire_preview(&mut self,preview:&mut Option<WindowConfigSnapshot>)->bool;
    fn preview_retirement_pending(&self)->bool;
    fn preview_retirement_demand(&self)->Result<RetirementDemand,ValueError>;
    fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
}

#[derive(semio_framework_value::RetireOwned)]
#[doc(hidden)]
pub struct DirectIngress<M:Send+'static>{#[doc(hidden)] pub mutations:Vec<WindowConfigMutation>,#[doc(hidden)] pub allocations:Vec<FactoryBoxedPublication<M>>,#[doc(hidden)] pub addresses:Vec<String>}

#[doc(hidden)]
pub struct TypedWindowConfigStoreOwner<O: WindowConfigOwner> {
    #[doc(hidden)]
    pub direct_ingress:ControlledRetirement<DirectIngress<O::Mutation>>,
    #[doc(hidden)]
    pub partitions: WindowRegistry<String, WindowConfigPartition<O>>,
    #[doc(hidden)]
    pub partition_close_cursor: usize,
    #[doc(hidden)]
    pub partition_address_retirement: Option<ControlledRetirement<String>>,
    #[doc(hidden)]
    pub actor: Option<protocol::ActorId>,
    #[doc(hidden)]
    pub actor_retirement: Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
}

impl<O: WindowConfigOwner> TypedWindowConfigStoreOwner<O> {
    #[doc(hidden)]
    pub async fn partition(&mut self, window_id: &str) -> Result<&mut WindowConfigPartition<O>, Fault> {
        if !self.partitions.contains_key(window_id) {
            let id = format!("window-config:{}:{window_id}", O::WINDOW_KIND_ID);
            let envelope = store::create_config_envelope::<O::State, O::Mutation>(O::SCHEMA, &id, O::State::default(), None).await;
            let config = store::ConfigStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, self.actor.as_ref().expect("live window owner retains its opened actor").clone()).await.map_err(|error| error.into_fault())?;
            self.partitions.insert(window_id.to_string(), WindowConfigPartition { store: config, disposer: Some(O::build_store_disposer()), pending_preview:None,pending_preview_reads:None,pending_preview_projection:None,pending_preview_projection_read:None,preview_retirement:None,returned_read_retirement:None });
        }
        let partition = self.partitions.get_mut(window_id).expect("initialized window config partition remains owned");
        if !partition.store.owned_disposer_installed() {
            store::install_unscheduled_catalog(&mut partition.store, O::build_store_owners()).map_err(|error| error.into_fault())?;
        }
        Ok(partition)
    }
}

impl<O: WindowConfigOwner> ErasedWindowConfigStoreOwner for TypedWindowConfigStoreOwner<O> {
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
                    window_id: window_id.to_string(),
                    window_kind_id: O::WINDOW_KIND_ID,
                    generation: partition.store.generation(),
                    revision: partition.store.content_revision_now(),
                    snapshot: partition.store.snapshot_read().map_err(|error|error.into_fault())?.into_erased(),
                },
            })
        })
    }

    fn dispatch<'a, 'b>(&'a mut self,actor:&'a str,mutation:WindowConfigMutation,identity:&'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>)->Pin<Box<dyn Future<Output=Result<(),Fault>>+'a>>{
        Box::pin(async move{
            if !mutation.mutation.as_any().is::<O::Mutation>(){self.direct_ingress.original_mut().expect("live direct ingress remains original").mutations.push(mutation);return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("window-config.address"),"window config mutation owner differs from its registered window"));}
            if self.actor.as_ref().is_none_or(|opened|opened.0.as_str()!=actor){self.direct_ingress.original_mut().expect("live direct ingress remains original").mutations.push(mutation);return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("window-config.actor"),"window config mutation actor differs from its opened session"));}
            if let Err(error)=self.partition(&mutation.window_id).await{self.direct_ingress.original_mut().expect("live direct ingress remains original").mutations.push(mutation);return Err(error);}
            let window_id=mutation.window_id;let typed=mutation.mutation.into_any().downcast::<FactoryBoxedValue<O::Mutation>>().expect("registered mutation retains its original typed carrier");let(value,ingress)=typed.take_for_publication();self.direct_ingress.original_mut().expect("live direct ingress remains original").allocations.push(ingress);let command=store::os_store::store_schema::ArtifactCommand::Apply{mutations:vec![value],transaction:None};self.partitions.get_mut(&window_id).expect("original window partition was admitted").store.dispatch(command,identity).await.map_err(|error|error.into_fault())?;Ok(())
        })
    }
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

    fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault> {
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
        let result=partition.store.advance_apply_batch(publication.publication.as_mut().expect("admitted original Store publication remains"),grant).map_err(|error|error.into_fault());
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
            authority.pending_snapshot=Some(WindowConfigSnapshot{window_id:String::new(),window_kind_id:O::WINDOW_KIND_ID,generation:partition.store.generation(),revision:partition.store.content_revision_now(),snapshot});
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
                packs.push(WindowConfigPack { window_id: window_id.clone(), window_kind_id: O::WINDOW_KIND_ID.to_string(), files });
            }
            Ok(packs)
        })
    }

    fn begin_retained_load(&mut self, registry_lifetime: u64, pack: WindowConfigPack) -> WindowConfigPackLoad {
        let partition_generation = self.partitions.get(&pack.window_id).map(|partition| partition.store.generation());
        retained::begin_typed_window_config_pack_load::<O>(registry_lifetime, partition_generation, pack, self.actor.as_ref().expect("live window owner retains its opened actor").clone())
    }

    fn commit_retained_load(&mut self, registry_lifetime: u64, load: &mut dyn retained::ErasedWindowConfigPackLoad) -> Result<WindowConfigPackLoadStep, WindowConfigPackLoadDiagnostic> {
        retained::commit_typed_window_config_pack_load::<O>(registry_lifetime, &mut self.partitions, load)
    }

    fn snapshot(&self, window_id: &str) -> Option<WindowConfigSnapshot> {
        let partition = self.partitions.get(window_id)?;
        Some(WindowConfigSnapshot { window_id: window_id.to_string(), window_kind_id: O::WINDOW_KIND_ID, generation: partition.store.generation(), revision: partition.store.content_revision_now(), snapshot: partition.store.snapshot_read().ok()?.into_erased() })
    }

    fn preview(&mut self, window_id: &str, mutations: &[&WindowConfigMutation]) -> Option<WindowConfigSnapshot> {
        let partition=self.partitions.get_mut(window_id)?;
        if partition.preview_retirement_pending(){return None;}
        let mut projection=partition.store.derived_snapshot_head();
        let mut reads=Vec::with_capacity(mutations.len());
        let mut current=None;
        for mutation in mutations.iter().filter_map(|mutation|mutation.mutation.as_any().downcast_ref::<O::Mutation>()){
            let next=match partition.store.derive_provisional_snapshot(&projection,mutation){Ok(Some(next))=>next,_=>continue};
            let read=match partition.store.retained_derived_read(&next){
                Ok(read)=>read,
                Err(_)=>{partition.pending_preview_projection=Some(next);partition.pending_preview_reads=Some(reads);partition.pending_preview_projection_read=current;return None;}
            };
            if let Some(previous)=current.replace(read){reads.push(previous);}
            projection=next;
        }
        partition.pending_preview_reads=Some(reads);
        let snapshot=current?;
        Some(WindowConfigSnapshot{window_id:window_id.to_string(),window_kind_id:O::WINDOW_KIND_ID,generation:partition.store.generation(),revision:partition.store.content_revision_now(),snapshot})
    }
    fn preview_available(&self,window_id:&str)->bool{self.partitions.get(window_id).is_some_and(|partition|!partition.preview_retirement_pending())}
    fn can_retire_preview(&self,preview:&WindowConfigSnapshot)->bool{preview.window_kind_id==O::WINDOW_KIND_ID&&preview.snapshot.get::<O::State>().is_some()&&self.partitions.get(&preview.window_id).is_some_and(|partition|partition.pending_preview.is_none()&&partition.pending_preview_projection.is_none()&&partition.pending_preview_projection_read.is_none())}
    fn retire_preview(&mut self,preview:&mut Option<WindowConfigSnapshot>)->bool{
        let Some(original)=preview.as_ref()else{return true;};
        if !self.can_retire_preview(original){return false;}
        let partition=self.partitions.get_mut(&original.window_id).expect("checked original window partition remains");partition.pending_preview=preview.take();true
    }
    fn preview_retirement_pending(&self)->bool{self.partitions.values().any(WindowConfigPartition::preview_retirement_pending)}
    fn preview_retirement_demand(&self)->Result<RetirementDemand,ValueError>{self.partitions.values().find(|partition|partition.preview_retirement_pending()).map_or(Ok(Default::default()),WindowConfigPartition::preview_retirement_demand)}
    fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}self.partitions.values_mut().find(|partition|partition.preview_retirement_pending()).map_or(Ok(RetainedCloneStep::Complete(Default::default())),|partition|partition.preview_retirement_step(grant))}

    fn pointer_values(&self, pointers: &[String]) -> Vec<(String, Vec<Option<semio_framework_value::DslValue>>)> {
        self.partitions
            .iter()
            .map(|(window_id, partition)| {
                let document = semio_framework_value::ToValue::to_value(partition.store.snapshot_owner().as_ref());
                (window_id.clone(), pointers.iter().map(|pointer| semio_framework_tool_run::tool_run_pointer_value(&document, pointer).cloned()).collect())
            })
            .collect()
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.direct_ingress.terminal_is_empty() { return self.direct_ingress_demands(body); }
        if let Some(owner) = self.partition_address_retirement.as_ref() { return controlled_owner_demands(owner, body); }
        if let Some(partition) = self.partitions.get_index(self.partition_close_cursor) {
            if partition.preview_retirement_pending() { return partition.preview_retirement_demand(); }
            if partition.returned_read_pending() { return partition.returned_read_demands(body); }
            return partition.disposer.as_ref().map_or(Ok(RetirementDemand { copy_bytes: size_of::<usize>(), depth: 1, ..Default::default() }), |disposer| {
                if disposer.terminal_is_empty(&partition.store) { Ok(RetirementDemand { release_bytes: size_of_val(disposer.as_ref()), depth: 1, ..Default::default() }) }
                else { disposer.retirement_demands(&partition.store, body) }
            });
        }
        if !self.partitions.is_empty() { return self.partitions.pop_demand(); }
        if !self.partitions.terminal_is_empty() { return self.partitions.backing_demand(); }
        window_actor_demands(&self.actor, &self.actor_retirement, body)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
        if !registry::fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if !self.direct_ingress.terminal_is_empty() { return self.direct_ingress.step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
        if let Some(owner) = self.partition_address_retirement.as_mut() {
            let step = owner.step(grant).map_err(ValueError::into_fault)?;
            if owner.terminal_is_empty() { self.partition_address_retirement = None; }
            return Ok(PluginLifecycleStep::retained(step, false));
        }
        if let Some(partition) = self.partitions.get_index_mut(self.partition_close_cursor) {
            if partition.preview_retirement_pending() { return partition.preview_retirement_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
            if partition.returned_read_pending() { return partition.returned_read_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
            if let Some(disposer) = partition.disposer.as_mut() {
                if disposer.terminal_is_empty(&partition.store) {
                    drop(partition.disposer.take());
                    return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
                }
                let step = disposer.close_step(&mut partition.store, grant)?;
                return Ok(match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other });
            }
            self.partition_close_cursor += 1;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
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

    fn direct_ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:self.direct_ingress.next_copy_byte_demand()?,capacity_bytes:self.direct_ingress.next_capacity_byte_demand(body)?,release_bytes:self.direct_ingress.next_release_byte_demand()?,depth:self.direct_ingress.next_depth_demand()?})}
    fn close_direct_ingress(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.direct_ingress.step(grant)}
    fn direct_ingress_terminal_is_empty(&self)->bool{self.direct_ingress.terminal_is_empty()}
    fn terminal_is_empty(&self) -> bool {
        self.partitions.terminal_is_empty() && self.partition_address_retirement.is_none() && self.direct_ingress.terminal_is_empty() && self.actor.is_none() && self.actor_retirement.is_none()
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
