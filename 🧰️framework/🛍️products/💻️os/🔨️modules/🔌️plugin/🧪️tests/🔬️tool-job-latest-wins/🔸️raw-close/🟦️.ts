/** 🔸️ The raw close phase of toolJobLatestWinsSelfTests. */
import { join } from "node:path";
import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { WORKSPACE_ROOT } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";
import { toolJobRustBlock, toolJobProductionSource, toolJobPublicationFreshnessBeforeEveryTurn, toolJobRetainedDispatchSetup, toolJobTypedRouteFailsClosedBeforePreparation, toolJobTypedPersistentFoundation, toolJobEphemeralOneItemPublicationBounded, toolJobStoreBatchPublicationBounded, toolJobSourceMutation, toolJobStoreSource } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";
import { retainedCloneWireSchemaUrl } from "../../../../../../../🔨️modules/🌱️value/📦️packages/🦀️rust/🧪️testing/🧫️fixtures/🟦️.ts";
import type { buildBaseState } from "../🔶️harness/🟦️.ts";

/** 🔹 Builds rawClose, mutateRawClose, rawHostiles, childCloseFixture of toolJobLatestWinsSelfTests. */
export function buildRawCloseState({ rawCloseBlock, rawSource, base, Ajv, source, body, ajv }: Pick<ReturnType<typeof buildBaseState>, "rawCloseBlock" | "rawSource" | "base" | "Ajv" | "source" | "body" | "ajv">) {
  const rawClose = (text: string): boolean => {
    const close = rawCloseBlock(text)?.body;
    const releaseStart = close?.indexOf("if self.raw.capacity()!=0{") ?? -1;
    const release = close && releaseStart >= 0 ? toolJobRustBlock(close, close.indexOf("{", releaseStart))?.body : undefined;
    return !!close && !close.includes("self.raw.is_empty()") && !close.includes("self.raw.truncate(") && !close.includes("self.raw.clear()") && !close.includes("self.raw.len().min(grant.maximum_copy_bytes)")
      && !!release && release.includes("let bytes=self.raw.capacity();")
      && release.includes("drop(std::mem::take(&mut self.raw))") && release.includes("released_bytes:bytes")
      && !release.includes("maximum_copy_bytes") && !close.includes("copied_bytes:bytes")
      && text.includes("RetirementDemand{release_bytes:self.raw.capacity(),depth:1,..Default::default()}")
      && text.includes("fn test_raw_allocation_close<A: ArtifactApp>()")
      && text.includes("assert_eq!(heap.released_bytes,progress.released_bytes)");
  };
  if (!rawClose(rawSource)) throw new Error("retained command raw close loses independent copy or physical release authority");
  const mutateRawClose = (before: string, after: string): string => {
    const close = rawCloseBlock(rawSource);
    if (!close || !close.body.includes(before)) throw new Error("retained raw close hostile target is missing from its actual owning method");
    return rawSource.replace(close.body, close.body.replace(before, after));
  };
  const rawHostiles = [
    mutateRawClose("if self.raw.capacity()!=0{", "if !self.raw.is_empty(){let bytes=self.raw.len().min(grant.maximum_copy_bytes);self.raw.truncate(self.raw.len()-bytes);return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}});}if self.raw.capacity()!=0{"),
    mutateRawClose("if self.raw.capacity()!=0{", "if !self.raw.is_empty(){self.raw.clear();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}});}if self.raw.capacity()!=0{"),
    mutateRawClose("let bytes=self.raw.capacity();", "let bytes=self.raw.capacity();if grant.maximum_copy_bytes<self.raw.capacity(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()});}"),
    mutateRawClose("released_bytes:bytes", "released_bytes:0"),
    rawSource.replace("RetirementDemand{release_bytes:self.raw.capacity(),depth:1,..Default::default()}", "RetirementDemand{copy_bytes:self.raw.capacity(),depth:1,..Default::default()}"),
    rawSource.replace("assert_eq!(heap.released_bytes,progress.released_bytes)", "assert_eq!(heap.released_bytes,0)"),
  ];
  for (const hostile of rawHostiles) if (hostile === rawSource || rawClose(hostile)) throw new Error("retained raw close accepts lost physical custody or capacity-sized copy deadlock");
  const childCloseFixture = JSON.parse(readFileSync(join(base, "🧵️retained-command/🧪️tests/🧫️fixtures/🧩️child-prepublication-close.json"), "utf8"));
  const validateChildGrant = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(retainedCloneWireSchemaUrl, "utf8")));
  if (!validateChildGrant(childCloseFixture.closeGrant)) throw new Error("original child close caller violates its genuine neutral production grant");
  for (const axis of Object.keys(childCloseFixture.closeGrant)) {
    const denied = { ...childCloseFixture.closeGrant };
    delete denied[axis];
    if (validateChildGrant(denied)) throw new Error("original child close caller lost an independent grant axis");
  }
  if (JSON.stringify(childCloseFixture.children.map((child: { id: string }) => child.id).reverse()) !== JSON.stringify(childCloseFixture.expectedRetirementOrder)) throw new Error("retained child close LIFO oracle diverged");
  if (childCloseFixture.children.some((child: { slot: string; childId: string; value: string }) => [child.slot, child.childId, child.value].some(value => Buffer.byteLength(value, "utf8") <= value.length))) throw new Error("retained child close fixture lost its multibyte scalar oracle");
  const childMain = [source, ...["📜️declaration/📨️emission/🦀️.rs", "📜️declaration/👷️job/🎯️completion-and-owned-context/🦀️.rs"].map((file) => readFileSync(join(base, file), "utf8"))].join("\n");
  const completionHandbackExact = (text: string): boolean => {
    const owner = text.indexOf("impl<A: ArtifactApp> InteractiveJob for ArtifactRetainedCommandJob<A>");
    const start = owner < 0 ? -1 : text.indexOf("fn step", owner);
    const publication = start < 0 ? "" : toolJobRustBlock(text, text.indexOf("{", start))?.body.replace(/\s+/gu, "") ?? "";
    return ["Emit", "Download"].every(lane => new RegExp("self\\.rejected_completion=Some\\(ArtifactToolCompletionValue::" + lane + "\\(rejected\\." + lane.toLowerCase() + "\\.map_err\\((?:crate::completion_fault::)?ArtifactCompletionFault::new\\),rejected\\.ephemeral\\)\\);self\\.reducer_fault\\(rejected\\.fault\\);returnOk\\(None\\);").test(publication));
  };
  const rejectedCloseExact = (text: string): boolean => {
    const close = rawCloseBlock(text)?.body;
    const start = close?.indexOf("if self.rejected_completion.is_some()") ?? -1;
    const admission = close && start >= 0 ? toolJobRustBlock(close, close.indexOf("{", start))?.body.replace(/\s+/gu, "") ?? "" : "";
    return admission.includes("matchA::admit_completion_retirement(&mutself.rejected_completion,grant)")
      && admission.includes("Ok(Some((owner,progress)))=>{self.rejected_retirement=Some(owner);InteractiveJobCloseStep::Pending{progress}}")
      && admission.includes("Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}");
  };
  const rejectedDepthExact = (text: string): boolean => {
    const start = text.indexOf("if let Some(value)=self.rejected_completion.as_ref()");
    const demand = start < 0 ? "" : toolJobRustBlock(text, text.indexOf("{", start))?.body.replace(/\s+/gu, "") ?? "";
    return demand.includes("emit.child_close_demands(copy)?") && demand.includes('demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"retained rejected emitted child depth overflow"))?;returnOk(demand);'.replace(/\s+/gu, ""));
  };
  const childCloseExact = (main: string, retained: string): boolean => main.includes("pub fn close_one(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>")
    && main.includes("pub fn close_child_one(&mut self, grant: RetainedCloneGrant) -> Result<Option<PluginLifecycleStep>, Fault>")
    && rawCloseBlock(retained)?.body.split("emit.close_child_one(child)").length === 3
    && retained.includes("InteractiveJobCloseStep::Pending{progress}")
    && rawCloseBlock(retained)?.body.split("if progress.fits(child)").length === 3
    && completionHandbackExact(retained)
    && rejectedCloseExact(retained)
    && rejectedDepthExact(retained)
    && retained.includes("artifact_retirement_box_close_step(&mut self.rejected_retirement,grant)")
    && retained.includes("self.rejected_completion.is_none()") && retained.includes("self.rejected_retirement.is_none()")
    && main.split("return Err(ArtifactToolCompletionRejection { emit, ephemeral, fault:").length - 1 === 2
    && main.includes("mounted owned child group requires its one exact admitted typed source family");
  if (!childCloseExact(childMain, rawSource)) throw new Error("retained ChildEmit full grant, physical receipt and rejected completion handback are incomplete");
  const childCloseHostiles = [
    [childMain.replaceAll("pub fn close_one(&mut self, grant: RetainedCloneGrant)", "fn close_one(&mut self, grant: RetainedCloneGrant)"), rawSource],
    [childMain, rawSource.replace("emit.close_child_one(child)", "drop(self.emit.take())")],
    [childMain, rawSource.replace("if progress.fits(child)", "if true")],
    [source, rawSource.replace("rejected.emit.map_err", "discarded_emit.map_err")],
    [source, rawSource.replace("rejected.download.map_err", "discarded_download.map_err")],
    [source, rawSource.replace("rejected.ephemeral", "discarded_ephemeral")],
    [source, rawSource.replace("A::admit_completion_retirement(&mut self.rejected_completion,grant)", "A::admit_completion_retirement(&mut discarded_completion,grant)")],
    [source, rawSource.replace("self.rejected_retirement=Some(owner)", "drop(owner)")],
    [source, rawSource.replace("self.rejected_retirement=Some(owner);InteractiveJobCloseStep::Pending{progress}", "self.rejected_retirement=Some(owner);InteractiveJobCloseStep::Pending{progress:Default::default()}")],
    [source, mutateRawClose("Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}", "Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:Default::default()}")],
    [source, rawSource.replace('demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"retained rejected emitted child depth overflow"))?', "demand.depth+=1")],
    [source, rawSource.replace("artifact_retirement_box_close_step(&mut self.rejected_retirement,grant)", "artifact_retirement_box_close_step(&mut self.rejected_retirement,Default::default())")],
    [childMain.replace("return Err(ArtifactToolCompletionRejection { emit, ephemeral, fault:", "return Err(ArtifactToolCompletionRejection { emit: discarded_emit, ephemeral, fault:"), rawSource],
    [childMain.replace("mounted owned child group requires its one exact admitted typed source family", "typed child output is accepted"), rawSource],
  ] as const;
  for (const [hostileMain, hostileRetained] of childCloseHostiles) if (childCloseExact(hostileMain, hostileRetained)) throw new Error("retained ChildEmit close oracle accepted lost ownership or bounded-factory publication");
  for (const path of ["🦀️.rs", "📜️declaration/📨️emission/🦀️.rs", "🧵️retained-command/🦀️.rs", "🧪️tests/🧩️composition/📨️emission/🦀️.rs"]) {
    const result = spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", join(base, path)], { stdio: ["ignore", "ignore", "pipe"], timeout: 60000 });
    if (result.status !== 0) throw new Error(`original child source grammar refused ${path}: ${result.error?.message ?? result.signal ?? result.status}; ${result.stderr?.toString()}`);
  }
  const storeSource=toolJobStoreSource(WORKSPACE_ROOT);
  const publisherStart = source.lastIndexOf("fn publish_mounted_typed_operation_unit(");
  const mutatePublisher = (before: string, after: string): string => source.slice(0, publisherStart) + source.slice(publisherStart).replace(before, after);
  const admittedBatchSourceExact = (text: string): boolean => {
    const publisher = body(text, "publish_mounted_typed_operation_unit").replace(/\s+/gu, ""), lanes = {store:"artifact_mutations",config_store:"config_mutations",draft_store:"draft_mutations"};
    const calls = [...publisher.matchAll(/self\.stores\.(store|config_store|draft_store)\.admit_apply_batch\((.*?)\)\.map_err/gu)].map(call => {
      const [text, lane, args] = call;
      if (text === undefined || args === undefined || call.index === undefined || (lane !== "store" && lane !== "config_store" && lane !== "draft_store")) throw new Error("Exact mounted publication source match required");
      return { text, lane, args, index: call.index } as const;
    });
    return text.replace(/\s+/gu, "").includes("self.publish_mounted_typed_operation_unit(mounted,semio_framework_plugin_reactor_app::original_plugin_turn_grant(self.mounted_policy.preparation,semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,semio_framework_plugin_reactor_app::remaining_plugin_turn_grant(turn,*spent))))?") && calls.length > 0 && !publisher.includes(".await") && !publisher.includes(".last().cloned()") && Object.values(lanes).every(lane => calls.some(call => call.args.includes(`&mutemit.${lane},HistoryLane::Document`)) && !publisher.includes(`emit.${lane}.clone()`) && publisher.split(`emit.${lane}.clear()`).length - 1 === Number(lane === "artifact_mutations" && publisher.includes("pending_transaction_proposal=Some(TransactionProposalDraft{local_ops,foreign});emit.artifact_mutations.clear();")) && !publisher.includes(`drop(std::mem::take(&mutemit.${lane}))`)) && calls.every(call => call.args.includes(`&mutemit.${lanes[call.lane]},HistoryLane::Document`) && call.args.endsWith("retained") && (toolJobRustBlock(publisher,publisher.indexOf("{",call.index+call.text.length))?.body??"").includes("mounted.publication_ownership_progress=Some((retained,ownership))"));
  };
  const ingress = body(storeSource, "admit_apply_batch"), admission = ingress.indexOf("if grant.maximum_items==0"), transfer = ingress.indexOf("std::mem::take(mutations)");
  if (admission < 0 || transfer <= admission || !["grant.maximum_copy_bytes<copied_bytes", "grant.maximum_capacity_bytes<retained_capacity_bytes", "grant.maximum_depth==0", "transaction:transaction.take()", "RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes,released_bytes:0}"].every(marker => ingress.includes(marker)) || ingress.includes("mutations.clone()")) throw new Error("actual borrowed batch ingress loses funded original custody or its receipt");
  const mountedChecks: Array<[string, (text: string) => boolean, string]> = [
    ["synchronous funded original unit", (text) => { const publisher = body(text, "publish_mounted_typed_operation_unit"); return text.includes("fn publish_mounted_typed_operation_unit") && !text.includes("async fn publish_mounted_typed_operation_unit") && admittedBatchSourceExact(text) && ["presence", "transient"].every(lane => publisher.includes(`ephemeral.${lane}.pop()`) && publisher.includes(`ephemeral.${lane}.push(mutation)`)); }, mutatePublisher("fn publish_mounted_typed_operation_unit", "async fn publish_mounted_typed_operation_unit")],
    ["synchronous fresh publisher", toolJobPublicationFreshnessBeforeEveryTurn, mutatePublisher("typed_operation_document_is_fresh(&mounted.operation", "accept_stale_operation(&mounted.operation")],
    ["exact extracted setup", (text) => !!toolJobRetainedDispatchSetup(text), source.replace("self.as_dispatch().start_typed_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await", "self.as_dispatch().unchecked_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await")],
    ["unsupported generic reducer denial", toolJobTypedRouteFailsClosedBeforePreparation, source.replace("QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::Bounded(_) => {", "QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::Bounded(_) => { return Ok(());")],
    ["mounted persistent operation", toolJobTypedPersistentFoundation, toolJobSourceMutation(source, "pub fn drive_worker_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>)", "session.pump_one_for_interactive_turn(same)", "session.run_to_terminal(same)")],
    ["full maintenance eligibility scan", toolJobTypedPersistentFoundation, toolJobSourceMutation(source, "fn maintenance_tool_selection(&self) -> MaintenanceTool", "for offset in 0..ARTIFACT_LIVE_OUTPUT_SLOTS", "for offset in 0..1")],
    ["worker input-wait classification", toolJobTypedPersistentFoundation, source.replace('Ok(_)=>Ok(PluginLifecycleStep::AwaitingInput{reason:"typed operation mounted worker awaits its next outcome"})', 'Ok(_)=>Ok(PluginLifecycleStep::Blocked{reason:"typed operation mounted worker awaits its next outcome"})')],
    ["unbounded host worker turn", toolJobTypedPersistentFoundation, toolJobSourceMutation(source, "pub fn drive_typed_operation_worker(", "for _ in 0..INTERACTIVE_TURN_WORKER_PUMPS", "loop")],
    ["retained ephemeral publisher", (text) => toolJobEphemeralOneItemPublicationBounded(storeSource, text), source.replace("self.stores.presence_one_item_factory.as_deref()", "A::build_presence_store_one_item_preparation_factory()")],
  ];
  const [firstCheck] = mountedChecks;
  if (firstCheck === undefined) throw new Error("Mounted publication baseline required");
  mountedChecks.push(["move-only mutation ownership", firstCheck[1], mutatePublisher("&mut emit.artifact_mutations,HistoryLane::Document", "&mut emit.artifact_mutations.clone(),HistoryLane::Document")]);
  for (const lane of ["artifact_mutations", "config_mutations", "draft_mutations"]) {
    for (const hostile of [mutatePublisher(`&mut emit.${lane},HistoryLane::Document`, `&mut emit.${lane}.clone(),HistoryLane::Document`), mutatePublisher(`&mut emit.${lane},HistoryLane::Document`, "&mut Vec::new(),HistoryLane::Document"), mutatePublisher(`&mut emit.${lane},HistoryLane::Document`, `&mut { emit.${lane}.clear(); Vec::new() },HistoryLane::Document`)]) {
      if (hostile === source || firstCheck[1](hostile)) throw new Error("mounted batch publication accepted a copied or dropped original mutation owner");
    }
  }
  for (const [name, check, hostile] of mountedChecks) {
    if (!check(source)) throw new Error(`mounted source binding rejected its real ${name}`);
    if (hostile === source || check(hostile)) throw new Error(`mounted source binding accepted hostile ${name}`);
  }
  if (!toolJobStoreBatchPublicationBounded(storeSource, source)) throw new Error("mounted Store source binding lost its retained owned-preparation helper");
  const replayingOwnedBegin = toolJobSourceMutation(storeSource, "pub fn admit_apply_batch(", "let factory=factory.ok_or_else(", "replay_mutations(); let factory=factory.ok_or_else(");
  if (replayingOwnedBegin === storeSource || toolJobStoreBatchPublicationBounded(replayingOwnedBegin, source)) throw new Error("mounted Store source binding accepted replay inside extracted preparation");
  const dispatchFixture = JSON.parse(readFileSync(join(base, "🧵️retained-command/🧪️tests/🧫️fixtures/📌️mounted-dispatch-binding.json"), "utf8"));
  
  
  const validDispatch = ajv.compile({ const: "none" });
  const production = toolJobProductionSource(source);
  const mutateFunction = (text: string, name: string, before: string, after: string): string => {
    const start = text.lastIndexOf(`fn ${name}(`);
    const block = start < 0 ? undefined : toolJobRustBlock(text, text.indexOf("{", start));
    if (!block) throw new Error(`mounted dispatch hostile target missing: ${name}`);
    return text.slice(0, start) + text.slice(start, block.end).replace(before, after) + text.slice(block.end);
  };
  const pipelineGuard = "self.media.require_complete_tool_operation_pipeline(&admission)?";
  return { mutateFunction, dispatchFixture, pipelineGuard, production, validDispatch, rawHostiles, mountedChecks, childCloseHostiles };
}
