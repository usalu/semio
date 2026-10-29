//! 🔎️ Declared verb probing: the declared verb lanes, fingerprints, boot, state and dispatch, the registered app render, the addressed and agent previews and the declared verb probes with their honour assertion. See [`super`].

use super::*;

/// 🏷️ The stable lower-case name of one published result lane.
fn declared_verb_lane_name(lane: TypedOperationResultLane) -> &'static str {

    match lane {
        TypedOperationResultLane::Artifact => "artifact",
        TypedOperationResultLane::Config => "config",
        TypedOperationResultLane::Draft => "draft",
        TypedOperationResultLane::Presence => "presence",
        TypedOperationResultLane::Transient => "transient",
        TypedOperationResultLane::WindowConfig => "windowConfig",
        TypedOperationResultLane::WindowTransient => "windowTransient",
        TypedOperationResultLane::Child => "child",
        TypedOperationResultLane::Interaction => "interaction",
        TypedOperationResultLane::Effect => "effect",
        TypedOperationResultLane::Event => "event",
        TypedOperationResultLane::Ui => "ui",
        TypedOperationResultLane::Download => "download",
        TypedOperationResultLane::Terminal => "terminal",
        TypedOperationResultLane::Fault => "fault",
    TypedOperationResultLane::Diagnostic => "diagnostic",
        }
}

/// #️⃣ FNV-1a over every observed part of one settled dispatch.
fn declared_verb_fingerprint<'a>(parts: impl IntoIterator<Item = &'a [u8]>) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for part in parts {
        for byte in part.iter().chain(&[0xff]) {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0100_0000_01b3);
        }
    }
    hash
}

/// 🚪️ Infrastructure refusals (the verb never reached its handler) versus a handler's own refusal.
/// `interactive-job.app-owned-output` is the code a retained job's fault page carries when the
/// handler's own `Fault` crossed the publication boundary, so it is the handler's refusal.
fn declared_verb_refusal(failure:impl Into<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>)->DeclaredVerbOutcome{
    let failure=failure.into();let(code,detail)=match &failure.primary{semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Fault(fault)=>(fault.code.0.clone(),fault.message.clone()),semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Value(error)=>("interactive-job.owner-refused".to_string(),format!("{error:?}")),semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Domain(domain)=>(domain.code().to_owned(),domain.message().to_owned()),semio_framework_plugin_ownership::bounded_ownership::PluginLifecyclePrimary::Receiving(original)=>("interactive-job.receiving-refused".to_owned(),original.message().to_owned())};
    if(code.starts_with("interactive-job.")&&code!="interactive-job.app-owned-output")||code=="app.command.unsupported"{DeclaredVerbOutcome::Unreachable{code,detail,failure:Some(failure)}}else{DeclaredVerbOutcome::Refused{code,detail,failure:Some(failure)}}
}

/// 🎬️ How a fresh probe fixture boots the app's own example: its `setActiveExample` invocation (the
/// staged declared defaults overlaid by the surface's example arguments) from the window that
/// presents it — replayed on every fixture, so an example that lands as operations, as owned
/// children or as a host `LoadDocument` boots the same way the shell boots it.
pub struct DeclaredVerbBoot {
    pub args: semio_framework::DslValue,
    pub view: semio_framework::ViewModel,
}

/// 🎬️ The arguments the shell's example picker boots with: the staged declared defaults, and for every
/// closed choice still without a value its first option — the example the navbar opens first.
fn declared_verb_boot_args(action: &semio_framework::ActionDefinition, staged: semio_framework::DslValue) -> semio_framework::DslValue {
    use semio_framework::{ArgSchema, DslValue};
    let DslValue::Object(mut entries) = staged else { return staged };
    for argument in &action.args {
        let ArgSchema::String { options, .. } = &argument.schema else { continue };
        let Some(first) = options.first() else { continue };
        if !entries.iter().any(|(key, value)| key == &argument.id && !matches!(value, DslValue::Null) && value.as_str() != Some("")) {
            entries.retain(|(key, _)| key != &argument.id);
            entries.push((argument.id.clone(), DslValue::String(first.value.clone())));
        }
    }
    DslValue::Object(entries)
}

/// 🧫️ The per-probe fixture: the registered app, bound as instance 1, booted with the app's own
/// example when it declares one.
async fn declared_verb_fixture_app<A, M>(definition: &semio_framework::AppDefinition, boot: Option<&DeclaredVerbBoot>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> VcsArtifactApp<A, M>
where
    A: ArtifactApp + Default,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    let mut app = VcsArtifactApp::<A, M>::with_registry(A::default(), AppActionRegistry::from_definition(definition), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await.expect("the reference snapshot births");
    app.bind_instance_id(meta("local").instance_id).await;
    settle_store_constructor(&mut app, mounted_policy);
    if let Some(boot) = boot {
        let action_meta = ActionMeta { view_state: Some(boot.view.clone()), ..meta("local") };
        let result = dispatch_fixture_action(&mut app,"setActiveExample",Some(boot.args.clone()),&action_meta,identity,cx,mounted_policy.close).await.unwrap_or_else(|fault| panic!("the app's own boot example must be admitted: {fault:?}"));
        let receipt = settle_registered_typed_operation(&mut app, action_meta.instance_id, mounted_policy, identity,cx).await.unwrap_or_else(|fault| panic!("the app's own boot example must settle: {fault:?}"));
        for effect in result.requested_effects.into_iter().chain(receipt.effects) {
            if let semio_framework::kernel::Effect::LoadDocument { pack, spr } = effect {
                load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, identity,cx).await.unwrap_or_else(|fault| panic!("the app's own boot example must load: {fault:?}"));
            }
        }
        app.child_composition.follow_derivable_children(&mut app.stores, identity).await.unwrap_or_else(|fault| panic!("the boot example's derivable children must follow its coordinates: {fault:?}"));
    }
    app
}

/// 📦️ The document (with every owned child's envelope) and config packs the stores hold right now.
async fn declared_verb_state<A, M>(app: &mut VcsArtifactApp<A, M>) -> (Vec<u8>, Vec<u8>)
where
    A: ArtifactApp,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    let mut document = app.snapshot().map(|snapshot| pack::snapshot::ArtifactPack::encode_pack(&snapshot)).unwrap_or_default();
    for child in PluginApp::child_packs(app).await.unwrap_or_default() {
        document.extend_from_slice(child.slot.as_bytes());
        document.extend_from_slice(child.child_id.as_bytes());
        document.extend_from_slice(&child.envelope_pack);
    }
    let config = match app.render_ctx().refresh_cache().await {
        Ok(()) => app.history.cache.as_ref().map(|(_, _, config, _)| pack::snapshot::ArtifactPack::encode_pack(config.as_ref())).unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    (document, config)
}

/// 🪆️ `slot/child_id` of every composed child the parent names right now that no child store holds.
async fn declared_verb_unheld_children<A, M>(app: &VcsArtifactApp<A, M>) -> Vec<String>
where
    A: ArtifactApp,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    let Ok(snapshot) = app.snapshot() else { return Vec::new() };
    let declared: Vec<(String, String)> = match semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(&snapshot) {
        Ok(projection) => (0..projection.len()).filter_map(|index| projection.get(index)).map(|(slot, fields)| (slot.to_string(), fields.child_id.to_string())).collect(),
        Err(error) => vec![(String::from("projection"), error.to_string())],
    };
    let mut unheld = Vec::new();
    for (slot, child_id) in declared {
        if app.child_store(&slot, &child_id).await.is_none() {
            unheld.push(format!("{slot}/{child_id}"));
        }
    }
    unheld
}

/// 🎯️ Dispatches one declared verb exactly as the shell does and reads back what it did.
async fn declared_verb_dispatch<A, M>(definition: &semio_framework::AppDefinition, boot: Option<&DeclaredVerbBoot>, verb: &str, args: &semio_framework::DslValue, view: &semio_framework::ViewModel, body_key: &str, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> DeclaredVerbOutcome
where
    A: ArtifactApp + Default,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    let mut original_caller=OriginalFixtureCaller::admit().expect("closed original fixture caller admission");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=async{
    let mut app = declared_verb_fixture_app::<A, M>(definition, boot, mounted_policy, identity,cx).await;
    let (document_before, config_before) = declared_verb_state(&mut app).await;
    let unheld_before = declared_verb_unheld_children(&app).await;
    let action_meta = ActionMeta { view_state: Some(view.clone()), ..meta("local") };
    let receiver = action_meta.instance_id;
    let outcome = match dispatch_fixture_action(&mut app,verb,Some(args.clone()),&action_meta,identity,cx,mounted_policy.close).await {
        Err(fault) => declared_verb_refusal(fault),
        Ok(result) => match settle_registered_typed_operation(&mut app, receiver, mounted_policy, identity,cx).await {
            Err(fault) => declared_verb_refusal(fault),
            Ok(receipt) => {
                let effects = result.requested_effects.iter().chain(receipt.effects.iter()).map(|effect| format!("{effect:?}")).collect::<Vec<_>>();
                let events = result.events.iter().chain(receipt.events.iter()).map(|event| format!("{event:?}")).collect::<Vec<_>>();
                let replaced = result.requested_effects.iter().chain(receipt.effects.iter()).any(|effect| matches!(effect, semio_framework::kernel::Effect::LoadDocument { .. }));
                let downloaded = result
                    .requested_effects
                    .iter()
                    .chain(receipt.effects.iter())
                    .any(|effect| matches!(effect, semio_framework::kernel::Effect::DownloadMediaExport { .. } | semio_framework::kernel::Effect::IconRenderExport { .. } | semio_framework::kernel::Effect::VideoRenderExport { .. }));
                let lanes = receipt.lanes.iter().map(|lane| declared_verb_lane_name(*lane)).collect::<std::collections::BTreeSet<_>>();
                let (document_after, config_after) = declared_verb_state(&mut app).await;
                let orphaned_children = declared_verb_unheld_children(&app).await.into_iter().filter(|child| !unheld_before.contains(child)).collect();
                let rendered = match render_registered_fixture_app(&mut app,body_key,view,cx).await {
                    Ok(projected) => projected,
                    Err(failure)=>{
                        close_registered_fixture_app(&mut app,mounted_policy,cx).expect("original failed render fixture closes under its existing caller");
                        return declared_verb_refusal(failure);
                    }
                };
                let lane_text = lanes.iter().copied().collect::<Vec<_>>().join(",");
                let fingerprint = declared_verb_fingerprint(
                    [document_after.as_slice(), config_after.as_slice(), lane_text.as_bytes(), rendered.as_bytes()].into_iter().chain(effects.iter().map(|effect| effect.as_bytes())).chain(events.iter().map(|event| event.as_bytes())),
                );
                DeclaredVerbOutcome::Settled(DeclaredVerbEffect {
                    user_path_written: downloaded || lanes.contains("download"),
                    lanes,
                    document_changed: document_after != document_before,
                    document_replaced: replaced,
                    config_changed: config_after != config_before,
                    host_effects: effects.len() + events.len(),
                    fingerprint,
                    orphaned_children,
                })
            }
        },
    };
    close_registered_fixture_app(&mut app, mounted_policy,cx).expect("registered fixture original close");
    outcome

    }.await;
    let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original fixture caller paid terminal retirement");
    result
}

/// 🎬️ Projects the same stationary render source without donating its native resources to the fixture.
#[doc(hidden)]
pub async fn render_registered_fixture_app<A,M>(app:&mut VcsArtifactApp<A,M>,body_key:&str,view:&semio_framework::ViewModel,cx:&mut semio_framework_job::StepContext<'_>)->Result<String,semio_framework_job::OriginalTurnFailure>
where A:ArtifactApp+Default,M:store::ReferenceSpaceMember+store::MemberFactory+Send+'static{
    render_registered_fixture_app_over(app,body_key,None,view,cx).await
}

/// 🎬️ [`render_registered_fixture_app`] over an explicit snapshot override document, the way the host renders an embedded payload.
#[doc(hidden)]
pub async fn render_registered_fixture_app_over<A,M>(app:&mut VcsArtifactApp<A,M>,body_key:&str,snapshot_override_json:Option<&str>,view:&semio_framework::ViewModel,cx:&mut semio_framework_job::StepContext<'_>)->Result<String,semio_framework_job::OriginalTurnFailure>
where A:ArtifactApp+Default,M:store::ReferenceSpaceMember+store::MemberFactory+Send+'static{
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        if cx.should_yield(){break}
        if app.render(body_key,snapshot_override_json,view,cx).await?.is_some(){
            let source=app.original_render_tree_source(cx)?;
            let tree=source.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original fixture render readiness lost its stationary tree"))?;
            let mut remaining=FIXTURE_TREE_MAX_NODES;
            return project_fixture_node(&tree.root,&mut remaining,0).and_then(|value|serde_json::to_string(&value).map_err(|_|"projection-json")).map_err(|source|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,source).into())
        }
    }
    Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original registered fixture render remained pending under its authored caller").into())
}

/// 🔭️ Observes the stationary rendered tree of `body_key` in place: the same bounded render loop as [`render_registered_fixture_app`], handing the built root to `observe` while the app keeps owning the tree.
#[doc(hidden)]
pub async fn observe_registered_fixture_app<A,M,R>(app:&mut VcsArtifactApp<A,M>,body_key:&str,view:&semio_framework::ViewModel,cx:&mut semio_framework_job::StepContext<'_>,observe:impl FnOnce(&semio_framework_ui_contract::BuiltNode)->R)->Result<R,semio_framework_job::OriginalTurnFailure>
where A:ArtifactApp+Default,M:store::ReferenceSpaceMember+store::MemberFactory+Send+'static{
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        if cx.should_yield(){break}
        if app.render(body_key,None,view,cx).await?.is_some(){
            let source=app.original_render_tree_source(cx)?;
            let tree=source.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original fixture render readiness lost its stationary tree"))?;
            return Ok(observe(&tree.root))
        }
    }
    Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original registered fixture render remained pending under its authored caller").into())
}

/// 🫴️ Deferred addressed fixture dispatch keeps its actual input slots and initiating context over every step.
pub async fn dispatch_addressed_fixture_on_original<PA:semio_framework_plugin_reactor_app::PluginApp>(app:&mut PA,address:&semio_framework::manifest::ActionAddress,args:&mut Option<semio_framework_value::DslValue>,fault:&mut Option<semio_framework::Fault>,active_mode:Option<&str>,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework::InvocationResult,semio_framework_job::OriginalTurnFailure>{
    let maximum_turns=OriginalFixtureCaller::maximum_turns();let mut original_meta=Some(meta.clone());
    for _ in 0..maximum_turns{
        if cx.should_yield(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original addressed fixture exhausted its initiating work").into())}
        if let Some(result)=app.handle_action_invocation(address,args,fault,active_mode,&mut original_meta,identity,cx).await?{return Ok(result)}
        let grant=cx.retained_grant();app.maintenance_step(cx,grant)?;
    }
    Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original addressed fixture reached its finite step limit").into())
}

/// 🪜️ Keeps one admitted preview source under its initiating fixture context through pending phases.
async fn preview_fixture_on_original<A,M>(app:&mut VcsArtifactApp<A,M>,invocation:&semio_framework::manifest::ActionInvocation,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework::InvocationResult,semio_framework_job::OriginalTurnFailure>
where A:ArtifactApp,M:store::ReferenceSpaceMember+store::MemberFactory+Send+'static{
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        if cx.should_yield(){break}
        if let Some(result)=app.command_ctx().preview_addressed_action(invocation,meta,cx).await?{return Ok(result)}
    }
    Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original fixture preview did not settle under its supplied context").into())
}

/// 🤖️ Previews `verb` with `args` exactly as the MCP gateway's `action_prepare` does — the agent lane's typed command
/// frame, addressed to the caller's declared window instance with explicit language and terminology — and answers how many document ops
/// the preview would commit (ticket 26/09/23, G12 session 14c).
pub async fn agent_preview<A, M>(app: &mut VcsArtifactApp<A, M>, definition: &semio_framework::AppDefinition, verb: &str, args: &semio_framework::DslValue, view: &semio_framework::ViewModel, _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<usize, semio_framework_job::OriginalTurnFailure>
where
    A: ArtifactApp + Default,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    let window_kind_id = definition.window_kinds.iter().find(|window| semio_framework::window_kind_actions(definition, window).iter().any(|action| action.id == verb)).map_or_else(|| "*".to_string(), |window| window.id.clone());
    let window_instance_id = view.window_instances.iter().find(|window| window.window_kind_id == window_kind_id).ok_or_else(|| semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new("fixture.agent-window"), "agent fixture requires its caller's declared window instance"))?.id.clone();
    let arguments = match args {
        semio_framework::DslValue::Object(entries) => entries.iter().cloned().collect(),
        _ => std::collections::BTreeMap::new(),
    };
    let invocation = semio_framework::manifest::ActionInvocation {
        address: semio_framework::manifest::ActionAddress {
            plugin_id: String::new(),
            app_id: app.app.instance_id().await.to_string(),
            mode_id: definition.default_mode_id.clone(),
            window_kind_id,
            window_instance_id,
            action_id: verb.to_string(),
        },
        arguments,
    };
    let action_meta = ActionMeta { view_state: Some(view.clone()), ..meta("agent") };
    let result = preview_fixture_on_original(app,&invocation,&action_meta,cx).await?;
    Ok(result.output.get("documentOps").and_then(|value| value.as_str()).and_then(|value| value.parse::<usize>().ok()).unwrap_or(0))
}

/// 🤖️ Dispatches one declared verb exactly as an agent does — the owner-qualified invocation the
/// MCP gateway sends as a typed command frame (`dispatch_command_frame`, its `action_prepare`
/// phase) — and reads back what that phase reports it would do.
async fn declared_verb_agent_dispatch<A, M>(definition: &semio_framework::AppDefinition, boot: Option<&DeclaredVerbBoot>, verb: &str, args: &semio_framework::DslValue, window_kind_id: &str, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> DeclaredVerbOutcome
where
    A: ArtifactApp + Default,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    let mut original_caller=OriginalFixtureCaller::admit().expect("closed original fixture caller admission");
    let mut original_context=original_caller.context();let cx=&mut original_context;
    let result=async{
    let mut app = declared_verb_fixture_app::<A, M>(definition, boot, mounted_policy, identity,cx).await;
    let app_id = app.app.instance_id().await.to_string();
    let arguments = match args {
        semio_framework::DslValue::Object(entries) => entries.iter().cloned().collect(),
        _ => std::collections::BTreeMap::new(),
    };
    let invocation = semio_framework::manifest::ActionInvocation {
        address: semio_framework::manifest::ActionAddress {
            plugin_id: String::new(),
            app_id,
            mode_id: definition.default_mode_id.clone(),
            window_kind_id: window_kind_id.to_string(),
            window_instance_id: "declared-verb-agent".to_string(),
            action_id: verb.to_string(),
        },
        arguments,
    };
    let view = semio_framework::ViewModel { active_mode_id: Some(definition.default_mode_id.clone()), active_window_kind_id: Some(window_kind_id.to_string()), window_id: Some("declared-verb-agent".to_string()), window_instances: vec![semio_framework::ViewWindowInstance { id: "declared-verb-agent".to_string(), window_kind_id: window_kind_id.to_string() }], ..semio_framework::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let action_meta = ActionMeta { view_state: Some(view), ..meta("agent") };
    let outcome = match preview_fixture_on_original(&mut app,&invocation,&action_meta,cx).await {
        Err(fault) => declared_verb_refusal(fault),
        Ok(result) => {
            let count = |key: &str| result.output.get(key).and_then(|value| value.as_str()).and_then(|value| value.parse::<usize>().ok()).unwrap_or(0);
            let (document, config, draft, children) = (count("documentOps"), count("configOps"), count("draftOps"), count("childGroups"));
            let lanes = [("artifact", document), ("config", config), ("draft", draft), ("child", children)].into_iter().filter(|(_, ops)| *ops != 0).map(|(lane, _)| lane).collect::<std::collections::BTreeSet<_>>();
            let host_effects = result.requested_effects.len() + result.events.len();
            let fingerprint = declared_verb_fingerprint([format!("{:?}", result.output).as_bytes()]);
            DeclaredVerbOutcome::Settled(DeclaredVerbEffect { lanes, document_changed: document != 0, document_replaced: false, config_changed: config != 0, user_path_written: false, host_effects, fingerprint, orphaned_children: Vec::new() })
        }
    };
    close_registered_fixture_app(&mut app, mounted_policy,cx).expect("registered fixture original close");
    outcome

    }.await;
    let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original fixture caller paid terminal retirement");
    result
}

/// 🧪️ Probes every verb `definition` declares, on the app's own boot example: each window kind's
/// resolved actions (framework-reserved, tool-run and host-owned utility/tool verbs excluded) are
/// dispatched from an instance of that window kind with the staged declared defaults — overlaid by
/// the surface's example arguments, `examples` = `{"verbs": {"<verb>": {"<arg>": <value>}}}`, the
/// language-agnostic `🧫️fixtures/⚖️declared-verb-examples.json` of a surface whose verbs need
/// document ids to act — then with two distinct values per perturbable declared argument. Every
/// probe runs on a fresh registered app.
pub async fn probe_declared_verbs<A, M>(definition: fn() -> semio_framework::AppDefinition, examples: Option<&str>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<DeclaredVerbProbe>
where
    A: ArtifactApp + Default,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    probe_declared_verbs_where::<A, M>(definition, examples, None, mounted_policy, identity).await
}

/// 🤖️ The agent lane beside the shell lane for exactly the named `verbs`, on the app's own boot example: each is
/// dispatched with its staged declared defaults (overlaid by `examples`) from every window kind that presents it,
/// and once through the agent lane at the address the capability catalog publishes — without argument
/// perturbation, so a surface whose boot is expensive pins what an agent gets for the verbs it names.
/// [`declared_verb_agent_divergences`] over the answer lists the named verbs whose two lanes disagree.
pub async fn probe_agent_lane<A, M>(definition: fn() -> semio_framework::AppDefinition, examples: Option<&str>, verbs: &[&str], mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<DeclaredVerbProbe>
where
    A: ArtifactApp + Default,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    let probes = probe_declared_verbs_where::<A, M>(definition, examples, Some(verbs), mounted_policy, identity).await;
    let missing = verbs.iter().filter(|verb| !probes.iter().any(|probe| probe.verb == **verb)).collect::<Vec<_>>();
    assert!(missing.is_empty(), "the surface declares no verb {missing:?}");
    probes
}

/// 🧪️ [`probe_declared_verbs`] over every declared verb, or — `only` — the agent-lane probe over the named verbs.
async fn probe_declared_verbs_where<A, M>(definition: fn() -> semio_framework::AppDefinition, examples: Option<&str>, only: Option<&[&str]>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<DeclaredVerbProbe>
where
    A: ArtifactApp + Default,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    use semio_framework::{effective_action_args, resolve_audience, window_kind_actions, DslValue};
    let definition = definition();
    let instances = definition.window_kinds.iter().enumerate().map(|(index, window)| semio_framework::ViewWindowInstance { id: format!("declared-verb-probe-{index}"), window_kind_id: window.id.clone() }).collect::<Vec<_>>();
    let view_of = |index: usize| semio_framework::ViewModel {
        active_mode_id: Some(definition.default_mode_id.clone()),
        active_window_kind_id: Some(definition.window_kinds[index].id.clone()),
        window_id: Some(instances[index].id.clone()),
        window_instances: instances.clone(),
        ..semio_framework::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let examples: serde_json::Value = examples.map(|text| serde_json::from_str(text).expect("declared-verb examples are JSON")).unwrap_or(serde_json::Value::Null);
    let staged_of = |action: &semio_framework::ActionDefinition| {
        let example = examples["verbs"].get(&action.id).map(DslValue::from).unwrap_or(DslValue::Object(Vec::new()));
        effective_action_args(&action.args, &example, None)
    };
    let boot = definition.window_kinds.iter().enumerate().find_map(|(index, window)| {
        window_kind_actions(&definition, window).into_iter().find(|action| action.id == "setActiveExample").map(|action| DeclaredVerbBoot { args: declared_verb_boot_args(action, staged_of(action)), view: view_of(index) })
    });
    let mut order: Vec<(semio_framework::ActionDefinition, Vec<usize>)> = Vec::new();
    for (index, window) in definition.window_kinds.iter().enumerate() {
        for action in window_kind_actions(&definition, window) {
            if semio_framework_plugin_job::reserved_jobs::is_framework_reserved_action_id(&action.id) || matches!(action.id.as_str(), semio_framework::SET_ACTIVE_TOOL_ACTION_ID | semio_framework::SET_ACTIVE_UTILITY_ACTION_ID) {
                continue;
            }
            match order.iter_mut().find(|(known, _)| known.id == action.id) {
                Some((_, windows)) => windows.push(index),
                None => order.push((action.clone(), vec![index])),
            }
        }
    }
    let mut probes = Vec::new();
    for (action, windows) in &order {
        if only.is_some_and(|only| !only.contains(&action.id.as_str())) {
            continue;
        }
        let boot = if action.id == "setActiveExample" { None } else { boot.as_ref() };
        let audience = resolve_audience(action);
        let staged = staged_of(action);
        let mut original_args=Some(staged.clone());let mut original_fault=None;
        let bridge = A::command_from_action(&action.id, &mut original_args, &mut original_fault).await.map(drop).map_err(|fault| format!("{}: {}", fault.code.0, fault.message));
        let mut window_probes = Vec::new();
        for index in windows {
            let view = view_of(*index);
            let window = &definition.window_kinds[*index];
            let staged_outcome = declared_verb_dispatch::<A, M>(&definition, boot, &action.id, &staged, &view, &window.body_key, mounted_policy, identity).await;
            let mut arguments = Vec::new();
            if only.is_none() && audience != semio_framework::CapabilityAudience::Input && !matches!(staged_outcome, DeclaredVerbOutcome::Unreachable { .. }) {
                for argument in &action.args {
                    let Some((first, second)) = declared_argument_alternatives(argument) else { continue };
                    let with = |value: DslValue| {
                        let mut args = staged.clone();
                        if let DslValue::Object(entries) = &mut args {
                            entries.retain(|(key, _)| key != &argument.id);
                            entries.push((argument.id.clone(), value));
                        }
                        args
                    };
                    let others_specified = action.args.iter().filter(|other| other.id != argument.id).all(|other| staged.get(&other.id).is_some_and(|value| !matches!(value, DslValue::Null)));
                    let first = declared_verb_dispatch::<A, M>(&definition, boot, &action.id, &with(first), &view, &window.body_key, mounted_policy, identity).await;
                    let second = declared_verb_dispatch::<A, M>(&definition, boot, &action.id, &with(second), &view, &window.body_key, mounted_policy, identity).await;
                    arguments.push(DeclaredArgumentProbe { argument: argument.id.clone(), others_specified, first, second });
                }
            }
            window_probes.push(DeclaredVerbWindowProbe { window: window.id.clone(), staged: staged_outcome, arguments });
        }
        let agent = match audience {
            semio_framework::CapabilityAudience::Agent => {
                let claimed = definition.window_kinds.iter().find(|window| window.actions.iter().any(|claimed| claimed.id == action.id)).map_or("*", |window| window.id.as_str());
                Some(declared_verb_agent_dispatch::<A, M>(&definition, boot, &action.id, &staged, claimed, mounted_policy, identity).await)
            }
            _ => None,
        };
        probes.push(DeclaredVerbProbe { verb: action.id.clone(), kind: action.kind, audience, destructive: action.semantics.effects.destructive, bridge, windows: window_probes, agent });
    }
    probes
}

/// ⚖️ Every verb `definition` declares honours its declaration ([`declared_verb_findings`] over
/// [`probe_declared_verbs`], with the surface's optional example arguments). Answers the probes, so a
/// caller pins their count and the outcome of the verbs its examples make act — a surface that stops
/// declaring its verbs or whose examples stop acting cannot turn the law vacuous.
pub async fn assert_declared_verbs_honour_their_declarations<A, M>(definition: fn() -> semio_framework::AppDefinition, examples: Option<&str>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<DeclaredVerbProbe>
where
    A: ArtifactApp + Default,
    M: store::ReferenceSpaceMember + store::MemberFactory + Send + 'static,
{
    let probes = probe_declared_verbs::<A, M>(definition, examples, mounted_policy, identity).await;
    let findings = probes.iter().flat_map(declared_verb_findings).filter(|finding| !matches!(finding, DeclaredVerbFinding::AgentLaneDiverges { .. })).map(|finding| finding.to_string()).collect::<Vec<_>>();
    assert!(findings.is_empty(), "{} finding(s) over {} declared verb(s):\n{}", findings.len(), probes.len(), findings.join("\n"));
    probes
}
