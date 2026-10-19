//! 👯️ Paired apps, history and convergence laws: the paired apps, the history verb settlement, the undo redo round trip, the declared actions bridge, the two instance convergence, the ingest idempotency and the transaction laws. See [`super`].

use super::*;

/// 🔗️ Two registry-less store-only instances joined by an in-memory backbone on `channel`.
pub async fn paired_apps<A: ArtifactApp + Default>(channel: &str, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (VcsArtifactApp<A>, VcsArtifactApp<A>) {
    let mut a = new_app::<A>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
    let mut b = new_app::<A>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
    let (backbone_a, backbone_b) = MemoryBackbone::pair(channel, channel).await;
    a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).await.expect("attach a");
    b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).await.expect("attach b");
    (a, b)
}

/// 🧬️ Two explicitly registered fixture instances joined by an in-memory backbone, over the
/// default (empty) member roster — the `NoMembers` spelling of
/// [`paired_registered_apps_with_members`].
pub async fn paired_registered_apps<A, Manifest, Build>(channel: &str, manifest: Build, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (VcsArtifactApp<A>, VcsArtifactApp<A>)
where
    A: ArtifactApp + Default,
    Manifest: std::future::Future<Output = App>,
    Build: Fn() -> Manifest,
{
    paired_registered_apps_with_members::<A, store::NoMembers, Manifest, Build>(channel, manifest, mounted_policy, identity).await
}

/// 🧬️ [`paired_registered_apps`] over an explicit member roster `M` — see
/// [`new_registered_app_with_members`] for why a composing app needs it.
pub async fn paired_registered_apps_with_members<A, M, Manifest, Build>(channel: &str, manifest: Build, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> (VcsArtifactApp<A, M>, VcsArtifactApp<A, M>)
where
    A: ArtifactApp + Default,
    M: store::SpaceMember + store::MemberFactory + Send + 'static,
    Manifest: std::future::Future<Output = App>,
    Build: Fn() -> Manifest,
{
    let mut a = new_registered_app_with_members::<A, M, _>(manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
    let mut b = new_registered_app_with_members::<A, M, _>(manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
    a.bind_instance_id(meta("local").instance_id).await;
    b.bind_instance_id(meta("local").instance_id).await;
    let (backbone_a, backbone_b) = MemoryBackbone::pair(channel, channel).await;
    a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).await.expect("attach a");
    b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).await.expect("attach b");
    (a, b)
}

/// ⏪️ Drives ONE framework-reserved history verb the whole way a shell does: `handle_action`
/// only ADMITS `"undo"`/`"redo"` — they are `framework_reserved_job!` routes
/// (`FrameworkUndoJob`/`FrameworkRedoJob`), so the admission comes back as an
/// `Effect::SpawnJob { kind: FRAMEWORK_RESERVED_JOB_KIND }` the caller has to run before the
/// store ever sees `ArtifactCommand::Undo`. Dropping that receipt and pumping only the typed
/// publication left every retained app's undo a no-op, which read as a flaky
/// `undo did not revert to the expected snapshot` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
/// A non-retained app admits no spawn job and
/// [`super::settle_framework_reserved_admission`] hands the same result straight back.
pub async fn settle_history_verb<A, M>(app: &mut VcsArtifactApp<A, M>, action: &str, receiver: u32, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)
where
    A: ArtifactApp,
    M: store::SpaceMember + store::MemberFactory + Send + 'static,
{
    let admitted = dispatch_fixture_action(app,action,None,&meta("local"),identity,cx,mounted_policy.close).await.unwrap_or_else(|fault| panic!("history verb {action} admission: {fault:?}"));
    semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(app, admitted).await.unwrap_or_else(|fault| panic!("history verb {action} reserved-job commit: {fault:?}"));
    settle_registered_typed_operation(app, receiver, mounted_policy, identity,cx).await.unwrap_or_else(|fault| panic!("history verb {action} publication: {fault:?}"));
}

/// 🧪️ Runs `command` once via the typed channel, asserts `probe(app)` matches `after`, undoes (still
/// the framework-reserved `"undo"` action) and asserts `before`, redoes and asserts `after` again — the
/// repeated undo/redo round-trip test body. B1: takes a typed `A::Command` value (`dispatch_typed`) —
/// `ArtifactApp::handle_action`'s stringly-typed dispatch no longer exists.
///
/// 🔁️ Every step SETTLES: a retained tool's `dispatch_typed` only returns an admission
/// receipt, so probing straight after it observes the pre-command document and the
/// round trip passes or fails against the wrong state entirely. A non-retained app has
/// nothing pending and settles in zero turns (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub async fn assert_undo_redo_round_trip<A, M, P>(app: &mut VcsArtifactApp<A, M>, command: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P, before: P, after: P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)
where
    A::Command:semio_framework_value::ToValue,
    A: ArtifactApp,
    M: store::SpaceMember + store::MemberFactory + Send + 'static,
    P: PartialEq + std::fmt::Debug,
{
    let receiver = meta("local").instance_id;
    dispatch_original_fixture_command(app,&command,&meta("local"),mounted_policy,identity,cx).await.expect("apply command");
    settle_registered_typed_operation(app, receiver, mounted_policy, identity,cx).await.expect("apply command publication");
    assert_eq!(probe(app), after, "command did not produce the expected snapshot");
    settle_history_verb(app, "undo", receiver, mounted_policy, identity,cx).await;
    assert_eq!(probe(app), before, "undo did not revert to the expected snapshot");
    settle_history_verb(app, "redo", receiver, mounted_policy, identity,cx).await;
    assert_eq!(probe(app), after, "redo did not reapply the expected snapshot");
}

/// 🧪️ Every declared app action bridges through `command_from_action` and round-trips `command_id`
/// with its declared defaults and schema-validated representatives for required inputs without defaults.
///
/// 🧱️ Window-KIT catalog rows are framework-owned surface verbs, not app actions: the shared
/// `window_kind_definition` scaffold below mints them (and stamps them `Migrated`) for the
/// editable variants of `TextWindowKit`/`TableWindowKit`/`TreeWindowKit`, so every app that
/// composes one of those kits inherits an id it never authored a command row for. They belong
/// in this skip list for exactly the same reason the framework-reserved verbs above do.
pub async fn assert_declared_actions_bridge_to_commands<A: ArtifactApp + Default>(manifest: fn() -> App) {
    let definition = manifest().definition;
    let _app = A::default();
    let skip = [
        "undo",
        "redo",
        "commitCheckpoint",
        "createAlternative",
        "switchAlternative",
        "checkoutCheckpoint",
        "copy",
        "cut",
        "paste",
        "revertToCommand",
        "setHistoryCommandFilter",
        "noteShellCommand",
        "recordTutorial",
        "startIntroduction",
        "startTutorial",
        "setActiveUtility",
        "setActiveTool",
        semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID,
        semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID,
        "interactionSelect",
        "interactionHover",
        "clearSelection",
        "selectAll",
        "setSelectionMode",
        "setInteractionGranularity",
    ];
    for action in definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)) {
        if skip.contains(&action.id.as_str())
            || semio_framework_plugin_job::is_tool_run_action_id(&action.id)
            || semio_framework_plugin_decl_app::is_time_travel_action_id(&action.id)
            || action.id == semio_framework_plugin_operation::CANCEL_TYPED_OPERATION_ACTION_ID
            || action.id == semio_framework::HOST_EVENT_ACTION_ID
        {
            continue;
        }
        if action.id == semio_framework_plugin_window_kits::text_kit::TextWindowKit::EDIT_ACTION_ID || matches!(action.id.as_str(), "set-cell" | "set-node") {
            continue;
        }
        let staged = declared_bridge_arguments(&action.args).unwrap_or_else(|error| panic!("action {} could not stage required arguments: {error}", action.id));
        let mut original_args=Some(staged.clone());let mut original_fault=None;
        let command = A::command_from_action(&action.id, &mut original_args, &mut original_fault).await.unwrap_or_else(|error| panic!("action {} failed to bridge: {}", action.id, error.message));
        assert_eq!(A::command_id(&command).await, action.id.as_str(), "command_id mismatch for action {}", action.id);
    }
}

/// 🧪️ `command_a`/`command_b` are applied to two `paired_apps` instances; each side then folds the
/// other's events, one side commits a checkpoint (a structural history event) and the other folds
/// it too — `probe` must agree after every exchange. The repeated two-instance-convergence test body
/// (see `playbook-plugin`'s `two_instances_converge_disjoint_edits_via_backbone` for the original,
/// app-specific version).
pub async fn assert_two_instances_converge<A, P>(channel: &str, command_a: A::Command, command_b: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
where
    A::Command:semio_framework_value::ToValue,
    A: ArtifactApp + Default,
    P: PartialEq + std::fmt::Debug,
{
    let mut original_caller=OriginalFixtureCaller::admit().unwrap();let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let (mut instance_a, mut instance_b) = paired_apps::<A>(channel, mounted_policy, identity).await;
    let genesis = probe(&instance_a);
    dispatch_original_fixture_command(&mut instance_a,&command_a,&meta("actor-a"),mounted_policy,identity,cx).await.expect("a applies its edit");
        dispatch_original_fixture_command(&mut instance_b,&command_b,&meta("actor-b"),mounted_policy,identity,cx).await.expect("b applies its edit");
    exchange_and_assert_convergence(&mut instance_a, &mut instance_b, "actor-a", &probe, mounted_policy, identity,cx).await;
    assert_ne!(probe(&instance_a), genesis, "the replicated edits must actually land, not converge on the untouched genesis");

    close_registered_fixture_app(&mut instance_a,mounted_policy,cx).expect("original first app close");close_registered_fixture_app(&mut instance_b,mounted_policy,cx).expect("original second app close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).unwrap();result
}

/// 🧬️ Registered twin of [`assert_two_instances_converge`]: every typed command is settled until its
/// retained publication retires, so both edits really land before the replicas exchange events.
/// The `NoMembers` spelling of [`assert_two_registered_instances_converge_with_members`].
pub async fn assert_two_registered_instances_converge<A, P, Manifest, Build>(channel: &str, manifest: Build, command_a: A::Command, command_b: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
where
    A: ArtifactApp + Default,
    P: PartialEq + std::fmt::Debug,
    Manifest: std::future::Future<Output = App>,
    Build: Fn() -> Manifest,
{
    assert_two_registered_instances_converge_with_members::<A, store::NoMembers, P, Manifest, Build>(channel, manifest, command_a, command_b, probe, mounted_policy, identity).await;
}

/// 🧬️ [`assert_two_registered_instances_converge`] over an explicit member roster `M` — see
/// [`new_registered_app_with_members`] for why a composing app needs it.
pub async fn assert_two_registered_instances_converge_with_members<A, M, P, Manifest, Build>(channel: &str, manifest: Build, command_a: A::Command, command_b: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
where
    A::Command:semio_framework_value::ToValue,
    A: ArtifactApp + Default,
    M: store::SpaceMember + store::MemberFactory + Send + 'static,
    P: PartialEq + std::fmt::Debug,
    Manifest: std::future::Future<Output = App>,
    Build: Fn() -> Manifest,
{
    let mut original_caller=OriginalFixtureCaller::admit().expect("closed original fixture caller admission");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=async{
    let (mut instance_a, mut instance_b) = paired_registered_apps_with_members::<A, M, Manifest, Build>(channel, manifest, mounted_policy, identity).await;
    let genesis = probe(&instance_a);
    let receiver = meta("actor-a").instance_id;
    instance_a.dispatch_typed(command_a, &meta("actor-a"), identity,cx).await.expect("a applies its edit");
    settle_registered_typed_operation(&mut instance_a, receiver, mounted_policy, identity,cx).await.expect("a's edit publishes");
    instance_b.dispatch_typed(command_b, &meta("actor-b"), identity,cx).await.expect("b applies its edit");
    settle_registered_typed_operation(&mut instance_b, receiver, mounted_policy, identity,cx).await.expect("b's edit publishes");
    exchange_and_assert_convergence(&mut instance_a, &mut instance_b, "actor-a", &probe, mounted_policy, identity,cx).await;
    assert_ne!(probe(&instance_a), genesis, "the replicated edits must actually land, not converge on the untouched genesis");
    instance_a.detach_backbone().await.expect("a releases its backbone");
    instance_b.detach_backbone().await.expect("b releases its backbone");
    close_registered_fixture_app(&mut instance_a, mounted_policy,cx).expect("registered fixture original close");
    close_registered_fixture_app(&mut instance_b, mounted_policy,cx).expect("registered fixture original close");

    }.await;
    let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original fixture caller paid terminal retirement");
    result
}

/// 🔀️ Both replicas fold each other's events, `a` commits a checkpoint over everything it holds, `b`
/// folds that commit — `probe` must agree after each exchange: edits and structural history alike
/// converge by projection of the shared event log.
async fn exchange_and_assert_convergence<A, M, P>(instance_a: &mut VcsArtifactApp<A, M>, instance_b: &mut VcsArtifactApp<A, M>, actor_a: &str, probe: &impl Fn(&VcsArtifactApp<A, M>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)
where
    A: ArtifactApp,
    M: store::SpaceMember + store::MemberFactory + Send + 'static,
    P: PartialEq + std::fmt::Debug,
{
    instance_a.tick_backbone(identity,cx).await.expect("a folds b's events");
    instance_b.tick_backbone(identity,cx).await.expect("b folds a's events");
    assert_eq!(probe(instance_a), probe(instance_b), "both instances must converge on the same snapshot");
    let receiver = meta(actor_a).instance_id;
    let admitted = dispatch_fixture_action(instance_a,"commitCheckpoint",None,&meta(actor_a),identity,cx,mounted_policy.close).await.expect("a commits a checkpoint");
    semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(instance_a, admitted).await.expect("a's checkpoint commit settles");
    settle_registered_typed_operation(instance_a, receiver, mounted_policy, identity,cx).await.expect("a's checkpoint publication settles");
    instance_b.tick_backbone(identity,cx).await.expect("b folds a's checkpoint");
    assert_eq!(probe(instance_a), probe(instance_b), "a replicated checkpoint keeps both instances converged");
}

// 🪦️ `assert_graph_merge_preserves_referential_integrity` DELETED
// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` M1): zero callers
// repo-wide (confirmed via grep) — a generic testkit helper offered for app tests to probe
// `(snapshot, Vec<..>)`-of-free-form-diagnostics-shaped reconcile results, but no app's own test suite ever
// called it; the one real exercise of this exact scenario is
// `🖥️host/🦀️.rs`'s `concurrent_delete_and_wire_reconciles_without_a_dangling_edge`,
// written directly against `OsWorkflowStore`/`reconcile_workflow_snapshot` rather than through
// this helper. Dead code deleted rather than migrated to `Vec<protocol::MutationMessage>`.

/// 🧪️ Applies `command` on a sender attached to a backbone, replays the resulting envelopes onto a
/// fresh receiver twice, and asserts `probe` sees the same result both times — feeding the same operation
/// twice must not double-apply.
pub async fn assert_ingest_idempotent<A, P>(command: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
where
    A::Command:semio_framework_value::ToValue,
    A: ArtifactApp + Default,
    P: PartialEq + std::fmt::Debug,
{
    let mut original_caller=OriginalFixtureCaller::admit().expect("closed original ingestion fixture caller");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut sender = new_app::<A>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
    let (near, mut far) = MemoryBackbone::pair("mem://testkit-idempotent", "mem://testkit-idempotent").await;
    sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach sender");
    dispatch_original_fixture_command(&mut sender,&command,&meta("local"),mounted_policy,identity,cx).await.expect("apply command");

    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
        }
    }
    let operations = protocol::encode_envelopes(&envelopes);

    let mut receiver = new_app::<A>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
    receiver.ingest_operations(&operations, identity,cx).await.expect("ingest once");
    let once = probe(&receiver);
    receiver.ingest_operations(&operations, identity,cx).await.expect("ingest twice");
    assert_eq!(probe(&receiver), once, "feeding the same operation twice must not double-apply");
    sender.detach_backbone().await.expect("original sender detaches");close_registered_fixture_app(&mut sender,mounted_policy,cx).expect("original sender closes");close_registered_fixture_app(&mut receiver,mounted_policy,cx).expect("original receiver closes");
    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original ingestion caller closes");result
}

/// 🧬️ Registered twin of [`assert_ingest_idempotent`] — the `NoMembers` spelling of
/// [`assert_registered_ingest_idempotent_with_members`].
pub async fn assert_registered_ingest_idempotent<A, P, Manifest, Build>(manifest: Build, command: A::Command, probe: impl Fn(&VcsArtifactApp<A>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
where
    A: ArtifactApp + Default,
    P: PartialEq + std::fmt::Debug,
    Manifest: std::future::Future<Output = App>,
    Build: Fn() -> Manifest,
{
    assert_registered_ingest_idempotent_with_members::<A, store::NoMembers, P, Manifest, Build>(manifest, command, probe, mounted_policy, identity).await;
}

/// 🧬️ [`assert_registered_ingest_idempotent`] over an explicit member roster `M` — see
/// [`new_registered_app_with_members`] for why a composing app needs it.
pub async fn assert_registered_ingest_idempotent_with_members<A, M, P, Manifest, Build>(manifest: Build, command: A::Command, probe: impl Fn(&VcsArtifactApp<A, M>) -> P, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)
where
    A::Command:semio_framework_value::ToValue,
    A: ArtifactApp + Default,
    M: store::SpaceMember + store::MemberFactory + Send + 'static,
    P: PartialEq + std::fmt::Debug,
    Manifest: std::future::Future<Output = App>,
    Build: Fn() -> Manifest,
{
    let mut original_caller=OriginalFixtureCaller::admit().expect("closed original fixture caller admission");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=async{
    let mut sender = new_registered_app_with_members::<A, M, _>(manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
    sender.bind_instance_id(meta("local").instance_id).await;
    let (near, mut far) = MemoryBackbone::pair("mem://testkit-idempotent", "mem://testkit-idempotent").await;
    sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach sender");
    dispatch_original_fixture_command(&mut sender,&command,&meta("local"),mounted_policy,identity,cx).await.expect("apply command");
    settle_registered_typed_operation(&mut sender, meta("local").instance_id, mounted_policy, identity,cx).await.expect("the edit publishes");
    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
        }
    }
    assert!(!envelopes.is_empty(), "the applied edit must reach the channel as events");
    let operations = protocol::encode_envelopes(&envelopes);
    let mut receiver = new_registered_app_with_members::<A, M, _>(manifest(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await;
    receiver.bind_instance_id(meta("local").instance_id).await;
    let genesis = probe(&receiver);
    receiver.ingest_operations(&operations, identity,cx).await.expect("ingest once");
    let once = probe(&receiver);
    assert_ne!(once, genesis, "ingesting the events must materialize the edit");
    receiver.ingest_operations(&operations, identity,cx).await.expect("ingest twice");
    assert_eq!(probe(&receiver), once, "feeding the same operation twice must not double-apply");
    sender.detach_backbone().await.expect("sender releases its backbone");
    close_registered_fixture_app(&mut sender, mounted_policy,cx).expect("registered fixture original close");
    close_registered_fixture_app(&mut receiver, mounted_policy,cx).expect("registered fixture original close");

    }.await;
    let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original fixture caller paid terminal retirement");
    result
}

/// 🔀️ Dispatches `command` and asserts it produced a `TransactionProposalDraft` instead of
/// applying anything (contract §5.1) — the guest-side half of "propose instead of apply".
/// Returns the draft so a test can inspect `local_ops`/`foreign`.
pub  async fn assert_proposes_transaction<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, command: A::Command, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> semio_framework_plugin_job::TransactionProposalDraft where A::Command:semio_framework_value::ToValue {
    let generation_before = app.stores.store.generation();
    let edits_before = app.stores.store.envelope().vcs.edits.len();
    dispatch_original_fixture_command(app,&command,&meta("local"),mounted_policy,identity,cx).await.expect("dispatch a command whose mutations carry foreign steps");
    settle_registered_typed_operation(app, meta("local").instance_id, mounted_policy, identity,cx).await.expect("a proposing dispatch settles its own operation");
    assert_eq!(app.stores.store.generation(), generation_before, "a proposed transaction must not bump the document store's generation");
    assert_eq!(app.stores.store.envelope().vcs.edits.len(), edits_before, "a proposed transaction must not add an Edit");
    app.take_pending_transaction_proposal().await.expect("dispatch_emit must stash a TransactionProposalDraft when foreign steps are present")
}

/// 🔀️ Scripts a whole pre-planned-form transaction against ONE in-process member without a
/// host: prepares `ops` under `txn_id`/`origin`, asserts the prepare was accepted,
/// commits, and asserts exactly one resulting `Edit` whose `group_id` is `txn_id` and whose
/// every `MutationMeta.origin` is `origin`. Returns the committed edit id.
pub async fn assert_transaction_commits_as_one_edit<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, txn_id: &str, ops: Vec<A::Mutation>, origin: protocol::MutationOrigin, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> String {
    let edits_before = app.stores.store.envelope().vcs.edits.len();
    let mut prepared_ops: Vec<Vec<u8>> = Vec::with_capacity(ops.len());
    for op in ops.iter() {
        prepared_ops.push(::protocol::OpBinary::encode_op(op).expect("encode prepared op"));
    }
    let outcome = app.transaction_prepare(txn_id, "", &[], &prepared_ops, &[], Some(origin.clone())).await;
    assert!(outcome.rejection.is_none(), "prepare unexpectedly rejected: {:?}", outcome.rejection.as_ref().map(|fault| &fault.message));
    let edit_id = app.transaction_commit(txn_id, &meta("local"), identity).await.expect("commit prepared transaction");
    assert_eq!(app.stores.store.envelope().vcs.edits.len(), edits_before + 1, "commit must produce exactly one new Edit");
    let edit = app.stores.store.envelope().vcs.edits.last().expect("committed edit must exist");
    assert_eq!(edit.id, edit_id, "transaction_commit must return the tail edit's own id");
    assert!(!edit.mutation_meta.is_empty(), "the committed edit must carry at least one MutationMeta entry");
    for op_meta in &edit.mutation_meta {
        assert_eq!(op_meta.group_id.as_deref(), Some(txn_id), "every op in the committed edit must carry the transaction's group_id");
        assert_eq!(op_meta.origin, origin, "every op in the committed edit must carry the prepared origin");
    }
    edit_id
}

/// 🔀️ Prepares then rolls back a transaction, asserting the store's generation and edit
/// count are completely untouched (contract §5.5: rollback discards pending state without
/// ever having applied anything).
pub async fn assert_transaction_rollback_leaves_state_untouched<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, txn_id: &str, ops: Vec<A::Mutation>) {
    let generation_before = app.stores.store.generation();
    let edits_before = app.stores.store.envelope().vcs.edits.len();
    let mut prepared_ops: Vec<Vec<u8>> = Vec::with_capacity(ops.len());
    for op in ops.iter() {
        prepared_ops.push(::protocol::OpBinary::encode_op(op).expect("encode prepared op"));
    }
    let outcome = app.transaction_prepare(txn_id, "", &[], &prepared_ops, &[], Some(protocol::MutationOrigin::Owner)).await;
    assert!(outcome.rejection.is_none(), "prepare unexpectedly rejected");
    app.transaction_rollback(txn_id).await.expect("rollback the pending transaction");
    assert_eq!(app.stores.store.generation(), generation_before, "rollback must not change the store generation");
    assert_eq!(app.stores.store.envelope().vcs.edits.len(), edits_before, "rollback must not add any edit");
}
