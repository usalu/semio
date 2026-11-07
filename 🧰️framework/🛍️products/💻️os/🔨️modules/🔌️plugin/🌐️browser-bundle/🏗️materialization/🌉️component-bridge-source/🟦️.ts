/** 🌉️ Component bridge source of the plugin web materialization: the generated jco component bridge that wraps one actor activation behind the actor api. */
import { originalOperationWorkerSource } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🎟️retained-turn/🟦️.ts";
import { ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES, encodeActorInstanceLifecycle } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🟦️.ts";
import { ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES, encodeActorUiPatchReceipt, validateActorUiPatchPairing } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch/🟦️.ts";
import { COMMAND_INGRESS_KINDS } from "../../../../../../../📦️packages/🟦️typescript/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts";
import { actorCodecRuntimeSourceV1 } from "../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧬️codec/🟦️.ts";
import { APP_CHANNEL_VERSION, admitGuestChannelVersion } from "../../../../../../../📦️packages/🟦️typescript/🧾️app-channel/🟦️.ts";
import { PLUGIN_HOST_SHIM_FILE } from "../🧾️contract/🟦️.ts";

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
 * 🚧 UNVERIFIED against a real compiled artifact of the PRODUCTION `world actor` component (the
 * wasm32-wasip2 fleet does not currently compile — a large in-flight conversion tracked elsewhere on
 * this ticket): the exact jco-generated export shape for a world that exports several *interfaces*
 * (rather than bare functions) is assumed here to be one JS binding per interface, named for the
 * interface (\`reactor\`/\`jobs\`/\`checkpoint\`/\`describe\`), field names camelCased from the WIT's
 * kebab-case. **This IS confirmed for a single-export-interface world against a real transpiled
 * component** (📓️terra-jco-spike-report.md's jcoprobe fixture: \`export * as probe from
 * './interfaces/...'\`, camelCased function names) — the multi-interface-export case (4 interfaces,
 * matching \`world actor\`) is extrapolated from that single-interface evidence plus jco's documented
 * per-interface naming convention, not independently re-confirmed here. If jco nests these
 * differently, only the four destructured names below need to change — every other line here is
 * interface-shape-agnostic.
 *
 * 🧪️ terra-web-bridges: every destructured method now returns a Promise (every WIT function in the
 * target world is `async func`) — made EXPLICITLY `async` here rather than relying on bare pass-
 * through. The component and its host imports use matching actor/activation/version URLs; a static
 * relative shim import would otherwise be shared across same-package activations. Each returned API
 * retains that immutable shim module for replies, including while other actors are interleaved.
 */
export function pluginComponentBridgeSource(componentBase: string, wasmFileName: string): string {
  return `/** @generated semio actor jco component bridge */
${originalOperationWorkerSource()}

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

export async function createActorApi(actorId, activationGeneration, originalAdmission, operationClockUs, originalReturn) {
  if (typeof activationGeneration !== "bigint" || activationGeneration <= 0n || activationGeneration > 0xffffffffffffffffn) throw new Error("actor-close.invalid-activation-generation");
  const componentUrl = new URL("./${componentBase}.js", import.meta.url);
  const rebuildVersion = new URL(import.meta.url).searchParams.get("v");
  componentUrl.searchParams.set("actor", actorId);
  componentUrl.searchParams.set("activation", activationGeneration.toString());
  if (rebuildVersion) componentUrl.searchParams.set("v", rebuildVersion);
  const hostUrl = new URL("./${PLUGIN_HOST_SHIM_FILE}", import.meta.url);
  hostUrl.search = componentUrl.search;
  const hostShim = await import(hostUrl.href);
  const { reactor, jobs, checkpoint, describe, codec } = await import(componentUrl.href);
  hostShim.__bindOriginalClock(operationClockUs);
  const originalSource = new OriginalOperationSource(originalAdmission, () => hostShim.operationClockUs());
  function receiveOriginal(call, workUnitsSpent) {
    let receipt;
    try { receipt = originalReceiptFromWit(call, workUnitsSpent); originalSource.receive(receipt); }
    catch (failure) { if (receipt) originalReturn.receipt(receipt); originalSource.holdNativeRefusal(call); originalReturn.hold(call); throw failure; }
    originalReturn.receipt(receipt);
  }
  const heldOutput = new Map();
  let sourceTail = Promise.resolve();
  const exclusive = (run) => { const work = sourceTail.then(run); sourceTail = work.then(() => {}, () => {}); return work; };
  const fundedJob = (budget, run) => exclusive(() => originalSourceJob(originalSource, (receipt) => originalReturn.receipt(receipt), originalJobWallUs(budget), run));
  let contextReady = false;
  async function admitWire(value) {
    const turn = originalSource.issue();
    let call;
    try { call = await reactor.admitActorInput(actorInputWire.encode(value), originalContextBudgetWit(turn)); }
    catch (failure) { originalSource.holdNativeRefusal(failure); originalReturn.hold(failure); throw failure; }
    receiveOriginal(call, call.workUnitsSpent);
    if (optionValue(call.refusal) !== undefined || !call.ready || call.disposition === "returned") throw Object.assign(new Error("original-turn.input-refused"), { originalCall: call });
  }
  const unreceipted = (wire, call) => exclusive(async () => {
    if (wire !== undefined) await admitWire(wire);
    const turn = originalSource.issue();
    try { return await call(originalContextBudgetWit(turn)); }
    finally { const receipt = { input: turn.input, spent: { copiedItems: 0, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0 }, workUnitsSpent: 0 }; originalSource.receive(receipt); originalReturn.receipt(receipt); }
  });
  const pairWire = (pair) => ({ pack: pair.pack, spr: pair.spr });
  const codecExports = {
    packSchemaHash: (artifactSchema) => unreceipted({ operation: "pack-schema-hash", artifactSchema }, (budget) => codec.packSchemaHash(budget)),
    genesis: (artifactSchema, documentId) => unreceipted({ operation: "genesis", artifactSchema, documentId }, (budget) => codec.genesis(budget)),
    printMirror: (artifactSchema, pair) => unreceipted({ operation: "print-mirror", artifactSchema, pair: pairWire(pair) }, (budget) => codec.printMirror(budget)),
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
        result = await reactor.poll(originalContextBudgetWit(turn));
      } catch (failure) {
        try {
          if (failure && failure.input && failure.spent && typeof failure.workUnitsSpent === "bigint") receiveOriginal(failure, failure.workUnitsSpent);
          if (originalSource.nativeRefusal === undefined) { originalSource.holdNativeRefusal(failure); originalReturn.hold(failure); }
        } finally {
          const raw = hostShim.__returnOriginal(turn);
          if (raw !== undefined) { originalSource.holdProducerRefusal(raw); originalReturn.hold(raw); }
        }
        throw failure;
      }
      let raw;
      try { receiveOriginal(result.retainedReceipt, result.retainedReceipt.workUnitsSpent); }
      finally { raw = hostShim.__returnOriginal(turn); if (raw !== undefined) { originalSource.holdProducerRefusal(raw); originalReturn.hold(raw); } }
      if (raw !== undefined) throw Object.assign(new Error("original-turn.producer-held"), { originalProducer: raw });
      return { ...result, nextWake: optionValue(result.nextWake) ?? null, lifecycleReceipt: lifecycleReceipt(result.lifecycleReceipt, activationGeneration), uiPatchReceipt: uiPatchReceipt(result, activationGeneration), commandIngress: normalizeCommandIngress(result.commandIngress) };
    }),
    startJob: async (job, kind, input) => fundedJob(undefined, (funds) => funds.start({ reactor, jobs }, job, kind, input)),
    stepJob: async (job, budget, control = {}) => fundedJob(budget, (funds) => funds.step(jobs, job, { ...control, held: heldOutput })),
    cancelJob: async (job) => fundedJob(undefined, (funds) => funds.cancel(jobs, job, { held: heldOutput })),
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
