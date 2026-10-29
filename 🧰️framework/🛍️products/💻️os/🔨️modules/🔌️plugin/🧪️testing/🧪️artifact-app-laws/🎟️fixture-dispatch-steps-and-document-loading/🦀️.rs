//! 🎟️ Fixture dispatch, steps and document loading: the fixture action dispatch, the mounted policy, retained turn and step context, the fixture step outcome, the document load helpers, the typing run, the maintenance pressure drain and the genesis children assertions. See [`super`].

use super::*;

/// 🎟️ Drives the actual borrowed action endpoint and pays remaining original fixture sources under its authored close loan.
pub async fn dispatch_fixture_action<P:PluginApp>(app:&mut P,action:&str,args:Option<semio_framework_value::DslValue>,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)->Result<semio_framework::kernel::InvocationResult,semio_framework_job::OriginalTurnFailure>{
    let mut original_args=args;let mut original_meta=Some(meta.clone());
    let mut result=Ok(None);
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        result=app.handle_action(action,&mut original_args,&mut original_meta,identity,cx).await;
        if !matches!(result,Ok(None)){break}
        if let Err(failure)=app.maintenance_step(cx,close_loan){result=Err(failure.into());break}
    }
    close_fixture_action_inputs(original_args,None,cx,close_loan);
    result?.ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original action fixture dispatch exceeded its authored turn bound").into())
}

/// 🧭️ Drives the actual addressed command endpoint and pays remaining original fixture sources under its authored close loan.
pub async fn dispatch_fixture_command<P:PluginApp>(app:&mut P,address:&semio_framework::manifest::CommandAddress,args:Option<semio_framework_value::DslValue>,active_mode:Option<&str>,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)->Result<semio_framework::kernel::InvocationResult,semio_framework_job::OriginalTurnFailure>{
    let mut original_args=args;let mut original_fault=None;let mut original_meta=Some(meta.clone());
    let mut result=Ok(None);
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        result=app.handle_command(address,&mut original_args,&mut original_fault,active_mode,&mut original_meta,identity,cx).await;
        if !matches!(result,Ok(None)){break}
        if let Err(failure)=app.maintenance_step(cx,close_loan){result=Err(failure.into());break}
    }
    close_fixture_action_inputs(original_args,original_fault,cx,close_loan);
    result?.ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original command fixture dispatch exceeded its authored turn bound").into())
}

/// 📤️ Exports one media port through the real original-caller loop: each pending turn funds one maintenance step under the authored close loan until the port answers.
pub async fn export_fixture_media<P:PluginApp>(app:&mut P,port:&str,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)->Result<semio_framework::Media,semio_framework_job::OriginalTurnFailure>{
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        if let Some(media)=app.export_media(port,cx).await?{return Ok(media)}
        if let Err(failure)=app.maintenance_step(cx,close_loan){return Err(failure.into())}
    }
    Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original media export fixture exceeded its authored turn bound").into())
}

/// 💾️ Produces one natural media artifact through the real original caller: each pending turn funds one maintenance step under the authored close loan until the port answers.
pub async fn produce_fixture_media<P:PluginApp>(app:&mut P,port:&str,owner:&mut semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner<'_,'_>,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)->Result<semio_framework_plugin_document_media::MediaArtifact,semio_framework_plugin_decl_app::MediaArtifactOperationFailure>{
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        if let Some(artifact)=app.produce_media(port,owner,cx).await?{return Ok(artifact)}
        if let Err(failure)=app.maintenance_step(cx,close_loan){return Err(semio_framework_job::OriginalTurnFailure::from(failure).into())}
    }
    Err(semio_framework_job::OriginalTurnFailure::from(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original media production fixture exceeded its authored turn bound")).into())
}

/// 🎬️ Submits one interactive media export through the real original receiving port: each pending turn funds one maintenance step under the authored close loan until the handle is received.
pub async fn submit_fixture_media_export<P:PluginApp>(app:&mut P,port:&str,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)->Result<semio_framework_plugin_decl_media::ArtifactMediaExportHandle,semio_framework_job::OriginalTurnFailure>{
    let mut receiver=None;
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        if app.submit_media_export(port,&mut receiver,cx).await?{break}
        if let Err(failure)=app.maintenance_step(cx,close_loan){return Err(failure.into())}
    }
    receiver.ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original media submission fixture exceeded its authored turn bound").into())
}

/// 📡️ Advances one submitted media export to its next poll answer through the real original receiving port.
pub async fn poll_fixture_media_export<P:PluginApp>(app:&mut P,handle:&semio_framework_plugin_decl_media::ArtifactMediaExportHandle,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)->Result<semio_framework_plugin_decl_media::ArtifactMediaExportPoll,semio_framework_job::OriginalTurnFailure>{
    let mut receiver=None;
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        if app.poll_media_export(handle.original_loan(),&mut receiver,cx).await?{break}
        if let Err(failure)=app.maintenance_step(cx,close_loan){return Err(failure.into())}
    }
    receiver.ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original media poll fixture exceeded its authored turn bound").into())
}

/// 📦️ Publishes one completed media export result as the segmented download the host would hand out.
pub async fn retain_fixture_media_export_result<P:PluginApp>(app:&mut P,handle:&semio_framework_plugin_decl_media::ArtifactMediaExportHandle,result:semio_framework_plugin_decl_media::ArtifactMediaExportResult,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)->Result<semio_framework_plugin_decl_media::ArtifactMediaExportPublishedStatus,semio_framework_job::OriginalTurnFailure>{
    let mut source=Some(semio_framework_plugin_decl_media::ArtifactMediaExportPoll::Complete(result));let mut status=None;
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        if app.retain_media_export_result(handle.original_loan(),&mut source,&mut status,cx).await?{break}
        if let Err(failure)=app.maintenance_step(cx,close_loan){return Err(failure.into())}
    }
    status.ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original media download publication fixture exceeded its authored turn bound").into())
}

/// ⬇️ Takes the next segmented download chunk of one retained media export; `None` is the end of the download.
pub async fn take_fixture_media_export_chunk<P:PluginApp>(app:&mut P,handle:&semio_framework_plugin_decl_media::ArtifactMediaExportHandle,terminal:&mut bool,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)->Result<Option<Vec<u8>>,semio_framework_job::OriginalTurnFailure>{
    let mut receiver=None;
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        if app.take_media_export_chunk(handle.original_loan(),&mut receiver,terminal,cx).await?{return Ok(receiver)}
        if let Err(failure)=app.maintenance_step(cx,close_loan){return Err(failure.into())}
    }
    Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original media chunk fixture exceeded its authored turn bound").into())
}

/// 📨️ Exercises the real continuation receiver with one original argument and fault pair.
pub async fn dispatch_fixture_response_action<P:PluginApp>(app:&mut P,action:&str,args:Option<semio_framework_value::DslValue>,fault:Option<semio_framework::Fault>,meta:&mut Option<ActionMeta>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant)->Result<semio_framework::kernel::InvocationResult,semio_framework_job::OriginalTurnFailure>{
    let(mut original_args,mut original_fault)=(args,fault);let mut result=Ok(None);
    for _ in 0..OriginalFixtureCaller::maximum_turns(){result=app.handle_response_action(action,&mut original_args,&mut original_fault,meta,identity,cx).await;if !matches!(result,Ok(None)){break}if let Err(failure)=app.maintenance_step(cx,close_loan){result=Err(failure.into());break}}
    close_fixture_action_inputs(original_args,original_fault,cx,close_loan);
    result?.ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original continuation fixture dispatch exceeded its authored turn bound").into())
}

fn close_fixture_action_inputs(args:Option<semio_framework_value::DslValue>,fault:Option<semio_framework::Fault>,cx:&mut semio_framework_job::StepContext<'_>,close_loan:RetainedCloneGrant){
    if args.is_some()||fault.is_some(){
        let mut receiving=Some(semio_framework_plugin_instance_stores::action_ingress::ActionIngressSource{original_typing_ready:false,original_typing_control_ready:false,original_press_result:None,original_tool_dispatch:None,original_press_tag:None,original_tool_tag:None,original_tool_tag_ready:false,action:None,args,prepared:None,assembled:None,fault});
        let mut original_caller=None;
        let mut owner=semio_framework_plugin_instance_stores::action_ingress::ActionIngressRecord::receive(&mut receiving,&mut original_caller,cx).expect("original fixture action receiving receipt").expect("authored caller must fund original unused argument receiving");
        owner.finish_parser();
        for _ in 0..OriginalFixtureCaller::maximum_turns(){
            if owner.terminal_is_empty(){break}
            cx.with_retained_loan(close_loan,|same|owner.close_original_step(same)).expect("original fixture action unused argument close");
        }
        assert!(owner.terminal_is_empty(),"original fixture action unused source did not complete its paid close");
    }
}

/// 🔬️ Borrows one real app registry publication for a synchronous integration-law closure.
pub async fn with_document_view<A, M, R>(app: &mut VcsArtifactApp<A, M>, inspect: impl for<'a> FnOnce(&semio_framework_plugin_decl_views::ArtifactView<'a, A::Snapshot>) -> R) -> Result<R, semio_framework::Fault>
where
    A: ArtifactApp,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    app.render_ctx().refresh_cache().await?;
    let read = app.stores.store.snapshot_read().map_err(semio_framework_diagnostic::FaultFrom::into_fault)?;
    let (generation, revision) = (app.stores.store.generation_now(), app.stores.store.content_revision());
    if !read.commit_authority_matches(generation, revision) {
        return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("framework.artifact-law.publication-invariant"), "law parent snapshot publication changed before document view capture").with_param("refusalKind", semio_framework_value::ValueRefusalKind::InvariantViolated.as_str()));
    }
    let (_, snapshot, _, history) = app.history.cache.as_ref().ok_or_else(|| semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault("law document view cache was not prepared"))?;
    if !std::ptr::eq(snapshot.as_ref(), read.get()) {
        return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("framework.artifact-law.publication-invariant"), "law history cache belongs to a different parent snapshot publication").with_param("refusalKind", semio_framework_value::ValueRefusalKind::InvariantViolated.as_str()));
    }
    let view = semio_framework_plugin_decl_views::ArtifactView::with_children(read.get(), history.as_ref(), semio_framework_plugin_decl_views::child_view::ChildContentView::clone(&app.child_composition.child_content_root));
    Ok(inspect(&view))
}

/// 🎟️ The explicit mounted owner policy every testkit fixture instance runs under.
pub fn fixture_mounted_policy() -> semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
    semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant }
}

/// 🎟️ One valid original retained turn input for a fixture `Budget`.
pub fn fixture_retained_turn() -> semio_framework::kernel::RetainedTurnInput {
    semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, work: semio_framework::kernel::OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: u64::MAX, fuel: 1_000_000 }, grant: fixture_mounted_policy().maintenance }
}

/// 🧮️ One fresh unfunded-by-default step context carrying exactly the operation, generation and grant of [`fixture_retained_turn`] (a kernel turn refuses a context its retained input does not name); its sequence and progress cells are leaked, which costs a few bytes per call.
pub fn fixture_step_context() -> semio_framework_job::StepContext<'static> {
    fixture_step_context_for(fixture_retained_turn())
}

/// 🧮️ [`fixture_step_context`] for any retained turn input: the context names exactly that input's operation, generation and grant, so a fixture that funds a turn with a narrower grant (or none) hands the kernel a context that agrees.
pub fn fixture_step_context_for(turn: semio_framework::kernel::RetainedTurnInput) -> semio_framework_job::StepContext<'static> {
    let sequence: &'static mut u64 = Box::leak(Box::new(0));
    let progress: &'static mut semio_framework_value::RetainedCloneProgress = Box::leak(Box::default());
    semio_framework_job::StepContext::new(semio_framework_job::OperationId(turn.operation), semio_framework_job::Generation(turn.generation), semio_framework_job::StepBudget::new(1_000_000, u64::MAX, turn.grant), semio_framework_job::root_cancel_token(), || Some(0), sequence, progress)
}

/// 🌀️ Polls one app's pending effects under a fresh fixture step context, as the refresh poll lane does.
pub async fn fixture_pending_effects<P: PluginApp>(app: &mut P, view: Option<&semio_framework::ViewModel>) -> Result<Vec<semio_framework::kernel::Effect>, semio_framework::Fault> {
    let mut cx = fixture_step_context();
    app.pending_effects(view, &mut cx).await
}

/// 📒️ One leaked retained-progress cell for a hand-built `StepContext`.
pub fn fixture_progress() -> &'static mut semio_framework_value::RetainedCloneProgress {
    Box::leak(Box::default())
}

/// 🤝️ Resolves a payload-free fixture job outcome (yield, cancelled or empty completion) from its descriptor.
pub fn fixture_job_outcome<'a>(descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
    match descriptor.kind() {
        semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
        semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
        semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
        _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "fixture job lends no payload outcome")),
    }
}

/// 📬️ Everything a fixture test reads from one lent job outcome; payload bytes are copied out, the payload itself stays in the job's custody.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FixtureStepOutcome {
    Idle,
    Yield,
    Preview(Vec<u8>),
    Checkpoint { applied_progress: u64, state: Vec<u8> },
    Complete { state: Vec<u8>, output: Vec<u8> },
    Cancelled,
    Fault(Vec<u8>),
}

/// 📖️ Concatenates every page of one retained payload.
pub fn fixture_payload_bytes(payload: &semio_framework_job_payload::RetainedJobPayload) -> Vec<u8> {
    (0..payload.page_count()).filter_map(|index| payload.page(index)).flatten().copied().collect()
}

/// ▶️ Drives one job turn and summarises the lent outcome so the borrow ends before the test touches the job again.
pub fn fixture_step<J: semio_framework_job::InteractiveJob + ?Sized>(job: &mut J, cx: &mut semio_framework_job::StepContext<'_>) -> FixtureStepOutcome {
    use semio_framework_job::JobOutcomeBorrow as B;
    match job.step(cx).expect("fixture job step") {
        None => FixtureStepOutcome::Idle,
        Some(B::Yield { .. }) => FixtureStepOutcome::Yield,
        Some(B::PreviewReady { payload, .. }) => FixtureStepOutcome::Preview(fixture_payload_bytes(payload)),
        Some(B::CheckpointReady { state, applied_progress, .. }) => FixtureStepOutcome::Checkpoint { applied_progress, state: fixture_payload_bytes(state) },
        Some(B::Complete { state, output, .. }) => FixtureStepOutcome::Complete { state: state.map(fixture_payload_bytes).unwrap_or_default(), output: output.map(fixture_payload_bytes).unwrap_or_default() },
        Some(B::Cancelled { .. }) => FixtureStepOutcome::Cancelled,
        Some(B::Fault { detail, .. }) => FixtureStepOutcome::Fault(fixture_payload_bytes(detail)),
    }
}

/// 🪪️ One local native authoring identity for a fixture call: `&mut fixture_identity()` lives for the whole statement, awaits included.
/// Its no-op observer is a zero-sized closure, so leaking it allocates nothing.
pub fn fixture_identity() -> semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'static> {
    let observer: &'static mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'static> = Box::leak(Box::new(|_: semio_framework_value::native_encoding::NativeEncodeProgress| true));
    semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1usize << 24, observer).expect("fixture authoring identity admission")
}

/// 🛬️ The archive operation a law's whole-document load runs under: bit 61 alone, below the restore (62) and cold-pair
/// (63) namespaces.
pub const LAW_DOCUMENT_LOAD_OPERATION: u64 = 1 << 61;

/// 🛬️ Loads `files` into `app` the only way a whole document loads (`📓️api-stepped-document-load.md` §4): the pair is
/// stamped with the live store's identity — exactly what the runtime does to an `Effect::LoadDocument` — admitted as a
/// whole-document archive load (`members: []`), polled to its terminal state over real polls and acknowledged. A load
/// that does not land answers its own fault.
pub async fn load_document<A, M>(app: &mut VcsArtifactApp<A, M>, files: &store::os_store::serialization::ArtifactPackFiles, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure>
where
    A: ArtifactApp,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    let archive = PluginApp::document_load_archive(app, files)?;
    PluginApp::begin_document_archive_load(app, LAW_DOCUMENT_LOAD_OPERATION, archive)?;
    let mut polls = 0usize;
    let status = loop {
        if cx.should_yield(){return Err(semio_framework::Fault::from("document archive requires its next original source turn").into())}cx.consume_fuel(1);let grant=cx.retained_grant();PluginApp::maintenance_step(app,cx,grant)?;
        let status = PluginApp::poll_document_archive_load(app, LAW_DOCUMENT_LOAD_OPERATION, identity,cx).await?;
        if !law_load_running(&status, &mut polls)? {
            break status;
        }
    };
    PluginApp::acknowledge_document_archive_load(app, LAW_DOCUMENT_LOAD_OPERATION)?;
    law_load_outcome(status).map_err(Into::into)
}

/// 🏗️ Drives one live instance's metered store constructor through the runtime exchange the reactor uses, so a fixture that
/// loads a document straight after opening the instance finds every store owner catalog installed.
pub async fn plugin_settle_store_constructor<PA: PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, instance_id: u32, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while semio_framework_plugin_runtime_ops::ops_document::plugin_store_constructor_pending(runtime, instance_id).await? {
        if std::time::Instant::now() >= deadline { return Err(semio_framework::Fault::from("the instance store constructor did not settle").into()); }
        if cx.should_yield(){return Err(semio_framework::Fault::from("store constructor requires its next original source turn").into())}cx.consume_fuel(1);
        semio_framework_plugin_runtime_exchange::exch_plugin_exchange::plugin_exchange(runtime, instance_id, None, identity, cx).await?;
    }
    Ok(())
}

/// 🛬️ [`load_document`] for one instance of a live runtime: the archive comes from the instance's own store stamp and goes
/// through the runtime's archive-load entry points — admit, poll to terminal, acknowledge — the path a checkpoint restore
/// takes.
pub async fn plugin_load_document<PA: PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, instance_id: u32, files: &store::os_store::serialization::ArtifactPackFiles, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure> {
    plugin_settle_store_constructor(runtime, instance_id, identity,cx).await?;
    let archive = semio_framework_plugin_runtime_ops::ops_document::plugin_document_load_archive(runtime, instance_id, files).await?;
    semio_framework_plugin_runtime_ops::ops_document::plugin_begin_document_archive_load(runtime, instance_id, LAW_DOCUMENT_LOAD_OPERATION, archive).await?;
    let mut polls = 0usize;
    let status = loop {
        if cx.should_yield(){return Err(semio_framework::Fault::from("runtime archive requires its next original source turn").into())}cx.consume_fuel(1);semio_framework_plugin_runtime_exchange::exch_plugin_exchange::plugin_exchange(runtime,instance_id,None,identity,cx).await?;
        let status = semio_framework_plugin_runtime_ops::ops_document::plugin_poll_document_archive_load(runtime, instance_id, LAW_DOCUMENT_LOAD_OPERATION, identity,cx).await?;
        if !law_load_running(&status, &mut polls)? {
            break status;
        }
    };
    semio_framework_plugin_runtime_ops::ops_document::plugin_acknowledge_document_archive_load(runtime, instance_id, LAW_DOCUMENT_LOAD_OPERATION).await?;
    law_load_outcome(status).map_err(Into::into)
}

/// 📜️ [`plugin_load_document`] for a text pair typed by `S`/`Mutation`: parsed and printed to the pack and SPR the archive
/// load takes, like [`load_document_text`].
pub async fn plugin_load_document_text<PA, S, Mutation>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, instance_id: u32, files: &store::os_store::text_format::ArtifactTextFiles, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure>
where
    PA: PluginApp,
    S: Clone + pack::snapshot::ArtifactDsl + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
    Mutation: store::OpText + store::OpBinary + store::Mutation<S> + store::ColdMutation<S> + 'static,
{
    let parsed: store::os_store::text_format::ParsedDocumentText<S, Mutation> = store::os_store::text_format::parse_document_text(&files.dsl, &files.ops, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(semio_framework_diagnostic::FaultFrom::into_fault)?;
    let envelope = parsed.into_envelope();
    let pack = store::os_store::text_format::print_document_pack(&envelope).await;
    envelope.retire_unadopted();
    plugin_load_document(runtime, instance_id, &pack.map_err(semio_framework_diagnostic::FaultFrom::into_fault)?, identity,cx).await
}

/// ⏳️ Whether a polled law load still runs; a load that never reaches its terminal state within a million polls is a fault.
fn law_load_running(status: &protocol::DocumentArchiveLoadStatus, polls: &mut usize) -> Result<bool, semio_framework::Fault> {
    if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
        return Ok(false);
    }
    *polls += 1;
    if *polls >= 1_000_000 {
        return Err(semio_framework::Fault::from("the document archive load never reached a terminal state"));
    }
    Ok(true)
}

/// 🏁️ A law load's acknowledged terminal: `Ready` loaded, `Cancelled` and `Fault` answer a fault (the archive's own on `Fault`).
fn law_load_outcome(status: protocol::DocumentArchiveLoadStatus) -> Result<(), semio_framework::Fault> {
    match status.state {
        protocol::DocumentArchiveLoadState::Ready => Ok(()),
        protocol::DocumentArchiveLoadState::Cancelled => Err(semio_framework::Fault::from("the document archive load was cancelled")),
        _ => Err(semio_framework_diagnostic::decode_fault_bytes(&status.fault)),
    }
}

/// 📜️ [`load_document`] for a text pair: the DSL and op log parse into the envelope whose pack and SPR the archive load
/// takes, through the same printers `document_pack` uses; the parsed envelope is retired through its exact owners, never
/// dropped.
pub async fn load_document_text<A, M>(app: &mut VcsArtifactApp<A, M>, files: &store::os_store::text_format::ArtifactTextFiles, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_framework_job::OriginalTurnFailure>
where
    A: ArtifactApp,
    A::Snapshot: pack::snapshot::ArtifactDsl,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    let parsed: store::os_store::text_format::ParsedDocumentText<A::Snapshot, A::Mutation> = store::os_store::text_format::parse_document_text(&files.dsl, &files.ops, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(semio_framework_diagnostic::FaultFrom::into_fault)?;
    let envelope = parsed.into_envelope();
    let pack = store::os_store::text_format::print_document_pack(&envelope).await;
    envelope.retire_unadopted();
    load_document(app, &pack.map_err(semio_framework_diagnostic::FaultFrom::into_fault)?, identity,cx).await
}

/// ⌨️ One long typing run (fixture `🧫️fixtures/⌨️typing-run/🔣️.json`: ≥ 1000 typed characters with pauses, caret moves back
/// into the run and Backspace/Delete corrections) as the full texts a text-editor host delivers, one per changed key.
pub struct TypingRun {
    pub initial: String,
    pub texts: Vec<String>,
    pub expected: String,
}

/// ⌨️ Replays the typing-run fixture under the caret-relative text-input model (typing inserts at the caret,
/// Backspace/Delete remove one character, arrows move one character, Home/End reach the line edges, Enter inserts a line
/// break) — the model the editor's own text-input law and Chromium's native textarea answer identically.
pub fn typing_run() -> TypingRun {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/⌨️typing-run/🔣️.json")).expect("typing-run fixture");
    let initial = fixture["initial"].as_str().expect("typing-run initial").to_string();
    let mut text: Vec<char> = initial.chars().collect();
    let mut caret = text.len();
    let mut texts = Vec::new();
    for key in fixture["keys"].as_array().expect("typing-run keys") {
        let before = text.clone();
        match key.as_str().expect("typing-run key") {
            "pause" => {}
            "Backspace" => {
                if caret > 0 {
                    caret -= 1;
                    text.remove(caret);
                }
            }
            "Delete" => {
                if caret < text.len() {
                    text.remove(caret);
                }
            }
            "ArrowLeft" => caret = caret.saturating_sub(1),
            "ArrowRight" => caret = (caret + 1).min(text.len()),
            "Home" => caret = text[..caret].iter().rposition(|ch| *ch == '\n').map_or(0, |index| index + 1),
            "End" => caret = text[caret..].iter().position(|ch| *ch == '\n').map_or(text.len(), |index| caret + index),
            "Enter" => {
                text.insert(caret, '\n');
                caret += 1;
            }
            typed => {
                for ch in typed.chars() {
                    text.insert(caret, ch);
                    caret += 1;
                }
            }
        }
        if text != before {
            texts.push(text.iter().collect());
        }
    }
    let expected = fixture["expect"]["text"].as_str().expect("typing-run expected text").to_string();
    assert_eq!(texts.last(), Some(&expected), "the typing-run model reaches the fixture's expected text");
    TypingRun { initial, texts, expected }
}

/// 🎟️ The fixture's original policy admits each quoted demand without creating new authority.
fn fixture_lifecycle_grant(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    let grant = fixture_mounted_policy().maintenance;
    assert!(grant.maximum_items > 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth, "fixture demand exceeds its original mounted policy");
    grant
}

/// 🌡️ Spends the maintenance the plugin runtime spends on a pressured instance: while a displaced-owner queue of the
/// document or a config-lane store sits at or above `store::os_store::snapshot_read::ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`, or a child
/// retirement registry at or above `MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY`, the runtime drains it in
/// one-item steps of the turn's own retained grant within the same turn. A law that commits faster
/// than the fair rotation retires (a typing run: every keystroke amends one edit and displaces its envelope, snapshot
/// and dag — and a content-addressed child's root and member) calls this after each settled command, as the live host does.
pub fn drain_maintenance_pressure<P: PluginApp>(app: &mut P,cx:&mut semio_framework_job::StepContext<'_>) {
    let mut idle_stages = 0;
    while app.maintenance_under_pressure() {
        let grant = fixture_lifecycle_grant(PluginApp::maintenance_retirement_demands(app, fixture_mounted_policy().maintenance.maximum_copy_bytes).expect("quoted maintenance demand"));
        if cx.should_yield(){return}
        let demand=PluginApp::maintenance_retirement_demands(app,grant.maximum_copy_bytes).expect("original maintenance quote");
        match semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,loan|Ok((PluginApp::maintenance_step(app,same,loan)?,PluginApp::maintenance_terminal_is_empty(app)))).expect("original bounded maintenance") {
            semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) if progress != semio_framework_value::retained_clone::RetainedCloneProgress::default() => idle_stages = 0,
            _ => idle_stages += 1,
        }
        assert!(idle_stages <= usize::from(semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES), "a pressured displaced-owner queue made no maintenance progress over a whole stage rotation");
    }
}

/// 🧪️ Replays `seed_genesis_children`'s roster lookup over a surface's initial document without
/// constructing the app, so a bundle proves at test time what `with_registry_on_bus` would only
/// discover by panicking inside a live guest. The probe snapshot is never dropped: retiring one
/// needs the owner factories a live `ArtifactStore` installs, and this law runs once per surface in
/// a test binary, so it hands the value to `ManuallyDrop` rather than trip the retirement assert.
fn genesis_children_declared<S, M>(snapshot: S, derives: impl Fn(&S, &str, &str) -> Result<bool, semio_framework_value::ValueError>) -> Result<usize, semio_framework::Fault>
where
    S: semio_framework_schema_composition::ArtifactCompositionFields,
    M: store::MemberFactory,
{
    let snapshot = std::mem::ManuallyDrop::new(snapshot);
    let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(&*snapshot).map_err(|error| semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault(format!("initial snapshot child projection failed: {error}")))?;
    let mut derived = 0;
    for index in 0..projection.len() {
        let Some((slot, fields)) = projection.get(index) else { break };
        if !derives(&snapshot as &S, slot, fields.child_id).map_err(|error| semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault(error.into_message()))? {
            continue;
        }
        let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
        semio_framework_plugin_instance_children::children_instance_root::genesis_member_schema::<M>(&dialect)?;
        derived += 1;
    }
    Ok(derived)
}

/// ✏️🌱️ Proves this editor's OWN roster (`E::Members`) declares every dialect its initial document
/// derives a genesis child for, and answers how many it checked — a caller pins that count so a
/// surface that stops deriving its children cannot turn the assertion vacuous. Any bundle
/// registering `E` instantiates that same roster, so one assertion covers every bundle.
pub fn assert_editor_genesis_children_declared<E: semio_framework_plugin_decl_surface::ArtifactEditor>() -> Result<usize, semio_framework::Fault> {
    genesis_children_declared::<E::Snapshot, E::Members>(E::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births"), |snapshot, slot, child_id| E::genesis_child_pack(snapshot, slot, child_id).map(|value| value.is_some()))
}

/// 👁️🌱️ Viewer twin of `assert_editor_genesis_children_declared`.
pub fn assert_viewer_genesis_children_declared<V: semio_framework_plugin_decl_surface::ArtifactViewer>() -> Result<usize, semio_framework::Fault> {
    genesis_children_declared::<V::Snapshot, V::Members>(V::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births"), |snapshot, slot, child_id| V::genesis_child_pack(snapshot, slot, child_id).map(|value| value.is_some()))
}

    /// 📨️ Projects authored external unit and data variants before their fixture action enters the original source.

    fn original_fixture_command_arguments(command:&impl semio_framework_value::ToValue)->Result<Option<semio_framework_value::DslValue>,semio_framework_value::ValueError>{

        match semio_framework_value::ToValue::to_value(command){semio_framework_value::DslValue::String(_)=>Ok(None),semio_framework_value::DslValue::Object(mut rows)if rows.len()==1=>Ok(Some(rows.pop().unwrap().1)),_=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"typed fixture command requires its authored external variant"))}

    }



    #[cfg(test)]

    #[test]

    fn original_fixture_command_projection_matches_the_closed_unit_and_data_cases(){

        #[derive(semio_framework_value_derive::ToValue)]

        enum Command{Noop,Set{value:u64}}

        let law:serde_json::Value=serde_json::from_str(include_str!("../../🎟️caller/🧪️tests/🧫️fixtures/📨️command.json")).unwrap();

        for row in law["cases"].as_array().unwrap(){let command=match row["variant"].as_str().unwrap(){"Noop"=>Command::Noop,"Set"=>Command::Set{value:row["value"].as_u64().unwrap()},_=>panic!("closed fixture command variant")};let actual=original_fixture_command_arguments(&command).unwrap();if row["variant"]=="Noop"{assert!(actual.is_none());assert!(row["expectedArgs"].is_null());}else{assert_eq!(actual,Some(semio_framework_value::DslValue::object([("value".to_owned(),semio_framework_value::ToValue::to_value(&row["expectedArgs"]["value"].as_u64().unwrap()))])));}}

    }



    /// 🎟️ Projects a cold typed fixture input into the actual retained action ingress under its persistent original caller.

    pub async fn dispatch_original_fixture_command<A,M>(app:&mut VcsArtifactApp<A,M>,command:&A::Command,meta:&ActionMeta,mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework::kernel::InvocationResult,semio_framework_job::OriginalTurnFailure>

    where A:ArtifactApp,A::Command:semio_framework_value::ToValue,M:store::ReferenceSpaceMember+store::MemberFactory+Send+'static{

        let action=A::command_id(command).await;

        let args=original_fixture_command_arguments(command)?;

        dispatch_fixture_action(app,action,args,meta,identity,cx,mounted_policy.maintenance).await

    }

    /// 🧳️ A fixture job retains one original consuming decoder and closes it under its caller's supplied grant.

    pub struct FixtureOriginalJob {

        decoder: semio_framework_job::OriginalJobDecodeOwner,

        cancel: Option<semio_framework_job::CancelToken>,

        payload: semio_framework_job_payload::JobPayloadAuthority,

        turn: semio_framework::kernel::RetainedTurnInput,

        sequence: u64,

        progress: semio_framework_value::RetainedCloneProgress,

    }



    impl FixtureOriginalJob {

        /// 🎟️ Captures an explicit original input without allocating decoder capacity.

        pub fn new(turn: semio_framework::kernel::RetainedTurnInput) -> Self {

            let ((cancel,progress),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::CancelToken::admit_root(turn.grant).expect("fixture original cancellation admission").expect("fixture original input funds cancellation"));

            assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));

            assert!(progress.fits(turn.grant));

            let ((payload,progress),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job_payload::JobPayloadAuthority::admit(semio_framework_job::OperationId(turn.operation),semio_framework_job::Generation(turn.generation),turn.grant).expect("fixture original payload admission").expect("fixture original input funds payload"));

            assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));

            assert!(progress.fits(turn.grant));

            Self { decoder: semio_framework_job::OriginalJobDecodeOwner::empty(),cancel:Some(cancel),payload,turn, sequence: 0, progress: Default::default() }

        }



        /// 🫴️ Reborrows the same issuer for one context carrying the caller's exact turn grant.

        pub fn context(&mut self, grant: RetainedCloneGrant) -> semio_framework_job::StepContext<'_> {

            self.progress = Default::default();

            semio_framework_job::StepContext::with_payload_authority(semio_framework_job::OperationId(self.turn.operation), semio_framework_job::Generation(self.turn.generation), semio_framework_job::StepBudget::new(1_000_000, u64::MAX, grant), self.cancel.as_ref().expect("fixture original cancellation retained"), || Some(0), &mut self.sequence, &mut self.progress,&self.payload).expect("fixture original payload loan").with_original_decoder(&mut self.decoder)

        }



        /// 🛬️ Loans original receiving to a synchronous semantic unit through its real context recipient.

        pub fn receive<T>(&mut self, grant: RetainedCloneGrant, operation: impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> T) -> T {

            self.context(grant).with_native_receiving(|receiving| Ok::<_, semio_framework_value::ValueError>(operation(receiving))).expect("fixture original receiving loan")

        }



        /// ♻️ Pays every original receiving retirement action with the supplied fixture input.

        pub fn close(&mut self) {

            for _ in 0..100_000 {

                if self.decoder.terminal_is_empty() { break; }

                let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.decoder.close_step(self.turn.grant).expect("fixture original decoder paid closure"));

                let progress = match step { semio_framework_value::RetainedCloneStep::Progress(progress) | semio_framework_value::RetainedCloneStep::Complete(progress) => progress };

                assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));

                assert!(progress.fits(self.turn.grant), "fixture receiving closure exceeded its original input");

                assert_ne!(progress, semio_framework_value::RetainedCloneProgress::default(), "fixture receiving closure stalled under its original input");

            }

            assert!(self.decoder.terminal_is_empty(),"fixture original receiving retirement did not finish");

            for _ in 0..100_000 {

                if self.payload.terminal_is_empty() { break; }

                let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.payload.close_step(self.turn.grant).expect("fixture original payload paid closure"));let progress=step.progress();

                assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));

                assert!(progress.fits(self.turn.grant));

                assert_ne!(progress,semio_framework_value::RetainedCloneProgress::default(),"fixture original payload closure stalled");

            }

            assert!(self.payload.terminal_is_empty());

            if let Some(cancel)=self.cancel.take() {

                let mut owner=semio_framework_async::CancelTokenRetirement::from_token(cancel);

                for _ in 0..100_000 {

                    if owner.terminal_is_empty() { break; }

                    let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(self.turn.grant).expect("fixture original cancellation paid closure"));let progress=step.progress();

                    assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));

                    assert!(progress.fits(self.turn.grant));

                    assert_ne!(progress,semio_framework_value::RetainedCloneProgress::default(),"fixture original cancellation closure stalled");

                }

                assert!(owner.terminal_is_empty());

            }

        }

    }



    impl Drop for FixtureOriginalJob {

        fn drop(&mut self) { if !std::thread::panicking() { self.close(); } }

    }
