import { isDeepStrictEqual } from "node:util";
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT, toolJobRustBlock, toolJobPublicationFreshnessBeforeEveryTurn, toolJobRetainedDispatchSetup, toolJobTypedRouteFailsClosedBeforePreparation, toolJobTypedPersistentFoundation, toolJobEphemeralOneItemPublicationBounded, toolJobStoreBatchPublicationBounded, toolJobProductionSource, toolJobMountedDispatchOneTurnExact } from "../../../../../../../📜️script.ts";

/** 🧪️ Cross-checks full-domain scope fixtures with system deep equality and semantic Ajv scope validation and guards the active retained admission/publication seam. */
export function toolJobLatestWinsSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
  const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🥇️tool-latest-wins.json"), "utf8"));
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
    const integration = JSON.parse(readFileSync(join(base, "🧫️fixtures/🔗️tool-latest-wins-integration.json"), "utf8"));
  
  
  for (const law of integration.cases) {
    if (isDeepStrictEqual(law.nextTarget, law.firstTarget) !== law.firstCancelled) throw new Error(`latest-wins integration equality oracle: ${law.id}`);
  }
  if (Buffer.byteLength(integration.keyRetirement.text, "utf8") !== integration.keyRetirement.utf8Bytes
    || JSON.stringify(Array.from(integration.keyRetirement.text as string).reverse().map((scalar) => Buffer.byteLength(scalar, "utf8"))) !== JSON.stringify(integration.keyRetirement.scalarBytes)) throw new Error("latest-wins UTF-8 retirement byte oracle");
  const source = readFileSync(join(base, "🦀️.rs"), "utf8");
  const runtimeContractTests = readFileSync(join(base, "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"), "utf8");
  const body = (text: string, name: string): string => {
    const owner = name === "has_runnable_typed_operations" ? "impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp<A, M>" : ["take_result_page", "reject_cancelled_publication"].includes(name) ? "impl<A: ArtifactApp> MountedTypedCommandFullOperation<A>" : undefined;
    const ownerStart = owner ? text.indexOf(owner) : -1;
    const scope = owner ? ownerStart < 0 ? "" : toolJobRustBlock(text, text.indexOf("{", ownerStart))?.body ?? "" : text;
    const start = scope.lastIndexOf(`fn ${name}(`);
    return start < 0 ? "" : toolJobRustBlock(scope, scope.indexOf("{", start))?.body ?? "";
  };
  const obligations: Array<[string, string]> = [
    ["publish_mounted_typed_operation_unit", "mounted.reject_cancelled_publication()?"],
    ["publish_mounted_typed_operation_unit", "ToolCancellationLease::try_claim_publication"],
    ["reject_cancelled_publication", "pending.begin_close()"],
    ["reject_cancelled_publication", "self.publication = completion.take()?"],
    ["advance_latest_wins_command_one", "self.latest_wins_order.items.front().copied()"],
    ["advance_latest_wins_command_one", "self.start_typed_command_operation(command, admission"],
    ["advance_latest_wins_admission_unit", "pending.restarting = true"],
    ["advance_latest_wins_admission_unit", "self.latest_wins_keys.begin(operation, key.clone()"],
    ["advance_latest_wins_admission_unit", "registration.latest_wins_target"],
    ["dispatch_typed_command_inner", "registration.latest_wins_command_disposer"],
    ["dispatch_typed_command_inner", "self.tool_cancellations.begin_keyed"],
    ["dispatch_typed_command_inner", "self.latest_wins_order.push(operation_id.0)"],
    ["dispatch_typed_command_inner", "self.admit_typed_operation_slot()"],
    ["admit_typed_operation_slot", "!self.latest_wins_order.allocation_admitted"],
    ["admit_typed_operation_slot", "self.latest_wins_order.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS"],
    ["admit_typed_operation_slot", "(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot| self.typed_operation_slot_is_vacant(*slot))?"],
    ["admit_typed_operation_slot", "allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, slot as u64)"],
    ["typed_operation_slot_is_vacant", "self.tool_operations.slot_is_vacant(slot)"],
    ["typed_operation_slot_is_vacant", "self.typed_operation_reservations[slot].is_none()"],
    ["typed_operation_slot_is_vacant", "self.latest_wins_commands.slot_is_vacant(slot)"],
    ["typed_operation_slot_is_vacant", "self.segmented_downloads.slot_is_vacant(slot)"],
    ["typed_operation_slot_is_vacant", "self.segmented_closures.slot_is_vacant(slot)"],
    ["advance_latest_wins_admission_unit", ".rebind_keyed(base_revision, generation)?"],
    ["rebind_keyed", "scope.operation != self.key"],
    ["rebind_keyed", "scope.operation.generation = generation"],
    ["advance_typed_operation_publication_unit", "self.next_advanceable_typed_operation()"],
    ["advance_typed_operation_publication_unit", "self.typed_publication_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS"],
    ["next_advanceable_typed_operation", "(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset|"],
    ["next_advanceable_typed_operation", "(self.typed_publication_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS"],
    ["next_advanceable_typed_operation", "self.tool_operations.entry(index)"],
    ["next_advanceable_typed_operation", "operation.stage != MountedTypedCommandFullOperationStage::AwaitingAck"],
    ["next_advanceable_typed_operation", ".map(|(id, _)| (index, *id))"],
    ["take_typed_operation_result_page", "(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset|"],
    ["take_typed_operation_result_page", "(self.typed_result_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS"],
    ["take_typed_operation_result_page", "self.tool_operations.entry(index)"],
    ["take_typed_operation_result_page", "operation.meta.instance_id == receiver"],
    ["take_typed_operation_result_page", "!operation.result_page_presented && operation.result_page.is_some()"],
    ["take_typed_operation_result_page", "self.typed_result_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS"],
    ["take_typed_operation_result_page", "self.tool_operations.get_mut(operation_id)?.take_result_page()"],
    ["take_result_page", "if self.result_page_presented"],
    ["take_result_page", "return None"],
    ["has_runnable_typed_operations", "!self.tool_operations.is_empty()"],
    ["cleanup_finished_slot", "scope.publication_claim.is_finished()"],
    ["release_current", "std::sync::Arc::ptr_eq(&scope.publication_claim, &self.publication_claim)"],
    ["try_claim_publication", "self.handle.publication_scope.try_claim()?"],
    ["try_claim_publication", "self.publication_claim.try_claim()?"],
  ];
  const ordered = (scope: string, tokens: readonly string[]): boolean => {
    let previous = -1;
    return tokens.every((token) => { const index = scope.indexOf(token); const valid = index > previous; previous = index; return valid; });
  };
  const exact = (text: string): boolean => obligations.every(([name, token]) => body(text, name).includes(token))
    && !body(text, "publish_mounted_typed_operation_unit").includes("dispatch_emit_group(")
    && text.includes("self.token.child_now()")
    && text.includes("compare_exchange(0, 1, std::sync::atomic::Ordering::AcqRel")
    && text.includes("scope.operation")
    && ["retained_latest_wins_real_document_publication_cancellation_and_delayed_ack_close", "a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work", "a_status_only_host_call_finishes_every_typed_operation_it_admitted"].every((name) => runtimeContractTests.includes(`async fn ${name}()`))
    && ordered(body(text, "dispatch_typed_command_inner"), ["self.live_runtime_instance_id != Some(meta.instance_id)", "self.require_complete_tool_operation_pipeline(&admission)?", "admission.verb != verb", "self.admit_typed_operation_slot()", "registration.latest_wins_command_disposer", "ToolLatestWinsKeyCopy::new(meta.instance_id", "self.tool_cancellations.begin_keyed", "self.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = Some(operation_id.0)", "self.latest_wins_order.push(operation_id.0)"]);

  if (!exact(source)) throw new Error("latest-wins production admission/publication authority is incomplete");
  for (const [, token] of obligations) if (exact(source.replaceAll(token, "unqualified_authority"))) throw new Error(`latest-wins accepts missing authority: ${token}`);
  const dispatch = body(source, "dispatch_typed_command_inner");
  const admissionToken = "self.admit_typed_operation_slot()";
  const reordered = source.replace(dispatch, dispatch.replace(admissionToken, "unqualified_authority()") + admissionToken);
  if (reordered === source || exact(reordered)) throw new Error("latest-wins accepts slot admission after cancellation binding and enqueue");
  if (body(source, "publish_mounted_typed_operation_unit").includes(".await")) throw new Error(`latest-wins publication awaits while holding app/document/operation claims; ${obligations.length} current authority obligations, their removal controls, and admission ordering control passed`);
  const rawFixture = JSON.parse(readFileSync(join(base, "🧵️retained-command/🧫️fixtures/🚪️raw-allocation-close.json"), "utf8"));
  
  
  for (const law of rawFixture.cases) {
    const oracle = Buffer.alloc(law.capacity).subarray(0, law.initializedBytes);
    if (oracle.byteLength !== law.expectedByteRelease || law.capacity <= rawFixture.maximumBytes) throw new Error(`retained raw allocation initialized-byte oracle: ${law.id}`);
  }
  const rawSource = readFileSync(join(base, "🧵️retained-command/🦀️.rs"), "utf8");
  const rawClose = (text: string): boolean => text.includes("if self.raw.capacity() != 0 {\n            if maximum_items == 0 {")
    && !text.includes("maximum_bytes < self.raw.capacity()") && !text.includes("let released = self.raw.capacity()")
    && text.includes("fn test_raw_allocation_close<A: ArtifactApp>()");
  if (!rawClose(rawSource)) throw new Error("retained command raw capacity incorrectly consumes semantic byte credit");
  if (rawClose(rawSource.replace("if self.raw.capacity() != 0 {\n            if maximum_items == 0 {", "if self.raw.capacity() != 0 {\n            if maximum_items == 0 || maximum_bytes < self.raw.capacity() {"))) throw new Error("retained raw close accepts capacity-sized byte deadlock");
  const childCloseFixture = JSON.parse(readFileSync(join(base, "🧵️retained-command/🧫️fixtures/🧩️child-prepublication-close.json"), "utf8"));
  
  
  if (JSON.stringify(childCloseFixture.children.map((child: { id: string }) => child.id).reverse()) !== JSON.stringify(childCloseFixture.expectedRetirementOrder)) throw new Error("retained child close LIFO oracle diverged");
  if (childCloseFixture.children.some((child: { slot: string; childId: string; value: string }) => [child.slot, child.childId, child.value].some(value => Buffer.byteLength(value, "utf8") <= value.length))) throw new Error("retained child close fixture lost its multibyte scalar oracle");
  const childCloseExact = (main: string, retained: string): boolean => main.includes("pub(crate) fn close_one(&mut self, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep")
    && retained.includes("if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes)")
    && retained.includes("self.emit = rejected.emit.ok()")
    && retained.includes("self.ephemeral = Some(rejected.ephemeral)")
    && main.includes("self.emit = Some(rejected.emit)")
    && main.includes("self.ephemeral = Some(rejected.ephemeral)")
    && main.includes("typed child output lacks a retained nested producer");
  if (!childCloseExact(source, rawSource)) throw new Error("retained ChildEmit close and rejected completion handback are incomplete");
  const childCloseHostiles = [
    [source.replace("pub(crate) fn close_one(&mut self, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep", "fn close_one(&mut self, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep"), rawSource],
    [source, rawSource.replace("if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes)", "drop(self.emit.take())")],
    [source, rawSource.replace("self.emit = rejected.emit.ok()", "drop(rejected.emit)")],
    [source.replace("self.emit = Some(rejected.emit)", "drop(rejected.emit)"), rawSource],
    [source.replace("typed child output lacks a retained nested producer", "typed child output is accepted"), rawSource],
  ];
  for (const [hostileMain, hostileRetained] of childCloseHostiles) if (childCloseExact(hostileMain, hostileRetained)) throw new Error("retained ChildEmit close oracle accepted lost ownership or bounded-factory publication");
  const storeSource = readFileSync(join(base, "../🏪️store/🦀️.rs"), "utf8");
  const publisherStart = source.lastIndexOf("fn publish_mounted_typed_operation_unit(");
  const mutatePublisher = (before: string, after: string): string => source.slice(0, publisherStart) + source.slice(publisherStart).replace(before, after);
  const mountedChecks: Array<[string, (text: string) => boolean, string]> = [
    ["synchronous move-only unit", (text) => { const publisher = body(text, "publish_mounted_typed_operation_unit"); return text.includes("fn publish_mounted_typed_operation_unit") && !text.includes("async fn publish_mounted_typed_operation_unit") && ["artifact_mutations", "config_mutations", "draft_mutations", "presence", "transient"].every((lane) => publisher.includes(`${lane}.pop()`)) && !publisher.includes(".last().cloned()") && !publisher.includes(".await"); }, mutatePublisher("fn publish_mounted_typed_operation_unit", "async fn publish_mounted_typed_operation_unit")],
    ["synchronous fresh publisher", toolJobPublicationFreshnessBeforeEveryTurn, mutatePublisher("typed_operation_document_is_fresh(&mounted.operation", "accept_stale_operation(&mounted.operation")],
    ["exact extracted setup", (text) => !!toolJobRetainedDispatchSetup(text), source.replace("self.start_typed_command_operation(command, admission, meta, operation_id, None).await", "self.unchecked_command_operation(command, admission, meta, operation_id, None).await")],
    ["unsupported generic reducer denial", toolJobTypedRouteFailsClosedBeforePreparation, source.replace("QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::Bounded(_) => {", "QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::Bounded(_) => { return Ok(());")],
    ["mounted persistent operation", toolJobTypedPersistentFoundation, source.replace("session.pump_one(pool, semio_framework_async::Lane::Interactive)", "session.run_to_terminal(pool)")],
    ["full maintenance eligibility scan", toolJobTypedPersistentFoundation, source.replace("for offset in 0..ARTIFACT_LIVE_OUTPUT_SLOTS", "for offset in 0..1")],
    ["worker input-wait classification", toolJobTypedPersistentFoundation, source.replace('Ok(_) => Ok(PluginCloseStep::AwaitingInput { reason: "typed operation mounted worker awaits its next outcome" })', 'Ok(_) => Ok(PluginCloseStep::Blocked { reason: "typed operation mounted worker awaits its next outcome" })')],
    ["transient scheduler-wait classification", toolJobTypedPersistentFoundation, source.replace('Ok(PluginCloseStep::AwaitingInput { reason: "typed operation mounted worker awaits transient scheduler authority" })', 'Ok(PluginCloseStep::Blocked { reason: "typed operation mounted worker awaits transient scheduler authority" })')],
    ["retained ephemeral publisher", (text) => toolJobEphemeralOneItemPublicationBounded(storeSource, text), source.replace("self.presence_one_item_factory.as_deref()", "A::build_presence_store_one_item_preparation_factory()")],
  ];
  mountedChecks.push(["move-only mutation ownership", mountedChecks[0][1], mutatePublisher("std::mem::take(&mut emit.artifact_mutations)", "emit.artifact_mutations.last().cloned()")]);
  for (const [name, check, hostile] of mountedChecks) {
    if (!check(source)) throw new Error(`mounted source binding rejected its real ${name}`);
    if (hostile === source || check(hostile)) throw new Error(`mounted source binding accepted hostile ${name}`);
  }
  if (!toolJobStoreBatchPublicationBounded(storeSource, source)) throw new Error("mounted Store source binding lost its retained owned-preparation helper");
  const replayingOwnedBegin = storeSource.replace("let footprint = match source.footprint(lane) {", "replay_mutations(); let footprint = match source.footprint(lane) {");
  if (replayingOwnedBegin === storeSource || toolJobStoreBatchPublicationBounded(replayingOwnedBegin, source)) throw new Error("mounted Store source binding accepted replay inside extracted preparation");
  const dispatchFixture = JSON.parse(readFileSync(join(base, "🧵️retained-command/🧫️fixtures/📌️mounted-dispatch-binding.json"), "utf8"));
  
  
  const validDispatch = ajv.compile({ const: "none" });
  const production = toolJobProductionSource(source);
  const mutateFunction = (text: string, name: string, before: string, after: string): string => {
    const start = text.lastIndexOf(`async fn ${name}(`);
    const block = start < 0 ? undefined : toolJobRustBlock(text, text.indexOf("{", start));
    if (!block) throw new Error(`mounted dispatch hostile target missing: ${name}`);
    return text.slice(0, start) + text.slice(start, block.end).replace(before, after) + text.slice(block.end);
  };
  const mutations: Record<string, (text: string) => string> = {
    none: (text) => text,
    "wrong-helper": (text) => text.replace("self.start_typed_command_operation(command, admission, meta, operation_id, None).await", "self.other_command_operation(command, admission, meta, operation_id, None).await"),
    "missing-helper": (text) => text.replace("async fn start_typed_command_operation(", "async fn unrelated_command_operation("),
    "duplicate-helper": (text) => `${text}\nasync fn start_typed_command_operation() {}`,
    "missing-pipeline-guard": (text) => mutateFunction(text, dispatchFixture.dispatcher, "self.require_complete_tool_operation_pipeline(&admission)?", "self.accept_incomplete_pipeline(&admission)?"),
    "duplicate-session": (text) => mutateFunction(text, dispatchFixture.helper, "let (session, session_rejected) = match semio_framework_job::MountedWorkerJobSession::try_new", "semio_framework_job::MountedWorkerJobSession::try_new(extra, params); let (session, session_rejected) = match semio_framework_job::MountedWorkerJobSession::try_new"),
    "duplicate-pump": (text) => mutateFunction(text, dispatchFixture.helper, "let _ = active.drive_worker_step(&pool, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)?", "let _ = active.drive_worker_step(&pool, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)?; let _ = active.drive_worker_step(&pool, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)?"),
    "direct-reducer": (text) => mutateFunction(text, dispatchFixture.dispatcher, "self.require_complete_tool_operation_pipeline(&admission)?", "A::handle(&command).await; self.require_complete_tool_operation_pipeline(&admission)?"),
    "direct-dispatch": (text) => mutateFunction(text, dispatchFixture.dispatcher, "self.require_complete_tool_operation_pipeline(&admission)?", "self.tool_jobs.dispatch(operation_spec); self.require_complete_tool_operation_pipeline(&admission)?"),
    "run-to-completion": (text) => mutateFunction(text, dispatchFixture.helper, "let _ = active.drive_worker_step(&pool, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)?", "let _ = active.run_to_completion(&pool)?; let _ = active.drive_worker_step(&pool, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)?"),
  };
  if (new Set(dispatchFixture.cases.map((law: { mutation: string }) => law.mutation)).size !== Object.keys(mutations).length) throw new Error("mounted dispatch fixture omits an exact hostile case");
  for (const law of dispatchFixture.cases) {
    const changed = mutations[law.mutation]!(production);
    if (validDispatch(law.mutation) !== law.admitted || (law.mutation !== "none" && changed === production)) throw new Error(`mounted dispatch fixture oracle: ${law.mutation}`);
    if (toolJobMountedDispatchOneTurnExact(changed) !== law.admitted) throw new Error(`mounted dispatch exact helper law: ${law.mutation}`);
  }
  return fixture.cases.length + 3 + obligations.length + integration.cases.length + rawFixture.cases.length + 4 + mountedChecks.length * 2 + 2 + dispatchFixture.cases.length * 2 + childCloseHostiles.length + 3;
}
