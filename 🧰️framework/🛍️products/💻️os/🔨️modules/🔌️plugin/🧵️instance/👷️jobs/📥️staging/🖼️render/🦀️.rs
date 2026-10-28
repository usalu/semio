//! 🧽️ Staged slice of the `render` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! 🖼️ Render side of the mounted artifact instance: composition graph, interaction state, render, media jobs and media export.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_ui_runtime::ComponentTree;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::{LocalInteraction, PeerRoster};
use std::mem::ManuallyDrop;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use crate::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_ui_contract::*;
use crate::jobs_instance_root::{OriginalRenderInteractionStamp, original_render_posted_since};

#[path = "work.rs"]
pub mod work;

impl<A: ArtifactApp> PendingReserved<A> {
    pub fn render_failure_terminal_is_empty(&self)->bool{self.pending_render_work.is_none()&&self.pending_render_source.is_none()&&self.pending_render_tree.is_none()&&self.pending_render_tree_close.is_none()&&self.pending_render_tree_identity.is_none()&&self.pending_render_failure.is_none()&&self.pending_render_close_failure.is_none()&&self.pending_render_compound.is_none()&&self.pending_render_retirement.is_none()&&self.pending_render_identity.is_none()}

    pub fn render_failure_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(work)=self.pending_render_work.as_ref(){return work.demand(body)}
        if let Some(source)=self.pending_render_source.as_ref(){return source.demand(body)}
        if self.pending_render_tree.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(tree)=self.pending_render_tree_close.as_ref(){return tree.next_demand(body)}
        if self.pending_render_close_failure.is_some()||self.pending_render_compound.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if let Some(owner)=self.pending_render_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)}}
        if self.pending_render_failure.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        Ok(Default::default())
    }

    pub fn close_render_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let before=cx.retained_progress();
        if !self.prepare_render_closure_failure(cx)?{return Ok(PluginLifecycleStep::Progress(original_render_posted_since(before,cx.retained_progress())?))}
        match self.close_pending_render_tree_on_original(cx){Ok(Some(true))=>{},Ok(_)=>return Ok(PluginLifecycleStep::Progress(original_render_posted_since(before,cx.retained_progress())?)),Err(failure)=>{self.retain_render_closure_failure(failure, cx);return Ok(PluginLifecycleStep::Progress(original_render_posted_since(before,cx.retained_progress())?))}}
        let demand=self.render_failure_demands(grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|cx,grant|{
            if let Some(owner)=self.pending_render_retirement.as_mut(){
                if owner.terminal_is_empty(){drop(self.pending_render_retirement.take());self.pending_render_identity=None;self.pending_render_tree_identity=None;return Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),true))}
                let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok((PluginLifecycleStep::Progress(step.progress()),false));
            }
            if let Some(source)=self.pending_render_failure.take(){
                match semio_framework_value::retirement::controlled::ControlledRetirement::new(source){Ok(owner)=>{*self.pending_render_retirement=Some(owner);Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))},Err((failure,source))=>{*self.pending_render_failure=Some(source);Err(failure.into())}}
            }else{self.pending_render_identity=None;self.pending_render_tree_identity=None;Ok((PluginLifecycleStep::Complete(Default::default()),true))}
        })
    }

    pub(crate) fn close_pending_render_tree_on_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<bool>,semio_framework_job::OriginalTurnFailure>{
        if let Some(identity)=self.pending_render_tree_identity.as_mut(){identity.validate(cx)?;}
        if self.close_pending_render_work_on_original(cx)?!=Some(true){return Ok(None)}
        if let Some(tree)=self.pending_render_tree.as_mut(){
            if let Some(resources)=tree.native_resources.as_mut(){
                if resources.close_step_on_original(cx)?!=Some(true){return Ok(None)}
                tree.native_resources=None;
                return Ok(Some(false));
            }
            let grant=cx.retained_grant();
            if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
            let tree=self.pending_render_tree.take().expect("original tree source remains stationary until funded close donation");
            *self.pending_render_tree_close=Some(semio_framework_ui_contract::OriginalBuiltTreeRetirement::new(tree.root));
            self.pending_render_tree_ready=false;
            cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
            cx.consume_fuel(1);
            return Ok(Some(false));
        }
        if let Some(body)=self.pending_render_tree_close.as_mut(){
            if !body.terminal_is_empty(){
                let demand=body.next_demand(cx.retained_grant().maximum_copy_bytes)?;
                semio_framework_job::close_original_retained_step(cx,demand,|grant|body.step(grant))?;
                return Ok(Some(false));
            }
            let grant=cx.retained_grant();
            if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
            drop(self.pending_render_tree_close.take());
            cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;
            cx.consume_fuel(1);
            return Ok(Some(false));
        }
        self.pending_render_tree_ready=false;
        Ok(Some(true))
    }

    pub fn close_pending_render_work_on_original(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<bool>,semio_framework_job::OriginalTurnFailure>{
        if self.pending_render_work.is_none()&&self.pending_render_source.is_none(){return Ok(Some(true))}
        let identity=self.pending_render_tree_identity.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"retained render work lost its initiating identity"))?;
        if let Some(work)=self.pending_render_work.as_mut(){
            return match work.close(identity,cx)?{
                Some(true)=>{drop(self.pending_render_work.take());Ok(Some(false))},
                step=>Ok(step),
            };
        }
        if let Some(source)=self.pending_render_source.as_mut(){
            return match source.close(identity,cx)?{
                Some(true)=>{drop(self.pending_render_source.take());Ok(Some(true))},
                step=>Ok(step),
            };
        }
        Ok(Some(true))
    }

    pub fn retain_pending_render_tree(&mut self,tree:ComponentTree){
        assert!(self.pending_render_tree.is_none()&&self.pending_render_tree_identity.is_some(),"original render tree destination must be reserved before production");
        self.pending_render_tree=std::mem::ManuallyDrop::new(Some(tree));
        self.pending_render_tree_ready=false;
    }

    pub fn retain_render_failure<T>(&mut self,source:semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<T>,semio_framework_job::OriginalTurnFailure>{
        if self.pending_render_failure.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original pending render source cannot be overwritten").into())}
        *self.pending_render_failure=Some(source);self.pending_render_identity=Some((cx.operation(),cx.generation()));let _=self.receive_render_failure(cx)?;Ok(None)
    }

    pub(crate) fn retain_render_closure_failure(&mut self,failure:semio_framework_job::OriginalTurnFailure,cx:&semio_framework_job::StepContext<'_>){
        assert!(self.pending_render_compound.is_none()&&self.pending_render_close_failure.is_none(),"original render closure failure cannot overwrite a retained cause");
        if self.pending_render_failure.is_some(){*self.pending_render_close_failure=Some(failure);}else{*self.pending_render_failure=Some(semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure::Original(failure));self.pending_render_identity=Some((cx.operation(),cx.generation()));}
    }

    pub(crate) fn prepare_render_closure_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.pending_render_compound.is_some(){
            if let Some(original)=semio_framework_job::admit_original_domain_failure(&mut self.pending_render_compound,cx)?{assert!(self.pending_render_failure.is_none());*self.pending_render_failure=Some(semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure::Original(original));}
            return Ok(self.pending_render_compound.is_none());
        }
        if self.pending_render_close_failure.is_none(){return Ok(true)}
        if !matches!(self.pending_render_failure.as_ref(),Some(semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure::Original(_))){
            if let Some(original)=semio_framework_job::admit_original_domain_failure(&mut self.pending_render_failure,cx)?{*self.pending_render_failure=Some(semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure::Original(original));}
            return Ok(false);
        }
        let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
        let Some(semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure::Original(initial))=self.pending_render_failure.take()else{unreachable!()};
        let closure=self.pending_render_close_failure.take().expect("original render closure cause remains stationary");
        *self.pending_render_compound=Some(semio_framework_job::OriginalClosureFailure{initial,closure});
        cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(false)
    }

    pub fn receive_render_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.pending_render_failure.is_none()&&self.pending_render_compound.is_none()&&self.pending_render_close_failure.is_none(){return Ok(false)}
        if !self.prepare_render_closure_failure(cx)?{return Ok(true)}
        match self.close_pending_render_tree_on_original(cx){Ok(Some(true))=>{},Ok(_)=>return Ok(true),Err(failure)=>{self.retain_render_closure_failure(failure, cx);return Ok(true)}}
        if matches!(self.pending_render_failure.as_ref(),Some(semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure::Original(_))){
            let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(true)}
            let Some(semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure::Original(failure))=self.pending_render_failure.take() else{unreachable!()};
            self.pending_render_identity=None;self.pending_render_tree_identity=None;
            cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Err(failure);
        }
        if self.pending_render_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"pending render domain requires its unchanged initiating source identity").into())}
        if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.pending_render_failure,cx)?{self.pending_render_identity=None;self.pending_render_tree_identity=None;return Err(failure)}
        Ok(true)
    }
}

impl<A: ArtifactApp> ToolOperations<A> {
    /// 🕹️ Task 5: recursively walks a just-presented `ComponentTree`, and for every `Component::Tree`
    /// carrying an `interaction_domain` caches a `DomainTopology` derived from its children (the
    /// `HierarchyProvider::UiTree` half of `resolve_domain_topology`) into `interaction_ui_topology`.
    /// Recursion is uniform over `TreeNode::children` now — every component nests the same way (no
    /// more per-variant `Stack`/`Section`/`Group`/`Field` match arms; a leaf is just a node with no
    /// children).
    ///
    /// 👥️ M2 (ticket 26/08/17 `design-unified.md`): ALSO derives one `PresenceUpdate` per node
    /// whose id is selected/hovered — own state off `state`/`self.interaction_hover`, peers off
    /// `InteractionView::peers_selecting`/`peers_hovering` — pushed onto `self.pending_presence`
    /// for `plugin_runtime::plugin_render_surface` to drain. This closes the gap the prior packet's
    /// own report named ("nothing in this function currently reads `state`" — see git history):
    /// selection/hover is `ui_contract::PresenceUpdate` now, never written back onto the tree
    /// itself (`TreeNode`/`Component::TreeItem` carry no presence field — the render-plane
    /// derivation lives entirely in this outbox, never in a document mutation).
    ///
    /// 🪟️ A windowed tree hands `ui_tree_domain_topology` only the rows this render
    /// materialised, never the container's full `TreeWindow::total` — see that function's own note.
    ///
    /// 🕹️ Materialized owned BEFORE the mutable walk — same "clone owned before the field-wise
    /// destructure" reasoning `dispatch_typed_command_inner` already uses: `pending_presence` is
    /// WRITTEN inside the walk while `interaction_hover`/`peer_presence` are only ever READ by
    /// it, so a snapshot avoids an aliasing conflict against `&mut self`.
    #[cfg(test)]
    pub async fn stamp_and_cache_interaction_ui(&mut self, roster: &mut PeerRoster<A>, interaction: &mut LocalInteraction<A>, tree: &ComponentTree, state: &protocol::InteractionState, body_key: &str) -> UiAssemblyResult<()> {
        OriginalRenderInteractionStamp { registry: &self.registry, interaction_hover: &interaction.interaction_hover, peer_presence: &roster.peer_presence, own_color: roster.own_color, local_interaction_authority: &mut interaction.local_interaction_authority, interaction_ui_topology: &mut interaction.interaction_ui_topology, pending_presence: &mut roster.pending_presence, presence_marked_keys: &mut roster.presence_marked_keys }.stamp_and_cache_interaction_ui(tree,state,body_key).await
    }
}
