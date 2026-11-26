/** 🌉️ Component bridge source of the plugin web materialization: the generated jco component bridge that wraps one actor activation behind the actor api. */
import { originalOperationWorkerSource } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🎟️retained-turn/🟦️.ts";
import { pluginStagedGuestSource } from "../../🎚️staged-guest/🟦️.ts";
import { ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES, encodeActorInstanceLifecycle } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts";
import { ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES, encodeActorUiPatchReceipt, validateActorUiPatchPairing } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch/🟦️.ts";
import { COMMAND_INGRESS_KINDS } from "../../../../../../../📦️packages/🟦️typescript/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts";
import { actorCodecRuntimeSourceV1 } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧬️codec/🟦️.ts";
import { APP_CHANNEL_VERSION, admitGuestChannelVersion } from "../../../../../../../📦️packages/🟦️typescript/🧾️app-channel/🟦️.ts";
import { PLUGIN_HOST_IMPORT, PLUGIN_HOST_SHIM_FILE } from "../🧾️contract/🟦️.ts";

/** 🧱️ The component table of a deployment that stages every component a descriptor declares in the one factory `componentBase`: each
 * declared id maps to it, so an activation that names its component reaches the module that carries all of them. Empty for a descriptor
 * that declares no components. */
export function monolithicComponentTable(descriptor: { readonly components?: readonly { readonly id: string }[] }, componentBase: string): Readonly<Record<string, string>> {
  return Object.fromEntries((descriptor.components ?? []).map((component) => [component.id, componentBase]));
}

/**
 * 🌉️ Normalizes ONE actor's jco-transpiled component (`world actor`: exports `reactor`/
 * `jobs`/`checkpoint`/`describe`, imports only `pure` — see `component.wit`) behind the flat
 * `createActorApi()` shape `🟨️shard-worker.js` calls: `poll`/`startJob`/`stepJob`/`cancelJob`/
 * `takeSegmentedDownloadChunk`/`checkpoint`/`restore`, and `codec` — the component's `codec` interface
 * answered by `actorCodecAnswer` (`🎭️actor/🧬️codec/🟦️.ts`), the browser's
 * kind identity and document genesis.
 *
 * DROPS the old `runSerialized` retry/reload loop entirely (design-runtime.md §3: "recovery is the
 * kernel's job now"). Under the old ABI a guest panic (`panic = "abort"`, no unwind) permanently
 * killed the wasm32-wasip2 instance, and — with no host-side supervisor — the ONLY recovery available
 * was this bridge silently re-importing the module and replaying. Now `ActivationRegistry`'s
 * `FailurePolicy` (design-runtime.md §1) owns that: a trap here just throws, `🟨️shard-worker.js`'s
 * `replyError` propagates it to `ShardClient` as a rejected turn, and the KERNEL decides
 * `Trapped{restarts}` → drop + re-instantiate (fresh `activate`) + `restore()` the last checkpoint —
 * the SAME re-instantiation this bridge used to do blindly, now a supervised decision instead of a
 * local guess with no visibility into checkpoint state.
 *
 * Browser publication instantiates the compiled guest and checks every interface and method
 * declared by the actor WIT before reading its descriptor. See
 * `PLUGIN_DESCRIPTOR_PROBE_SOURCE` in `../../🛂️descriptor/🟦️.ts`.
 *
 * 🧪️ terra-web-bridges: every destructured method now returns a Promise (every WIT function in the
 * target world is `async func`) — made EXPLICITLY `async` here rather than relying on bare pass-
 * through.
 *
 * 🏭️ The component module is a jco instantiation factory imported once per worker under its rebuild
 * version only: each activation creates its own host (`createActorHost`) and instance, and asks the
 * worker's `coreModule(url)` for every core module, so a core module is compiled once per page and
 * shared by every activation, while the instance, its linear memory and its host live only as long as
 * the returned API. A per-activation module URL would have kept every instance in the browser's module
 * map for the page's lifetime.
 *
 * 🧩️ A package of several deployable components (`PackageDescriptor.components`) stages each one's factory beside the others and
 * passes `components` (component id → its jco base name): an activation names its component, the bridge imports only that factory
 * and asks only for its core modules, so a page compiles the code of the apps it opens and no other. An activation that names no
 * component runs `componentBase`, the package's only or first component.
 */
export function pluginComponentBridgeSource(componentBase: string, components: Readonly<Record<string, string>> = {}): string {
  return `/** @generated semio actor jco component bridge */
${originalOperationWorkerSource()}
${pluginStagedGuestSource()}

const ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES = ${ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES};
const encodeActorInstanceLifecycle = ${encodeActorInstanceLifecycle.toString()};
const ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES = ${ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES};
const encodeActorUiPatchReceipt = ${encodeActorUiPatchReceipt.toString()};
const validateActorUiPatchPairing = ${validateActorUiPatchPairing.toString()};
const commandIngressKinds = new Map([${COMMAND_INGRESS_KINDS.map((tag, kind) => `[${kind}, "${tag}"]`).join(", ")}]);
${actorCodecRuntimeSourceV1()}
const HOST_CHANNEL = ${APP_CHANNEL_VERSION};
const admitGuestChannelVersion = ${admitGuestChannelVersion.toString()};


function lifecycleBody(value) {
  return { lifetime: value.lifetime, requestSequence: BigInt(value.requestSequence), ...("closeGeneration" in value ? { closeGeneration: value.closeGeneration } : {}) };
}

function lifecycleEvent(kind, payload, activationGeneration) {
  if (kind === "patch-ack" || kind === "patch-rejected") {
    encodeActorUiPatchReceipt(payload.receipt);
    if (payload.receipt.lifetime.activationGeneration !== activationGeneration || payload.surface?.instance !== payload.receipt.lifetime.instanceId) throw new Error("actor-ui-patch.activation-mismatch");
    return { tag: kind, val: payload };
  }
  if (kind === "instance-open") {
    encodeActorInstanceLifecycle({ kind: "open", activationGeneration: payload.activationGeneration, instanceId: payload.instance, requestSequence: payload.requestSequence });
    if (payload.activationGeneration !== activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
    return { tag: kind, val: { ...payload, requestSequence: BigInt(payload.requestSequence) } };
  }
  if (kind === "instance-close" || kind === "instance-lifecycle-ack") {
    if (payload?.kind !== (kind === "instance-close" ? "close" : "ack")) throw new Error("actor-lifecycle.event-kind");
    encodeActorInstanceLifecycle(payload);
    const receipt = kind === "instance-close" ? payload : payload.receipt;
    if (receipt.lifetime.activationGeneration !== activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
    return { tag: kind, val: kind === "instance-close" ? lifecycleBody(receipt) : { tag: receipt.kind, val: lifecycleBody(receipt) } };
  }
  return kind === "wake" ? ({ tag: kind }) : ({ tag: kind, val: payload });
}

function lifecycleReceipt(raw, activationGeneration) {
  const value = optionValue(raw);
  if (value === undefined || value === null) return undefined;
  if (value.tag !== "captured" && value.tag !== "accepted" && value.tag !== "retired") throw new Error("actor-lifecycle.receipt-required");
  const body = value.val;
  if (typeof body?.requestSequence !== "bigint" || body.requestSequence <= 0n || body.requestSequence > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("actor-lifecycle.request-sequence");
  if (body.lifetime?.activationGeneration !== activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
  return encodeActorInstanceLifecycle({ kind: value.tag, lifetime: body.lifetime, requestSequence: Number(body.requestSequence), ...(value.tag === "captured" ? {} : { closeGeneration: body.closeGeneration }) });
}

function normalizeCommandIngress(status) {
  const originalMediaAck = optionValue(status.originalMediaAck);
  if (originalMediaAck !== undefined && originalMediaAck !== null) return { tag: "original-media-ack", val: originalMediaAck };

  const tag = commandIngressKinds.get(status.kind);
  if (!tag) throw new Error(\`unknown command ingress kind: \${status.kind}\`);
  if (tag === "idle") return { tag };
  if (tag === "fault") return { tag, val: { cursor: status.cursor, fault: { tag: "fault", val: status.fault } } };
  return { tag, val: status.cursor };
}

function uiPatchReceipt(result, activationGeneration) {
  if (!Array.isArray(result.uiPatches)) throw new Error("actor-ui-patch.envelope");
  const receipt = optionValue(result.uiPatchReceipt);
  validateActorUiPatchPairing(result.uiPatches.length, receipt);
  if (receipt === undefined || receipt === null) return undefined;
  if (receipt.lifetime.activationGeneration !== activationGeneration) throw new Error("actor-ui-patch.activation-mismatch");
  return encodeActorUiPatchReceipt(receipt);
}

function nativeRefusalError(failure, detail) {
  const refusal = (failure?.payload ?? failure)?.failure;
  const tag = refusal?.tag;
  if (typeof tag === "string") return new Error("original-turn.native-return-refused: " + tag + (typeof refusal.val === "string" ? " (" + refusal.val + ")" : "") + (detail ? ": " + detail : ""), { cause: failure });
  return failure instanceof Error ? failure : undefined;
}

const COMPONENT_BASES = new Map(${JSON.stringify(Object.entries(components))});

function componentBaseFor(component) {
  if (component === null || component === undefined) return ${JSON.stringify(componentBase)};
  const base = COMPONENT_BASES.get(component);
  if (base === undefined) throw new Error("actor-component.unknown: this module stages no component " + JSON.stringify(component));
  return base;
}

function versionedUrl(path) {
  const url = new URL(path, import.meta.url);
  const rebuildVersion = new URL(import.meta.url).searchParams.get("v");
  if (rebuildVersion) url.searchParams.set("v", rebuildVersion);
  return url.href;
}

export async function createActorApi(actorId, activationGeneration, originalAdmission, operationClockUs, originalReturn, coreModule, component = null) {
  if (typeof activationGeneration !== "bigint" || activationGeneration <= 0n || activationGeneration > 0xffffffffffffffffn) throw new Error("actor-close.invalid-activation-generation");
  if (typeof coreModule !== "function") throw new Error("actor-core-module.loader-required");
  const base = componentBaseFor(component);
  const [hostModule, factory] = await Promise.all([import(versionedUrl("./${PLUGIN_HOST_SHIM_FILE}")), import(versionedUrl("./" + base + ".js"))]);
  if (typeof factory.instantiate !== "function" || typeof hostModule.createActorHost !== "function") throw new Error("actor-component.not-a-factory: the staged component must be a jco instantiation factory (--instantiation async) with a host factory");
  const hostShim = hostModule.createActorHost(actorId, activationGeneration);
  const instance = await factory.instantiate((name) => coreModule(versionedUrl("./" + name)), { ...factory.wasiImports, ${JSON.stringify(PLUGIN_HOST_IMPORT)}: hostShim });
  const { reactor, jobs, checkpoint, describe, codec } = pluginStagedGuest(instance);
  hostShim.__bindOriginalClock(operationClockUs);
  const originalSource = new OriginalOperationSource(originalAdmission, () => hostShim.operationClockUs());
  let refusalDetail;
  function receiveOriginal(call, workUnitsSpent) {
    let receipt;
    try { receipt = originalReceiptFromWit(call, workUnitsSpent); originalSource.receive(receipt); }
    catch (failure) { if (receipt) originalReturn.receipt(receipt); originalSource.holdNativeRefusal(call); originalReturn.hold(call); throw failure; }
    originalReturn.receipt(receipt);
  }
  const heldOutput = new Map();
  const succession = new OriginalStockSuccession(originalSource, reactor, originalReturn, heldOutput);
  let sourceTail = Promise.resolve();
  const exclusive = (run, cleanup = false) => {
    const work = sourceTail.then(async () => {
      const refusal = nativeRefusalError(originalSource.nativeRefusal, refusalDetail);
      if (refusal !== undefined) throw refusal;
      if (contextReady) await succession.advance();
      if (!cleanup) originalSource.requireStock(succession.refusal);
      return run();
    });
    sourceTail = work.then(() => {}, () => {});
    return work;
  };
  let contextReady = false;
  let jobsStarted = false;
  const fundedJob = (budget, run, cleanup = false) => exclusive(() => originalSourceJob(originalSource, (receipt) => originalReturn.receipt(receipt), originalJobWallUs(budget), run, cleanup), cleanup);
  async function admitWire(value) {
    const turn = originalSource.issue();
    let call;
    const wireBytes = actorInputWire.encode(value);
    try { call = await reactor.admitActorInput(wireBytes, originalContextBudgetWit(turn)); }
    catch (failure) { originalSource.holdNativeRefusal(failure); originalReturn.hold(failure); throw failure; }
    receiveOriginal(call, call.workUnitsSpent);
    if (optionValue(call.refusal) !== undefined || !call.ready || call.disposition === "returned") throw Object.assign(new Error("original-turn.input-refused: refusal=" + JSON.stringify(optionValue(call.refusal) ?? null) + " ready=" + call.ready + " disposition=" + call.disposition + " wireBytes=" + (wireBytes?.length ?? 0) + " grant=" + JSON.stringify(turn.input.grant, (k, v) => typeof v === "bigint" ? v.toString() : v)), { originalCall: call });
  }
  const unreceipted = (wire, call) => exclusive(async () => {
    if (wire !== undefined) await admitWire(wire);
    const turn = originalSource.issue();
    try { return await call(originalContextBudgetWit(turn)); }
    finally { const receipt = { input: turn.input, spent: { copiedItems: 0, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0 }, workUnitsSpent: 0 }; originalSource.receive(receipt); originalReturn.receipt(receipt); }
  });
  const pairWire = (pair) => ({ pack: pair.pack, spr: pair.spr });
  const codecCall = (method, wire) => typeof codec?.[method] === "function" ? unreceipted(wire, (budget) => codec[method](budget)) : Promise.reject(new Error("actor-codec.unexported"));
  const codecExports = {
    packSchemaHash: (artifactSchema) => codecCall("packSchemaHash", { operation: "pack-schema-hash", artifactSchema }),
    genesis: (artifactSchema, documentId) => codecCall("genesis", { operation: "genesis", artifactSchema, documentId }),
    printMirror: (artifactSchema, pair) => codecCall("printMirror", { operation: "print-mirror", artifactSchema, pair: pairWire(pair) }),
  };
  async function receiveContext(method) {
    const turn = originalSource.issue();
    const call = await reactor[method](originalContextBudgetWit(turn));
    receiveOriginal(call, call.workUnitsSpent);
    if (optionValue(call.refusal) !== undefined) throw Object.assign(new Error("original-turn.context-refused"), { originalCall: call });
    return call;
  }
  async function admitContext() {
    while (!contextReady) {
      const call = await receiveContext("admitActorContext");
      contextReady = call.ready;
      if (!contextReady && call.spent.copiedItems === 0n && call.spent.copiedBytes === 0n && call.spent.retainedCapacityBytes === 0n && call.spent.releasedBytes === 0n) throw Object.assign(new Error("original-turn.context-held"), { originalCall: call });
    }
  }
  const channelRefusal = admitGuestChannelVersion(await reactor.channelVersion(), HOST_CHANNEL);
  if (channelRefusal !== null) throw Object.assign(new Error(\`\${channelRefusal.code}: \${channelRefusal.message}\`), { fault: channelRefusal });
  return {
    originalSource,
    closeOriginal: async () => {
      if (originalSource.outstanding || originalSource.nativeRefusal !== undefined || originalSource.producerRefusal !== undefined || originalSource.receiptRefusal !== undefined) throw new Error("original-turn.original-return-pending");
      if (jobsStarted) await fundedJob(undefined, (funds) => funds.close(jobs), true);
      for (;;) {
        const call = await receiveContext("closeActorContext");
        if (call.complete) {
          const acknowledgement = originalSource.issue();
          if (acknowledgement.input.grant.maximumItems < 1 || acknowledgement.input.grant.maximumDepth < 1 || acknowledgement.fuel < 1) throw new Error("original-turn.source-acknowledgement-held");
          const receipt = { input: acknowledgement.input, spent: { copiedItems: 1, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0 }, workUnitsSpent: 1 };
          originalSource.receive(receipt);
          originalReturn.receipt(receipt);
          originalSource.close();
          return { operation: originalSource.operation, generation: originalSource.generation, sourceClosed: true };
        }
        if (call.blocked || (call.spent.copiedItems === 0n && call.spent.copiedBytes === 0n && call.spent.retainedCapacityBytes === 0n && call.spent.releasedBytes === 0n)) return { operation: originalSource.operation, generation: originalSource.generation, sourceClosed: false };
      }
    },
    poll: (events, commandPage, coldPairPage, budget) => exclusive(async () => {
      await admitContext();
      if (events.length > 1) throw new Error("actor-poll.event-count: a reactor turn admits at most one event");
      const wireEvents = events.map(({ kind, payload }) => lifecycleEvent(kind, payload, activationGeneration));
      await admitWire({ operation: "poll", events: wireEvents.map(actorInputWire.event), shape: { maxEffects: budget.maxEffects, maxPatchBytes: budget.maxPatchBytes, maxFrames: budget.maxFrames } });
      const turn = originalSource.issue();
      hostShim.__checkoutOriginal(turn);
      let result;
      try {
        if (commandPage) await reactor.stageCommandPage(commandPage.cursor, commandPage.bytes);
        if (coldPairPage) await reactor.stageColdPairPage(coldPairPage);
        const slice = originalContextBudgetWit(turn);
        if (typeof budget.fuel === "number" && budget.fuel > 0 && BigInt(budget.fuel) < slice.fuel) slice.fuel = BigInt(budget.fuel);
        result = await reactor.poll(slice);
      } catch (failure) {
        try {
          const receipt = failure?.payload ?? failure;
          const accounted = !!(receipt && receipt.input && receipt.spent && typeof receipt.workUnitsSpent === "bigint");
          if (accounted) receiveOriginal(receipt, receipt.workUnitsSpent);
          const refusal = receipt?.failure;
          const overrun = accounted && refusal?.tag === "authority" && refusal.val === "work-limit";
          if (!overrun && originalSource.nativeRefusal === undefined) { originalSource.holdNativeRefusal(failure); originalReturn.hold(failure); }
        } finally {
          const raw = hostShim.__returnOriginal(turn);
          if (raw !== undefined) { originalSource.holdProducerRefusal(raw); originalReturn.hold(raw); }
        }
        const detail = hostShim.__takeActorFault();
        if (originalSource.nativeRefusal === undefined) throw nativeRefusalError(failure, detail) ?? failure;
        refusalDetail ??= detail;
        throw nativeRefusalError(failure, refusalDetail) ?? failure;
      }
      let raw;
      try { receiveOriginal(result.retainedReceipt, result.retainedReceipt.workUnitsSpent); }
      finally { raw = hostShim.__returnOriginal(turn); if (raw !== undefined) { originalSource.holdProducerRefusal(raw); originalReturn.hold(raw); } }
      if (raw !== undefined) throw Object.assign(new Error("original-turn.producer-held"), { originalProducer: raw });
      return { ...result, nextWake: optionValue(result.nextWake) ?? null, lifecycleReceipt: lifecycleReceipt(result.lifecycleReceipt, activationGeneration), uiPatchReceipt: uiPatchReceipt(result, activationGeneration), commandIngress: normalizeCommandIngress(result.commandIngress) };
    }),
    startJob: async (job, kind, input) => fundedJob(undefined, async (funds) => { jobsStarted = true; await funds.start({ reactor, jobs }, job, kind, input); succession.started(job); }),
    stepJob: async (job, budget, control = {}) => fundedJob(budget, async (funds) => { const step = await funds.step(jobs, job, { ...control, held: heldOutput }); if (step.status !== "running") succession.settled(job); return step; }),
    cancelJob: async (job) => fundedJob(undefined, async (funds) => { await funds.cancel(jobs, job, { held: heldOutput }); succession.settled(job); }, true),
    takeSegmentedDownloadChunk: async (instanceId, operationId) => optionValue(await jobs.takeSegmentedDownloadChunk(instanceId, operationId)),
    checkpoint: () => unreceipted(undefined, (budget) => checkpoint.checkpoint(budget)),
    restore: (state) => unreceipted({ operation: "restore", state }, (budget) => checkpoint.restore(budget)),
    describe: () => unreceipted(undefined, (budget) => describe.describe(budget)),
    codec: (request) => codecAnswer(codecExports, request),
    resolveEffect: (requestId, value) => hostShim.__resolveEffect(requestId, value),
    rejectEffect: (requestId, message) => hostShim.__rejectEffect(requestId, message),
  };
}
`;
}
