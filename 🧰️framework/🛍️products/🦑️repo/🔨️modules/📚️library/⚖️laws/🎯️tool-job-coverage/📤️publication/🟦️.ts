/** 🧭️ Tool-job coverage: import preparation, dispatch, publication freshness and bounded publication laws.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
 */
import { toolJobProductionSource, toolJobRustBlock } from "../📋️inventory/🟦️.ts";

export function toolJobImportPreparationBounded(source: string): boolean {
  const start = source.lastIndexOf("async fn dispatch_import_media");
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  return !!block && !block.body.includes("serde_json::to_vec(&(port, media))") && !block.body.includes("raw.clone()") && block.body.includes("build_artifact_reserved_media_job(port, media");
}

export function toolJobTypedCaptureBody(source: string): string | undefined {
  const start = source.lastIndexOf("async fn capture_typed_command_roots(");
  if (start < 0 || source.split("async fn capture_typed_command_roots(").length !== 2) return undefined;
  const body = toolJobRustBlock(source, source.indexOf("{", start))?.body;
  const expected = "\n            self.refresh_cache().await?;\n            let (snapshot, config, history) = self.command_cache_inputs();\n            let canonical_base_revision = self.store.content_revision();\n            let window_config_authority = self.window_config_store.capture(meta.view_state.as_ref()).await?;\n            let targeted_window_transient_view = match A::retained_window_transient_target(command) {\n                Some((window_id, expected_kind)) => {\n                    let view = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault(\"targeted window transient capture requires an exact ViewModel roster\"))?;\n                    let window = view.window_instances.iter().find(|window| window.id == window_id).ok_or_else(|| plugin_sdk_fault(\"targeted window transient capture requires an attached window instance\"))?;\n                    if window.window_kind_id != expected_kind {\n                        return Err(plugin_sdk_fault(\"targeted window transient capture does not match the command owner's expected window kind\"));\n                    }\n                    Some(view.for_window_instance(window_id).ok_or_else(|| plugin_sdk_fault(\"targeted window transient capture lost its attached window instance\"))?)\n                }\n                None => None,\n            };\n            let window_transient_authority = self.window_transient_store.capture(targeted_window_transient_view.as_ref().or(meta.view_state.as_ref()))?;\n            Ok(TypedCommandRoots {\n                snapshot,\n                config,\n                history,\n                children: std::sync::Arc::new(ChildContentView::clone(&self.child_content_root)),\n                draft_snapshot: self.draft_store.snapshot_root(),\n                interaction_state: self.interaction_store.snapshot_root(),\n                interaction_hover: self.interaction_hover.clone(),\n                presence_peers: self.presence_store.peers_root(),\n                transient: self.transient_store.current_root(),\n                peer_presence: std::sync::Arc::clone(&self.peer_presence),\n                canonical_base_revision,\n                base_revision: semio_framework_job::RevisionId(u64::from_be_bytes(canonical_base_revision[..8].try_into().expect(\"revision lane width\"))),\n                generation: semio_framework_job::Generation(self.store.generation()),\n                config_generation: self.config_store.generation(),\n                draft_generation: self.draft_store.generation(),\n                presence_generation: self.presence_store.generation().await,\n                transient_generation: self.transient_store.generation().await,\n                window_config_authority,\n                window_transient_authority,\n            })\n        ";
  return body && body.replace(/\s+/gu, "") === expected.replace(/\s+/gu, "") ? body : undefined;
}

export function toolJobRetainedDispatchSetup(source: string): string | undefined {
  const body = (name: string): string | undefined => {
    const start = source.lastIndexOf(`async fn ${name}(`);
    return start < 0 ? undefined : toolJobRustBlock(source, source.indexOf("{", start))?.body;
  };
  const dispatch = body("dispatch_typed_command_inner");
  const setup = body("start_typed_command_operation");
  if (!dispatch || !setup) return undefined;
  const guard = dispatch.indexOf("self.require_complete_tool_operation_pipeline(&admission)?");
  const direct = dispatch.indexOf("self.start_typed_command_operation(command, admission, meta, operation_id, None).await");
  const retained = dispatch.indexOf("self.latest_wins_commands.insert_admitted(");
  const reservation = setup.indexOf("assert!(self.tool_operations.can_insert(operation_id.0)");
  const capture = toolJobTypedCaptureBody(source);
  const prepare = setup.indexOf("self.capture_typed_command_roots(command.as_ref(), meta).await?");
  return guard >= 0 && direct > guard && retained > guard && !dispatch.includes("refresh_cache().await")
    && !!capture && reservation >= 0 && prepare > reservation && setup.includes("self.typed_operation_reservations[") && setup.includes("self.can_admit_typed_operation(operation_id.0)") ? setup : undefined;
}

export function toolJobMountedDispatchOneTurnExact(raw: string): boolean {
  const source = toolJobProductionSource(raw);
  const count = (text: string, token: string): number => text.split(token).length - 1;
  if (["dispatch_typed_command_inner", "start_typed_command_operation"].some((name) => count(source, `async fn ${name}(`) !== 1)) return false;
  const start = source.lastIndexOf("async fn dispatch_typed_command_inner(");
  const inner = start < 0 ? undefined : toolJobRustBlock(source, source.indexOf("{", start));
  const setup = toolJobRetainedDispatchSetup(source);
  if (!inner || !setup) return false;
  const workerSignature = "fn drive_worker_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault>";
  if (count(source, workerSignature) !== 1) return false;
  const direct = "self.start_typed_command_operation(command, admission, meta, operation_id, None).await";
  const stages = ["dispatch_wire_retained_with_spec", "MountedWorkerJobSession::try_new", "self.tool_operations.insert_admitted(", "active.drive_worker_step(self.mounted_policy.maintenance)"];
  const offsets = stages.map((token) => setup.indexOf(token));
  return count(inner.body, direct) === 1 && stages.every((token) => count(setup, token) === 1)
    && offsets.every((offset, index) => offset >= 0 && (index === 0 || offset > offsets[index - 1]!))
    && !["self.tool_jobs.dispatch", "MountedWorkerJobSession::try_new", "drive_worker_step("].some((token) => inner.body.includes(token))
    && !["run_on_worker_async", "run_to_completion", "thread_local!", "A::handle(", "A::ephemeral("].some((token) => inner.body.includes(token) || setup.includes(token));
}

export function toolJobFullOperationBounded(source: string): boolean {
  const start = source.lastIndexOf("async fn dispatch_typed_command_inner");
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  if (!block) return false;
  const worker = block.body.indexOf("WorkerJobSession::new");
  const before = block.body.slice(0, worker);
  const after = block.body.slice(worker);
  const preJobWork = ["refresh_cache().await", "draft_store.snapshot().await", "interaction_store.snapshot().await", "ChildContentView::new", "presence_store.peers().await", "transient_store.current().await", "store.content_revision().await", "store.envelope().await"];
  const postJobWork = ["presence_store.apply", "transient_store.apply", "dispatch_emit("];
  const operationStart = source.indexOf("impl<A: ArtifactApp> semio_framework_job::InteractiveJob for TypedCommandFullOperationJob<A>");
  const operationOpen = operationStart < 0 ? -1 : source.indexOf("{", operationStart);
  const operation = operationOpen < 0 ? undefined : toolJobRustBlock(source, operationOpen);
  const operationJob = source.includes("struct TypedCommandFullOperationJob") && !!operation;
  const exactStages = ["typed-command-prepare", "typed-command-reducer", "typed-command-output-validation", "typed-command-ephemeral", "typed-command-emit", "typed-command-expose"].every((stage) => source.includes(stage));
  const boundedControl = !!operation && operation.body.includes("cx.is_cancelled()") && operation.body.includes("cx.should_yield()") && operation.body.includes("max_decoded_items") && operation.body.includes("max_output_bytes") && operation.body.includes("PreviewReady") && operation.body.includes("CheckpointReady");
  const freshness = !!operation && operation.body.includes("validate_commit") && operation.body.includes("base_revision") && operation.body.includes("generation");
  const monolithicOutput = !!operation && ["serde_json::to_vec(&emit.effects)", "serde_json::to_vec(&emit.events)", "for child in emit.child_emits.iter()", "bounded_command_output_bytes"].some((needle) => operation.body.includes(needle));
  const invokesGenericReducer = !!operation && ["resolve_ready(A::ephemeral(", "resolve_ready(A::handle(", "A::ephemeral(command", "A::handle(command"].some((needle) => operation.body.includes(needle));
  const boundedFirstStepReducer =
    !!operation &&
    operation.body.includes("ToolExecutionShape::BoundedFirstStep") &&
    operation.body.includes("max_work_units_per_step") &&
    operation.body.includes("max_step_micros") &&
    source.includes("QualifiedToolProof::Bounded(_) => {") &&
    source.includes("TypedCommandFullOperationJob::<A>");
  const monolithicReducer = invokesGenericReducer && !boundedFirstStepReducer;
  const legacyMountedOperation = worker >= 0 && operationJob && exactStages && boundedControl && freshness && !monolithicOutput && !monolithicReducer && preJobWork.every((needle) => !before.includes(needle)) && postJobWork.every((needle) => !after.includes(needle));
  const publisherStart = source.lastIndexOf("fn publish_mounted_typed_operation_unit");
  const publisherOpen = publisherStart < 0 ? -1 : source.indexOf("{", publisherStart);
  const publisher = publisherOpen < 0 ? undefined : toolJobRustBlock(source, publisherOpen);
  const retainedSetup = toolJobRetainedDispatchSetup(source);
  const capturedRoots = toolJobTypedCaptureBody(source);
  const retainedMountedOperation =
    !!retainedSetup &&
    retainedSetup.includes("MountedWorkerJobSession::try_new") &&
    retainedSetup.includes("dispatch_wire_retained_with_spec") &&
    retainedSetup.includes("self.tool_operations.insert_admitted(") &&
    capturedRoots?.includes("draft_snapshot: self.draft_store.snapshot_root()") &&
    capturedRoots?.includes("interaction_state: self.interaction_store.snapshot_root()") &&
    retainedSetup.includes("presence_local: Some(self.presence_store.local_read().map_err(Fault::from)?)") &&
    capturedRoots?.includes("presence_peers: self.presence_store.peers_root()") &&
    capturedRoots?.includes("transient: self.transient_store.current_root()") &&
    source.includes("MountedTypedCommandFullOperationStage::AwaitingAck") &&
    source.includes("fn acknowledge_result_page") &&
    source.includes("if page.token != token") &&
    source.includes("advance_typed_operation_publication_one") &&
    source.includes("let mut mounted =") &&
    source.includes(".remove(operation_id)") &&
    source.includes("self.tool_operations.insert_admitted(operation_id, mounted)") &&
    !!publisher &&
    !source.includes("async fn publish_mounted_typed_operation_unit") &&
    ["presence", "transient"].every((lane) => publisher.body.includes(`${lane}.pop()`)) &&
    ["artifact_mutations", "config_mutations", "draft_mutations"].every((lane) => publisher.body.includes(`std::mem::take(&mut emit.${lane})`)) &&
    !publisher.body.includes(".last().cloned()") &&
    !publisher.body.includes(".await") &&
    publisher.body.includes("pending_artifact_publication") &&
    publisher.body.includes("begin_apply_batch") &&
    publisher.body.includes("begin_publish_one") &&
    publisher.body.includes("advance_apply_batch") &&
    publisher.body.includes("advance_publish_one") &&
    !publisher.body.includes(".apply_one(") &&
    publisher.body.includes(".pop()") &&
    toolJobPublicationFreshnessBeforeEveryTurn(source) &&
    publisher.body.includes("content_revision_now()") &&
    !publisher.body.includes("while ") &&
    operationJob &&
    exactStages &&
    boundedControl &&
    freshness &&
    !monolithicOutput;
  return !!(legacyMountedOperation || retainedMountedOperation);
}

/** 🔒️ Requires document freshness before either resuming or beginning one retained publication turn. */
export function toolJobPublicationFreshnessBeforeEveryTurn(source: string): boolean {
  const publisherStart = source.lastIndexOf("fn publish_mounted_typed_operation_unit");
  const publisherOpen = publisherStart < 0 ? -1 : source.indexOf("{", publisherStart);
  const publisher = publisherOpen < 0 ? undefined : toolJobRustBlock(source, publisherOpen);
  if (!publisher) return false;
  const directFreshness = publisher.body.indexOf("validate_commit(&mounted.operation");
  const helperFreshness = publisher.body.indexOf("typed_operation_document_is_fresh(&mounted.operation");
  const helperStart = source.lastIndexOf("fn typed_operation_document_is_fresh");
  const helperOpen = helperStart < 0 ? -1 : source.indexOf("{", helperStart);
  const helper = helperOpen < 0 ? undefined : toolJobRustBlock(source, helperOpen);
  const helperExact =
    !!helper &&
    helper.body.includes("canonical_revision == live_revision") &&
    helper.body.includes("validate_commit(operation") &&
    helper.body.includes("Generation(live_generation)") &&
    helper.body.includes("CommitValidation::Accepted");
  const freshness = directFreshness >= 0 ? directFreshness : helperExact ? helperFreshness : -1;
  const pendingPublication = publisher.body.indexOf("if let Some(pending) = mounted.pending_artifact_publication.as_mut()");
  const beginTurns = [
    "self.store.begin_apply_batch",
    "self.config_store.begin_apply_batch",
    "self.draft_store.begin_apply_batch",
    "self.presence_store.begin_publish_one",
    "self.transient_store.begin_publish_one",
  ]
    .map((needle) => publisher.body.indexOf(needle))
    .filter((index) => index >= 0);
  return (
    freshness >= 0 &&
    pendingPublication >= 0 &&
    beginTurns.length > 0 &&
    freshness < pendingPublication &&
    beginTurns.every((begin) => freshness < begin) &&
    (publisher.body.includes("mounted.canonical_revision != live_revision") || helperExact)
  );
}

/** 🧵️ Rejects the last generic typed-command false green: a batched publisher that
 * enters whole mutation replay, history construction, or outbound serialization in one turn.
 * ONE gesture stages N mutations into ONE `Edit` and ONE ledger slot, one fold per turn. */
export function toolJobStoreBatchPublicationBounded(storeSource: string, pluginSource: string): boolean {
  const blockOf = (source: string, needle: string, fromEnd = false) => {
    const start = fromEnd ? source.lastIndexOf(needle) : source.indexOf(needle);
    const open = start < 0 ? -1 : source.indexOf("{", start);
    return open < 0 ? undefined : toolJobRustBlock(source, open);
  };
  const begin = blockOf(storeSource, "fn begin_apply_batch");
  const ownedBegin = blockOf(storeSource, "fn begin_apply_batch_owned");
  const preparation = begin?.body.includes("self.begin_apply_batch_owned(") && ownedBegin?.body.includes("source.footprint(lane)") ? ownedBegin : begin;
  const advance = blockOf(storeSource, "fn advance_apply_batch");
  const cancel = blockOf(storeSource, "fn cancel_apply_batch");
  const fold = blockOf(storeSource, "fn fold_batch_item");
  const base = blockOf(storeSource, "fn batch_item_base");
  const publicationState = blockOf(storeSource, "pub struct ArtifactStoreBatchPublication<P, Mutation>");
  const publication = blockOf(storeSource, "impl<P, Mutation> ArtifactStoreBatchPublication<P, Mutation>");
  const close = publication ? blockOf(publication.body, "fn close_step") : undefined;
  const terminal = publication ? blockOf(publication.body, "fn terminal_is_empty") : undefined;
  const publisher = blockOf(pluginSource, "fn publish_mounted_typed_operation_unit", true);
  const monolithic = ["replay_mutations(", "apply_command(", "flush_outbound(", "mutation.diff(", "diff.apply(", "try_reserve", "with_capacity(", ".collect::<Vec", ".to_vec("];
  return (
    storeSource.includes("pub struct ArtifactStoreBatchPublication") &&
    storeSource.includes("pub enum ArtifactStoreOneItemPublicationPhase") &&
    storeSource.includes("pub trait ArtifactStoreOneItemPreparationFactory") &&
    storeSource.includes("pub trait ArtifactStoreOneItemPreparation") &&
    storeSource.includes("pub struct ArtifactStoreOneItemPrepared") &&
    storeSource.includes("trait ArtifactStoreBatchSource") &&
    !!begin &&
    !!advance &&
    !!cancel &&
    !!fold &&
    !!base &&
    !!publicationState &&
    !!publication &&
    !!close &&
    !!terminal &&
    !!preparation && preparation.body.includes("ArtifactStoreBatchPublication") &&
    storeSource.includes("fn advance_apply_batch") &&
    storeSource.includes("fn cancel_apply_batch") &&
    publication.body.includes("fn begin_close") &&
    close.body.includes("min(1)") &&
    close.body.includes("maximum_bytes") &&
    !close.body.includes(".clear()") &&
    terminal.body.includes("ArtifactStoreOneItemPublicationPhase::Complete") &&
    (terminal.body.includes(".is_empty()") || terminal.body.includes(".is_none()")) &&
    publication.body.includes("maximum_items") &&
    publication.body.includes("maximum_bytes") &&
    /\b(?:expected_)?generation\b/.test(publicationState.body) &&
    storeSource.includes("impl<P, Mutation> Drop for ArtifactStoreBatchPublication") &&
    monolithic.every((needle) => !begin.body.includes(needle) && !preparation.body.includes(needle) && !advance.body.includes(needle)) &&
    fold.body.includes("authority.validate_prepared(candidate)") &&
    fold.body.includes("inverse.reverse();") &&
    !fold.body.includes("replay_mutations(") &&
    !fold.body.includes("apply_command(") &&
    !advance.body.includes("while ") &&
    !fold.body.includes("while ") &&
    pluginSource.includes("pending_artifact_publication") &&
    pluginSource.includes("artifact_one_item_factory: Option<") &&
    pluginSource.includes("config_one_item_factory: Option<") &&
    pluginSource.includes("draft_one_item_factory: Option<") &&
    pluginSource.includes("unsupported_publication_contracts") &&
    !!publisher &&
    publisher.body.includes("begin_apply_batch") &&
    publisher.body.includes("advance_apply_batch") &&
    ["artifact_mutations", "config_mutations", "draft_mutations"].every((lane) => publisher.body.includes(`std::mem::take(&mut emit.${lane})`) && !publisher.body.includes(`${lane}.last().cloned()`)) &&
    publisher.body.includes("self.artifact_one_item_factory.as_ref()") &&
    publisher.body.includes("self.config_one_item_factory.as_ref()") &&
    publisher.body.includes("self.draft_one_item_factory.as_ref()") &&
    !publisher.body.includes("A::build_artifact_store_one_item_preparation_factory") &&
    !publisher.body.includes("A::build_config_store_one_item_preparation_factory") &&
    !publisher.body.includes("A::build_draft_store_one_item_preparation_factory") &&
    !publisher.body.includes(".apply_one(")
  );
}

export function toolJobEphemeralOneItemPublicationBounded(storeSource: string, pluginSource: string): boolean {
  const blocks = (source: string, needle: string) => {
    const result: { body: string; end: number }[] = [];
    let cursor = 0;
    for (;;) {
      const start = source.indexOf(needle, cursor);
      if (start < 0) return result;
      const open = source.indexOf("{", start);
      const block = open < 0 ? undefined : toolJobRustBlock(source, open);
      if (!block) return result;
      result.push(block);
      cursor = block.end;
    }
  };
  const advances = blocks(storeSource, "pub fn advance_publish_one");
  const begins = blocks(storeSource, "pub fn begin_publish_one");
  const cancels = blocks(storeSource, "pub fn cancel_publish_one");
  const publication = storeSource.indexOf("impl<P, Mutation> ArtifactEphemeralOneItemPublication<P, Mutation>");
  const publicationOpen = publication < 0 ? -1 : storeSource.indexOf("{", publication);
  const publicationBlock = publicationOpen < 0 ? undefined : toolJobRustBlock(storeSource, publicationOpen);
  const forbidden = ["mutation.diff(", "diff.apply(", "apply_command(", "replay_mutations(", "try_reserve", ".to_vec(", "drop(previous)"];
  const publisherStart = pluginSource.lastIndexOf("fn publish_mounted_typed_operation_unit");
  const publisherOpen = publisherStart < 0 ? -1 : pluginSource.indexOf("{", publisherStart);
  const publisher = publisherOpen < 0 ? undefined : toolJobRustBlock(pluginSource, publisherOpen);
  return (
    storeSource.includes("pub trait ArtifactEphemeralOneItemPreparationFactory") &&
    storeSource.includes("pub trait ArtifactEphemeralOneItemPreparation") &&
    storeSource.includes("pub struct ArtifactEphemeralOneItemPrepared") &&
    storeSource.includes("pub struct ArtifactEphemeralOneItemPublication") &&
    begins.length === 2 &&
    advances.length === 2 &&
    cancels.length === 2 &&
    advances.every((block) => forbidden.every((needle) => !block.body.includes(needle)) && block.body.includes("retirement")) &&
    !!publicationBlock &&
    publicationBlock.body.includes("fn close_step") &&
    publicationBlock.body.includes("fn terminal_is_empty") &&
    storeSource.includes("impl<P, Mutation> Drop for ArtifactEphemeralOneItemPublication") &&
    !!publisher &&
    pluginSource.includes("presence_one_item_factory: Option<") &&
    pluginSource.includes("transient_one_item_factory: Option<") &&
    pluginSource.includes("presence_local_root_retirement_factory: Option<") &&
    pluginSource.includes("transient_local_root_retirement_factory: Option<") &&
    pluginSource.includes("unsupported_publication_contracts") &&
    pluginSource.includes("if let Some(lane) = self.unsupported_publication_contracts.get(verb)") &&
    publisher.body.includes("PendingArtifactStorePublication::Presence") &&
    publisher.body.includes("PendingArtifactStorePublication::Transient") &&
    (publisher.body.match(/begin_publish_one/g) ?? []).length >= 2 &&
    (publisher.body.match(/advance_publish_one/g) ?? []).length >= 2 &&
    publisher.body.includes("self.presence_one_item_factory.as_deref()") &&
    publisher.body.includes("self.transient_one_item_factory.as_deref()") &&
    publisher.body.includes("self.presence_local_root_retirement_factory.clone()") &&
    publisher.body.includes("self.transient_local_root_retirement_factory.clone()") &&
    !publisher.body.includes("A::build_presence_store_one_item_preparation_factory") &&
    !publisher.body.includes("A::build_transient_store_one_item_preparation_factory") &&
    !publisher.body.includes("presence_store.apply_one") &&
    !publisher.body.includes("transient_store.apply_one") &&
    !publisher.body.includes("presence_store.apply(") &&
    !publisher.body.includes("transient_store.apply(")
  );
}

export function toolJobTypedRouteFailsClosedBeforePreparation(source: string): boolean {
  const start = source.lastIndexOf("async fn dispatch_typed_command_inner");
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  const guard = block?.body.indexOf("require_complete_tool_operation_pipeline(&admission)?") ?? -1;
  const preparation = block?.body.indexOf("self.start_typed_command_operation(") ?? -1;
  const authorityStart = source.indexOf("fn require_complete_tool_operation_pipeline");
  const authorityOpen = authorityStart < 0 ? -1 : source.indexOf("{", authorityStart);
  const authority = authorityOpen < 0 ? undefined : toolJobRustBlock(source, authorityOpen);
  if (guard < 0 || preparation <= guard || !toolJobRetainedDispatchSetup(source) || !authority) return false;
  const rejected = authority.body.indexOf("QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::Bounded(_) =>");
  const rejection = rejected < 0 ? undefined : toolJobRustBlock(authority.body, authority.body.indexOf("{", rejected));
  return authority.body.includes("self.require_tool_operation_authority(admission)?") && authority.body.includes("QualifiedToolProof::AppOwned(_) => Ok(())")
    && !!rejection && rejection.body.includes('FaultCode::new("interactive-job.missing-owned-reducer")') && rejection.body.includes("Err(") && !rejection.body.includes("Ok(");
}

export function toolJobTypedPersistentFoundation(source: string): boolean {
  const blockOf = (needle: string) => {
    const start = source.indexOf(needle);
    const open = start < 0 ? -1 : source.indexOf("{", start);
    return open < 0 ? undefined : toolJobRustBlock(source, open);
  };
  const dispatchStart = source.lastIndexOf("async fn dispatch_typed_command_inner");
  const dispatchOpen = dispatchStart < 0 ? -1 : source.indexOf("{", dispatchStart);
  const dispatch = dispatchOpen < 0 ? undefined : toolJobRustBlock(source, dispatchOpen);
  const active = blockOf("impl<A: ArtifactApp> MountedTypedCommandFullOperation<A>");
  const maintenanceStart = source.indexOf("impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp");
  const maintenanceName = source.indexOf("fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize)", maintenanceStart);
  const maintenanceOpen = maintenanceName < 0 ? -1 : source.indexOf("{", maintenanceName);
  const maintenance = maintenanceOpen < 0 ? undefined : toolJobRustBlock(source, maintenanceOpen);
  const setup = toolJobRetainedDispatchSetup(source);
  if (!dispatch || !active || !maintenance || !setup) return false;
  const guard = dispatch.body.indexOf("require_complete_tool_operation_pipeline(&admission)?");
  const preparation = setup.indexOf("refresh_cache().await");
  const preAdmission = dispatch.body.indexOf("!self.can_admit_typed_operation(operation_id.0)");
  const session = setup.indexOf("MountedWorkerJobSession::try_new");
  const retained = setup.indexOf("self.tool_operations.insert_admitted(");
  return (
    guard >= 0 &&
    preparation >= 0 &&
    preAdmission >= 0 &&
    preAdmission > guard &&
    session > preparation &&
    retained > session &&
    setup.includes("active.drive_worker_step(&pool, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)?") &&
    dispatch.body.includes('DslValue::String(operation_id.0.to_string())') &&
    !setup.includes("session.step(&pool") &&
    !setup.includes("let outcome = loop") &&
    source.includes("tool_operations: ArtifactFixedRegistry<MountedTypedCommandFullOperation<A>>") &&
    active.body.includes("session.pump_one(pool, semio_framework_async::Lane::Interactive)") &&
    active.body.includes("session.take_checked_out_outcome()") &&
    active.body.includes("MountedTypedCommandFullOperationStage::AwaitingAck") &&
    active.body.includes('Ok(_) => Ok(PluginCloseStep::AwaitingInput { reason: "typed operation mounted worker awaits its next outcome" })') &&
    active.body.includes("WorkerJobSubmitFault::Contention(_)") &&
    active.body.includes("WorkerSubmitErrorKind::Saturated") &&
    active.body.includes("WorkerJobTakeFault::Pending") &&
    active.body.includes('Ok(PluginCloseStep::AwaitingInput { reason: "typed operation mounted worker awaits transient scheduler authority" })') &&
    maintenance.body.includes("for offset in 0..ARTIFACT_LIVE_OUTPUT_SLOTS") &&
    maintenance.body.includes("self.tool_operations.entry(index)") &&
    maintenance.body.includes("every_operation_awaits_presented_ack") &&
    maintenance.body.includes("PluginCloseStep::AwaitingInput") &&
    maintenance.body.includes(".drive_worker_step(&pool, maximum_bytes)") &&
    maintenance.body.includes("operation.retirement_step(maximum_items.min(1), maximum_bytes)?") &&
    active.body.includes("fn terminal_is_empty") &&
    source.includes("retained_latest_wins_registered_dispatch_rebases_worker_and_publishes_real_document")
  );
}
