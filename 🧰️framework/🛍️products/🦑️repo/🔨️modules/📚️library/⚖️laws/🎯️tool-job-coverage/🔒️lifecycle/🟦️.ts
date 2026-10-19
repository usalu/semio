/** 🔒️ Tool-job coverage: importer inventory, cancellation, close and disposal laws.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts
 */
import { policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { TOOL_JOB_IMPORTER_INVENTORY, type ToolJobCoverageReport, type ToolJobProof, toolJobRustBlock } from "../📋️inventory/🟦️.ts";
import { toolJobSegmentedQueueHardBounded } from "../🛣️routes/🟦️.ts";

export function toolJobMediaExportBounded(source: string): boolean {
  const pollStart = source.indexOf("async fn poll_owned_media_export");
  const pollOpen = pollStart < 0 ? -1 : source.indexOf("{", pollStart);
  const poll = pollOpen < 0 ? undefined : toolJobRustBlock(source, pollOpen);
  const submitStart = source.indexOf("async fn submit_owned_media_export");
  const submitOpen = submitStart < 0 ? -1 : source.indexOf("{", submitStart);
  const submit = submitOpen < 0 ? undefined : toolJobRustBlock(source, submitOpen);
  const exactPreflight = submit?.body.indexOf("!self.media_exports.can_insert(operation_id.0)") ?? -1;
  const outputOwner = submit?.body.indexOf("ArtifactOutputChunks::new(proof.contract().max_output_bytes)") ?? -1;
  const persistentOwner = submit?.body.indexOf("self.media_closures.insert_admitted(") ?? -1;
  const mountedWorker = submit?.body.indexOf("MountedWorkerJobSession::try_new(dispatch.job, params)") ?? -1;
  const retainedWorker = submit?.body.indexOf("active.session = session", mountedWorker) ?? -1;
  const liveExposure = submit?.body.indexOf("self.media_exports.insert_admitted(operation_id.0, active)", retainedWorker) ?? -1;
  return (
    !!poll &&
    !!submit &&
    exactPreflight >= 0 &&
    outputOwner > exactPreflight &&
    persistentOwner > outputOwner &&
    mountedWorker > persistentOwner &&
    retainedWorker > mountedWorker &&
    liveExposure > retainedWorker &&
    !submit.body.includes("WorkerJobSession::new") &&
    poll.body.includes("session.pump_one(&pool, semio_framework_async::Lane::Interactive)") &&
    poll.body.includes("MountedWorkerJobSession::take_checked_out_outcome") &&
    poll.body.includes("self.media_exports.insert_admitted(handle.operation_id.0, active)") &&
    !poll.body.includes("pending_step.try_recv()") &&
    poll.body.includes("active.output_credit.validate_terminal()") &&
    poll.body.includes("validate_media_export_structure(&result, &active.output_chunks") &&
    !poll.body.includes("serde_json::to_vec") &&
    !poll.body.includes("into_batch_media") &&
    /result\s*\.\s*schema\s*\.\s*len\(\)\s*\.\s*checked_add\(result\s*\.\s*chunks\s*\.\s*bytes\(\)\)/.test(source) &&
    source.includes("result.chunks.same_operation(operation_chunks)") &&
    source.includes("result.chunks.is_sealed()") &&
    source.includes("actual_bytes != credited_bytes || actual_bytes > maximum") &&
    toolJobSegmentedQueueHardBounded(source) &&
    source.includes("std::sync::Arc::ptr_eq") &&
    source.includes("ArtifactToolCompletionValue::Download") &&
    source.includes("complete_download") &&
    source.includes("plugin_take_segmented_download_chunk") &&
    source.includes("semio-segmented-handle-v1") &&
    source.includes("segmented_output_accepts_exact_cap_and_rejects_plus_one_or_foreign_authority") &&
    source.includes("final_poll_credit_accepts_maximum_and_rejects_plus_one_without_allocating_media") &&
    source.includes("struct RuntimeAppCell<PA: PluginApp>") &&
    source.includes("media_construction_failure_at_each_fallible_seam_keeps_exact_close_authority") &&
    source.includes("media_registry_detach_preserves_unrelated_live_owner_and_exact_close_handoff") &&
    source.includes("worker_session_contention_rejection_take_resume_terminal_drop_and_close_are_exact") &&
    !/plugin_(?:submit|poll|cancel)_media_export[\s\S]{0,700}?resolve_ready/.test(source)
  );
}

export function toolJobDetachedOutputOwnershipExact(source: string): boolean {
  const blockOf = (needle: string) => {
    const start = source.indexOf(needle);
    const open = start < 0 ? -1 : source.indexOf("{", start);
    return open < 0 ? undefined : toolJobRustBlock(source, open);
  };
  const registry = blockOf("impl<T> ArtifactFixedRegistry<T>");
  const submit = blockOf("async fn submit_owned_media_export");
  const finish = blockOf("fn finish_media_poll");
  const poll = blockOf("async fn poll_owned_media_export");
  const cancel = blockOf("async fn cancel_owned_media_export");
  const typedStart = source.lastIndexOf("async fn dispatch_typed_command_inner");
  const typedOpen = typedStart < 0 ? -1 : source.indexOf("{", typedStart);
  const typed = typedOpen < 0 ? undefined : toolJobRustBlock(source, typedOpen);
  const ackStart = source.lastIndexOf("fn acknowledge_typed_operation_result(&mut self");
  const ackOpen = ackStart < 0 ? -1 : source.indexOf("{", ackStart);
  const ack = ackOpen < 0 ? undefined : toolJobRustBlock(source, ackOpen);
  if (!registry || !submit || !finish || !poll || !cancel || !typed || !ack) return false;
  const registryRejectsOccupied =
    registry.body.includes("if self.entry(index).is_some()") &&
    registry.body.includes("return Err(value)") &&
    registry.body.includes("fn can_insert(&self, id: u64)") &&
    registry.body.includes("fn insert_admitted(&mut self, id: u64, value: T)");
  const mediaOperation = submit.body.indexOf("let operation_id = semio_framework_job::allocate_operation_id()");
  const mediaPreAdmission = submit.body.indexOf("!self.media_exports.can_insert(operation_id.0)", mediaOperation);
  const mediaConstruction = submit.body.indexOf("ArtifactSnapshotCloseLease::new", mediaOperation);
  const wireAdmission = submit.body.indexOf("self.tool_jobs.admit_exact_wire");
  const cancellationAdmission = submit.body.indexOf("self.tool_cancellations.begin(operation_key)", mediaOperation);
  const persistentConstruction = submit.body.indexOf("self.media_closures.insert_admitted(", mediaOperation);
  const builder = submit.body.indexOf("A::build_media_export_job(request)", mediaOperation);
  const retainedJob = submit.body.indexOf("closing_job = Some(job.clone())", mediaOperation);
  const dispatch = submit.body.indexOf("self.tool_jobs.dispatch(", mediaOperation);
  const sessionConstruction = submit.body.indexOf("MountedWorkerJobSession::try_new(dispatch.job, params)", mediaOperation);
  const retainedSession = submit.body.indexOf("active.session = session", mediaOperation);
  const exposeLive = submit.body.indexOf("self.media_closures.remove(operation_id.0)", mediaOperation);
  const mediaExact =
    mediaOperation >= 0 &&
    mediaPreAdmission > mediaOperation &&
    mediaConstruction > mediaPreAdmission &&
    wireAdmission >= 0 &&
    wireAdmission < mediaConstruction &&
    cancellationAdmission > mediaPreAdmission &&
    cancellationAdmission < mediaConstruction &&
    persistentConstruction > mediaConstruction &&
    builder > persistentConstruction &&
    retainedJob > builder &&
    dispatch > retainedJob &&
    sessionConstruction > dispatch &&
    retainedSession > sessionConstruction &&
    exposeLive > retainedSession &&
    submit.body.includes("!self.media_closures.can_insert(operation_id.0)") &&
    submit.body.includes("!self.snapshot_retirements.can_insert(operation_id.0)") &&
    submit.body.includes("self.snapshot_retirements.insert_admitted(operation_id.0, snapshot_retention)") &&
    submit.body.includes(".media_exports") &&
    submit.body.includes(".insert_admitted(") &&
    source.includes("#[derive(Clone)]\n    pub struct ArtifactReservedToolJob") &&
    source.includes("state: std::sync::Arc<std::sync::Mutex<ArtifactReservedToolJobState>>") &&
    !submit.body.includes("std::mem::forget");
  const pollAdmission = poll.body.indexOf("!self.media_closures.can_insert(handle.operation_id.0)");
  const pollDetach = poll.body.indexOf("self.media_exports.remove(handle.operation_id.0)");
  const detachedExact =
    pollAdmission >= 0 &&
    pollDetach > pollAdmission &&
    poll.body.includes("self.finish_media_poll(handle.operation_id.0, active") &&
    !poll.body.includes("drop(active)") &&
    finish.body.includes("self.media_closures.insert_admitted(operation_id, active)") &&
    finish.body.includes("self.media_exports.insert_admitted(operation_id, active)") &&
    !finish.body.includes("std::mem::forget") &&
    !finish.body.includes("drop(active)") &&
    cancel.body.includes("!self.media_closures.can_insert(handle.operation_id.0)") &&
    cancel.body.indexOf("!self.media_closures.can_insert(handle.operation_id.0)") < cancel.body.indexOf("self.media_exports.remove(handle.operation_id.0)") &&
    cancel.body.includes("self.media_closures.insert_admitted(handle.operation_id.0, active)") &&
    !cancel.body.includes("drop(active)");
  const segmentedOperation = typed.body.indexOf("let operation_id = semio_framework_job::allocate_operation_id()");
  const segmentedPreAdmission = typed.body.indexOf("!self.segmented_downloads.can_insert(operation_id.0)", segmentedOperation);
  const segmentedConstruction = typed.body.indexOf("ArtifactOutputChunks::new", segmentedOperation);
  const segmentedExact =
    segmentedOperation >= 0 &&
    segmentedPreAdmission > segmentedOperation &&
    segmentedConstruction > segmentedPreAdmission &&
    typed.body.includes("!self.segmented_closures.can_insert(operation_id.0)") &&
    ack.body.indexOf("!self.segmented_downloads.can_insert(token.operation)") < ack.body.indexOf("operation.acknowledge_result_page(token)?") &&
    ack.body.includes("!self.segmented_closures.can_insert(token.operation)") &&
    ack.body.indexOf("operation.acknowledge_result_page(token)?") < ack.body.indexOf("self.segmented_downloads.insert_admitted(token.operation, download)") &&
    !ack.body.includes("drop(download)") &&
    source.includes("segmented_closures: ArtifactFixedRegistry<ArtifactDownloadOutput>") &&
    source.includes("close_segment_cleanup_cursor: usize") &&
    source.includes("duplicate_id_never_drops_active_media_snapshot_or_segmented_download_ownership");
  return registryRejectsOccupied && mediaExact && detachedExact && segmentedExact;
}

export function toolJobLiveConstructionCleanupExact(plugin: string, reactor: string): boolean {
  const blockOf = (needle: string, after = 0) => {
    const start = plugin.indexOf(needle, after);
    const open = start < 0 ? -1 : plugin.indexOf("{", start);
    return open < 0 ? undefined : toolJobRustBlock(plugin, open);
  };
  const appImpl = plugin.indexOf("impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp");
  const maintenance = blockOf("fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize)", appImpl);
  const job = blockOf("impl<PA: PluginApp> semio_framework_job::InteractiveJob for RuntimeLiveCleanupJob<PA>");
  const run = blockOf("fn run_runtime_live_cleanup_turn<");
  const pump = blockOf("fn runtime_live_cleanup_pump_one<");
  const schedule = blockOf("pub fn plugin_step_live_cleanup<");
  const destroy = blockOf("pub async fn plugin_destroy_app<");
  if (!maintenance || !job || !run || !pump || !schedule || !destroy) return false;
  const generationAdmission = destroy.body.indexOf("checked_runtime_close_generation(runtime.close_generation.get())?");
  const ownerDetach = destroy.body.indexOf("instances.take(instance_id)");
  const checks = {
    trait: plugin.includes("fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>;"),
    mediaNext: maintenance.body.includes("self.media_closures.next_id_from(self.maintenance_media_cursor)"),
    mediaRemove: maintenance.body.includes("self.media_closures.remove(operation_id)"),
    mediaTerminal: maintenance.body.includes("active.terminal_is_empty()?"),
    segmentNext: maintenance.body.includes("self.segmented_closures.next_id_from(self.maintenance_segment_cursor)"),
    chunk: maintenance.body.includes("close_take_chunk()?"),
    snapshotNext: maintenance.body.includes("self.snapshot_retirements.next_id_from(self.maintenance_snapshot_cursor)"),
    snapshotLive: maintenance.body.includes("self.media_exports.get(operation_id).is_some() || self.media_closures.get(operation_id).is_some()"),
    snapshotTerminal: maintenance.body.includes("ArtifactSnapshotCloseRetention::terminal_is_empty"),
    jobType: plugin.includes("struct RuntimeLiveCleanupJob<PA: PluginApp>"),
    jobLock: job.body.includes("cell.instance.try_lock()"),
    jobFuel: job.body.includes("cx.consume_fuel(1)"),
    jobStep: job.body.includes("instance.app.maintenance_step(1, RUNTIME_CLOSE_BYTES_PER_STEP)"),
    jobLimit: job.body.includes("released_items <= 1 && released_bytes <= RUNTIME_CLOSE_BYTES_PER_STEP"),
    pumpSite: pump.body.includes('site: "plugin_runtime_live_cleanup"'),
    pumpFuel: pump.body.includes("fuel_per_step: 1"),
    pumpBudget: pump.body.includes("step_budget_ms: RUNTIME_CLOSE_INNER_GRANT_MS"),
    pumpSession: pump.body.includes("BatchJobSession::try_new(job, params)"),
    runElapsed: run.body.includes("started.elapsed().as_micros()"),
    runWall: run.body.includes("elapsed_us > RUNTIME_CLOSE_CALLBACK_WALL_US"),
    scheduleCursor: schedule.body.includes("runtime.live_cleanup_cursor"),
    scheduleEntry: schedule.body.includes(".entry_at(index)"),
    scheduleGeneration: schedule.body.includes("maintenance_generation"),
    scheduleWeak: schedule.body.includes("std::sync::Arc::downgrade(&cell)"),
    scheduleSubmit: schedule.body.includes("pool.try_submit(semio_framework_async::Lane::Maintenance, job)"),
    scheduleReturn: schedule.body.includes("RUNTIME_MAINTENANCE_QUEUED, RUNTIME_MAINTENANCE_READY"),
    scheduleHandback: schedule.body.includes("drop(error.into_job())"),
    reactor: reactor.includes("plugin_step_live_cleanup(runtime)"),
    generation: generationAdmission >= 0 && ownerDetach > generationAdmission,
    hostile: ["media_construction_failure_at_each_fallible_seam_keeps_exact_close_authority", "erased_dispatch_clone_release_preserves_unique_bounded_job_disposal_authority", "exhausted_close_generation_is_rejected_before_exact_owner_detachment", "permanently_blocked_live_cleanup_faults_without_claiming_released_ownership", "contended_live_cleanup_does_not_consume_structural_stall_credit"].every((name) => plugin.includes(name)),
    noBlockingLock: !job.body.includes(".lock()"),
    noMaintenanceLoop: !/(?:^|\n)\s*(?:while|for)\s+/.test(maintenance.body),
  };
  return Object.values(checks).every(Boolean);
}

type ToolJobImporterInventory = {
  count: number;
  importers: { owner: string; file: string; route: string; currentMonolith: boolean }[];
};

export function toolJobReservedImporterOwners(root: string, productionFiles: ReadonlyMap<string, string>): { pending: ToolJobCoverageReport["reservedImporterOwners"]; failures: string[] } {
  const inventory = JSON.parse(policyReadFileSafe(root, TOOL_JOB_IMPORTER_INVENTORY)) as ToolJobImporterInventory;
  const failures: string[] = [];
  if (inventory.count !== inventory.importers.length || inventory.count !== 36) failures.push(`reserved importer inventory is not the exact 36-owner census (${inventory.count} declared, ${inventory.importers.length} rows)`);
  const owners = new Set<string>();
  const pending: ToolJobCoverageReport["reservedImporterOwners"] = [];
  for (const row of inventory.importers) {
    const identity = `${row.owner}\0${row.file}\0${row.route}`;
    if (owners.has(identity)) failures.push(`duplicate reserved importer inventory identity ${identity}`);
    owners.add(identity);
    const source = productionFiles.get(row.file) ?? "";
    const ownerStart = source.indexOf(`impl ArtifactEditor for ${row.owner}`);
    const ownerOpen = ownerStart < 0 ? -1 : source.indexOf("{", ownerStart);
    const ownerImpl = ownerOpen < 0 ? undefined : toolJobRustBlock(source, ownerOpen);
    if (!ownerImpl || row.route !== "import-media") {
      failures.push(`reserved importer inventory owner ${row.owner} does not resolve to its exact source implementation`);
      pending.push({ owner: row.owner, file: row.file, id: "import-media", status: "failClosedPendingFactory" });
      continue;
    }
    const migrated =
      ownerImpl.body.includes("fn register_tool_job_factories(") &&
      ownerImpl.body.includes("fn build_reserved_tool_job(") &&
      ownerImpl.body.includes("ArtifactReservedToolInput::Media") &&
      ownerImpl.body.includes("ArtifactReservedToolJob::new(") &&
      source.includes('"import-media"') &&
      source.includes("impl InteractiveJob for ") &&
      source.includes("cx.is_cancelled()");
    if (row.currentMonolith === migrated) failures.push(`reserved importer inventory currentMonolith is stale for ${row.owner}`);
    if (!migrated) pending.push({ owner: row.owner, file: row.file, id: "import-media", status: "failClosedPendingFactory" });
  }
  return { pending, failures };
}

export function toolJobDecodeAfterAdmission(source: string): boolean {
  const ordered = (functionName: string, admission: string, decode: string) => {
    const start = source.lastIndexOf(functionName);
    const open = start < 0 ? -1 : source.indexOf("{", start);
    const block = open < 0 ? undefined : toolJobRustBlock(source, open);
    return !!block && block.body.indexOf(admission) >= 0 && block.body.indexOf(admission) < block.body.indexOf(decode);
  };
  const rejectsUndecodedIntent = (() => {
    const start = source.lastIndexOf("async fn handle_intent_frame(&mut self");
    const open = start < 0 ? -1 : source.indexOf("{", start);
    const block = open < 0 ? undefined : toolJobRustBlock(source, open);
    return (
      !!block &&
      block.body.includes("qualified_tool_proof(verb)") &&
      block.body.includes('FaultCode::new("interactive-job.intent-raw-owner-required")') &&
      !block.body.includes("A::command_from_intent(intent).await") &&
      !block.body.includes("serde_json::to_value")
    );
  })();
  return (
    ordered("async fn dispatch_action", "admit_command_json", "A::command_from_action(action, args).await") &&
    ordered("async fn dispatch_command(&mut self", "admit_command_json", "A::command_from_action(command_id, Some(&args)).await") &&
    (ordered("async fn handle_intent_frame(&mut self", "admit_command_wire", "A::command_from_intent(intent).await") || rejectsUndecodedIntent)
  );
}

export function toolJobRuntimeProofQualified(source: string): boolean {
  const activationStart = source.indexOf("pub async fn with_registry_on_bus(");
  const activationOpen = activationStart < 0 ? -1 : source.indexOf("{", activationStart);
  const activation = activationOpen < 0 ? "" : toolJobRustBlock(source, activationOpen)?.body ?? "";
  const registered = activation.indexOf("A::register_tool_job_factories(&mut app_tool_registry)");
  const joined = activation.indexOf("tool_job_registration::<A>(");
  return (
    source.includes("pub struct ArtifactBoundedFirstStepProof") &&
    source.includes("macro_rules! bounded_first_step_tool_proofs") &&
    source.includes("struct QualifiedBoundedFirstStepProof") &&
    source.includes("pub struct ToolOwnerWitness") &&
    source.includes("std::any::TypeId::of::<A>()") &&
    source.includes("std::any::type_name::<A>()") &&
    registered >= 0 && joined > registered &&
    source.includes("A::bounded_first_step_tool_proofs())") &&
    source.includes("row.owner == owner && row.controller_id == runtime_controller_id && row.artifact_schema == artifact_schema && (generic || exact_registered)") &&
    source.includes("row.factory == BOUNDED_FIRST_STEP_FACTORY && row.factory_type_id.is_none() && row.factory_type_name.is_none() && registered.is_none()") &&
    source.includes("row.factory_type_id == Some(registration.factory_type_id)") &&
    source.includes("row.factory_type_name == Some(registration.factory_type_name)") &&
    source.includes("registration.owner == owner") &&
    source.includes("registration.key.controller_id == runtime_controller_id") &&
    source.includes("registration.key.tool_id == row.tool_id") &&
    source.includes("registration.contract == row.contract") &&
    source.includes("bus.admit_exact_wire(runtime_controller_id, row.tool_id, &registration.schema_id, &[])") &&
    source.includes("QualifiedToolProof::AppOwned(registration.clone()).admits::<A>(&admission)") &&
    source.includes("proof.with_factory_type::<$owner, $factory_type>()") &&
    source.includes("seen != expected") &&
    /tool_job_registration\s*::<A>\s*\(\s*&registry,\s*&app_id,\s*A::DOCUMENT_SCHEMA,/.test(activation) &&
    source.includes("pub fn qualified_tool_proof(&self, stores: &InstanceStores<A>, verb: &str)") &&
    source.includes("owner: ToolOwnerWitness") &&
    source.includes("runtime_controller_id: &str") &&
    source.includes("addressed_controller_id: &str") &&
    source.includes("async fn tool_owner_witness(&self) -> ToolOwnerWitness") &&
    source.includes("admission.factory_type_id == std::any::TypeId::of::<TypedCommandFullOperationJobFactory<A>>()") &&
    source.includes("admission.factory_type_name == std::any::type_name::<TypedCommandFullOperationJobFactory<A>>()") &&
    source.includes("TypedCommandFullOperationJobFactory::<A>::from_proof(proof.clone())") &&
    source.includes("bounded_tool_contracts") &&
    !source.includes("bounded_first_step_contract(artifact_schema, id)") &&
    !source.includes("const BOUNDED_FIRST_STEP_PROOFS")
  );
}

export function toolJobTypedDispatchExact(source: string): boolean {
  const start = source.indexOf("pub fn dispatch(&self");
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  return !!block && !block.body.includes("aliases.get") && block.body.includes("factory_by_key.get(&key)");
}

export function toolJobWireDispatchExact(source: string): boolean {
  const start = source.indexOf("pub fn dispatch_wire(&self");
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  return !!block && !block.body.includes("aliases.get") && block.body.includes("factory_by_key.get(&key)");
}

export function toolJobLiveInstanceIsolated(source: string): boolean {
  return (
    source.includes("let app_id = app.instance_id().await.to_string()") &&
    source.includes('format!("{}/{}/{verb}/{}", meta.instance_id') &&
    source.includes("app_instance_id: meta.instance_id") &&
    source.includes("tool_cancellation_isolated_by_live_instance_with_same_controller_and_document") &&
    source.includes("cancel_document(ArtifactDocumentAuthority(41))")
  );
}

export function toolJobQualifiedProofBeforeDecode(source: string): boolean {
  const ordered = (functionName: string, proof: string, decode: string) => {
    const start = source.lastIndexOf(functionName);
    const open = start < 0 ? -1 : source.indexOf("{", start);
    const block = open < 0 ? undefined : toolJobRustBlock(source, open);
    return !!block && block.body.indexOf(proof) >= 0 && block.body.indexOf(proof) < block.body.indexOf(decode);
  };
  const intentStart = source.lastIndexOf("async fn handle_intent_frame");
  const intentOpen = intentStart < 0 ? -1 : source.indexOf("{", intentStart);
  const intent = intentOpen < 0 ? undefined : toolJobRustBlock(source, intentOpen);
  const admissionStart = source.lastIndexOf("async fn admit_command_json");
  const admissionOpen = admissionStart < 0 ? -1 : source.indexOf("{", admissionStart);
  const admission = admissionOpen < 0 ? undefined : toolJobRustBlock(source, admissionOpen);
  const hostAdmissionStart = source.lastIndexOf("async fn admit_host_configuration_json");
  const hostAdmissionOpen = hostAdmissionStart < 0 ? -1 : source.indexOf("{", hostAdmissionStart);
  const hostAdmission = hostAdmissionOpen < 0 ? undefined : toolJobRustBlock(source, hostAdmissionOpen);
  const helperStart = source.lastIndexOf("async fn admit_command_json_with_proof");
  const helperOpen = helperStart < 0 ? -1 : source.indexOf("{", helperStart);
  const helper = helperOpen < 0 ? undefined : toolJobRustBlock(source, helperOpen);
  const delegatedAfterQualifiedAdmission =
    !!admission &&
    admission.body.indexOf("let proof = self.qualified_tool_proof(verb)?") >= 0 &&
    admission.body.indexOf("let proof = self.qualified_tool_proof(verb)?") < admission.body.indexOf("admit_command_json_with_proof(proof, verb, args)") &&
    !!hostAdmission &&
    hostAdmission.body.indexOf("let proof = self.qualified_host_configuration_tool_proof(verb)?") >= 0 &&
    hostAdmission.body.indexOf("let proof = self.qualified_host_configuration_tool_proof(verb)?") < hostAdmission.body.indexOf("admit_command_json_with_proof(proof, verb, args)") &&
    !!helper &&
    helper.body.indexOf("let key = proof.key()") >= 0 &&
    helper.body.indexOf("let key = proof.key()") < helper.body.indexOf("begin_exact_wire") &&
    helper.body.indexOf("begin_exact_wire") < helper.body.indexOf("serde_json::to_writer") &&
    helper.body.indexOf("serde_json::to_writer") < helper.body.indexOf("proof.admits::<A>(&admission)") &&
    !helper.body.includes("bounded_json_items") &&
    !helper.body.includes("serde_json::to_vec");
  const streamsOnlyAfterQualifiedAdmission =
    delegatedAfterQualifiedAdmission ||
    (!!admission &&
      admission.body.indexOf("qualified_tool_proof(verb)") >= 0 &&
      admission.body.indexOf("qualified_tool_proof(verb)") < admission.body.indexOf("begin_exact_wire") &&
      admission.body.indexOf("begin_exact_wire") < admission.body.indexOf("serde_json::to_writer") &&
      !admission.body.includes("bounded_json_items") &&
      !admission.body.includes("serde_json::to_vec"));
  const rejectsUndecodedIntent =
    !!intent &&
    intent.body.includes("qualified_tool_proof(verb)") &&
    intent.body.includes('FaultCode::new("interactive-job.intent-raw-owner-required")') &&
    !intent.body.includes("serde_json::to_value") &&
    !intent.body.includes("A::command_from_intent");
  return (ordered("async fn admit_command_json", "qualified_tool_proof", "bounded_json_items") || streamsOnlyAfterQualifiedAdmission) && (ordered("async fn handle_intent_frame", "qualified_tool_proof", "serde_json::to_value") || rejectsUndecodedIntent);
}

export function toolJobLimitsMatch(proofs: readonly ToolJobProof[], publicLimits: ReadonlyMap<string, number>): boolean {
  return proofs.every((proof) => publicLimits.get(`${proof.controllerId}\0${proof.toolId}`) === proof.values[0]);
}

export function toolJobExternalCancellationOwned(source: string): boolean {
  return /pub struct ToolOperationKey\s*\{[\s\S]*app_instance_id:[\s\S]*document:\s*ArtifactDocumentAuthority[\s\S]*operation_id:[\s\S]*base_revision:[\s\S]*generation:/.test(source) && source.includes("pub struct ArtifactDocumentAuthority(pub u32)") && source.includes("struct ToolDocumentCancellationScope") && source.includes("app_scope: semio_framework_job::CancelToken") && source.includes("self.tool_cancellations.begin(operation_key)") && source.includes("cancellation_lease.is_cancelled().await") && source.includes("cancel_scope_generation");
}

export function toolJobDropCancellationBounded(source: string): boolean {
  const start = source.indexOf("Drop for VcsArtifactApp<");
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  return !!block && !block.body.includes("resolve_ready(self.tool_cancellations.cancel_all())") && !block.body.includes("drain()") && block.body.includes("cancel_scope_generation");
}

export function toolJobCancellationScopesBounded(source: string): boolean {
  const start = source.indexOf("impl ToolCancellationHandle");
  const open = start < 0 ? -1 : source.indexOf("{", start);
  const block = open < 0 ? undefined : toolJobRustBlock(source, open);
  if (!block) return false;
  const forbidden = [".iter().filter", ".keys()", ".drain()", "collect::<Vec", "resolve_ready(", "for token in", ".lock()", "HashMap<", "String"];
  return forbidden.every((needle) => !block.body.includes(needle)) && block.body.includes("self.app_scope.child_now()") && block.body.includes("self.try_state()?") && block.body.includes("state.take(index)") && block.body.includes("drop(state)") && block.body.includes("scope.token.cancel_now()") && block.body.includes("self.app_scope.cancel_now()") && source.includes("const TOOL_CANCELLATION_SLOTS: usize = 1_024") && source.includes("slots: Box<[std::mem::MaybeUninit<ToolDocumentCancellationScope>]>") && source.includes("occupied: [u64; TOOL_CANCELLATION_WORDS]") && source.includes("app_generation: std::sync::Arc<std::sync::atomic::AtomicU64>") && source.includes("self.token.cancel_now()") && source.includes("cancellation_supersession_and_saturated_app_close_are_parent_scope_constant_time") && source.includes("cancellation_numeric_authority_rejects_collision_capacity_and_contention_without_blocking") && source.includes("poisoned_cancellation_authority_fails_closed_without_recovery_or_waiting") && !source.includes("self.media_exports.iter().filter");
}

export function toolJobHardBoundedCloseExact(plugin: string, reactor: string): boolean {
  const fixedAuthority =
    plugin.includes("pub struct ArtifactDocumentAuthority(pub u32)") &&
    plugin.includes("media_exports: ArtifactFixedRegistry<ActiveMediaExport>") &&
    plugin.includes("segmented_downloads: ArtifactFixedRegistry<ArtifactDownloadOutput>") &&
    !/HashMap\s*<[^>]*(?:ActiveMediaExport|ArtifactDownloadOutput|ToolDocumentCancellation)/.test(plugin);
  const productionClose = toolJobRuntimeRegistryFixedClose(plugin, reactor);
  const cleanupJob =
    plugin.includes("struct RuntimeCloseCleanupJob<PA: PluginApp>") &&
    plugin.includes("impl<PA: PluginApp> semio_framework_job::InteractiveJob for RuntimeCloseCleanupJob<PA>") &&
    plugin.includes("const RUNTIME_CLOSE_ITEMS_PER_STEP: usize = 1") &&
    plugin.includes("const RUNTIME_CLOSE_BYTES_PER_STEP: usize = 4_096") &&
    plugin.includes("plugin_step_close_cleanup") &&
    reactor.includes("plugin_step_close_cleanup(runtime)") &&
    plugin.includes("app_close_step_drains_at_most_one_segment_and_one_chunk_budget");
  const noImplicitDestruction = plugin.includes("PluginCloseStep::Complete") && plugin.includes("artifact_close_final_destructor_is_constant_after_every_owned_field_is_drained") && plugin.includes("cleanup_queue_saturation_preserves_detached_app_ownership");
  return fixedAuthority && productionClose && cleanupJob && noImplicitDestruction && toolJobSnapshotRetirementBounded(plugin) && toolJobVcsOwnedDisposalExplicit(plugin) && toolJobErasedCloseTerminalExact(plugin) && toolJobDetachedOutputOwnershipExact(plugin) && toolJobRuntimeCloseCallbackBounded(plugin) && toolJobLiveConstructionCleanupExact(plugin, reactor);
}

export function toolJobVcsOwnedDisposalExplicit(plugin: string): boolean {
  const driverStart = plugin.indexOf("fn drive_artifact_owned_disposer");
  const driverOpen = driverStart < 0 ? -1 : plugin.indexOf("{", driverStart);
  const driver = driverOpen < 0 ? undefined : toolJobRustBlock(plugin, driverOpen);
  const closeStart = plugin.indexOf("fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>", plugin.indexOf("impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp"));
  const closeOpen = closeStart < 0 ? -1 : plugin.indexOf("{", closeStart);
  const close = closeOpen < 0 ? undefined : toolJobRustBlock(plugin, closeOpen);
  if (!driver || !close) return false;
  const terminalCheck = driver.body.indexOf("disposer_ref.terminal_is_empty(owner)");
  const release = driver.body.indexOf("drop(disposer.take())");
  return (
    plugin.includes("pub trait ArtifactOwnedDisposer<T>: Send") &&
    plugin.includes("fn close_step(&mut self, owner: &mut T, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault>;") &&
    plugin.includes("fn terminal_is_empty(&self, owner: &T) -> bool;") &&
    terminalCheck >= 0 &&
    release > terminalCheck &&
    driver.body.includes('FaultCode::new("interactive-job.close-owned-disposer-missing")') &&
    driver.body.includes('FaultCode::new("interactive-job.close-owned-terminal-not-empty")') &&
    plugin.includes("close_document_disposer: std::mem::ManuallyDrop<Option<Box<dyn ArtifactOwnedDisposer<ArtifactStore") &&
    plugin.includes("close_config_disposer: std::mem::ManuallyDrop<Option<Box<dyn ArtifactOwnedDisposer<ConfigStore") &&
    plugin.includes("close_draft_disposer: std::mem::ManuallyDrop<Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore") &&
    plugin.includes("close_presence_disposer: std::mem::ManuallyDrop<Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore") &&
    plugin.includes("close_transient_disposer: std::mem::ManuallyDrop<Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore") &&
    plugin.includes("close_interaction_disposer: std::mem::ManuallyDrop<Option<Box<dyn ArtifactOwnedDisposer<ConfigStore<protocol::InteractionState, InteractionConfigMutation>>>>>") &&
    ["document-store", "config-store", "draft-store", "presence-store", "transient-store", "interaction-store"].every((lane) => close.body.includes(`drive_artifact_owned_disposer("${lane}"`)) &&
    close.body.includes("self.close_retained_fields_step(maximum_items, maximum_bytes)") &&
    plugin.includes("fn retained_fields_terminal_is_empty(&self) -> bool") &&
    plugin.includes("self.close_owned_stage >= 6") &&
    plugin.includes("self.retained_fields_terminal_is_empty()") &&
    plugin.includes("ArtifactCacheRetirement<A: ArtifactApp>") &&
    plugin.includes("self.composition.close_step(maximum_items.min(1), maximum_bytes)") &&
    plugin.includes("retained_field_maximum_and_maximum_plus_one_are_language_neutral") &&
    !close.body.includes('FaultCode::new("interactive-job.close-app-retained-fields-missing")')
  );
}

export function toolJobRuntimeCloseCallbackBounded(plugin: string): boolean {
  const blockOf = (needle: string) => {
    const start = plugin.indexOf(needle);
    const open = start < 0 ? -1 : plugin.indexOf("{", start);
    return open < 0 ? undefined : toolJobRustBlock(plugin, open);
  };
  const job = blockOf("impl<PA: PluginApp> semio_framework_job::InteractiveJob for RuntimeCloseCleanupJob<PA>");
  const inner = blockOf("fn run_runtime_close_turn_inner");
  const whole = blockOf("fn run_runtime_close_turn<");
  const pump = blockOf("fn runtime_close_cleanup_pump_one<");
  const nonterminal = blockOf("fn runtime_close_nonterminal_status");
  return (
    !!job &&
    !!inner &&
    !!whole &&
    !!pump &&
    !!nonterminal &&
    plugin.includes("const RUNTIME_CLOSE_INNER_GRANT_MS: u64 = 2") &&
    plugin.includes("const RUNTIME_CLOSE_CALLBACK_WALL_US: u64 = 8_000") &&
    pump.body.includes('site: "plugin_runtime_close_cleanup"') &&
    pump.body.includes("fuel_per_step: 1") &&
    pump.body.includes("step_budget_ms: RUNTIME_CLOSE_INNER_GRANT_MS") &&
    pump.body.includes("BatchJobSession::try_new(job, params)") &&
    whole.body.indexOf("std::time::Instant::now()") < whole.body.indexOf("run_runtime_close_turn_inner(state)") &&
    whole.body.indexOf("run_runtime_close_turn_inner(state)") < whole.body.indexOf("started.elapsed().as_micros()") &&
    whole.body.includes("elapsed_us > RUNTIME_CLOSE_CALLBACK_WALL_US") &&
    whole.body.includes("state.status.store(RUNTIME_CLOSE_FAULT") &&
    job.body.includes("Err(std::sync::TryLockError::WouldBlock)") &&
    job.body.includes("self.contended = true") &&
    job.body.includes("Err(std::sync::TryLockError::Poisoned(_))") &&
    nonterminal.body.indexOf("if contended") < nonterminal.body.indexOf("stalled_steps.fetch_add") &&
    plugin.includes("repeated_transient_close_lock_contention_never_consumes_structural_livelock_credit") &&
    plugin.includes("structural_zero_progress_exhausts_its_exact_close_credit")
  );
}

export function toolJobErasedCloseTerminalExact(plugin: string): boolean {
  return (
    plugin.includes("fn close_terminal_is_empty(&self) -> bool;") &&
    plugin.includes("fn terminal_is_empty(&self) -> bool;") &&
    plugin.includes("interactive-job.reserved-terminal-not-empty") &&
    plugin.includes("plugin close reported Complete without a terminal-empty app witness") &&
    plugin.includes("std::sync::Arc::try_unwrap(detached)") &&
    plugin.includes("slots: Box<[std::mem::MaybeUninit<(u32, T)>]>") &&
    plugin.includes("slots: Box<[std::mem::MaybeUninit<(u64, T)>]>") &&
    plugin.includes("cell: std::sync::Mutex<std::mem::ManuallyDrop<Option<std::sync::Arc<RuntimeAppCell<PA>>>>>") &&
    !plugin.includes("slots: Box<[Option<(u32, T)>]>") &&
    !plugin.includes("slots: Box<[Option<(u64, T)>]>")
  );
}

export function toolJobSnapshotRetirementBounded(plugin: string): boolean {
  return (
    plugin.includes("pub trait ArtifactSnapshotDisposer<T>: Send") &&
    plugin.includes("fn terminal_is_empty(&self, snapshot: &Option<std::sync::Arc<T>>) -> bool") &&
    plugin.includes("inner: std::mem::ManuallyDrop<Option<Box<dyn ErasedArtifactSnapshotDisposer>>>") &&
    plugin.includes("interactive-job.snapshot-terminal-not-empty") &&
    plugin.includes("snapshot_retirements: ArtifactFixedRegistry<ArtifactSnapshotCloseRetention>") &&
    plugin.includes("self.snapshot_retirements.insert_admitted(operation_id.0, snapshot_retention)") &&
    plugin.includes("close_snapshot_cursor: usize") &&
    plugin.includes(".snapshot_retirements") &&
    plugin.includes(".close_step(maximum_items, maximum_bytes)?") &&
    plugin.includes("snapshot_a_survives_cache_b_and_only_the_bounded_retirement_owner_performs_final_drop") &&
    !plugin.includes("std::sync::Arc::strong_count(snapshot) > 1")
  );
}

export function toolJobRuntimeRegistryFixedClose(plugin: string, reactor: string): boolean {
  const destroyStart = plugin.indexOf("pub async fn plugin_destroy_app");
  const destroyOpen = destroyStart < 0 ? -1 : plugin.indexOf("{", destroyStart);
  const destroy = destroyOpen < 0 ? undefined : toolJobRustBlock(plugin, destroyOpen);
  return (
    !!destroy &&
    plugin.includes("const PLUGIN_RUNTIME_INSTANCE_SLOTS: usize = 1_024") &&
    plugin.includes("struct RuntimeInstanceRegistry<T>") &&
    plugin.includes("slots.try_reserve_exact(PLUGIN_RUNTIME_INSTANCE_SLOTS)") &&
    plugin.includes("instance_id as usize % PLUGIN_RUNTIME_INSTANCE_SLOTS") &&
    plugin.includes("instances: RefCell<RuntimeInstanceRegistry<std::sync::Arc<RuntimeAppCell<PA>>>>") &&
    plugin.includes("close_quarantine: RefCell<RuntimeInstanceRegistry<RuntimeCloseEntry<PA>>>") &&
    plugin.includes("instance_actors: RefCell<RuntimeInstanceRegistry<RuntimeActorAuthority>>") &&
    plugin.includes("bytes: [u8; PLUGIN_RUNTIME_ACTOR_BYTES]") &&
    !plugin.includes("instances: LocalAsyncMutex<Vec<std::sync::Arc<RuntimeAppCell<PA>>>>") &&
    !plugin.includes("struct LocalAsyncMutex") &&
    destroy.body.includes("instances.take(instance_id)") &&
    destroy.body.includes("quarantine.insert_admitted(instance_id, RuntimeCloseEntry") &&
    !destroy.body.includes(".iter()") &&
    !destroy.body.includes(".position(") &&
    !destroy.body.includes("list.remove(") &&
    reactor.includes("plugin_destroy_app(runtime, *numeric_instance)") &&
    plugin.includes("runtime_instance_registry_has_fixed_capacity_collision_and_reuse") &&
    plugin.includes("runtime_instance_close_quarantine_never_implicitly_drops_nested_value")
  );
}

export function toolJobReactorCloseBounded(reactor: string, requests: string, executor: string): boolean {
  const cancelStart = reactor.indexOf("pub(crate) fn cancel_instance_tasks_step");
  const cancelOpen = cancelStart < 0 ? -1 : reactor.indexOf("{", cancelStart);
  const cancel = cancelOpen < 0 ? undefined : toolJobRustBlock(reactor, cancelOpen);
  const requestStart = requests.indexOf("pub fn cancel_instance_step(&self");
  const requestOpen = requestStart < 0 ? -1 : requests.indexOf("{", requestStart);
  const requestCancel = requestOpen < 0 ? undefined : toolJobRustBlock(requests, requestOpen);
  const closeStart = executor.indexOf("pub fn close_instance_step(&self");
  const closeOpen = closeStart < 0 ? -1 : executor.indexOf("{", closeStart);
  const executorClose = closeOpen < 0 ? undefined : toolJobRustBlock(executor, closeOpen);
  const runStart = executor.indexOf("pub fn run_until_deadline(&self");
  const runOpen = runStart < 0 ? -1 : executor.indexOf("{", runStart);
  const executorRun = runOpen < 0 ? undefined : toolJobRustBlock(executor, runOpen);
  return (
    !!cancel &&
    !!requestCancel &&
    !!executorClose &&
    !!executorRun &&
    reactor.includes("static REACTOR_EXECUTOR: executor::ReactorExecutor") &&
    reactor.includes("static TEST_FUTURE_EXECUTOR: executor::ColdFutureExecutor") &&
    !reactor.includes("static REACTOR_EXECUTOR: executor::ColdFutureExecutor") &&
    executor.includes("pub trait ReactorTask") &&
    executor.includes("fn step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep") &&
    executor.includes("fn close_step(&mut self, budget: ReactorTaskBudget) -> ReactorTaskStep") &&
    executor.includes("fn terminal_is_empty(&self) -> bool") &&
    executor.includes("pub operation: u64") &&
    executor.includes("pub generation: u64") &&
    executor.includes("pub cancellation_generation: u64") &&
    executor.includes("slots: VecDeque<ReactorTaskSlot>") &&
    executor.includes("free: VecDeque<usize>") &&
    !executor.includes("Result<TaskId, Box<dyn ReactorTask>>") &&
    reactor.includes("struct FixedResumeQueue") &&
    reactor.includes("fn cancel_instance_step(&mut self, instance: u32, remaining: &mut usize)") &&
    reactor.includes("struct FixedTimerRegistry") &&
    reactor.includes("struct ReactorFixedSlots<T>") &&
    reactor.includes("values: Box<[std::mem::MaybeUninit<T>]>") &&
    reactor.includes("impl<T> Drop for ReactorFixedSlots<T>") &&
    reactor.includes("instance: u32") &&
    reactor.includes("fn cancel_instance_step(&mut self, instance: u32, cursor: &mut usize)") &&
    reactor.includes("plugin.timer-owner-missing") &&
    !reactor.includes("static ARMED_TIMERS: RefCell<Vec<u64>>") &&
    !reactor.includes("static OPEN_INSTANCES: RefCell<Vec<") &&
    !reactor.includes("static TASK_RECORDS: RefCell<HashMap<") &&
    !reactor.includes("static TASK_KEYS: RefCell<HashMap<") &&
    !reactor.includes("static INSTANCE_QUOTAS: RefCell<HashMap<") &&
    requests.includes("slots: Box<[std::mem::MaybeUninit<SlotEntry>]>") &&
    requests.includes("outbound: std::mem::ManuallyDrop<VecDeque<(u32, Effect)>>") &&
    !requests.includes("slots: Box<[Option<SlotEntry>]>") &&
    !cancel.body.includes(".iter().filter") &&
    !cancel.body.includes("collect::<Vec") &&
    !requestCancel.body.includes(".iter().filter") &&
    !executorClose.body.includes("ready.retain") &&
    !executorClose.body.includes("free.contains") &&
    !/ReactorTaskStep::Blocked[\s\S]{0,160}\bbreak\b/.test(executorRun.body) &&
    !reactor.includes("for waker in state.waiters.drain(..)") &&
    reactor.includes("reactor_close_drains_requests_resumes_tasks_timers_and_metadata_in_bounded_steps") &&
    executor.includes("rejected_reactor_task_is_bounded_disposed_without_drop") &&
    executor.includes("blocked_reactor_task_does_not_starve_ready_peer") &&
    executor.includes("reactor_executor_shutdown_drains_every_slot_before_terminal_drop") &&
    executor.includes("stale_generation_cannot_commit")
  );
}

/** \ud83d\udcbc\ufe0f There is no opaque-future job path left to fail closed: the reactor's job module admits
 * ONE registry of `BoundedJobFactory` entries and nothing else, so a test build and a shipped
 * component admit the same kinds. The oracle used to require the opposite shape — a `JobFn`
 * registry parked as `JobBody::ExplicitStateMachineRequired` under `#[cfg(not(test))]` — which is
 * precisely how every builtin kind (`semio.infer`, `semio.io-run`, `semio.io-sniff`,
 * `semio.mutation-plan`, `semio.migrate`) shipped dead in every production build. It now refuses
 * the opaque future, the `cfg`-split admission, and a builtin set that is not declared. */
export function toolJobOpaqueFutureProductionFailClosed(jobs: string): boolean {
  const spawnStart = jobs.indexOf("async fn spawn_job(");
  const spawnOpen = spawnStart < 0 ? -1 : jobs.indexOf("{", spawnStart);
  const spawn = spawnOpen < 0 ? undefined : toolJobRustBlock(jobs, spawnOpen);
  return (
    !!spawn &&
    !jobs.includes("executor::ColdFutureExecutor") &&
    !jobs.includes("pub type JobFn") &&
    !jobs.includes("pub struct JobCtx") &&
    !jobs.includes("JobBody::ExplicitStateMachineRequired") &&
    !jobs.includes('"job.explicit-state-machine-required"') &&
    jobs.includes("static KIND_REGISTRY: RefCell<HashMap<&'static str, BoundedJobFactory>>") &&
    jobs.includes("pub const BUILTIN_JOB_KINDS: [&str; 5]") &&
    jobs.includes("pub type BoundedJobFactory = fn(u64, &[u8], Option<&[u8]>)") &&
    !spawn.body.includes("#[cfg(") &&
    spawn.body.includes("JobBody::Bounded(owner)") &&
    spawn.body.indexOf("JobBody::UnknownKind") < spawn.body.indexOf("JobBody::Bounded(owner)")
  );
}
