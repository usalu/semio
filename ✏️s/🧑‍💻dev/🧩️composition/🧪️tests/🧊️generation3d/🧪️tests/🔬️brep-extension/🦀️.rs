//! 🔁️ The `brep` extension, served IN-PROCESS — the host half of the preview round trip.
//!
//! `flowEvalTick` declares its geometry work through `Emit::extension_invocations`
//! (`✏️editor/🦀️.rs`'s `preview_tessellate_invocations`, `⏱️flow-eval-tick/🦀️.rs`); the SDK mints the
//! request id and queues one `Effect::InvokeExtension`, and the SHELL is what runs the capability
//! and answers with `Event::Completed`. A `--lib` fixture has no shell, so without this the tick
//! chain converges with every preview handle still pending and the preview paints nothing.
//!
//! The two handlers below are the SAME two expressions the packaged extension's guest bundle
//! declares (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs`'s `bundle()`), so this serves the real
//! kernel, not a stub (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

use semio_plugin::artifact_app_laws::{settle_extension_invocations, PendingExtensionInvocation, SettledExtensionInvocations};
use semio_plugin::{ActionMeta, Fault, FaultCode, FaultOrigin, PluginApp};

fn bad_request(capability: &str, detail: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::Plugin, FaultCode::new(format!("extension.{capability}.bad-request")), detail.into())
}

static GEOMETRY_CONTEXT: std::sync::Mutex<Option<semio_s_plugin_flow_brep::geometry_inference::GeometryInferenceContext>> = std::sync::Mutex::new(None);
static BREP_EVALUATION: std::sync::Mutex<Option<semio_os_flow::ExtensionEvaluationResources>> = std::sync::Mutex::new(None);
static MATH_CONTEXT: std::sync::Mutex<Option<semio_os_flow::ExtensionEvaluationResources>> = std::sync::Mutex::new(None);

/// 🎟️ Borrows the original packaged geometry execution owner across every fixture hop.
pub(crate) fn with_geometry_context<T>(operation: impl FnOnce(&semio_s_plugin_flow_brep::geometry_inference::GeometryInferenceContext) -> T) -> T {
    let mut owner = GEOMETRY_CONTEXT.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    operation(owner.get_or_insert_with(|| semio_s_plugin_flow_brep::geometry_inference::GeometryInferenceContext::new(crate::flow_operators::geometry_session().capture())))
}

/// 🧹️ Releases the existing fixture execution owners through their bounded close authority.
pub(crate) fn retire_execution_owners() {
    use semio_plugin::{ExtensionResourceOwner, PluginLifecycleStep};
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut geometry = GEOMETRY_CONTEXT.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(owner) = geometry.as_mut() {
        owner.begin_close();
        for _ in 0..1_000_000 {
            if matches!(owner.close_step(scope.cx()).expect("geometry fixture owner closure"), PluginLifecycleStep::Complete(_)) && owner.terminal_is_empty() { break; }
        }
        assert!(owner.terminal_is_empty(), "geometry fixture owner must retire before release");
        geometry.take();
    }
    for slot in [&BREP_EVALUATION, &MATH_CONTEXT] {
        let mut resources = slot.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(owner) = resources.as_mut() {
            owner.begin_close();
            for _ in 0..1_000_000 {
                if matches!(owner.close_step(scope.cx()).expect("evaluation fixture owner closure"), PluginLifecycleStep::Complete(_)) && owner.terminal_is_empty() { break; }
            }
            assert!(owner.terminal_is_empty(), "evaluation fixture owner must retire before release");
            resources.take();
        }
    }
}

/// 🧹️ Cancels only work in the original geometry evaluator’s own registry.
pub(crate) fn cancel_brep_evaluations(target: Option<(&str, u64)>) -> (usize, bool) {
    let resources = BREP_EVALUATION.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(registry) = resources.as_ref().and_then(semio_os_flow::ExtensionEvaluationResources::registry) else {
        return (0, false);
    };
    let retired = match target {
        Some((operator, hash)) => usize::from(semio_os_flow::cancel_evaluation(registry, operator, hash)),
        None => semio_os_flow::cancel_all_evaluations(registry),
    };
    (retired, semio_os_flow::evaluation_retirement_pending(registry))
}

/// 🔌️ Runs one declared capability exactly the way the packaged guest bundle's handler does.
///
/// 🪪️ `extension_id` is matched against the CONTRIBUTING PLUGIN's id, never the flow manifest's own
/// `math`/`brep` id — that is precisely the resolution `dispatchInvokeExtensionEffect`
/// (`🏛️ShellHost/🟦️.tsx`) performs against `handle.pluginId`, and a fixture that answered the flow id
/// instead would hide the exact miss that stalled the served app
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn serve(pending: &PendingExtensionInvocation) -> Result<Vec<u8>, Fault> {
    use crate::flow_operators::{BREP_EXTENSION_PLUGIN_ID, MATH_EXTENSION_PLUGIN_ID};
    match (pending.extension_id.as_str(), pending.capability.as_str()) {
        (BREP_EXTENSION_PLUGIN_ID | MATH_EXTENSION_PLUGIN_ID, "evaluate") => {
            let brep = pending.extension_id == BREP_EXTENSION_PLUGIN_ID;
            let mut slot = (if brep { &BREP_EVALUATION } else { &MATH_CONTEXT })
                .lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            let owner = slot.get_or_insert_with(|| {
                let registry = if brep {
                    semio_s_plugin_flow_brep::module_registry(crate::flow_operators::geometry_session())
                } else {
                    let mut registry = semio_os_flow::neural::Registry::new();
                    semio_s_plugin_flow_math::register(&mut registry);
                    registry
                };
                semio_os_flow::ExtensionEvaluationResources::new(registry)
            });
            crate::preview_reply::invoke_original(
                |cx| semio_os_flow::evaluate_invoke_json(owner, pending.request_json.as_bytes(), cx), |_| {},
            ).map_err(|failure| failure.primary_fault().cloned().unwrap_or_else(|| bad_request("evaluate", format!("{failure:?}"))))
        }
        (BREP_EXTENSION_PLUGIN_ID, "tessellate") => with_geometry_context(|owner|
            crate::preview_reply::invoke(owner, pending.request_json.as_bytes(), |_| {})
        ).map_err(|failure| failure.primary_fault().cloned().unwrap_or_else(|| bad_request("tessellate", format!("{failure:?}")))),
        (BREP_EXTENSION_PLUGIN_ID, "evaluateCancel") => {
            let request = semio_framework_pack_json::parse(&pending.request_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| bad_request("evaluateCancel", error.to_string()))?;
            let target = request.get("operatorId").and_then(semio_framework_pack_json::Value::as_str)
                .zip(request.get("nodeHash").and_then(semio_framework_pack_json::Value::as_u64));
            let (retired, pending) = cancel_brep_evaluations(target);
            let value = crate::preview_reply::dto(serde_json::json!({"ok":true,"retired":retired,"pending":pending}));
            Ok(semio_os_flow::os_pack::pack_rt::encode_wire_value(&value))
        }
        (BREP_EXTENSION_PLUGIN_ID, "tessellateCancel") => {
            let request = semio_framework_pack_json::parse(&pending.request_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| bad_request("tessellateCancel", error.to_string()))?;
            let retired = match (request.get("handle").and_then(semio_framework_pack_json::Value::as_str), request.get("tolerance").and_then(semio_framework_pack_json::Value::as_f64)) {
                (Some(handle), Some(tolerance)) => usize::from(crate::flow_operators::geometry_session().cancel_tessellation(handle, tolerance)),
                _ => crate::flow_operators::geometry_session().cancel_all_tessellations(),
            };
            let value = crate::preview_reply::dto(serde_json::json!({"ok":true,"retired":retired}));
            Ok(semio_os_flow::os_pack::pack_rt::encode_wire_value(&value))
        }
        (extension, capability) => Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.missing"), format!("no loaded plugin is addressed by '{extension}' (capability '{capability}')"))),
    }
}

/// 🔁️ Answers every invocation this app has queued, redispatching each response action back into it
/// under the CALLER'S OWN `ActionMeta` — the shell answers a continuation with its live view
/// attached, and the effects those response actions emit (a re-armed `flowEvalTick` among them) come
/// back to the caller instead of being dropped (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 🪟️ The view is not optional. `flowEvalResolve` and `flowTessellateResolve` are window-addressed
/// routes — they publish the wave they continue into the addressed preview window's transient — so a
/// fixture that answered them with a viewless meta would be modelling a shell that does not exist and
/// would fault `targeted window transient capture requires an exact ViewModel roster`.
pub async fn settle<P: PluginApp>(app: &mut P, receiver: u32, action_meta: &ActionMeta, cx: &mut semio_framework_job::StepContext<'_>) -> SettledExtensionInvocations {
    settle_extension_invocations(app, receiver, action_meta, &mut |pending| {
        let outcome = serve(pending);
        outcome
    }, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx)
    .await
    .expect("in-process brep extension round trip")
}

/// 🏛️ Redispatches one `Effect::DispatchAction` naming an app COMMAND exactly the way
/// `makeEffectDispatchOne` (`🛠️ShellHelpers/🟦️.tsx`) does: through the typed command channel with the
/// shell's own live view attached, never the scoped action channel.
pub async fn dispatch_effect_command<P: PluginApp>(app: &mut P, command_id: &str, args: Option<&semio_framework_value::DslValue>, action_meta: &ActionMeta, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
    use semio_framework::manifest::{CommandAddress, CommandOwnerAddress};
    let arguments = match args {
        Some(arguments @ semio_framework_value::DslValue::Object(_)) => Some(arguments.clone()),
        _ => None,
    };
    let app_id = app.app_id().await.to_string();
    let address = CommandAddress { owner: CommandOwnerAddress::App { plugin_id: String::new(), app_id }, command_id: command_id.to_string() };
    semio_plugin::artifact_app_laws::dispatch_fixture_command(app, &address, arguments, None, action_meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.map(|_| ()).map_err(|failure| Fault::from(format!("{failure:?}")))
}

/// 🧾️ What one driven `previewEval` run did: the hops it dispatched (and to which windows), the
/// extension answers served, the release hops a closed run owed, the run actions the host fed back and
/// the state the run settled in.
#[derive(Clone, Debug, Default)]
pub struct PreviewRunReceipt {
    pub hops: usize,
    pub hop_windows: Vec<String>,
    pub answered: usize,
    pub releases: usize,
    pub run_actions: Vec<String>,
    pub state: Option<String>,
}

/// 🔁️ The REAL served loop for EITHER surface, with nothing hand-addressed: `pending_effects` off the
/// host's attached-window roster, every framework-reserved run action fed back through
/// `PluginApp::handle_action`, every hop the run's port hands the host redispatched as the typed command
/// the shell sends, every extension invocation answered by `serve`. Stops once nothing is owed, pending
/// or outstanding and the run rests in a terminal or complete state.
pub async fn drive_preview_run<P: PluginApp>(app: &mut P, shell_view: &semio_plugin::ViewModel, initial: &[semio_plugin::Effect], serve: &mut dyn FnMut(&PendingExtensionInvocation) -> Result<Vec<u8>, Fault>, cx: &mut semio_framework_job::StepContext<'_>) -> PreviewRunReceipt {
    use semio_plugin::artifact_app_laws::{meta, settle_registered_typed_operation};
    use semio_plugin::Effect;
    let action_meta = ActionMeta { view_state: Some(shell_view.clone()), ..meta("local") };
    let mut receipt = PreviewRunReceipt::default();
    let mut effects: Vec<Effect> = initial.to_vec();
    for _ in 0..100_000 {
        effects.extend(app.pending_effects(Some(shell_view)).await);
        if app.has_pending_typed_operations() {
            effects.extend(settle_registered_typed_operation(app, action_meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx).await.expect("the run's driver turns settle").effects);
        }
        let settled = settle_extension_invocations(app, action_meta.instance_id, &action_meta, serve, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx).await.expect("in-process extension round trip");
        receipt.answered += settled.answered;
        effects.extend(settled.effects);
        let dispatches: Vec<(String, Option<semio_framework_value::DslValue>)> = std::mem::take(&mut effects)
            .into_iter()
            .filter_map(|effect| match effect {
                Effect::DispatchAction { action, args, .. } => Some((action, args)),
                _ => None,
            })
            .collect();
        receipt.state = app.tool_run_presence().map(|presence| presence.state.wire_name().to_string());
        if dispatches.is_empty() && settled.answered == 0 && !app.has_pending_typed_operations() && receipt.state.as_deref().is_none_or(|state| matches!(state, "complete" | "aborted" | "faulted" | "finalized")) {
            return receipt;
        }
        for (action, args) in dispatches {
            if semio_plugin::is_tool_run_action_id(&action) {
                receipt.run_actions.push(action.clone());
                semio_plugin::artifact_app_laws::dispatch_fixture_action(app, &action, args.clone(), &action_meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap_or_else(|fault| panic!("{action} dispatch: {fault:?}"));
                continue;
            }
            if action == "flowEvalTick" {
                receipt.hops += 1;
                receipt.hop_windows.push(args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str).unwrap_or_default().to_string());
            }
            receipt.releases += usize::from(action == "flowEvalRelease");
            dispatch_effect_command(app, &action, args.as_ref(), &action_meta, cx).await.unwrap_or_else(|fault| panic!("the shell redispatches the run's {action} hop: {fault:?}"));
            let hop = settle_registered_typed_operation(app, action_meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), cx).await.expect("retained publication");
            assert!(!hop.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Fault), "the run's {action} hop faulted in the retained job ladder: args={args:?}");
            effects.extend(hop.effects);
        }
    }
    panic!("the previewEval run did not settle within 100 000 host turns; last state {:?}", receipt.state);
}

/// 🚦️ The framework run actions a refresh over `shell_view` owes the `previewEval` run right now.
pub async fn owed_run_actions<P: PluginApp>(app: &mut P, shell_view: &semio_plugin::ViewModel) -> Vec<String> {
    run_actions(&app.pending_effects(Some(shell_view)).await)
}

/// 🚦️ The framework run actions an effect list carries.
pub fn run_actions(effects: &[semio_plugin::Effect]) -> Vec<String> {
    effects
        .iter()
        .filter_map(|effect| match effect {
            semio_plugin::Effect::DispatchAction { action, .. } if semio_plugin::is_tool_run_action_id(action) => Some(action.clone()),
            _ => None,
        })
        .collect()
}
