/** 🔶️ The base phase of toolJobLatestWinsSelfTests. */
import { isDeepStrictEqual } from "node:util";
import { join } from "node:path";
import { readModuleTreeSource } from "../../../../../../../🔨️modules/📚️compiler/📦️packages/🟦️typescript/📖️syntax/🌳️module-tree/🟦️.ts";
import { readFileSync, readdirSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";
import { toolJobRustBlock, toolJobPluginInstanceSource } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";

/** 🔷 Builds base, fixture, schema, Ajv of toolJobLatestWinsSelfTests. */
export function buildBaseState() {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
  const fixture = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/🥇️tool-latest-wins.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(base, "🧵️retained-command/🧬️schema/🔣️.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(schema);
  
  
  const resultAckTrace = Array.from({ length: fixture.resultAck.preAckPolls + 1 }, (_, poll) => poll === 0 ? { attempt: 1 } : null);
  const resultAckObservation = { preAckPolls: fixture.resultAck.preAckPolls, deliveries: resultAckTrace.filter(Boolean).length, attempt: resultAckTrace.find(Boolean)?.attempt };
  if (!isDeepStrictEqual(resultAckObservation, fixture.resultAck)) throw new Error("latest-wins result ACK single-delivery oracle diverged");
  for (const law of fixture.cases) {
    if (Buffer.byteLength(law.next.target, "utf8") !== fixture.targetBytes || isDeepStrictEqual(law.next, fixture.first) !== law.superseded) throw new Error(`latest-wins exact scope oracle: ${law.id}`);
  }
  const validateScope = ajv.getSchema(`${schema.$id}#/$defs/ToolLatestWinsScope`)!;
  if (!validateScope(fixture.first) || fixture.cases.some((law: any) => !validateScope(law.next))) throw new Error("latest-wins scope violates its semantic contract");
  for (const hostile of [{ ...fixture.first, document: undefined }, { ...fixture.first, inventedAuthority: true }]) if (validateScope(hostile)) throw new Error("latest-wins scope accepted missing document or foreign authority");
    const integration = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/🔗️tool-latest-wins-integration.json"), "utf8"));
  
  
  for (const law of integration.cases) {
    if (isDeepStrictEqual(law.nextTarget, law.firstTarget) !== law.firstCancelled) throw new Error(`latest-wins integration equality oracle: ${law.id}`);
  }
  if (Buffer.byteLength(integration.keyRetirement.text, "utf8") !== integration.keyRetirement.utf8Bytes
    || JSON.stringify(Array.from(integration.keyRetirement.text as string).reverse().map((scalar) => Buffer.byteLength(scalar, "utf8"))) !== JSON.stringify(integration.keyRetirement.scalarBytes)) throw new Error("latest-wins UTF-8 retirement byte oracle");
  const source=toolJobPluginInstanceSource(WORKSPACE_ROOT);
  const contractRoot=join(base,"🧪️tests");
  const runtimeContractTests = readdirSync(contractRoot).filter((name) => name.startsWith("🔬️plugin-runtime-plugin-builder-contract")).sort().map((name) => readModuleTreeSource(join(contractRoot, name, "🦀️.rs"))).join("\n");
  const body = (text: string, name: string): string => {
    const owner = name === "has_runnable_typed_operations" ? "impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp<A, M>" : ["take_result_page", "reject_cancelled_publication"].includes(name) ? "impl<A: ArtifactApp> MountedTypedCommandFullOperation<A>" : undefined;
    const ownerStart = owner ? text.indexOf(owner) : -1;
    const scope = owner ? ownerStart < 0 ? "" : toolJobRustBlock(text, text.indexOf("{", ownerStart))?.body ?? "" : text;
    const start = scope.lastIndexOf(`fn ${name}(`);
    return start < 0 ? "" : toolJobRustBlock(scope, scope.indexOf("{", start))?.body ?? "";
  };
  const claimSource = readFileSync(join(WORKSPACE_ROOT, "🧰️framework/🔨️modules/⏳️async/♻️cancel-return/📬️claim/🦀️.rs"), "utf8");
  const claimIdentityExact = (text: string): boolean => body(text, "same_original").replace(/\s+/gu, "") === "Arc::ptr_eq(&left.inner,&right.inner)" && body(text, "try_claim").replace(/\s+/gu, "").includes("self.inner.state.compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).ok()?");
  if (!claimIdentityExact(claimSource) || claimIdentityExact(claimSource.replace("Arc::ptr_eq(&left.inner,&right.inner)", "true")) || claimIdentityExact(claimSource.replace("compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).ok()?", "store(1,Ordering::Release)"))) throw new Error("publication claim identity accepts a different original issuer");
  const obligations: Array<[string, string]> = [
    ["publish_mounted_typed_operation_unit", "mounted.reject_cancelled_publication()?"],
    ["publish_mounted_typed_operation_unit", "ToolCancellationLease::try_claim_publication"],
    ["reject_cancelled_publication", "pending.begin_close()"],
    ["reject_cancelled_publication", "self.publication = completion.take()?"],
    ["advance_latest_wins_command_one", "self.tools.latest_wins_order.items.front().copied()"],
    ["advance_latest_wins_command_one", "self.as_dispatch().start_typed_command_operation(command, admission"],
    ["advance_latest_wins_command_one", "pending.admit_original_lease(&mut self.tools.tool_cancellations,cx)"],
    ["advance_latest_wins_admission_unit", "pending.restarting=true"],
    ["advance_latest_wins_admission_unit", "self.latest_wins_keys.begin(operation,key,pending.lease.as_ref()"],
    ["advance_latest_wins_admission_unit", "registration.latest_wins_target"],
    ["advance_latest_wins_admission_unit", "ToolLatestWinsKeyCopy::admit_original(pending.meta.instance_id,parts,retained)"],
    ["dispatch_typed_command_inner", "key_copy: None"],
    ["dispatch_typed_command_inner", "registration.latest_wins_command_disposer"],
    ["dispatch_typed_command_inner", "self.tools.latest_wins_order.push(operation_id.0)"],
    ["dispatch_typed_command_inner", "self.media.admit_typed_operation_slot(&*self.pending, &*self.tools)"],
    ["admit_typed_operation_slot", "!tools.latest_wins_order.allocation_admitted"],
    ["admit_typed_operation_slot", "tools.latest_wins_order.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS"],
    ["admit_typed_operation_slot", "(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot| self.typed_operation_slot_is_vacant(pending, tools, *slot))?"],
    ["admit_typed_operation_slot", "allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, slot as u64)"],
    ["typed_operation_slot_is_vacant", "tools.tool_operations.slot_is_vacant(slot)"],
    ["typed_operation_slot_is_vacant", "tools.typed_operation_reservations[slot].is_none()"],
    ["typed_operation_slot_is_vacant", "tools.latest_wins_commands.slot_is_vacant(slot)"],
    ["typed_operation_slot_is_vacant", "pending.pending_reserved.slot_is_vacant(slot)"],
    ["typed_operation_slot_is_vacant", "self.segmented_downloads.slot_is_vacant(slot)"],
    ["typed_operation_slot_is_vacant", "self.segmented_closures.slot_is_vacant(slot)"],
    ["advance_latest_wins_admission_unit", ".rebind_keyed(base_revision,generation)?"],
    ["rebind_keyed", "scope.operation != original.key"],
    ["rebind_keyed", "scope.operation.generation = generation"],
    ["advance_typed_operation_publication_unit", "self.tools.next_advanceable_typed_operation()"],
    ["advance_typed_operation_publication_unit", "self.tools.typed_publication_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS"],
    ["next_advanceable_typed_operation", "(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset|"],
    ["next_advanceable_typed_operation", "(self.typed_publication_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS"],
    ["next_advanceable_typed_operation", "self.tool_operations.entry(index)"],
    ["next_advanceable_typed_operation", "operation.stage != MountedTypedCommandFullOperationStage::AwaitingAck"],
    ["next_advanceable_typed_operation", ".map(|(id, _)| (index, *id))"],
    ["take_typed_operation_result_page", "(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset|"],
    ["take_typed_operation_result_page", "(self.tools.typed_result_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS"],
    ["take_typed_operation_result_page", "self.tools.tool_operations.entry(index)"],
    ["take_typed_operation_result_page", "operation.meta.instance_id == receiver"],
    ["take_typed_operation_result_page", "!operation.result_page_presented && operation.result_page.is_some()"],
    ["take_typed_operation_result_page", "self.tools.typed_result_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS"],
    ["take_typed_operation_result_page", "self.tools.tool_operations.get_mut(operation_id)?.take_result_page()"],
    ["take_result_page", "if self.result_page_presented"],
    ["take_result_page", "return None"],
    ["has_runnable_typed_operations", "!self.tools.tool_operations.is_empty()"],
    ["cleanup_slot", "scope.publication_claim.is_finished()"],
    ["rebind_keyed", "PublicationClaimHandle::same_original(&scope.publication_claim, &original.publication_claim)"],
    ["try_claim_publication", "self.handle.publication_scope.try_claim()?"],
    ["try_claim_publication", "self.publication_claim.try_claim()?"],
  ];
  const ordered = (scope: string, tokens: readonly string[]): boolean => {
    let previous = -1;
    return tokens.every((token) => { const index = scope.indexOf(token); const valid = index > previous; previous = index; return valid; });
  };
  const publicationClaim = readFileSync(join(base, "../../../../🔨️modules/⏳️async/♻️cancel-return/📬️claim/🦀️.rs"), "utf8");
  const exact = (text: string): boolean => obligations.every(([name, token]) => body(text, name).includes(token))
    && !body(text, "publish_mounted_typed_operation_unit").includes("dispatch_emit_group(")
    && text.includes("admit_child(grant)")
    && publicationClaim.includes("compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire)")
    && text.includes("scope.operation")
    && ["retained_latest_wins_real_document_publication_cancellation_and_delayed_ack_close", "retained_latest_wins_registered_dispatch_rebases_worker_and_publishes_real_document", "a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work", "a_status_only_host_call_finishes_every_typed_operation_it_admitted"].every((name) => runtimeContractTests.includes(`async fn ${name}()`))
    && ordered(body(text, "dispatch_typed_command_inner"), ["self.tools.live_runtime_instance_id != Some(meta.instance_id)", "self.media.require_complete_tool_operation_pipeline(&admission)?", "admission.verb != verb", "self.media.admit_typed_operation_slot(&*self.pending, &*self.tools)", "registration.latest_wins_command_disposer", "self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = Some(operation_id.0)", "self.tools.latest_wins_order.push(operation_id.0)"]);

  if (!exact(source)) throw new Error("latest-wins production admission/publication authority is incomplete");
  for (const [, token] of obligations) if (exact(source.replaceAll(token, "unqualified_authority"))) throw new Error(`latest-wins accepts missing authority: ${token}`);
  const dispatch = body(source, "dispatch_typed_command_inner");
  const admissionToken = "self.media.admit_typed_operation_slot(&*self.pending, &*self.tools)";
  const reordered = source.replace(dispatch, dispatch.replace(admissionToken, "unqualified_authority()") + admissionToken);
  if (reordered === source || exact(reordered)) throw new Error("latest-wins accepts slot admission after cancellation binding and enqueue");
  if (body(source, "publish_mounted_typed_operation_unit").includes(".await")) throw new Error(`latest-wins publication awaits while holding app/document/operation claims; ${obligations.length} current authority obligations, their removal controls, and admission ordering control passed`);
  const taskSchema = JSON.parse(readFileSync(join(base, "⚛️reactor/🧵️task/🧬️schema/🔣️.json"), "utf8"));
  const taskFixture = JSON.parse(readFileSync(join(base, "⚛️reactor/🧵️task/🧪️tests/🧫️fixtures/📥️admission/🔣️.json"), "utf8"));
  const validateTask = ajv.compile(taskSchema);
  if (!validateTask(taskFixture) || new Set(taskFixture.observations.map((row: { phase: string }) => row.phase)).size !== 7) throw new Error("task admission fixture violates its synchronous ownership boundary");
  const reactor = ["⚛️reactor/⏲️tasks/🧵️task-lane/🦀️.rs", "⚛️reactor/⏲️tasks/🔗️extension-continuation/🦀️.rs", "📜️declaration/📨️emission/🦀️.rs"].map((file) => readFileSync(join(base, file), "utf8")).join("\n");
  const host = readFileSync(join(base, "🌐host/🦀️.rs"), "utf8");
  const taskAdmissionExact = (main: string, tasks: string, handles: string): boolean => {
    const start = tasks.indexOf("pub fn spawn_task<");
    const block = start < 0 ? undefined : toolJobRustBlock(tasks, tasks.indexOf("{", start));
    const installed = block?.body.indexOf("reservation.install(Box::pin(async move {") ?? -1;
    if (!block || installed < 0) return false;
    const admission = block.body.slice(0, installed);
    const execution = block.body.slice(installed);
    return !main.includes("async fn publish_mounted_typed_operation_unit")
      && !tasks.includes("pub async fn into_parts") && tasks.includes("pub fn into_parts(self)")
      && !tasks.includes("async fn instance_task_quota") && tasks.includes("fn instance_task_quota")
      && !tasks.includes("pub async fn host_for_instance") && tasks.includes("pub fn host_for_instance")
      && !handles.includes("pub async fn new(registry:") && handles.includes("pub fn new(registry:")
      && !admission.includes(".await") && !admission.includes("run(ctx)")
      && execution.includes("let future = cold_run.take().run(ctx);") && execution.includes("future.await")
      && ordered(admission, ["instance_task_quota(instance)", "if live >= quota", "task.into_parts()", "executor.reserve()", "insert_admitted(task_id", "let mut cold_run = crate::app_emission::ColdTaskRun::new(run)"]);
  };
  if (!taskAdmissionExact(source, reactor, host)) throw new Error("task publication invokes or polls user work before releasing its synchronous admission claim");
  const taskHostiles = [
    [source.replace("fn publish_mounted_typed_operation_unit", "async fn publish_mounted_typed_operation_unit"), reactor, host],
    [source, reactor.replace("pub fn spawn_task<", "pub async fn spawn_task<"), host],
    [source, reactor.replace("reservation.install(Box::pin(async move {", "let future = run.run(ctx); reservation.install(Box::pin(async move {"), host],
    [source, reactor, host.replace("pub fn new(registry:", "pub async fn new(registry:")],
  ] as const;
  for (const [main, tasks, handles] of taskHostiles) if (taskAdmissionExact(main, tasks, handles)) throw new Error("task admission accepts a suspended or prematurely invoked user factory");
  const rawFixture = JSON.parse(readFileSync(join(base, "🧵️retained-command/🧪️tests/🧫️fixtures/🚪️raw-allocation-close.json"), "utf8"));
  for (const law of rawFixture.cases) {
    const allocation=new ArrayBuffer(law.capacity);
    const oracle=Buffer.from(allocation).subarray(0,law.initializedBytes);
    if (law.initializedBytes > law.capacity || oracle.byteLength !== law.initializedBytes || oracle.buffer !== allocation || law.expectedCopiedBytes !== 0 || rawFixture.maximumCopyBytes !== 0 || oracle.buffer.byteLength !== law.expectedReleasedBytes) throw new Error(`retained raw allocation copy and physical release oracle: ${law.id}`);
  }
  const rawSource = readModuleTreeSource(join(base, "🧵️retained-command/🦀️.rs"));
  const rawCloseSignature = "fn close_one_original(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>";
  const rawCloseBlock = (text: string) => {
    const start = text.indexOf(rawCloseSignature);
    return start < 0 ? undefined : toolJobRustBlock(text, text.indexOf("{", start + rawCloseSignature.length - 1));
  };
  return { rawCloseBlock, rawSource, base, Ajv, source, body, ajv, fixture, obligations, integration, taskFixture, taskHostiles, rawFixture };
}
