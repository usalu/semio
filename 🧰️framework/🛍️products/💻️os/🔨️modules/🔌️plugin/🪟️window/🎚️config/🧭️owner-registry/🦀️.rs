//! 🧭️ Owner registry of the window config: the pack load turns, the owner registry, the controlled owner and window actor demands and the registry operations. See [`super`].

use super::*;
use crate::owner_contract_and_mutation::WindowConfigBorrow;

/// ⏳️ Turns one registry-driven retained window config load may spend before it faults `window-config.load-bound`.
pub const WINDOW_CONFIG_PACK_LOAD_TURNS: usize = 1_048_576;

/// 🗂️ Runtime registry of heterogeneous persisted-local window config schemas.
pub struct WindowConfigOwnerRegistry {
    #[doc(hidden)]
    pub owners: WindowRegistry<&'static str, Box<dyn ErasedWindowConfigStoreOwner>>,
    owner_close_cursor: usize,
    #[doc(hidden)]
    pub retiring: semio_framework_value_persistent::list::PagedList<WindowConfigPackLoad, {usize::MAX}>,
    lifetime: u64,
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
        Self { owners: WindowRegistry::new(), owner_close_cursor: 0, retiring: Default::default(), lifetime: NEXT_LIFETIME.fetch_add(1, std::sync::atomic::Ordering::Relaxed), actor: Some(actor), actor_retirement: None }
    }

    pub fn local_actor_id(&self) -> &protocol::ActorId {
        self.actor.as_ref().expect("live window registry retains its opened actor")
    }

    pub fn register<O: WindowConfigOwner>(&mut self) -> Result<(), Fault> {
        if O::WINDOW_KIND_ID.is_empty() || O::SCHEMA.is_empty() || self.owners.contains_key(O::WINDOW_KIND_ID) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config owner identity is empty or already registered"));
        }
        self.owners.insert(O::WINDOW_KIND_ID, Box::new(TypedWindowConfigStoreOwner::<O> { direct_ingress:ControlledRetirement::new(DirectIngress{mutations:Vec::new(),allocations:Vec::new(),addresses:Vec::new()}).map_err(|(error,_)|error).expect("direct ingress owns genuine facets"),partitions: WindowRegistry::new(), partition_close_cursor: 0, partition_address_retirement: None, actor: Some(self.local_actor_id().clone()), actor_retirement: None }));
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

    /// 📏️ Quotes the next retained window press input of `window` under `epoch`.
    #[doc(hidden)]
    pub fn preview_input_demands(&self,window_kind_id:&str,window_id:&str,epoch:u64,body:usize)->Result<RetirementDemand,ValueError>{self.preview_owner(window_kind_id)?.preview_input_demands(window_id,epoch,body)}
    /// 🪞️ Moves one retained press operation toward window `window_id`'s provisional config of `epoch`; `Some` carries the progress of the catalog, birth or input step, `None` a refusal that waits for its owners.
    #[doc(hidden)]
    pub fn preview_input(&mut self,input:&semio_framework_value::retained_clone::RetainedOwnedProjection<WindowConfigMutation>,epoch:u64,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{let kind=input.borrow()?.get().window_kind_id;self.preview_owner_mut(kind)?.preview_input(input,epoch,grant)}
    /// ✅️ Whether the press operations admitted for `epoch` are completely applied to window `window_id`'s provisional config.
    #[doc(hidden)]
    pub fn preview_leaf_ready(&self,window_kind_id:&str,window_id:&str,epoch:u64)->bool{self.owners.get(window_kind_id).is_some_and(|owner|owner.preview_leaf_ready(window_id,epoch))}
    /// 🔒️ Whether window `window_id` still holds an admitted press operation of `epoch`.
    #[doc(hidden)]
    pub fn preview_input_held(&self,window_kind_id:&str,window_id:&str,epoch:u64)->bool{self.owners.get(window_kind_id).is_some_and(|owner|owner.preview_input_held(window_id,epoch))}
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
    /// ⏯️ Advances partition `index` of the pass: finishing publishes its provisional config of `epoch` and retires the previous one, closing cancels the unfinished source and keeps the published config. The flag answers whether the partition has nothing left for this pass.
    #[doc(hidden)]
    pub fn preview_partition_step(&mut self,index:usize,epoch:u64,closing:bool,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{
        let Some((owner,local))=self.preview_locate(index)else{return Ok((true,Default::default()));};
        let mut proceed=|_|true;let mut receiving=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut proceed);
        self.owners.get_index_mut(owner).expect("located preview owner remains registered").preview_partition_step(local,epoch,closing,grant,&mut receiving)
    }
    /// 🪞️ The window config a render seam reads for `committed`'s window: its published provisional config when it was folded on that very generation, else `committed`.
    #[doc(hidden)]
    pub fn preview_or<'a>(&'a self,committed:&'a WindowConfigSnapshot)->&'a WindowConfigSnapshot{self.owners.get(committed.window_kind_id).map_or(committed,|owner|owner.preview_or(committed))}
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
    pub fn advance(&mut self, publication: &mut dyn ErasedWindowConfigPublication, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item_publication::ArtifactStoreOneItemAdvance, Fault> {
        let body = grant.maximum_copy_bytes;
        let mut proceed = |_| true;
        let mut receiving = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut proceed);
        self.owners.get_mut(publication.window_kind_id()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config publication lost its registered concrete window owner"))?.advance(publication, body, grant, &mut receiving)
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

    pub async fn load(&mut self, pack: WindowConfigPack) -> Result<(), Fault> {
        self.load_within(pack, WINDOW_CONFIG_PACK_LOAD_TURNS).await
    }

    /// ⏳️ Drives one cold load under its explicit copy, capacity, release and depth ceilings for at most `turns` turns. Exhausting the bound
    /// is a typed `window-config.load-bound` fault, never `Ok`; a load that cannot retire within the bound is
    /// parked on the registry, whose bounded close retires it, so no load reaches `Drop` unretired.
    #[doc(hidden)]
    pub async fn load_within(&mut self, pack: WindowConfigPack, turns: usize) -> Result<(), Fault> {
        let mut load = self.begin_retained_load(pack)?;
        let mut rejected = None;
        let mut settled = false;
        for _ in 0..turns {
            let grant = load.cold_work_grant();
            if rejected.is_some() || matches!(load.phase(), WindowConfigPackLoadPhase::RetiringDisplacedStore) {
                match self.close_retained_load_step(&mut load, grant) {
                    Ok(PluginLifecycleStep::Complete(_)) if load.terminal_is_empty() => {
                        settled = true;
                        break;
                    }
                    Ok(_) => continue,
                    Err(fault) => return Err(self.park_retained_load(load, fault)),
                }
            }
            match self.advance_retained_load(&mut load, grant) {
                WindowConfigPackLoadStep::Pending(_)|WindowConfigPackLoadStep::OwnershipProgress(_,_) => {}
                WindowConfigPackLoadStep::Ready => {
                    if let WindowConfigPackLoadStep::Rejected(diagnostic) = self.commit_retained_load(&mut load) {
                        rejected = Some(diagnostic);
                    }
                }
                WindowConfigPackLoadStep::Rejected(diagnostic) => rejected = Some(diagnostic),
                WindowConfigPackLoadStep::Complete => {
                    settled = true;
                    break;
                }
            }
        }
        let fault = rejected.map_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.load-bound"), "retained window config load exhausted its declared turn bound before it settled"), Self::load_fault);
        if !settled {
            load.request_cancel();
            for _ in 0..turns {
                let grant = load.next_grant();
                match self.close_retained_load_step(&mut load, grant) {
                    Ok(PluginLifecycleStep::Complete(_)) if load.terminal_is_empty() => break,
                    Ok(_) => {}
                    Err(retirement) => return Err(self.park_retained_load(load, retirement)),
                }
            }
            return Err(if load.terminal_is_empty() { fault } else { self.park_retained_load(load, fault) });
        }
        match rejected {
            Some(_) => Err(fault),
            None => Ok(()),
        }
    }

    fn park_retained_load(&mut self, load: WindowConfigPackLoad, fault: Fault) -> Fault {
        self.retiring.push(load);
        fault
    }

    /// 📥️ Begins a retained exact-partition Pack load without changing live authority.
    pub fn begin_retained_load(&mut self, pack: WindowConfigPack) -> Result<WindowConfigPackLoad, Fault> {
        let kind = pack.window_kind_id.clone();
        self.owners
            .get_mut(kind.as_str())
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.owner"), "window config pack has no registered concrete window owner"))
            .map(|owner| owner.begin_retained_load(self.lifetime, pack))
    }

    /// 🧵️ Advances one bounded retained decode or hydration unit.
    pub fn advance_retained_load(&mut self, load: &mut WindowConfigPackLoad, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        let Some(inner) = load.inner.as_mut() else { return WindowConfigPackLoadStep::Complete; };
        if inner.registry_lifetime() != self.lifetime || !self.owners.contains_key(inner.window_kind_id()) {
            return inner.reject_stale();
        }
        inner.advance(grant)
    }

    /// 🔐️ Publishes a ready candidate against its original exact partition.
    pub fn commit_retained_load(&mut self,load:&mut WindowConfigPackLoad)->WindowConfigPackLoadStep{
        let Some(inner)=load.inner.as_mut()else{return WindowConfigPackLoadStep::Complete;};
        if inner.registry_lifetime()!=self.lifetime{return inner.reject_stale();}
        let Some(owner)=self.owners.get_mut(inner.window_kind_id())else{return inner.reject_stale();};
        match owner.commit_retained_load(self.lifetime,inner.as_mut()){Ok(step)=>step,Err(_)=>inner.reject_stale()}
    }
    /// ♻️ Retires a cancelled, rejected, committed, or displaced load candidate under an exact grant.
    pub fn close_retained_load_step(&mut self, load: &mut WindowConfigPackLoad, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        load.close_step(grant).map_err(|message| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.load-retirement"), message))
    }

    fn load_fault(diagnostic: WindowConfigPackLoadDiagnostic) -> Fault {
        let code = match diagnostic {
            WindowConfigPackLoadDiagnostic::EnvelopeIdentity => "window-config.pack-envelope",
            WindowConfigPackLoadDiagnostic::Pack => "window-config.pack",
            WindowConfigPackLoadDiagnostic::TypedState => "window-config.typed-state",
            WindowConfigPackLoadDiagnostic::History => "window-config.history",
            WindowConfigPackLoadDiagnostic::InnerIdentity => "window-config.inner-identity",
            WindowConfigPackLoadDiagnostic::Replay => "window-config.replay",
            WindowConfigPackLoadDiagnostic::Capacity => "window-config.capacity",
            WindowConfigPackLoadDiagnostic::Stale => "window-config.stale",
            WindowConfigPackLoadDiagnostic::Cancelled => "window-config.cancelled",
            WindowConfigPackLoadDiagnostic::Retirement => "window-config.retirement",
        };
        Fault::new(FaultOrigin::Framework, FaultCode::new(code), "retained exact window config Pack load was rejected")
    }

    /// 📏️ The exact physical release the next [`Self::close_step`] owes: the open owner's first partition store, or its opened actor's retirement. Parked loads pay their own demand.
    #[doc(hidden)]
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(load) = self.retiring.last() {
            if !load.terminal_is_empty() { return load.retirement_demands(body); }
            return Ok(RetirementDemand { copy_bytes: size_of::<WindowConfigPackLoad>(), depth: self.retiring.next_pop_depth_demand().map_err(|error| ValueError::literal(ValueRefusalKind::OwnershipLimit, error.reason))?, ..Default::default() });
        }
        if !self.retiring.terminal_is_empty() {
            return Ok(RetirementDemand { release_bytes: self.retiring.next_release_allocation_bytes().map_err(|error| ValueError::literal(ValueRefusalKind::OwnershipLimit, error.reason))?, depth: self.retiring.next_release_depth_demand().map_err(|error| ValueError::literal(ValueRefusalKind::DepthLimit, error.reason))?, ..Default::default() });
        }
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
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if self.terminal_is_empty() { return Ok(PluginLifecycleStep::Complete(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
        if !registry::fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        if let Some(load) = self.retiring.last_mut() {
            if load.terminal_is_empty() {
                drop(self.retiring.pop());
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() }));
            }
            return load.close_step(grant).map(|step| match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other }).map_err(|message| Fault::new(FaultOrigin::Framework, FaultCode::new("window-config.load-retirement"), message));
        }
        if !self.retiring.terminal_is_empty() {
            let step = self.retiring.release_empty_page(grant.maximum_release_bytes).map_err(|error| ValueError::literal(ValueRefusalKind::OwnershipLimit, error.reason).into_fault())?;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: usize::from(step.progressed), released_bytes: step.released_allocation_bytes, ..Default::default() }));
        }
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
        self.owners.terminal_is_empty() && self.retiring.terminal_is_empty() && self.actor.is_none() && self.actor_retirement.is_none()
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
