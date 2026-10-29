//! 🏗️ Registered app construction and typed operation settlement: the reserved action factories, the app constructors and registries, the store constructor settlement, the typed operation receipts and settlement, the extension invocations and the registered app close. See [`super`].

use super::*;

/// 🧪️ Mounts the production shared reserved-action factories on an isolated harness bus.
pub fn register_framework_reserved_action_factories<A: ArtifactApp>(bus: &semio_framework::ActionBus, controller_id: &str) -> Result<(), semio_framework::Fault> {
    let mut registry = ArtifactToolFactoryRegistry::<A>::new(bus, controller_id);
    register_framework_reserved_factories(&mut registry)
}

/// 🪪️ A local-actor `ActionMeta` for test dispatch (`instance_id: 1`).
// 🚫️async: E1 pure constructor, called pervasively as `&meta("...")` — see R9.
pub fn meta(actor: &str) -> ActionMeta {
    ActionMeta { actor: actor.into(), instance_id: 1, view_state: None }
}

// 🏛️ 👁️✏️ W3 dissolution (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET): the two-
// generations-obsolete `assert_constitutional_crates`/`assert_taxonomy_components` gate (ticket
// 26/07/29/MOVE-APPS-INTO-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES) hunted for a literal
// `🎛️apps/` directory next to a plugin's manifest dir and hard-`panic!`ked when none was found.
// Repo-wide grep confirmed zero callers (not even `Plugin::builder`, despite its own docstring's
// stale claim) — dead code whose ONE reachable branch would now panic unconditionally, since
// `🎛️apps` can never exist again. Deleted rather than retargeted: the shape it enforced (an
// app-root ⚙️engine, `constitutional-crate` slots) is itself retired by the ENGINELESS ticket and
// by this ticket's surface dissolution, and the same completeness duty is covered live by the
// TS-side `validateTaxonomyTree` (📇️registry/📜️script.ts) and `policyTaxonomyDirsBreaches` (root
// 📜️script.ts), both already surface-aware.

/// 🧬️ A registry-less wrapper (`VcsArtifactApp::new`) for store-only tests. UI and typed
/// command dispatch fail closed until a real registry and exact factory are supplied.
pub async fn new_app<A: ArtifactApp + Default>(actor: protocol::ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<A> {
    let mut app = VcsArtifactApp::new(A::default(), actor, mounted_policy, identity).await.expect("the reference snapshot births");
    settle_store_constructor(&mut app, mounted_policy);
    app
}

/// 🧬️ A registry-backed wrapper carrying `manifest`'s real `AppActionRegistry` — needed whenever a
/// test must exercise declared-arg defaults/required-arg enforcement or View/Shell kind discipline.
pub async fn new_app_with_registry<A: ArtifactApp + Default>(manifest: fn() -> App, actor: protocol::ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<A> {
    new_app_with_registry_and_members::<A, store::NoMembers>(manifest, actor, mounted_policy, identity).await
}

/// 🧬️ `new_app_with_registry` over an explicit member roster `M` — for an app whose snapshot
/// composes children it derives (`ArtifactApp::genesis_child_pack`), which `NoMembers` can never open.
pub async fn new_app_with_registry_and_members<A: ArtifactApp + Default, M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static>(manifest: fn() -> App, actor: protocol::ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<A, M> {
    let definition = manifest().definition;
    let registry = AppActionRegistry::from_definition(&definition);
    drop(definition);
    let mut app = VcsArtifactApp::with_registry(A::default(), registry, actor, mounted_policy, identity).await.expect("the reference snapshot births");
    settle_store_constructor(&mut app, mounted_policy);
    app
}

/// 🧬️ A registry-backed wrapper whose manifest is authored asynchronously by the concrete fixture,
/// over the default (empty) member roster. The body lives in
/// [`new_registered_app_with_members`]; this is the `NoMembers` spelling of it.
pub async fn new_registered_app<A, Manifest>(manifest: Manifest, actor: protocol::ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<A>
where
    A: ArtifactApp + Default,
    Manifest: std::future::Future<Output = App>,
{
    new_registered_app_with_members::<A, store::NoMembers, Manifest>(manifest, actor, mounted_policy, identity).await
}

/// 🧬️ [`new_registered_app`] over an explicit member roster `M`. An app whose
/// `ArtifactApp::genesis_child_pack` mints a composed child cannot be constructed on the default
/// roster at all — its own construction fails `derived child dialect '…' is not declared by this
/// app's member roster` — so every registered law was unreachable for a composing app
/// (mathematical's equation and imperative's procedure, ticket
/// 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, knowledge). Rust has no default type parameter on
/// a function, so the roster-carrying generic is the twin and the plain name delegates to it.
///
/// 🪪️ A REGISTERED fixture app is by definition mounted: `dispatch_typed_command_inner`
/// refuses `interactive-job.live-instance` for any command whose `ActionMeta.instance_id`
/// is not the app's bound live runtime instance, and a fresh app has none. `meta` stamps 1,
/// so every caller of this helper gets the instance its own `meta(...)` addresses; a caller
/// that binds a different id afterwards simply overwrites this one.
pub async fn new_registered_app_with_members<A, M, Manifest>(manifest: Manifest, actor: protocol::ActorId, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<A, M>
where
    A: ArtifactApp + Default,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
    Manifest: std::future::Future<Output = App>,
{
    let definition = manifest.await.definition;
    let mut app = VcsArtifactApp::with_registry(A::default(), AppActionRegistry::from_definition(&definition), actor, mounted_policy, identity).await.expect("the reference snapshot births");
    app.bind_instance_id(meta("local").instance_id).await;
    settle_store_constructor(&mut app, mounted_policy);
    app
}

/// 🏗️ A `VcsArtifactApp` is born constructing: its store owner catalogs are installed by metered constructor turns
/// the reactor drives before it admits any command. A fixture that reaches the stores directly settles that
/// constructor first, claiming and validating every phase receipt exactly as the reactor's exchange does.
pub fn settle_store_constructor<A: ArtifactApp, M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1) -> Vec<TypedOperationFixtureOwnershipReceipt> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    let mut receipts = Vec::new();
    loop {
        retain_original_ownership_receipts(app, mounted_policy, &mut receipts).expect("original fixture constructor receipts");
        if !app.store_constructor_is_pending() { return receipts; }
        assert!(std::time::Instant::now() < deadline, "registered fixture store constructor did not settle");
        app.stores.drive_store_constructor_unit(&mut app.constructor, mounted_policy.preparation).expect("registered fixture store constructor turn");
    }
}

/// 🚰️ Drains a standalone Store's displaced-owner maintenance queue to terminal emptiness under its own quoted grant — the caller no scheduler funds — so a fixture that edits a long history frees the fixed 1 024-slot retirement authority between batches.
pub fn drain_store_displaced_retirements<P, Mutation>(store: &mut store::ArtifactStore<P, Mutation>)
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::Mutation<P> + store::ColdMutation<P> + store::OpBinary + store::OpText + Send + 'static,
{
    for _ in 0..1_000_000 {
        if store.maintenance_retirements_terminal_is_empty() { return; }
        let demand = store.maintenance_retirements_demands(4096).expect("original displaced-owner quote");
        store.maintenance_retirements_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(4096), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }).expect("original displaced-owner step");
    }
    panic!("the displaced-owner queue did not drain under its quoted grants");
}

/// 🧹 The close-side twin of [`settle_store_constructor`]: drives the constructor's own close phase to terminal emptiness, claiming every phase receipt, so the close quote that follows names the next application owner instead of the constructor.
pub fn settle_store_constructor_close<A: ArtifactApp, M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1) -> Vec<TypedOperationFixtureOwnershipReceipt> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    let mut receipts = Vec::new();
    loop {
        retain_original_ownership_receipts(app, mounted_policy, &mut receipts).expect("original fixture constructor close receipts");
        if app.constructor.constructor_terminal_is_empty() { return receipts; }
        assert!(std::time::Instant::now() < deadline, "registered fixture store constructor close did not settle");
        app.stores.close_store_constructor_unit(&mut app.constructor, mounted_policy.close).expect("registered fixture store constructor close turn");
    }
}

/// 📬️ Host-visible output collected while one registered fixture operation reaches quiescence.
/// 🧾️ Fixtures retain the original producer receipt under the caller's declared phase policy.
pub enum TypedOperationFixtureOwnershipReceipt {
    Constructor { kind: semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorKind, phase: semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase, grant: RetainedCloneGrant, progress: RetainedCloneProgress },
    Preparation { operation: semio_framework_job::OperationId, grant: RetainedCloneGrant, progress: RetainedCloneProgress },
    Retirement { operation: semio_framework_job::OperationId, grant: RetainedCloneGrant, progress: RetainedCloneProgress },
    History { grant: RetainedCloneGrant, progress: RetainedCloneProgress },
    ToolRun { grant: RetainedCloneGrant, progress: RetainedCloneProgress },
    LocalInteraction { grant: RetainedCloneGrant, progress: RetainedCloneProgress },
    Maintenance { grant: RetainedCloneGrant, progress: RetainedCloneProgress },
}

pub struct TypedOperationFixtureReceipt {
    pub ownership_receipts: Vec<TypedOperationFixtureOwnershipReceipt>,
    pub lanes: Vec<TypedOperationResultLane>,
    pub effects: Vec<semio_framework::kernel::Effect>,
    pub events: Vec<semio_framework::kernel::AppEvent>,
    pub ui_scope: Option<semio_framework::kernel::UiDirtyScope>,
    pub completions: usize,
    pub completion_operations: Vec<u64>,
    pub revisions: Vec<u64>,
    /// 🔁️ One entry per task follow-up this settle saw settled: its mounted command's receipt, or the fault that refused its delivery.
    pub resume_outcomes: Vec<Result<semio_framework::kernel::InvocationResult, semio_framework::Fault>>,
    /// 🧩️ One entry per composed gesture this settle observed — see
    /// [`crate::app::ComposedGestureResult`].
    pub composed: Vec<semio_framework_plugin_composition_publication::ComposedGestureResult>,
}

/// 🎚️ Projects current config from the concrete window registry, including its applied event history.
pub async fn capture_fixture_window_config<O: semio_framework_plugin_window_config::WindowConfigOwner, A: ArtifactApp, M: store::ReferenceSpaceMember + store::MemberFactory>(app: &mut VcsArtifactApp<A, M>, view: &semio_framework::ViewModel) -> Result<Option<O::State>, semio_framework::Fault> {
    Ok(app.stores.window_config_store.capture(Some(view)).await?.and_then(|authority| authority.snapshot.get::<O>().cloned()))
}

/// 🔁️ Drives the same bounded continuation and exact ACK protocol as the plugin host.
///
/// 📄️ EVERY presented page, not one per turn: `has_pending_typed_operations` counts the
/// effect/event/ui/completion outboxes and the mounted operations, but NOT a presented
/// page waiting for its ACK — so a turn that leaves a second page queued can be the turn
/// the loop exits on, and that page is never seen. A command that publishes an artifact,
/// a config and an app transient reported two of the three
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 🧩️ The composed lane rides the same bounded outbox and is counted by
/// `has_pending_typed_operations`, so leaving it undrained would spin this loop to its
/// 30 s deadline exactly the way the terminal witness once did.
///
/// 🧹️ The terminal witness lands in its OWN outbox (`typed_completion_outbox`), which
/// `has_pending_typed_operations` counts — leaving it undrained spun this loop until
/// its 30 s deadline for every operation that produced no lane page at all
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 📜️ A pending history patch (a time-travel stage change) rides a UI-progress frame the host takes every turn;
/// left untaken it kept `has_pending_typed_operations` true until the deadline for the first operation after a
/// history edit (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, forms frozen-press law).
pub async fn settle_registered_typed_operation<P: PluginApp>(app: &mut P, receiver: u32, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<TypedOperationFixtureReceipt, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let mut turns=0;let maximum_turns=OriginalFixtureCaller::maximum_turns();
    let mut receipt = TypedOperationFixtureReceipt { ownership_receipts: Vec::new(), lanes: Vec::new(), effects: Vec::new(), events: Vec::new(), ui_scope: None, completions: 0, completion_operations: Vec::new(), revisions: Vec::new(), resume_outcomes: Vec::new(), composed: Vec::new() };
    retain_registered_ownership_receipts(app, mounted_policy, &mut receipt)?;
    while app.has_pending_typed_operations() || !app.maintenance_terminal_is_empty() {
        if cx.should_yield()||turns>=maximum_turns{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original fixture settlement exhausted its caller opportunity").into())}turns+=1;
        let grant = mounted_policy.maintenance;
        let demand = app.maintenance_retirement_demands(grant.maximum_copy_bytes)?;
        let maintenance = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,grant| {
            let step = app.maintenance_step(same,grant)?;
            Ok((step, app.maintenance_terminal_is_empty()))
        })?;
        if let Some(progress) = maintenance.progress() { receipt.ownership_receipts.push(TypedOperationFixtureOwnershipReceipt::Maintenance { grant, progress }); }
        app.advance_typed_operation_publication(identity,cx).await?;
        retain_registered_ownership_receipts(app, mounted_policy, &mut receipt)?;
        while let Some(mut page) = crate::artifact_app_laws::take_fixture_result_page(app,receiver,mounted_policy.preparation).expect("original fixture page presentation") {
            let lane = page.lane;
            let fault = (lane == TypedOperationResultLane::Fault).then(|| semio_framework_plugin_decl_app::tool_contract::decode_typed_operation_fault_page(page.bytes()));
            if !app.acknowledge_typed_operation_result(page.token)? {
                return Err(semio_framework::Fault::from("registered fixture typed operation rejected its exact result ACK").into());
            }
            receipt.lanes.push(lane);
            if let Some(fault) = fault { return Err(fault.into()); }
        }
        while let Some(effect) = app.take_typed_operation_effect() {
            receipt.effects.push(effect);
        }
        while let Some(event) = app.take_typed_operation_event() {
            receipt.events.push(event);
        }
        while let Some(scope) = app.take_typed_operation_ui_scope() {
            receipt.ui_scope = Some(scope);
        }
        while let Some(progress) = app.take_typed_operation_ui_progress() {
            receipt.ui_scope = receipt.ui_scope.or(Some(progress.ui_scope));
        }
        while let Some(outcome) = app.take_task_resume_outcome() {
            receipt.resume_outcomes.push(outcome);
        }
        while let Some(composed) = app.take_typed_operation_composed_result() {
            receipt.composed.push(composed);
        }
        while let Some(completion) = app.take_typed_operation_completion().await? {
            receipt.completions += 1;
            receipt.completion_operations.push(completion.operation);
            receipt.revisions.push(completion.revision);
        }
        while let Some(reply) = app.take_local_interaction_query_reply() {
            if let protocol::LocalInteractionQueryReply::Page { page } = reply {
                let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
                if !app.acknowledge_local_interaction_query(&token) {
                    return Err(semio_framework::Fault::from("registered fixture typed operation rejected its exact local-interaction ACK").into());
                }
            }
        }
        std::thread::yield_now();
    }
    Ok(receipt)
}

fn retain_registered_ownership_receipts<P: PluginApp>(app: &mut P, policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, recipient: &mut TypedOperationFixtureReceipt) -> Result<(), semio_framework::Fault> {
    retain_original_ownership_receipts(app, policy, &mut recipient.ownership_receipts)
}

/// 🧾️ Collects original producer receipts once before any subsequent semantic fixture callback.
#[doc(hidden)]
pub fn retain_original_ownership_receipts<P: PluginApp>(app: &mut P, policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, recipient: &mut Vec<TypedOperationFixtureOwnershipReceipt>) -> Result<(), semio_framework::Fault> {
    if let Some((kind, phase, grant, progress)) = app.take_constructor_receipt() {
        let original = match phase { semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase::Preparation => policy.preparation, semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase::Close => fixture_close_grant(policy) };
        if grant != original || !progress.fits(grant) || grant.maximum_depth == 0 && progress != RetainedCloneProgress::default() { return Err(semio_framework::Fault::from("registered fixture constructor receipt differs from its original caller authority")); }
        recipient.push(TypedOperationFixtureOwnershipReceipt::Constructor { kind, phase, grant, progress });
    }
    if let Some((operation, grant, progress)) = app.take_typed_operation_preparation_receipt() {
        if grant != policy.preparation || !progress.fits(grant) || grant.maximum_depth == 0 && progress != RetainedCloneProgress::default() { return Err(semio_framework::Fault::from("registered fixture preparation receipt differs from its original caller authority")); }
        recipient.push(TypedOperationFixtureOwnershipReceipt::Preparation { operation, grant, progress });
    }
    if let Some((operation, grant, progress)) = app.take_typed_operation_retirement_receipt() {
        if grant != policy.close || !progress.fits(grant) || grant.maximum_depth == 0 && progress != RetainedCloneProgress::default() { return Err(semio_framework::Fault::from("registered fixture Close receipt differs from its original caller authority")); }
        recipient.push(TypedOperationFixtureOwnershipReceipt::Retirement { operation, grant, progress });
    }
    if let Some((grant, progress)) = app.take_history_command_receipt() {
        if grant != policy.maintenance || !progress.fits(grant) || grant.maximum_depth == 0 && progress != RetainedCloneProgress::default() { return Err(semio_framework::Fault::from("registered fixture history receipt differs from its original caller authority")); }
        recipient.push(TypedOperationFixtureOwnershipReceipt::History { grant, progress });
    }
    if let Some((grant, progress)) = app.take_tool_run_receipt() {
        if grant != policy.maintenance || !progress.fits(grant) || grant.maximum_depth == 0 && progress != RetainedCloneProgress::default() { return Err(semio_framework::Fault::from("registered fixture tool run receipt differs from its original caller authority")); }
        recipient.push(TypedOperationFixtureOwnershipReceipt::ToolRun { grant, progress });
    }
    if let Some((grant,progress))=app.take_local_interaction_query_receipt(){
        if grant!=policy.preparation||!progress.fits(grant){return Err(semio_framework::Fault::from("registered fixture interaction receipt differs from its original caller authority"))}
        recipient.push(TypedOperationFixtureOwnershipReceipt::LocalInteraction{grant,progress});
    }
    Ok(())
}

/// 🔁️ One minted extension invocation as a fixture observes it — the `Effect::InvokeExtension`
/// `reactor::poll` would have handed the shell.
pub struct PendingExtensionInvocation {
    pub extension_id: String,
    pub capability: String,
    pub request_json: String,
}

/// 🔁️ The HOST half of the extension round trip, for a fixture that drives a `PluginApp`
/// directly and therefore has no `reactor::poll` to carry it.
///
/// An app declares its request through `Emit::extension_invocations`; the SDK mints the
/// `req`, parks a continuation and queues one `Effect::InvokeExtension` on the actor's
/// outbound queue. In production the shell runs the capability and answers with
/// `Event::Completed`, which `poll` routes back into `plugin_dispatch_response_action`. This
/// helper performs exactly those three steps: it drains the queued invocations, asks
/// `serve` for each capability's bytes, and dispatches the recorded `response_action` — with
/// the ORIGINAL request's own correlation fields plus `ok`/`outputJson` — back into `app`
/// through the same `PluginApp::handle_command` seam, settling each redispatch. Returns how
/// many invocations were answered, so a caller can loop until the chain quiesces
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
/// 🔁️ What one settled round of extension answers produced: how many invocations were
/// answered, and every `Effect` the redispatched response actions emitted in turn — a
/// continuation that re-arms its own chain (e.g. `flowEvalResolve` → `flowEvalTick`) is only
/// observable to the caller through these (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub struct SettledExtensionInvocations {
    pub ownership_receipts: Vec<TypedOperationFixtureOwnershipReceipt>,
    pub answered: usize,
    pub effects: Vec<semio_framework::kernel::Effect>,
}

/// 📦️ Exactly what `runCapturedExtensionEffect` (`🏛️ShellHost/🟦️.tsx`) does with the
/// capability's own JSON answer before it crosses the ABI:
/// `encodePackValue(JSON.parse(outputJson))`. A fixture that handed the raw JSON
/// bytes over instead was green against a shape the shell never sends
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub async fn settle_extension_invocations<P: PluginApp>(
    app: &mut P,
    receiver: u32,
    action_meta: &ActionMeta,
    serve: &mut dyn FnMut(&PendingExtensionInvocation) -> Result<Vec<u8>, semio_framework::Fault>,
    mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,
    identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<SettledExtensionInvocations,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let mut settled = SettledExtensionInvocations { ownership_receipts: Vec::new(), answered: 0, effects: Vec::new() };
    for effect in semio_framework_plugin_reactor_tasks::tasks_extension_continuation::drain_queued_effects(receiver) {
        let semio_framework::kernel::Effect::InvokeExtension { req, extension_id, capability, request_json, .. } = effect else { continue };
        let pending = PendingExtensionInvocation { extension_id, capability, request_json };
        let outcome = serve(&pending).and_then(|json| {
            let text = String::from_utf8(json).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("extension.answer-not-json"), error.to_string()))?;
            let value = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject)
                .map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("extension.answer-not-json"), format!("the served capability answer is not JSON: {error}")))?;
            Ok(store::pack_rt::encode_wire_value(&value))
        });
        let Ok((instance, action, args, original_fault)) = semio_framework_plugin_reactor_tasks::tasks_extension_continuation::take_extension_response(req, outcome) else { continue };
        let meta = ActionMeta { instance_id: instance, ..action_meta.clone() };
        dispatch_fixture_response_action(app,&action,Some(args),original_fault,&mut Some(meta),identity,cx,mounted_policy.close).await?;
        let receipt = settle_registered_typed_operation(app, receiver, mounted_policy, identity,cx).await?;
        settled.ownership_receipts.extend(receipt.ownership_receipts);
        settled.effects.extend(receipt.effects);
        settled.answered += 1;
    }
    Ok(settled)
}

/// 🪪️ The close grant every fixture turn is driven under: exactly the caller's close policy, as a mounted runtime drives it.
#[doc(hidden)]
pub fn fixture_close_grant(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1) -> RetainedCloneGrant {
    mounted_policy.close
}

/// 🧹️ Closes one registered fixture through the exact retained app close state machine.
pub fn close_registered_fixture_app<A: PluginApp>(app: &mut A, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<TypedOperationFixtureOwnershipReceipt>,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
    let mut pending_authority = None;
    let mut last_quote = None;
    let mut close_turn = 0usize;let maximum_turns=OriginalFixtureCaller::maximum_turns();
    let mut ownership_receipts = Vec::new();
    retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts)?;
    while close_turn<maximum_turns {
        if app.close_terminal_is_empty() { return Ok(ownership_receipts); }
        if cx.retirement_should_yield(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original fixture close exhausted its caller opportunity").into())}
        let grant = fixture_close_grant(mounted_policy);
        let demand = app.close_retirement_demands(grant.maximum_copy_bytes)?;
        last_quote = Some((demand, grant));
        let step = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,grant| {
            Ok((app.close_step(same,grant)?, app.close_terminal_is_empty()))
        })?;
        retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts)?;
        match step {
            semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) | semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress) => {
                assert!(progress.fits(grant), "registered fixture close turn {close_turn} exceeded its exact physical grant");
                if matches!(step, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(_)) { break; }
                if progress == Default::default() { std::thread::yield_now(); }
            }
            semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::AwaitingInput { reason } | semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked { reason } => {
                pending_authority = Some(reason);
                std::thread::yield_now();
            }
        }
        close_turn = close_turn.saturating_add(1);
    }
    assert!(app.close_terminal_is_empty(), "registered fixture did not reach its exact terminal-empty witness{}{}", pending_authority.map(|reason| format!(", last pending close authority: {reason}")).unwrap_or_default(), last_quote.map(|(demand, grant)| format!(", last close quote {demand:?} against grant {grant:?} on rung {}", app.close_diagnostic())).unwrap_or_default());
    Ok(ownership_receipts)
}
