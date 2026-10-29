/// 🎛️ Settle and dispatch drivers: the settled outcome, the segmented download drain, the history readers and every dispatch, selection and hover driver.

/// 📬️ Everything the HOST forwards to the shell once a dispatch has run to quiescence — the same
/// four channels `plugin_runtime` carries, so a law reads exactly what a client would.
///
/// 🧲️ [`Self::completions`] is the channel the browser reads as `AppFrame::OperationCompleted` and the
/// shell applies through `subscribeOperationCompletions` → `applyHostEffects`. It used to be drained
/// and DROPPED here, which is why a command that answers before it runs — every latest-wins verb
/// enters that channel and its accepted invocation returns
/// `{operationId, generation}` immediately — had no observable outcome in-process at all, and a
/// refusal on that lane could not be asserted (`📓️2026-09-09-wave-L-…md` §5.4).
#[derive(Debug, Default)]
pub struct Puzzle3dSettled {
    /// ⏱️ How many host continuation turns this settle spent. Ticket 26/09/02/PUZZLE-3D-END-TO-END
    /// wave B44: the observable a latency law reads, because per-command latency in the browser is
    /// turns × per-turn cost and a turn count that grows with the document is the shape that burns a
    /// 30-second interaction budget.
    pub turns: usize,
    /// 📊️ What those turns spent their publication units on, split by ladder. Ticket
    /// 26/09/02/PUZZLE-3D-END-TO-END wave B54: a turn now drives a bounded RUN of units, so `turns`
    /// alone can no longer say where a mutation's cost went and `turns` vs `census.units` is the
    /// wave's whole before/after ratio.
    pub census: semio_framework_plugin::app::TypedOperationUnitCensus,
    pub effects: Vec<Effect>,
    pub events: Vec<semio_framework_plugin::AppEvent>,
    pub scope: Option<UiDirtyScope>,
    pub completions: Vec<Puzzle3dCompletion>,
    pub downloads: Vec<Puzzle3dDownload>,
}

/// ⬇️ One segmented download handle a typed operation published — the metadata the host turns into
/// `download-media-export` with a `semio-segmented-handle-v1:` marker, plus the operation id the chunks
/// are drained by ([`drain_segmented_download`]).
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle3dDownload {
    pub operation: u64,
    pub filename: String,
    pub mime_type: String,
    pub encoding: Option<String>,
    pub bytes: usize,
}

impl Puzzle3dDownload {
    /// 📦️ The Download lane's wire shape is a flat 4-element JSON array
    /// (`DownloadResultPayload`'s hand-written `ToValue`), read here exactly as the renderer reads it.
    fn from_page(operation: u64, page: &[u8]) -> Self {
        let text = String::from_utf8_lossy(page).into_owned();
        let value: Value = parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| panic!("download result page must be JSON: {text}"));
        let row = value.as_array().unwrap_or_else(|| panic!("download result page must be a 4-element array: {text}"));
        Self {
            operation,
            filename: row.first().and_then(Value::as_str).unwrap_or_default().to_string(),
            mime_type: row.get(1).and_then(Value::as_str).unwrap_or_default().to_string(),
            encoding: row.get(2).and_then(Value::as_str).map(str::to_string),
            bytes: row.get(3).and_then(Value::as_u64).unwrap_or_default() as usize,
        }
    }
}

/// ⬇️ Drains one published segmented download the way the host's `drainSegmentedMediaExport` does: one
/// bounded chunk per await until the producer answers `None`, concatenated in order. The whole point of
/// the lane is that the payload never crosses as one contiguous block, so a law that asserts the
/// REASSEMBLED bytes has to drain it exactly like this.
pub async fn drain_segmented_download(app: &mut Puzzle3dApp, download: &Puzzle3dDownload) -> Vec<u8> {
    let mut assembled = Vec::with_capacity(download.bytes);
    let mut chunks = 0usize;
    while let Some(chunk) = app.take_segmented_download_chunk(download.operation).await.expect("take one segmented download chunk") {
        assert!(!chunk.is_empty(), "a segmented download chunk is never empty");
        assert!(chunk.len() <= semio_framework_plugin::app::ArtifactOutputChunks::CHUNK_BYTES, "chunk {} exceeds the wire's own page cap", chunks);
        assembled.extend_from_slice(&chunk);
        chunks += 1;
        assert!(chunks <= download.bytes / semio_framework_plugin::app::ArtifactOutputChunks::CHUNK_BYTES + 2, "segmented drain never terminated");
    }
    assembled
}

/// 📬️ One `AppFrame::OperationCompleted` witness, projected to what a law can assert on.
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle3dCompletion {
    pub operation: u64,
    pub ui_scope: UiDirtyScope,
    pub history_patch: Option<semio_framework::kernel::HistoryPatch>,
}

/// 📏️ One continuation turn's byte grant. Deliberately one fixed envelope page, not a huge number: the
/// point of the loop is that every unit is bounded exactly as a host actor tick bounds it.
const SETTLE_TURN_BYTES: usize = 16_384;

/// 🕹️ Plays the CLIENT half of the local interaction query protocol: drains the replies the app
/// published this turn and acknowledges every page, which is what returns the document snapshot-read
/// leases the query captured — the same Started → Page → acknowledge → Closed round trip
/// `local_interaction_query_return_does_not_fault_the_next_maintenance_step` drives by hand.
fn acknowledge_local_interaction_pages(app: &mut Puzzle3dApp) {
    while let Some(reply) = app.take_local_interaction_query_reply() {
        if let protocol::LocalInteractionQueryReply::Page { page } = reply {
            let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
            assert!(app.acknowledge_local_interaction_query(&token), "the app's own local-interaction page must accept its exact token");
        }
    }
}

async fn settle_into_reporting_with_items(app: &mut Puzzle3dApp, result: Result<InvocationResult, Fault>, maximum_items: usize) -> (Result<InvocationResult, Fault>, Puzzle3dSettled) {
    let settled = settle_with_items(app, maximum_items).await;
    let folded = result.map(|mut result| {
        result.requested_effects.extend(settled.effects.iter().cloned());
        result.events.extend(settled.events.iter().cloned());
        if let Some(scope) = settled.scope.clone().or_else(|| settled.completions.last().map(|completion| completion.ui_scope.clone())) {
            result.ui_scope = scope;
        }
        result
    });
    (folded, settled)
}

/// 🧾️ How many command-log rows one settle published on the completion lane — the `historyUpserts` a
/// browser client counts on its `AppFrame::OperationCompleted` frames.
pub fn history_rows(settled: &Puzzle3dSettled) -> usize {
    settled.completions.iter().filter_map(|completion| completion.history_patch.as_ref()).map(|patch| patch.upserts.len()).sum()
}

/// 🧾️ `(seq, action_id, kind, applied)` of every history row one settle recorded — what a row-count
/// assertion must print, or "2 vs 1" names neither the extra row nor the verb that wrote it.
pub fn history_row_labels(settled: &Puzzle3dSettled) -> Vec<String> {
    settled
        .completions
        .iter()
        .filter_map(|completion| completion.history_patch.as_ref())
        .flat_map(|patch| patch.upserts.iter())
        .map(|entry| format!("#{} {} kind={} applied={} ops={}", entry.seq, entry.action_id, entry.kind, entry.applied, entry.op_lines.len()))
        .collect()
}

/// 📂️ How many whole-document loads (`Effect::LoadDocument`) one settle published — the only shape a natural-file
/// import takes, since a load is a genesis of the document, never a history row.
pub fn loads_in(settled: &Puzzle3dSettled) -> usize {
    settled.effects.iter().filter(|effect| matches!(effect, Effect::LoadDocument { .. })).count()
}

/// 📂️ The projection of the one document a settle's `Effect::LoadDocument` carries, decoded from its pack bytes.
pub fn imported_projection(settled: &Puzzle3dSettled) -> Value {
    let pack = settled.effects.iter().find_map(|effect| match effect {
        Effect::LoadDocument { pack, .. } => Some(pack.clone()),
        _ => None,
    });
    let document = <Puzzle3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack.expect("an import publishes one whole-document load")).expect("the load carries a puzzle 3D document");
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&document))
}

/// 🧪️ B1: test-only replacement for the deleted `VcsArtifactApp::handle_action` app-dispatch path
/// (that method is FRAMEWORK-reserved now — an app's own actions go exclusively through the typed
/// `Self::Command` channel). Reconstructs the `Puzzle3dCommand` from the same
/// `(action, args, window_id)` triple every pre-B1 test already passed.
///
/// 🛰️ Wave B7: a framework-reserved verb is a TWO-half gesture since `dispatch_framework_reserved_action`
/// began answering every non-clipboard route with `Effect::SpawnJob { kind: FRAMEWORK_RESERVED_JOB_KIND }`.
/// `handle_action` only ADMITS it; the document changes when the host drives that Isolated job to a
/// terminal step and hands the bytes back through `complete_reserved_spawned_job`. A scene_snapshot that stopped
/// at the admission observed `undo`/`interactionSelect`/`interactionHover` succeeding while the store and
/// the interaction snapshot never moved — the exact silent no-op shape wave B5 reported. [`settle_reserved`]
/// plays that host half, and is a no-op for the clipboard routes that still commit inline.
pub async fn dispatch(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    dispatch_reporting(app, action, args, window_id).await.0
}

/// 🧾️ [`dispatch`] plus the settle census, for laws that must read the completion lane — above all
/// [`history_rows`], the command-log delta a MUTATING verb publishes on
/// `AppFrame::OperationCompleted` rather than on its accepted invocation. See
/// [`settle_into_reporting`] for why the two channels stay separate.
pub async fn dispatch_reporting(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> (Result<InvocationResult, Fault>, Puzzle3dSettled) {
    dispatch_reporting_with_items(app, action, args, window_id, 1).await
}

/// 📥️ [`dispatch_reporting`] through the SDK's own action dispatch (`handle_action`) instead of the typed channel —
/// the lane a shell's picked-file chunks take, where the framework's import staging admits them first.
pub async fn dispatch_action_reporting(app: &mut Puzzle3dApp, action: &str, args: &semio_framework_value::DslValue) -> (Result<InvocationResult, Fault>, Puzzle3dSettled) {
    let window_id = semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID;
    app.ensure_window(window_id);
    let action_meta = ActionMeta { view_state: Some(app.window_view(window_id)), ..app.meta("local") };
    let (raw, cx) = app.parts();
    let answered = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(raw, action, Some(args.clone()), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.map_err(fault_of);
    settle_into_reporting_with_items(app, answered, 1).await
}

/// ⏱️ [`dispatch_reporting`] settling at the HOST's own per-turn item grant, so `Puzzle3dSettled::turns`
/// counts host↔guest round trips rather than paging items. See [`SETTLE_HOST_TURN_ITEMS`].
pub async fn dispatch_reporting_with_items(
    app: &mut Puzzle3dApp,
    action: &str,
    args: Option<&Value>,
    window_id: Option<&str>,
    maximum_items: usize,
) -> (Result<InvocationResult, Fault>, Puzzle3dSettled) {
    let window_id = window_id.unwrap_or(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
    app.ensure_window(window_id);
    if matches!(action, SET_ACTIVE_TOOL_ACTION_ID | semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID) {
        app.activate(action, args, window_id);
    }
    let action_meta = ActionMeta { view_state: Some(app.window_view(window_id)), ..app.meta("local") };
    if matches!(
        action,
        "undo"
            | "redo"
            | "commitCheckpoint"
            | "createAlternative"
            | "switchAlternative"
            | "checkoutCheckpoint"
            | "revertToCommand"
            | "setHistoryCommandFilter"
            | "noteShellCommand"
            | "recordTutorial"
            | "startIntroduction"
            | "startTutorial"
            | "copy"
            | "cut"
            | "paste"
            | "setActiveUtility"
            | "setActiveTool"
            | "interactionSelect"
            | "interactionHover"
            | "clearSelection"
            | "selectAll"
            | "setSelectionMode"
            | "setInteractionGranularity"
    ) {
        let dsl_args = args.map(semio_framework_pack_json::to_dsl_value);
        let (raw, cx) = app.parts();
        let admitted = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(raw, action, dsl_args, &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.map_err(fault_of);
        let reserved = match admitted {
            Ok(admitted) => settle_reserved(app, admitted).await,
            Err(fault) => Err(fault),
        };
        return settle_into_reporting_with_items(app, reserved, maximum_items).await;
    }
    let (raw, cx) = app.parts();
    let typed = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(raw, action, action_args(args, window_id), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.map_err(fault_of);
    settle_into_reporting_with_items(app, typed, maximum_items).await
}

/// 🧾 The addressed window rides in the action arguments, exactly as the shell sends it.
fn action_args(args: Option<&Value>, window_id: &str) -> Option<semio_framework_value::DslValue> {
    let mut value = args.cloned().unwrap_or_else(|| Value::Object(Default::default()));
    if let Value::Object(object) = &mut value {
        object.insert("windowId".into(), Value::String(window_id.into()));
    }
    Some(semio_framework_pack_json::to_dsl_value(&value))
}

/// 📤 `dispatch` up to the point the host has minted the typed operation and NOT one continuation
/// turn further: the operation is mounted, its worker session holds one of the process-wide
/// [`semio_framework_job_worker::WORKER_JOB_SESSION_SLOTS`] admissions, and nothing has retired it. Dropping
/// the scene_snapshot here is exactly what a closed tab, a cancelled command or a torn-down app does, which
/// is the ONLY way a law can witness the retirement array a live app is expected to give back.
pub async fn dispatch_unsettled(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    let window_id = window_id.unwrap_or(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
    app.ensure_window(window_id);
    let action_meta = ActionMeta { view_state: Some(app.window_view(window_id)), ..app.meta("local") };
    let (raw, cx) = app.parts();
    semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(raw, action, action_args(args, window_id), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.map_err(fault_of)
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionSelect`
/// for one `(granularity, id)` pair in the `vortex` domain — the test-side replacement for the
/// deleted `worldPick`/`worldSelect`/`worldVortexSelect`/`setSelection` actions.
pub async fn select_id(app: &mut Puzzle3dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
    let targets = to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    dispatch(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None).await
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: dispatches `interactionHover`
/// for one `(granularity, id)` pair on the `pointer` channel in the `vortex` domain — the
/// test-side replacement for the deleted `worldHover`/`setHover`/`worldVortexHover`/`setKindHover`
/// actions. `id: None` clears the hover (mirrors the old "hover nothing" call shape).
pub async fn hover_id(app: &mut Puzzle3dApp, granularity: &str, id: Option<&str>) -> Result<InvocationResult, Fault> {
    let targets: Vec<InteractionTarget> = id.map(|id| InteractionTarget { granularity: granularity.into(), id: id.into() }).into_iter().collect();
    let targets_json = to_json_string(&targets);
    dispatch(app, "interactionHover", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "channel": "pointer", "targets": targets_json })), None).await
}

/// 🖱️ Browser pointermove over vortices admits many `interactionHover`s before Isolated reserved
/// jobs finish. Laws that reproduce that storm must not `settle` between admits.
pub async fn dispatch_reserved_unsettled(app: &mut Puzzle3dApp, action: &str, args: Option<&Value>, window_id: Option<&str>) -> Result<InvocationResult, Fault> {
    let window_id = window_id.unwrap_or(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
    app.ensure_window(window_id);
    let action_meta = ActionMeta { view_state: Some(app.window_view(window_id)), ..app.meta("local") };
    let dsl_args = args.map(semio_framework_pack_json::to_dsl_value);
    let (raw, cx) = app.parts();
    semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(raw, action, dsl_args, &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.map_err(fault_of)
}

pub async fn hover_id_unsettled(app: &mut Puzzle3dApp, granularity: &str, id: Option<&str>) -> Result<InvocationResult, Fault> {
    let targets: Vec<InteractionTarget> = id.map(|id| InteractionTarget { granularity: granularity.into(), id: id.into() }).into_iter().collect();
    let targets_json = to_json_string(&targets);
    dispatch_reserved_unsettled(app, "interactionHover", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "channel": "pointer", "targets": targets_json })), None).await
}

pub async fn select_id_unsettled(app: &mut Puzzle3dApp, granularity: &str, id: &str) -> Result<InvocationResult, Fault> {
    let targets = to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    dispatch_reserved_unsettled(app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })), None).await
}

pub async fn settle_reserved(app: &mut Puzzle3dApp, admitted: InvocationResult) -> Result<InvocationResult, Fault> {
    let (raw, cx) = app.parts();
    semio_framework_plugin::app::settle_framework_reserved_admission(raw, admitted, cx).await.map_err(fault_of)
}
