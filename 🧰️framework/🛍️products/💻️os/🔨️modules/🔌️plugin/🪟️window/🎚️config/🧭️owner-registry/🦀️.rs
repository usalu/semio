//! 🧭️ Owner registry of the window config: the pack load turns, the owner registry, the controlled owner and window actor demands and the registry operations. See [`super`].

use super::*;
use crate::owner_contract_and_mutation::WindowConfigBorrow;

/// 🗂️ Runtime registry of heterogeneous persisted-local window config schemas.
pub struct WindowConfigOwnerRegistry {
    #[doc(hidden)]
    pub owners: WindowRegistry<&'static str, Box<dyn ErasedWindowConfigStoreOwner>>,
    owner_close_cursor: usize,
    pub(super) preview_visible_epoch:u64,
    pub(super) preview_active_epoch:Option<u64>,
    pub(super) lifetime: u64,
    #[doc(hidden)]
    pub actor: Option<protocol::ActorId>,
    #[doc(hidden)]
    pub actor_retirement: Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
}

pub(super) fn controlled_owner_demands<T: semio_framework_value::retirement::RetireOwned>(owner: &ControlledRetirement<T>, body: usize) -> Result<RetirementDemand, ValueError> {
    Ok(owner.next_demand(body)?)
}

pub(super) fn window_actor_demands(actor: &Option<protocol::ActorId>, retirement: &Option<ControlledRetirement<semio_framework_value::SharedUtf8>>, body: usize) -> Result<RetirementDemand, ValueError> {
    retirement.as_ref().map_or(Ok(RetirementDemand { copy_bytes: if actor.is_some() { size_of::<protocol::ActorId>() } else { 0 }, depth: usize::from(actor.is_some()), ..Default::default() }), |owner| controlled_owner_demands(owner, body))
}

pub(super) fn close_window_actor(actor: &mut Option<protocol::ActorId>, retirement: &mut Option<ControlledRetirement<semio_framework_value::SharedUtf8>>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
    if actor.is_none() && retirement.is_none() { return Ok(PluginLifecycleStep::Complete(Default::default())); }
    let demand = window_actor_demands(actor, retirement, grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
    if !registry::fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
    if let Some(actor) = actor.take() {
        *retirement = Some(ControlledRetirement::new(actor.0).map_err(|(error, _)| error).expect("original actor address has controlled retirement"));
        return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
    }
    let owner = retirement.as_mut().expect("original actor retirement remains");
    let step = owner.step(grant).map_err(ValueError::into_fault)?;
    if owner.terminal_is_empty() { *retirement = None; }
    Ok(PluginLifecycleStep::retained(step, retirement.is_none()))
}

impl WindowConfigOwnerRegistry {
    /// 🧪️ Opens a real declared fixture partition and returns its original issued authority.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub async fn fixture_partition_authority(&mut self, kind: &str, id: &str) -> Result<WindowConfigAuthorityIdentity, Fault> {
        let owner = self.owners.get_mut(kind).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.fixture-owner"), "fixture partition requires its declared original owner"))?;
        let original = owner.capture(id).await?;
        Ok(original.snapshot.authority)
    }

    pub fn new(actor: protocol::ActorId) -> Self {
        static NEXT_LIFETIME: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        Self { owners: WindowRegistry::new(), owner_close_cursor: 0,preview_visible_epoch:0,preview_active_epoch:None, lifetime: NEXT_LIFETIME.fetch_add(1, std::sync::atomic::Ordering::Relaxed), actor: Some(actor), actor_retirement: None }
    }

    pub fn local_actor_id(&self) -> &protocol::ActorId {
        self.actor.as_ref().expect("live window registry retains its opened actor")
    }

    pub fn register<O: WindowConfigOwner>(&mut self) -> Result<(), Fault> {
        if O::WINDOW_KIND_ID.is_empty() || O::SCHEMA.is_empty() || self.owners.contains_key(O::WINDOW_KIND_ID) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config owner identity is empty or already registered"));
        }
        self.owners.insert(O::WINDOW_KIND_ID,Box::new(TypedWindowConfigStoreOwner::<O>::new(self.local_actor_id().clone())));
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.owners.is_empty()
    }

    /// 📸️ The current config snapshot of window `window_id` of kind `window_kind_id`, without creating its partition.
    pub fn snapshot(&self, window_kind_id: &str, window_id: &str) -> Option<WindowConfigSnapshot> {
        self.owners.get(window_kind_id)?.snapshot(window_id)
    }

    /// 👁️ Lends committed window state without a new snapshot lease or address allocation.
    pub fn borrow<'a>(&'a self, window_kind_id: &str, window_id: &'a str) -> Option<WindowConfigBorrow<'a>> {
        self.owners.get(window_kind_id)?.borrow(window_id)
    }

    /// 👁️ Lends only the visible complete original epoch without issuing another snapshot read.
    pub fn borrow_preview<'a>(&'a self,kind:&str,window:&'a str)->Option<WindowConfigBorrow<'a>>{self.owners.get(kind)?.borrow_preview(window,self.preview_visible_epoch)}
    /// 🧬️ Retains the actual press projection for one native semantic preview leaf.
    /// 📏️ Quotes one original leaf without finishing or publishing its window prefix.
    pub fn preview_leaf_demands(&self,kind:&str,window:&str,epoch:u64,body:usize)->Result<RetirementDemand,ValueError>{self.owners.get(kind).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Window preview owner is absent"))?.preview_leaf_demands(window,epoch,body)}
    /// 🪜️ Advances only the issued original leaf while the complete candidate remains private.
    pub fn preview_leaf_step(&mut self,kind:&str,window:&str,epoch:u64,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<(bool,RetainedCloneProgress),ValueError>{self.owners.get_mut(kind).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Window preview owner is absent"))?.preview_leaf_step(window,epoch,body,grant,receiving)}
    /// ✅️ The complete leaf has returned its original input and semantic source aliases.
    pub fn preview_leaf_ready(&self,kind:&str,window:&str,epoch:u64)->bool{self.owners.get(kind).is_some_and(|owner|owner.preview_leaf_ready(window,epoch))}
    /// 🔗️ Identifies an original leaf still held by its actual partition.
    pub fn preview_input_held(&self,kind:&str,window:&str,epoch:u64)->bool{self.owners.get(kind).is_some_and(|owner|owner.preview_input_held(window,epoch))}
    pub fn preview_input(&mut self,input:&semio_framework_value::retained_clone::RetainedOwnedProjection<WindowConfigMutation>,epoch:u64,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{let original=input.borrow()?;let kind=original.get().window_kind_id();self.owners.get_mut(kind).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Window preview owner is absent"))?.preview_input(input,epoch,grant)}
    /// 📏️ Quotes the next actual original press admission.
    pub fn preview_input_demands(&self,kind:&str,window:&str,epoch:u64,body:usize)->Result<RetirementDemand,ValueError>{self.owners.get(kind).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Window preview owner is absent"))?.preview_input_demands(window,epoch,body)}
    /// 👁️ Borrows the current original published preview.
    pub fn preview_or<'a>(&'a self,committed:&'a WindowConfigSnapshot)->&'a WindowConfigSnapshot{self.owners.get(committed.window_kind_id()).map_or(committed,|owner|owner.preview_or(committed,self.preview_visible_epoch))}
    /// 🔢️ The generation of window `window_id`'s published provisional config.
    #[doc(hidden)]
    pub fn preview_generation(&self,window_kind_id:&str,window_id:&str)->Option<u64>{self.owners.get(window_kind_id)?.preview_generation(window_id)}
    /// 🗂️ Every live partition of every window kind, the frontier a retained preview pass walks.
    #[doc(hidden)]
    pub fn preview_partition_count(&self)->usize{self.owners.values().map(|owner|owner.preview_partition_count()).sum()}
    /// 🧭️ The position of window `window_id`'s partition in the pass, absent before its first capture.
    #[doc(hidden)]
    pub fn preview_partition_index(&self,window_kind_id:&str,window_id:&str)->Option<usize>{
        let mut before=0;
        for(kind,owner)in self.owners.iter(){if *kind==window_kind_id{return owner.preview_partition_index(window_id).map(|index|before+index);}before+=owner.preview_partition_count();}
        None
    }
    #[doc(hidden)]
    pub fn preview_partition_demands(&self,index:usize,epoch:u64,closing:bool,body:usize)->Result<RetirementDemand,ValueError>{
        let Some((owner,local))=self.preview_locate(index)else{return Ok(Default::default());};
        self.owners.get_index(owner).expect("located preview owner remains registered").preview_partition_demands(local,epoch,closing,body)
    }
    #[doc(hidden)]
    pub fn preview_retirement_pending(&self)->bool{self.owners.values().any(|owner|owner.preview_retirement_pending())}
    #[doc(hidden)]
    pub fn preview_retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.owners.values().find(|owner|owner.preview_retirement_pending()).map_or(Ok(Default::default()),|owner|owner.preview_retirement_demand(body))}
    #[doc(hidden)]
    pub fn preview_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}self.owners.values_mut().find(|owner|owner.preview_retirement_pending()).map_or(Ok(RetainedCloneStep::Complete(Default::default())),|owner|owner.preview_retirement_step(grant))}
    fn preview_owner(&self,window_kind_id:&str)->Result<&dyn ErasedWindowConfigStoreOwner,ValueError>{self.owners.get(window_kind_id).map(|owner|owner.as_ref()).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original window preview has no installed native owner"))}
    fn preview_owner_mut(&mut self,window_kind_id:&str)->Result<&mut Box<dyn ErasedWindowConfigStoreOwner>,ValueError>{self.owners.get_mut(window_kind_id).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original window preview has no installed native owner"))}
    fn preview_locate(&self,index:usize)->Option<(usize,usize)>{
        let mut rest=index;
        for(position,owner)in self.owners.values().enumerate(){let count=owner.preview_partition_count();if rest<count{return Some((position,rest));}rest-=count;}
        None
    }

    /// ⏯️ The values `pointers` (RFC 6901) name in every window config partition of `window_kind_id`, by window id
    /// in window id order — what a tool run's declared window config reads compare across publications.
    pub fn pointer_values(&self, window_kind_id: &str, pointers: &[String]) -> Vec<(String, Vec<Option<semio_framework_value::DslValue>>)> {
        self.owners.get(window_kind_id).map_or_else(Vec::new, |owner| owner.pointer_values(pointers))
    }

    /// 🪟️ The window whose config this call speaks for: the addressed instance, and for a PANEL
    /// projection — which `ViewModel::for_panel` deliberately strips of `window_id` — the shell's
    /// focused pane. A panel's controls are addressed at `focused_window_id` (that is the only carrier of
    /// "which window is the user looking at" a panel is given), so handing the panel no window config at
    /// all made every panel render a per-window option from `Default` while writing to the focused pane:
    /// the puzzle3d Settings panel displayed spacing 10.0 and bumped it to 10.5 against a pane whose rail
    /// stood at 12.5 (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B36). A surface reads exactly the state
    /// it writes.
    /// 🌱️ Pumps the exact focused Window's original native partition birth before immutable capture.
    pub fn ensure_for_view(&mut self,view_state:Option<&ViewModel>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let Some(view)=view_state else{return Ok(true)};let Some(id)=view.window_id.as_deref().or(view.focused_window_id.as_deref())else{return Ok(true)};
        let window=view.window_instances.iter().find(|window|window.id==id).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original target Window is absent from its ViewModel roster"))?;
        let Some(owner)=self.owners.get_mut(window.window_kind_id.as_str())else{return Ok(true)};owner.ensure_partition(id,cx)
    }

    #[doc(hidden)]
    pub async fn capture(&mut self, view_state: Option<&ViewModel>) -> Result<Option<WindowConfigAuthority>, Fault> {
        let Some(view_state) = view_state else { return Ok(None) };
        let Some(window_id) = view_state.window_id.as_deref().or(view_state.focused_window_id.as_deref()) else { return Ok(None) };
        let window = view_state
            .window_instances
            .iter()
            .find(|window| window.id == window_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.window-context"), "target window is absent from the exact ViewModel window instance roster"))?;
        let Some(owner) = self.owners.get_mut(window.window_kind_id.as_str()) else { return Ok(None) };
        if owner.borrow(window_id).is_none(){return Ok(None)}
        owner.capture(window_id).await.map(Some)
    }

    #[doc(hidden)]
    pub async fn dispatch(&mut self, authority: &WindowConfigAuthority, actor: &str, mutation: WindowConfigMutation, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        self.validate_address(authority, &mutation)?;
        self.owners.get_mut(authority.window_kind_id.as_str()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config emission has no registered concrete window owner"))?.dispatch(actor, mutation, identity).await
    }

    pub(crate) fn admit_catalog(&mut self,authority:&WindowConfigAuthority,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{self.owners.get_mut(&authority.window_kind_id).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Window catalog owner is absent"))?.admit_catalog(authority,grant)}
    #[doc(hidden)]
    pub fn admit_begin(
        &mut self, operation: semio_framework_job::OperationId,
        actor: &mut Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
        mutations: &mut Vec<WindowConfigMutation>, authority: &WindowConfigAuthority, grant: RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn ErasedWindowConfigPublication>,RetainedCloneProgress)>,ValueError> {
        let original=mutations.last().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original window publication input is absent"))?;
        if original.window_id!=authority.window_id||original.window_kind_id!=authority.window_kind_id{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original window publication address differs from its captured authority"));}
        let owner=self.owners.get_mut(authority.window_kind_id.as_str()).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original window publication has no installed native owner"))?;
        owner.admit_begin(operation,actor,mutations,authority,grant)
    }

    #[doc(hidden)]
    pub fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, body:usize, grant: store::os_store::one_item::ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault> {
        self.owners.get_mut(publication.window_kind_id()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config publication lost its registered concrete window owner"))?.advance(publication, body,grant,receiving)
    }

    #[doc(hidden)]
    pub fn refresh_demands(&self,authority:&WindowConfigAuthority,body:usize)->Result<RetirementDemand,ValueError>{self.owners.get(authority.window_kind_id.as_str()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original window config owner unavailable"))?.refresh_demands(authority,body)}
    #[doc(hidden)]
    pub fn refresh(&mut self,authority:&mut WindowConfigAuthority,grant:RetainedCloneGrant)->Result<crate::window_mutation::WindowAuthorityRefreshStep,Fault>{self.owners.get_mut(authority.window_kind_id.as_str()).ok_or_else(||Fault::new(FaultOrigin::Framework,FaultCode::new("window-config.owner"),"original window config owner unavailable"))?.refresh(authority,grant)}
    pub async fn packs(&self) -> Result<Vec<WindowConfigPack>, Fault> {
        let mut packs = Vec::new();
        for owner in self.owners.values() {
            packs.extend(owner.packs().await?);
        }
        Ok(packs)
    }

    /// 📥️ Admits the original exact-partition source through its actual native owner without changing live authority.
    pub fn admit_retained_load(&mut self,pack:&mut Option<WindowConfigPack>,cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<WindowConfigPackLoad>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let original=pack.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native Window load requires its original Pack source"))?;
        let owner=self.owners.get_mut(original.window_kind_id.as_str()).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native Window Pack has no registered concrete owner"))?;
        let actor=self.actor.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native Window registry actor is already closing"))?;
        owner.admit_retained_load(self.lifetime,pack,actor,cx,output)
    }

    /// 🧵️ Loans the actual mounted caller to the retained Window decoder and native replay initializer.
    pub fn advance_retained_load_context(&mut self,load:&mut WindowConfigPackLoad,cx:&mut semio_framework_job::StepContext<'_>)->Result<WindowConfigPackLoadStep,semio_framework_job::OriginalTurnFailure>{
        let Some(inner)=load.inner.as_mut()else{return Ok(WindowConfigPackLoadStep::Complete)};
        if inner.registry_lifetime()!=self.lifetime||!self.owners.contains_key(inner.window_kind_id()){return Ok(inner.reject_stale())}inner.advance_context(cx)
    }

    /// 🔐️ Publishes one private registry unit under the original caller's physical receiving scope.
    pub fn commit_retained_load_context(&mut self,load:&mut WindowConfigPackLoad,cx:&mut semio_framework_job::StepContext<'_>)->Result<WindowConfigPackLoadStep,semio_framework_job::OriginalTurnFailure>{
        let Some(inner)=load.inner.as_mut()else{return Ok(WindowConfigPackLoadStep::Complete)};if inner.registry_lifetime()!=self.lifetime{return Ok(inner.reject_stale())}
        let owner=self.owners.get_mut(inner.window_kind_id()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native Window publication lost its registered owner"))?;let demand=owner.retained_load_commit_demands(inner.as_ref())?;let grant=cx.retained_grant();let mut output=None;
        semio_framework_job::admit_original_construction(cx,&mut output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},|_|{let step=owner.commit_retained_load(self.lifetime,inner.as_mut(),grant).unwrap_or(WindowConfigPackLoadStep::Rejected(WindowConfigPackLoadDiagnostic::Stale));let receipt=match &step{WindowConfigPackLoadStep::OwnershipProgress(_,receipt)=>*receipt,_=>RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}};Ok((step,receipt))})?;
        Ok(output.unwrap_or_else(||WindowConfigPackLoadStep::Pending(inner.progress())))
    }

    /// ♻️ Rolls back private registry preparation before closing the original native load.
    pub fn close_retained_load_context(&mut self,load:&mut WindowConfigPackLoad,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
        if let Some(inner)=load.inner.as_mut(){if inner.insertion_pending(){let owner=self.owners.get_mut(inner.window_kind_id()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original pending Window insertion lost its registered owner"))?;let demand=owner.retained_load_insert_demands(inner.as_ref())?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window rollback demand absent"))?;let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.cancel_retained_load_insert(inner.as_mut(),grant).map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))))?;return Ok(PluginLifecycleStep::Progress(step.progress()))}}
        load.close_step_context(cx)
    }

    /// 📏️ The exact physical release the next [`Self::close_step`] owes: the open owner's first partition store, or its opened actor's retirement. Parked loads pay their own demand.
    #[doc(hidden)]
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.owners.get_index(self.owner_close_cursor) {
            if owner.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<usize>(), depth: 1, ..Default::default() }); }
            return owner.retirement_demands(body);
        }
        if let Some((_, owner)) = self.owners.last() {
            let mut demand = self.owners.pop_demand()?;
            demand.release_bytes = size_of_val(owner.as_ref());
            return Ok(demand);
        }
        if !self.owners.terminal_is_empty() { return self.owners.backing_demand(); }
        window_actor_demands(&self.actor, &self.actor_retirement, body)
    }

    /// ♻️ Retires parked loads first, each under its own exact release demand (paying back credits it was admitted), then owners.
    #[doc(hidden)]
    pub fn direct_ingress_terminal_is_empty(&self)->bool{self.owners.values().all(|owner|owner.direct_ingress_terminal_is_empty())}
    #[doc(hidden)]
    pub fn direct_ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.owners.values().find(|owner|!owner.direct_ingress_terminal_is_empty()).map_or(Ok(Default::default()),|owner|owner.direct_ingress_demands(body))}
    #[doc(hidden)]
    pub fn close_direct_ingress(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{match self.owners.values_mut().find(|owner|!owner.direct_ingress_terminal_is_empty()){Some(owner)=>owner.close_direct_ingress(grant).map(|step|RetainedCloneStep::Progress(step.progress())),None=>Ok(RetainedCloneStep::Complete(Default::default()))}}
    #[doc(hidden)]
    /// ♻️ Preserves original domain failures while closing a partially born Window partition.
    pub fn close_step_context(&mut self,grant:RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        if let Some(owner)=self.owners.get_index_mut(self.owner_close_cursor){if !owner.terminal_is_empty(){let demand=owner.retirement_demands(grant.maximum_copy_bytes)?;if !registry::fits(grant,demand){return Ok(PluginLifecycleStep::Progress(Default::default()))}return owner.close_step_context(grant,cx).map(|step|match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),other=>other});}}
        let d=self.retirement_demands(grant.maximum_copy_bytes)?;
        semio_framework_job::close_original_retained_turn(cx,d,|grant|self.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress().unwrap_or_default())).map_err(Into::into)).map(|step|PluginLifecycleStep::retained(RetainedCloneStep::Progress(step.progress()),self.terminal_is_empty())).map_err(Into::into)
    }
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,Fault>{
        if self.terminal_is_empty() { return Ok(PluginLifecycleStep::Complete(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
        if !registry::fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if let Some(owner) = self.owners.get_index_mut(self.owner_close_cursor) {
            if owner.terminal_is_empty() {
                self.owner_close_cursor += 1;
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
            }
            return owner.close_step(grant).map(|step| match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other });
        }
        if !self.owners.is_empty() {
            let Some(((kind, owner), mut progress)) = self.owners.pop_original(grant).map_err(ValueError::into_fault)? else { return Ok(PluginLifecycleStep::Progress(Default::default())); };
            assert!(owner.terminal_is_empty());
            drop((kind, owner));
            progress.released_bytes = demand.release_bytes;
            return Ok(PluginLifecycleStep::Progress(progress));
        }
        if !self.owners.terminal_is_empty() { return self.owners.close_backing_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_fault); }
        close_window_actor(&mut self.actor, &mut self.actor_retirement, grant)
    }

    #[doc(hidden)]
    pub fn terminal_is_empty(&self) -> bool {
        self.preview_active_epoch.is_none() && self.owners.terminal_is_empty() && self.actor.is_none() && self.actor_retirement.is_none()
    }

    /// 🧹️ One maintenance step over the live partitions' displaced-owner queues — the first owner
    /// that still owes retirements advances; `Complete` means every partition is drained.
    #[doc(hidden)]
    pub fn maintenance_retirements_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.owners.values().find(|owner| !owner.maintenance_retirements_terminal_is_empty()).map_or(Ok(Default::default()), |owner| owner.maintenance_retirements_demands(body))
    }
    #[doc(hidden)]
    pub fn maintenance_retirements_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let Some(owner) = self.owners.values_mut().find(|owner| !owner.maintenance_retirements_terminal_is_empty()) else { return Ok(RetainedCloneStep::Complete(Default::default())) };
        owner.maintenance_retirements_step(grant).map(|step| RetainedCloneStep::Progress(step.progress()))
    }
    #[doc(hidden)]
    pub fn maintenance_retirements_terminal_is_empty(&self) -> bool {
        self.owners.values().all(|owner| owner.maintenance_retirements_terminal_is_empty())
    }

    /// 🌡️ True while any live partition's displaced-owner queue sits at or above the store's pressure mark.
    #[doc(hidden)]
    pub fn maintenance_retirements_under_pressure(&self) -> bool {
        self.owners.values().any(|owner| owner.maintenance_retirements_under_pressure())
    }

    fn validate_address(&self, authority: &WindowConfigAuthority, mutation: &WindowConfigMutation) -> Result<(), Fault> {
        if mutation.window_id != authority.window_id || mutation.window_kind_id != authority.window_kind_id {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.address"), "window config emission does not match the operation's exact captured window authority"));
        }
        Ok(())
    }
}

#[path="🌱️registration/🦀️.rs"]
mod original_registration;
pub use original_registration::OriginalWindowOwnerRegistration;
