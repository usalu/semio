import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT, toolJobRustBlock, toolJobPublicationFreshnessBeforeEveryTurn, toolJobRetainedDispatchSetup, toolJobTypedRouteFailsClosedBeforePreparation, toolJobTypedPersistentFoundation, toolJobEphemeralOneItemPublicationBounded, toolJobStoreBatchPublicationBounded, toolJobProductionSource, toolJobMountedDispatchOneTurnExact } from "../../../../../../../📜️script.ts";

/** 🧪️ Cross-checks full-domain scope fixtures with Ajv equality and guards the active retained admission/publication seam. */
export function toolJobLatestWinsSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
  const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🥇️tool-latest-wins.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(base, "🧵️retained-command/🧬️schema/🔣️.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(schema);
  const validate = ajv.compile({ $ref: schema.$id + "#/$defs/ToolLatestWinsV1" });
  if (!validate(fixture)) throw new Error(`latest-wins schema: ${JSON.stringify(validate.errors)}`);
  const resultAckTrace = Array.from({ length: fixture.resultAck.preAckPolls + 1 }, (_, poll) => poll === 0 ? { attempt: 1 } : null);
  const resultAckObservation = { preAckPolls: fixture.resultAck.preAckPolls, deliveries: resultAckTrace.filter(Boolean).length, attempt: resultAckTrace.find(Boolean)?.attempt };
  if (!ajv.compile({ const: fixture.resultAck })(resultAckObservation)) throw new Error("latest-wins result ACK single-delivery oracle diverged");
  const equal = ajv.compile({ const: fixture.first });
  for (const law of fixture.cases) {
    if (Buffer.byteLength(law.next.target, "utf8") !== fixture.targetBytes || equal(law.next) !== law.superseded) throw new Error(`latest-wins exact scope oracle: ${law.id}`);
  }
  const hostiles = [
    { ...fixture, maximumItems: 2 },
    { ...fixture, maximumBytes: 8_192 },
    { ...fixture, first: { ...fixture.first, document: undefined } },
    { ...fixture, first: { ...fixture.first, inventedAuthority: true } },
  ];
  for (const hostile of hostiles) if (validate(hostile)) throw new Error("latest-wins schema accepted a forged scope or enlarged grant");
  const integration = JSON.parse(readFileSync(join(base, "🧫️fixtures/🔗️tool-latest-wins-integration.json"), "utf8"));
  const validateIntegration = ajv.compile({ $ref: schema.$id + "#/$defs/ToolLatestWinsIntegrationV1" });
  if (!validateIntegration(integration)) throw new Error(`latest-wins integration schema: ${JSON.stringify(validateIntegration.errors)}`);
  for (const law of integration.cases) {
    const same = ajv.compile({ const: law.firstTarget });
    if (same(law.nextTarget) !== law.firstCancelled) throw new Error(`latest-wins integration equality oracle: ${law.id}`);
  }
  if (Buffer.byteLength(integration.keyRetirement.text, "utf8") !== integration.keyRetirement.utf8Bytes
    || JSON.stringify(Array.from(integration.keyRetirement.text as string).reverse().map((scalar) => Buffer.byteLength(scalar, "utf8"))) !== JSON.stringify(integration.keyRetirement.scalarBytes)) throw new Error("latest-wins UTF-8 retirement byte oracle");
  const integrationHostiles = [
    { ...integration, maximumItems: 64 },
    { ...integration, rebase: { ...integration.rebase, cancelOldKey: true } },
    { ...integration, slotReservation: { ...integration.slotReservation, collisionAdmitted: true } },
    { ...integration, reclamation: { ...integration.reclamation, acceptedTargets: 64 } },
    { ...integration, fairness: { ...integration.fairness, secondPublishesWithinMetadataVisits: 65 } },
    { ...integration, keyRetirement: { ...integration.keyRetirement, utf8Bytes: 3 } },
    { ...integration, lostReservations: integration.lostReservations.map((law: object) => ({ ...law, rejectionAfterVacancy: false })) },
  ];
  for (const hostile of integrationHostiles) if (validateIntegration(hostile)) throw new Error("latest-wins integration schema accepted stale authority, collision, starvation, or a missing accepted target");
  const source = readFileSync(join(base, "🦀️.rs"), "utf8");
  const runtimeContractTests = readFileSync(join(base, "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"), "utf8");
  const body = (text: string, name: string): string => {
    const start = text.lastIndexOf(`fn ${name}(`);
    return start < 0 ? "" : toolJobRustBlock(text, text.indexOf("{", start))?.body ?? "";
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
    ["dispatch_typed_command_inner", "self.can_admit_typed_operation(operation_id.0)"],
    ["advance_latest_wins_admission_unit", ".rebind_keyed(base_revision, generation)?"],
    ["rebind_keyed", "scope.operation != self.key"],
    ["rebind_keyed", "scope.operation.generation = generation"],
    ["advance_typed_operation_publication_one", "next_id_from(self.typed_publication_cursor)"],
    ["take_typed_operation_result_page", "next_id_from(self.typed_result_cursor)"],
    ["take_result_page", "if self.result_page_presented"],
    ["take_result_page", "return None"],
    ["has_runnable_work", "MountedTypedCommandFullOperationStage::AwaitingAck => !self.result_page_presented"],
    ["cleanup_finished_slot", "scope.publication_claim.is_finished()"],
    ["release_current", "std::sync::Arc::ptr_eq(&scope.publication_claim, &self.publication_claim)"],
    ["try_claim_publication", "self.handle.publication_scope.try_claim()?"],
    ["try_claim_publication", "self.publication_claim.try_claim()?"],
  ];
  const exact = (text: string): boolean => obligations.every(([name, token]) => body(text, name).includes(token))
    && !body(text, "publish_mounted_typed_operation_unit").includes(".await")
    && !body(text, "publish_mounted_typed_operation_unit").includes("dispatch_emit_group(")
    && text.includes("self.token.child_now()")
    && text.includes("compare_exchange(0, 1, std::sync::atomic::Ordering::AcqRel")
    && text.includes("scope.operation")
    && runtimeContractTests.includes("async fn retained_latest_wins_real_document_publication_cancellation_and_delayed_ack_close()");
  if (!exact(source)) throw new Error("latest-wins production admission/publication authority is incomplete");
  for (const [, token] of obligations) if (exact(source.replaceAll(token, "unqualified_authority"))) throw new Error(`latest-wins accepts missing authority: ${token}`);
  const rawFixture = JSON.parse(readFileSync(join(base, "🧵️retained-command/🧫️fixtures/🚪️raw-allocation-close.json"), "utf8"));
  const validateRaw = ajv.compile({ $ref: schema.$id + "#/$defs/RawAllocationCloseV1" });
  if (!validateRaw(rawFixture)) throw new Error(`retained raw allocation schema: ${JSON.stringify(validateRaw.errors)}`);
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
  const validateChildClose = ajv.compile({ $ref: schema.$id + "#/$defs/ChildPrepublicationCloseV1" });
  if (!validateChildClose(childCloseFixture)) throw new Error(`retained child close fixture: ${JSON.stringify(validateChildClose.errors)}`);
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
  const validateDispatch = ajv.compile({ $ref: schema.$id + "#/$defs/MountedDispatchBindingV1" });
  if (!validateDispatch(dispatchFixture)) throw new Error(`mounted dispatch fixture: ${JSON.stringify(validateDispatch.errors)}`);
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
    "duplicate-pump": (text) => mutateFunction(text, dispatchFixture.helper, "let _ = active.drive_worker_step(&pool)?", "let _ = active.drive_worker_step(&pool)?; let _ = active.drive_worker_step(&pool)?"),
    "direct-reducer": (text) => mutateFunction(text, dispatchFixture.dispatcher, "self.require_complete_tool_operation_pipeline(&admission)?", "A::handle(&command).await; self.require_complete_tool_operation_pipeline(&admission)?"),
    "direct-dispatch": (text) => mutateFunction(text, dispatchFixture.dispatcher, "self.require_complete_tool_operation_pipeline(&admission)?", "self.tool_jobs.dispatch(operation_spec); self.require_complete_tool_operation_pipeline(&admission)?"),
    "run-to-completion": (text) => mutateFunction(text, dispatchFixture.helper, "let _ = active.drive_worker_step(&pool)?", "let _ = active.run_to_completion(&pool)?; let _ = active.drive_worker_step(&pool)?"),
  };
  if (new Set(dispatchFixture.cases.map((law: { mutation: string }) => law.mutation)).size !== Object.keys(mutations).length) throw new Error("mounted dispatch fixture omits an exact hostile case");
  for (const law of dispatchFixture.cases) {
    const changed = mutations[law.mutation]!(production);
    if (validDispatch(law.mutation) !== law.admitted || (law.mutation !== "none" && changed === production)) throw new Error(`mounted dispatch fixture oracle: ${law.mutation}`);
    if (toolJobMountedDispatchOneTurnExact(changed) !== law.admitted) throw new Error(`mounted dispatch exact helper law: ${law.mutation}`);
  }
  return fixture.cases.length + hostiles.length + obligations.length + integration.cases.length + integrationHostiles.length + rawFixture.cases.length + 4 + mountedChecks.length * 2 + 2 + dispatchFixture.cases.length * 2 + childCloseHostiles.length + 3;
}
