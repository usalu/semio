//! 🔌️ Ref reads and extension of the instance tool run turns: the read-only pending work view, the busy fault and the tool run extension that registers the turns with the instance. See [`super`].

use super::*;
use semio_framework::Fault;
use semio_framework::FaultOrigin;
use semio_framework::FaultCode;
use semio_framework_ui_locale::Terminology;
use semio_framework_os_kernel::DslValue;
use semio_framework::InvocationResult;
use semio_framework_plugin_interaction::UiAssemblyResult;

impl<'v, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> ToolRunRef<'v, A, M, X> {
    /// 🏃️ Ledger work plus an unobserved store or settings generation change the watch still owes.
    pub fn tool_run_has_pending_work(&self) -> bool {
        self.tool_runs.has_pending_work()
            || self.tool_runs.is_ui_dirty()
            || self.tool_runs.entries.iter().any(|entry| {
                matches!(entry.slot.state, ToolRunState::Running | ToolRunState::Paused | ToolRunState::Complete | ToolRunState::Finalizing)
                    && (entry.base_generation != self.stores.store.generation()
                        || entry.settings_generation != self.tool_run_settings_generation()
                        || entry.member.as_ref().is_some_and(|member| self.child_composition.children.get(&(member.slot.clone(), member.child_id.clone())).is_some_and(|child| child.member.visit_member(ToolRunMemberGeneration) != member.base_generation)))
            })
    }

    /// 🎚️ The committed settings generations the driver watches (config store, window-config publications).
    pub(super) fn tool_run_settings_generation(&self) -> (u64, u64) {
        (self.stores.config_store.generation(), self.tool_runs.driver.window_config_publications)
    }

    /// 🔢️ The live generation of the member the selected run edits; `None` for a document run or a member that is gone.
    pub(super) fn tool_run_member_generation(&self) -> Option<u64> {
        let member = selected_entry!(self.tool_runs)?.member.as_ref()?;
        self.child_composition.children.get(&(member.slot.clone(), member.child_id.clone())).map(|child| child.member.visit_member(ToolRunMemberGeneration))
    }

    pub fn tool_run_retirement_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.tool_runs.entries.iter().any(|entry|entry.member.as_ref().is_some_and(ToolRunMemberRun::is_work))||!self.tool_runs.retired_members.is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(demand)=self.tool_run_entry_close_demand(body)?{return Ok(demand);}
        self.tool_runs.retirement_demands(&self.stores.store,body)
    }

    /// 🚪️ Quotes the next close unit of the newest live entry of a closing ledger: the final publication and ingress owners of its finalize, then the entry itself.
    pub(super) fn tool_run_entry_close_demand(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        if !self.tool_runs.closing{return Ok(None);}
        let Some(entry)=self.tool_runs.entries.last()else{return Ok(None)};
        if let Some(finalize)=entry.finalize.as_ref(){
            if let Some(publication)=finalize.publication.as_ref(){
                if publication.terminal_is_empty(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<Option<store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Snapshot,A::Mutation>>>(),depth:1,..Default::default()}));}
                let mut demand=publication.retirement_demands(body)?;
                demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"tool entry final publication depth overflow"))?;
                return Ok(Some(demand));
            }
            if !finalize.ingress.terminal_is_empty(){return finalize.ingress.close_demands(body).map(Some);}
        }
        Ok(Some(RetirementDemand{copy_bytes:size_of::<ToolRunEntry<A>>(),release_bytes:size_of::<ToolRunEntry<A>>(),depth:1,..Default::default()}))
    }
}

/// 🛑️ The `toolRun.busy` refusal of `verb` while a freezing tool run is non-terminal on this instance.
fn tool_run_busy_fault(verb: &str) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.busy"), format!("verb {verb:?} would emit artifact mutations while a freezing tool run is non-terminal on this instance"))
}

/// 🎛️ Tool runs as an instance extension: the `toolRun*` verbs, the mutation hold a freezing run keeps, the run's
/// maintenance turn, its panel body and the trace lane it adds to the app's own scene.
#[doc(hidden)]
pub struct ToolRunExtension;

impl InstanceExtension for ToolRunExtension {
    fn claims_action(action: &str) -> bool {
        is_tool_run_action_id(action)
    }

    async fn dispatch_action<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, action: &str, args: Option<&DslValue>, meta: &ActionMeta, _identity: &mut EntityIdentityAuthority<'_>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        ToolRunCtx::of_extension(ctx).dispatch_tool_run_action(action, args, meta).await
    }

    fn mutation_refusal<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>, verb: &str) -> Option<Fault> {
        tool_runs.freezes_local_emits().then(|| tool_run_busy_fault(verb))
    }

    fn window_config_published<A: ArtifactApp>(_time_travel: &mut TimeTravelLedger<A>, tool_runs: &mut ToolRunLedger<A>) {
        tool_runs.note_window_config_published();
    }

    fn has_pending_work<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &ExtensionRef<'_, A, M, X>) -> bool {
        ToolRunRef::of_extension_ref(ctx).tool_run_has_pending_work()
    }

    fn holds_receipt<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>) -> bool {
        tool_runs.has_original_receipt()
    }

    fn terminal_is_empty<A: ArtifactApp>(_time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>) -> bool {
        tool_runs.terminal_is_empty()
    }

    async fn drive_turn<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, retained: &mut RetainedCloneGrant, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let mut ctx = ToolRunCtx::of_extension(ctx);
        if !ctx.shared().tool_run_has_pending_work() {
            return Ok(false);
        }
        let before = cx.retained_progress();
        ctx.drive_tool_run_turn(cx, identity).await?;
        if cx.retained_progress() != before {
            if !ctx.tool_runs.has_original_receipt() {
                let progress = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before, cx.retained_progress())?;
                ctx.tool_runs.retain_original_receipt(*retained, progress)?;
            }
            return Ok(true);
        }
        Ok(ctx.tool_runs.has_original_receipt())
    }

    fn render_body<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, body_key: &str, view_state: &ViewModel) -> Option<UiAssemblyResult<BuiltNode>> {
        if body_key != FRAMEWORK_TOOL_RUN_BODY_KEY {
            return None;
        }
        let (registry, locale) = (&ctx.tools.registry, view_state.locale);
        let ready_tool = [view_state.active_utility_id.as_deref(), view_state.active_tool_id.as_deref()].into_iter().flatten().find(|tool_id| registry.tool_run(tool_id).is_some());
        Some(ctx.tool_runs.panel(&registry.controller_id, locale, ready_tool, |tool_id| registry.tool_run(tool_id).map(|(label, _)| label.resolve(Terminology::Native, locale).to_string())))
    }

    fn after_app_render<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, body_key: &str, view_state: &ViewModel) -> Result<(), Fault> {
        ToolRunCtx::of_extension(ctx).inject_tool_run_trace_lane(body_key, view_state)
    }
}
