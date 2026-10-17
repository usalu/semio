//! 🎵️ Staged `tool-machine` members of the `instance-render` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 🛠️ The tool overlays: gesture, press and typing state of the continuous tools, the overlay refold and its retirement, and the test clock.
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::Fault;
use semio_framework_plugin_decl_emission::HostEvent;
use store::Mutation;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::PressLeaf;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ScrubTag;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolTag;
use semio_framework_plugin_window_config::WindowConfigMutation;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::gesture_host_event;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework::FaultFrom;
use semio_framework_value_derive::FromValue;
use store::MutationDiff;
use protocol::OpBinary;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value_derive::ToValue;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework::kernel::ActorId;
use store::{ArtifactStore, MemberFactory, SpaceMember};
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
use semio_framework::{FaultCode, FaultOrigin};
use std::future::Future;
use semio_framework_plugin_decl_views::gesture_contract::{GestureCapture, GestureSlot};
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation;
use crate::render_bundle::{RenderCtx, RenderRef};
use semio_framework_ui_contract::*;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::*;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::gesture_issuer::GestureOperation;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::gesture_issuer::reserve_live;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::gesture_issuer::issue_live;
use semio_framework_tool_machine::ToolAbortReason;
use semio_framework_tool_machine::ToolStep;
use semio_framework_tool_machine::TypingInput;
use std::collections::BTreeMap;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::OriginalAliasBatch as ToolOriginalAliasBatch;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_plugin_instance_jobs::jobs_close_ladder::handoff;

//#region 🔖️Driver
impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> RenderCtx<'ctx, A, M, X> {
    /// 🧪️ Pins the continuous tools' clock (`None`: the wall clock again), so a law types, pauses and lapses deterministically
    /// however slowly its machine runs.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn set_tool_clock_ms(&mut self, now_ms: Option<u64>) {
        self.tool_machines.now_ms = now_ms;
    }

    /// 🧊️ `frozen`: a history edit opens, so every open press and every run still open leaves zero trace (a run normally
    /// commits before the history verb reaches here).
    pub fn freeze_tool_machines(&mut self) {
        let presses = self.tool_machines.presses.abort_all(ToolAbortReason::Frozen);
        let runs = self.tool_machines.typing.abort_all(ToolAbortReason::Frozen);
        if !presses.is_empty() || !runs.is_empty() {
            self.follow_tool_machines(true);
        }
    }

    /// 🎟️ Admits the dispatch's original window gesture into a fixed runtime slot and caller-funded strong-only capture.
    /// Completed decisions retain their slot until physical maintenance; occupied slots preserve the refused admission.
    pub fn admit_gesture_slot(&mut self,operation:u64,base:&[u8;32],meta:&ActionMeta,birth:RetainedCloneGrant,handoff:RetainedCloneGrant)->Result<GestureCapture<A::Mutation>,Fault>{
        let index=self.tool_machines.gesture_operations.iter().position(Option::is_none);
        let available=index.is_some()&&!self.tool_machines.gesture_operations.iter().flatten().any(|original|original.operation==operation);
        let prepared=reserve_live::<A::Mutation>(available,birth,handoff).map_err(|error|error.into_fault())?;
        let view=meta.view_state.as_ref();
        if let Some(view)=view.filter(|view|!view.window_instances.is_empty()){
            let retired=self.tool_machines.gestures.retain_windows(|window|window.is_empty()||view.window_instances.iter().any(|instance|instance.id==window));
            if !retired.is_empty(){self.tool_machines.gesture_ended=true;self.follow_tool_machines(true);}
        }
        let window=view.and_then(|view|view.window_id.clone()).unwrap_or_default();
        let base_revision=base.iter().map(|byte|format!("{byte:02x}")).collect();
        let open=self.tool_machines.gestures.open(&window).cloned();
        let closed=self.tool_machines.gestures.closed(&window).map(str::to_string);
        let(original,capture)=issue_live(prepared,GestureSlot::of(window,base_revision,open,closed),operation,handoff);
        self.tool_machines.gesture_operations[index.expect("original fixed gesture slot was reserved")]=Some(original);
        Ok(capture)
    }

    /// 🖋️ Settlement retains the original captured slot and rejected decision until funded maintenance.
    pub fn settle_gesture_slot(&mut self,operation:u64,publishes:bool)->bool{
        let frozen=X::freezes_document(&*self.time_travel, &*self.tool_runs);
        let Some(original)=self.tool_machines.gesture_operations.iter_mut().flatten().find(|original|original.operation==operation&&!original.settled)else{return false};
        original.settled=true;
        original.decision=original.slot.take();
        let Some(decision)=original.decision.as_mut()else{return false};
        if publishes&&!frozen&&self.tool_machines.gestures.open(original.slot.window())==decision.admitted.as_ref(){
            if let Some(press)=decision.closing.take(){self.tool_machines.gestures.close(original.slot.window(),press);}
            if let Some(decided)=decision.decided.take(){self.tool_machines.gestures.settle(original.slot.window(),decided);self.follow_tool_machines(true);}
        }
        true
    }

    /// 🛑️ Cancellation records intent while preserving the original slot and every decided field.
    pub fn request_gesture_cancellation(&mut self,operation:u64){if let Some(original)=self.tool_machines.gesture_operations.iter_mut().flatten().find(|original|original.operation==operation){original.request_cancellation();}}

    /// 🪦️ The bounded fixed registry closes every original operation through its existing maintenance child.
    pub fn begin_gesture_retirement(&mut self){for original in self.tool_machines.gesture_operations.iter_mut().flatten(){original.request_cancellation();}}

    fn gesture_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let demand=self.shared().gesture_retirement_demand(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(owner)=self.tool_machines.gesture_retirement.as_mut(){
            if owner.terminal_is_empty(){drop(self.tool_machines.gesture_retirement.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
            return owner.step(grant);
        }
        let Some(index)=self.tool_machines.gesture_operations.iter().position(|original|original.as_ref().is_some_and(|original|original.settled))else{return Ok(RetainedCloneStep::Complete(Default::default()));};
        let original=self.tool_machines.gesture_operations[index].take().expect("selected original settled gesture");
        match ControlledRetirement::new(original){Ok(owner)=>{self.tool_machines.gesture_retirement=Some(owner);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))},Err((error,original))=>{self.tool_machines.gesture_operations[index]=Some(original);Err(error)}}
    }

    /// 🛎️ A host fact of one window: its open gesture ends with the fact's reason and zero trace — the ONE place a blur, a
    /// lost capture, a utility switch, a closing window, a moved base or a history edit ends a gesture; no editor maps it.
    pub fn end_window_gesture(&mut self, event: &HostEvent) {
        if matches!(self.tool_machines.gestures.host_event(event.window_id(), gesture_host_event(event)), ToolStep::Aborted(..)) {
            self.tool_machines.gesture_ended = true;
            self.follow_tool_machines(true);
        }
    }

    /// 🌐️ An instance-wide host fact (a history edit froze the document, a remote edit moved the base): every window's
    /// open gesture ends with the fact's reason and zero trace, whether or not a view lists the window.
    pub fn end_every_gesture(&mut self, event: &HostEvent) {
        if !self.tool_machines.gestures.host_event_all(gesture_host_event(event)).is_empty() {
            self.tool_machines.gesture_ended = true;
            self.follow_tool_machines(true);
        }
    }

    /// 🎨️ Whether a host fact ended a gesture since the last ask: the windows that previewed it owe a repaint.
    pub fn take_gesture_ended(&mut self) -> bool {
        std::mem::take(&mut self.tool_machines.gesture_ended)
    }

    /// 🪞️ Refolds the tool overlays — the document on the committed head, the app config on the committed config, every held
    /// window config on its window's committed partition — retires every displaced alias through its store (a snapshot with
    /// retire-owned roots panics on a plain drop of its last owner), and drops every run the moved head conflicts with.
    pub fn follow_tool_machines(&mut self, changed: bool) {
        if self.shared().tool_overlay_retirement_pending(){self.tool_machines.refold_owed=true;return;}
        self.tool_machines.refold_owed=false;
        let machines = &*self.tool_machines;
        if machines.presses.is_empty() && machines.gestures.is_empty() && machines.typing.is_empty() && machines.overlay.is_none() && machines.config_overlay.is_none() && machines.window_overlays.is_empty() {
            return;
        }
        let committed = self.stores.store.snapshot_owner();
        let generation = self.stores.store.generation();
        let (displaced, _aborted) = self.tool_machines.follow(&committed, generation, changed);
        if !displaced.is_empty()||displaced.capacity()!=0{self.tool_machines.pending_overlay_aliases=Some(displaced);}
        let committed_config = self.stores.config_store.snapshot_owner();
        let config_displaced=self.tool_machines.follow_config(&committed_config,self.stores.config_store.generation(),changed);
        if !config_displaced.is_empty()||config_displaced.capacity()!=0{self.tool_machines.pending_config_aliases=Some(config_displaced);}
        let mut targets: BTreeMap<String, (String, Vec<&WindowConfigMutation>)> = BTreeMap::new();
        for mutation in self.tool_machines.presses.provisional().filter_map(PressLeaf::window_config) {
            targets.entry(mutation.window_id().to_string()).or_insert_with(|| (mutation.window_kind_id().to_string(), Vec::new())).1.push(mutation);
        }
        let stale: Vec<String> = self.tool_machines.window_overlays.keys().filter(|window| !targets.contains_key(*window)).cloned().collect();
        for window in stale {
            if let Some(overlay) = self.tool_machines.window_overlays.remove(&window) {
                let mut original=Some(overlay);
                if !self.stores.window_config_store.retire_preview(&mut original){self.tool_machines.window_overlays.insert(window,original.expect("refused original preview remains"));self.tool_machines.refold_owed=true;}
            }
        }
        for (window, (kind, mutations)) in targets {
            let committed_generation = self.stores.window_config_store.snapshot(&kind, &window).map(|snapshot| snapshot.generation());
            let current = self.tool_machines.window_overlays.get(&window).map(|overlay| overlay.generation());
            if !changed && current.is_some() && current == committed_generation {
                continue;
            }
            if !self.stores.window_config_store.preview_available(&kind,&window)||self.tool_machines.window_overlays.get(&window).is_some_and(|original|!self.stores.window_config_store.can_retire_preview(original)){self.tool_machines.refold_owed=true;continue;}
            let preview = self.stores.window_config_store.preview(&kind, &window, &mutations);
            let displaced = match preview {
                Some(preview) => self.tool_machines.window_overlays.insert(window, preview),
                None => self.tool_machines.window_overlays.remove(&window),
            };
            if let Some(displaced) = displaced {
                let mut original=Some(displaced);
                assert!(self.stores.window_config_store.retire_preview(&mut original),"prevalidated original window preview remains admissible in its unchanged native partition");
            }
        }
    }

    /// 🎟️ Original refold Vec/Arc owners wait for this actual full grant; no cold-drain fallback is admitted.
    pub fn tool_overlay_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if !self.shared().tool_overlay_retirement_pending(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if self.shared().gesture_retirement_pending(){return self.gesture_retirement_step(grant);}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let demand=self.shared().tool_overlay_retirement_demand(grant.maximum_copy_bytes)?;
        if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(owner)=self.tool_machines.overlay_alias_retirement.as_mut(){
            if owner.terminal_is_empty(){drop(self.tool_machines.overlay_alias_retirement.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
            return owner.advance(ArtifactStore::<A::Snapshot,A::Mutation>::snapshot_alias_retirement_birth_bytes(),grant,|alias,child|self.stores.store.retire_snapshot_alias(alias,child));
        }
        if self.tool_machines.pending_overlay_aliases.is_some(){
            let(owner,progress)=ToolOriginalAliasBatch::admit_original(&mut self.tool_machines.pending_overlay_aliases,grant)?.expect("admitted original overlay batch");self.tool_machines.overlay_alias_retirement=Some(owner);return Ok(RetainedCloneStep::Progress(progress));
        }
        if let Some(owner)=self.tool_machines.config_alias_retirement.as_mut(){
            if owner.terminal_is_empty(){drop(self.tool_machines.config_alias_retirement.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
            return owner.advance(ArtifactStore::<A::Config,A::ConfigMutation>::snapshot_alias_retirement_birth_bytes(),grant,|alias,child|self.stores.config_store.retire_snapshot_alias(alias,child));
        }
        if self.tool_machines.pending_config_aliases.is_some(){let(owner,progress)=ToolOriginalAliasBatch::admit_original(&mut self.tool_machines.pending_config_aliases,grant)?.expect("admitted original config overlay batch");self.tool_machines.config_alias_retirement=Some(owner);return Ok(RetainedCloneStep::Progress(progress));}
        self.stores.window_config_store.preview_retirement_step(grant)
    }

    /// 🎚️ Rides every lane of `emit` — its artifact leaves, its owned children's shares (design §12), its app-config and
    /// window-config mutations (§20.1) — on `tag`'s press as ONE scrub opened on the document revision `base` by `actor`. A
    /// tick holds them as the press's provisional overlay and publishes nothing; the release commits them as ONE transaction
    /// and hands every lane back to `emit` — the document lanes stamped with the press's `TransactionRef` (a ref the verb
    /// minted itself never outlives the press), the config lanes as ONE config edit that is never a history row; a late input
    /// of the press the window already closed stays silent on every lane. One ledger step decides every lane, so a refused
    /// input leaves the press, its overlays and its held config exactly as they were until a retry or a host abort decides.
    /// Answers whether `emit` publishes the press.
    pub fn settle_press<D>(&mut self, tag: &ScrubTag, base: &[u8], actor: &str, emit: &mut Emit<A::Mutation, A::ConfigMutation, D>) -> Result<bool, Fault> {
        let base: String = base.iter().map(|byte| format!("{byte:02x}")).collect();
        let leaves = std::mem::take(&mut emit.artifact_mutations)
            .into_iter()
            .map(PressLeaf::Member)
            .chain(std::mem::take(&mut emit.child_emits).into_iter().map(PressLeaf::Child))
            .chain(std::mem::take(&mut emit.config_mutations).into_iter().map(PressLeaf::Config))
            .chain(std::mem::take(&mut emit.window_config_mutations).into_iter().map(PressLeaf::WindowConfig))
            .collect();
        emit.transaction = None;
        let step = self
            .tool_machines
            .presses
            .send(&tag.window, &tag.tool, &ActorId(actor.into()), &base, tag.phase.clone().input(leaves), semio_framework_tool_machine::authoring_clock(0))
            .map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("continuous control {:?} refused its press", tag.tool)))?;
        let ToolStep::Committed(transaction, leaves) = step else { return Ok(false) };
        for leaf in leaves {
            match leaf {
                PressLeaf::Member(leaf) => emit.artifact_mutations.push(leaf),
                PressLeaf::Child(child) => emit.child_emits.push(child),
                PressLeaf::Config(config) => emit.config_mutations.push(config),
                PressLeaf::WindowConfig(window_config) => emit.window_config_mutations.push(window_config),
            }
        }
        if !emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty() {
            emit.transaction = Some(transaction);
        }
        Ok(true)
    }

    /// 🛠️ The ONE point an operation's completion becomes its tools' publication. The gesture slot its dispatch drove follows
    /// first ([`Self::settle_gesture_slot`]); a dispatch that only advanced or ended its gesture logs no history row. A tagged
    /// operation then settles its press or run: a press settles every lane of its emit on
    /// the window's press ([`Self::settle_press`]) on the operation's document revision. A typed edit: its leaves fold into
    /// the window's run; nothing publishes while the run is open, and a run the edit ended (idle lapse, caret jump, another
    /// buffer) publishes in this emit as ONE edit stamped with its own `TransactionRef`. An emit that carries no transaction
    /// logs no history row; every other lane of the emit (effects, events, UI scope) publishes as usual.
    pub fn settle_tool_operation(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, publication: &mut ArtifactToolCompletionValue<A>) -> Result<(), Fault> {
        let operation = mounted.operation.operation.0;
        let driven = self.settle_gesture_slot(operation, matches!(publication, ArtifactToolCompletionValue::Emit(Ok(_), _)));
        let tag = self.tool_machines.take_operation(operation);
        let ArtifactToolCompletionValue::Emit(Ok(emit), _) = publication else { return Ok(()) };
        if driven && emit.transaction.is_none() && emit.artifact_mutations.is_empty() {
            mounted.command_logged = true;
        }
        let Some(tag) = tag else { return Ok(()) };
        match tag {
            ToolTag::Scrub(tag) => {
                self.settle_press(&tag, &mounted.canonical_revision, &mounted.meta.actor, emit)?;
            }
            ToolTag::Typing(tag) => {
                let leaves = std::mem::take(&mut emit.artifact_mutations);
                let clock = self.tool_machines.clock();
                let steps = self
                    .tool_machines
                    .typing
                    .send(&tag.window, &tag.tool, &ActorId(mounted.meta.actor.clone()), TypingInput::Edit { buffer: tag.buffer, leaves }, A::typing_fold, clock)
                    .map_err(|refusal| Fault::new(FaultOrigin::Framework, FaultCode::new(refusal.code()), format!("typing run {:?} refused its edit", tag.tool)))?;
                if let Some((transaction, mutations)) = steps.into_iter().find_map(|step| if let ToolStep::Committed(transaction, mutations) = step { Some((transaction, mutations)) } else { None }) {
                    emit.artifact_mutations = mutations;
                    emit.transaction = Some(transaction);
                }
            }
        }
        if emit.transaction.is_none() {
            mounted.command_logged = true;
        }
        self.follow_tool_machines(true);
        Ok(())
    }
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> RenderRef<'v, A, M, X> {
    /// 🧪️ The document every render seam reads now: the committed head with every open press's and typing run's provisional
    /// leaves.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn rendered_snapshot(&self) -> Arc<A::Snapshot> {
        let committed = self.stores.store.snapshot_owner();
        Arc::clone(self.tool_machines.overlay_or(&committed))
    }

    /// 🧪️ The app config every render seam reads now: the committed config with every held press's config mutations.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn rendered_config(&self) -> Arc<A::Config> {
        Arc::clone(self.tool_machines.config_overlay_or(&self.stores.config_store.snapshot_owner()))
    }

    fn gesture_retirement_pending(&self)->bool{self.tool_machines.gesture_retirement.is_some()||self.tool_machines.gesture_operations.iter().flatten().any(|original|original.settled)}

    fn gesture_retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.tool_machines.gesture_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<GestureOperation<A::Mutation>>>>(),depth:1,..Default::default()})}else{Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})};}
        Ok(RetirementDemand{copy_bytes:size_of::<GestureOperation<A::Mutation>>(),depth:1,..Default::default()})
    }

    pub fn tool_overlay_retirement_pending(&self)->bool{
        self.gesture_retirement_pending()||self.tool_machines.pending_overlay_aliases.is_some()||self.tool_machines.overlay_alias_retirement.is_some()||self.tool_machines.pending_config_aliases.is_some()||self.tool_machines.config_alias_retirement.is_some()||self.stores.window_config_store.preview_retirement_pending()
    }

    fn tool_overlay_refold_owed(&self)->bool{self.tool_machines.refold_owed}

    pub fn tool_overlay_retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.gesture_retirement_pending(){return self.gesture_retirement_demand(body);}
        if let Some(owner)=self.tool_machines.overlay_alias_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ToolOriginalAliasBatch<A::Snapshot>>>(),depth:1,..Default::default()})}else{owner.next_demand(ArtifactStore::<A::Snapshot,A::Mutation>::snapshot_alias_retirement_birth_bytes())};}
        if self.tool_machines.pending_overlay_aliases.is_some(){return Ok(ToolOriginalAliasBatch::<A::Snapshot>::constructor_demand());}
        if let Some(owner)=self.tool_machines.config_alias_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ToolOriginalAliasBatch<A::Config>>>(),depth:1,..Default::default()})}else{owner.next_demand(ArtifactStore::<A::Config,A::ConfigMutation>::snapshot_alias_retirement_birth_bytes())};}
        if self.tool_machines.pending_config_aliases.is_some(){return Ok(ToolOriginalAliasBatch::<A::Config>::constructor_demand());}
        self.stores.window_config_store.preview_retirement_demand()
    }
}
//#endregion 🔖️Driver
