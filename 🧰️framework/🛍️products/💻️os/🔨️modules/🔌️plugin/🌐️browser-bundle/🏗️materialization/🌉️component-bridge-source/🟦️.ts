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

/** 🎁️ jco lifts \`option<t>\` as a tagged \`{ tag: "none" | "some" }\` variant; every host-side reader
 * below wants the bare value (or nothing), so unwrap exactly that shape and pass anything else through. */
function unwrapOption(value) {
  if (value === null || value === undefined) return undefined;
  if (typeof value === "object" && (value.tag === "none" || value.tag === "some")) return value.tag === "none" ? undefined : value.val;
  return value;
}

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

// 🧩️ \`jobs::job-step\` reaches the worker as jco's raw \`{tag, val}\` variant; \`ShardJobStep\`
// (\`📮️shard-client/🟦️.ts\`) is the \`{status, ...}\` shape every caller declares. Converting here — the
// one place that owns the guest boundary — keeps that declaration honest instead of leaving the two
// shapes silently different, which is what they were while nothing on this target ever stepped a job.
function normalizeJobStep(step) {
  if (step?.tag === "running") return { status: "running", progress: unwrapOption(step.val) };
  if (step?.tag === "done") return { status: "done", value: step.val };
  if (step?.tag === "failed") return { status: "failed", value: step.val };
  throw new Error(\`unknown job-step shape: \${JSON.stringify(step)}\`);
}

function lifecycleReceipt(raw, activationGeneration) {
  const value = unwrapOption(raw);
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
  const receipt = unwrapOption(result.uiPatchReceipt);
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
  let contextReady = false;
  async function receiveContext(method) {
    const turn = originalSource.issue();
    const call = await reactor[method](originalTurnWit(turn.input));
    receiveOriginal(call, 0n);
    if (unwrapOption(call.refusal) !== undefined) throw Object.assign(new Error("original-turn.context-refused"), { originalCall: call });
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
    poll: async (events, commandPage, coldPairPage, budget) => {
      await admitContext();
      if (events.length > 1) throw new Error("actor-poll.event-count: a reactor turn admits at most one event");
      const wireEvents = events.map(({ kind, payload }) => lifecycleEvent(kind, payload, activationGeneration));
      const turn = originalSource.issue();
      hostShim.__checkoutOriginal(turn);
      let result;
      try {
        if (commandPage) await reactor.stageCommandPage(commandPage.cursor, commandPage.bytes);
        if (coldPairPage) await reactor.stageColdPairPage(coldPairPage);
        result = await reactor.poll(wireEvents, { retained: originalTurnWit(turn.input), fuel: BigInt(turn.fuel), deadlineUs: turn.deadlineUs, maxEffects: budget.maxEffects, maxPatchBytes: budget.maxPatchBytes, maxFrames: budget.maxFrames });
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
      try { receiveOriginal(result.retainedReceipt, result.fuelUsed); }
      finally { raw = hostShim.__returnOriginal(turn); if (raw !== undefined) { originalSource.holdProducerRefusal(raw); originalReturn.hold(raw); } }
      if (raw !== undefined) throw Object.assign(new Error("original-turn.producer-held"), { originalProducer: raw });
      return { ...result, nextWake: unwrapOption(result.nextWake) ?? null, lifecycleReceipt: lifecycleReceipt(result.lifecycleReceipt, activationGeneration), uiPatchReceipt: uiPatchReceipt(result, activationGeneration), commandIngress: normalizeCommandIngress(result.commandIngress) };
    },
    startJob: async (job, kind, input) => jobs.startJob(job, kind, input),
    stepJob: async (job, budget) => normalizeJobStep(await jobs.stepJob(job, budget)),
    cancelJob: async (job) => jobs.cancelJob(job),
    // 📤️ THE 2026-09-12 segmented-export fault: this export returns \`option<list<u8>>\`, and jco's flat
    // lift of an option yields the TAGGED variant object (\`_liftFlatOption\` delegates to
    // \`_liftFlatVariant\`, which builds \`{ tag }\`/\`{ tag, val }\`; only \`_liftFlatEnum\` reduces to
    // \`.tag\`). Handed on raw it reached the shard worker as \`[object Object]\` — so chunk 0 faulted the
    // worker and the terminal \`{ tag: "none" }\` was not \`undefined\` either. Every other guest option in
    // this bridge already routes through \`unwrapOption\`; this one is no exception.
    takeSegmentedDownloadChunk: async (instanceId, operationId) => unwrapOption(await jobs.takeSegmentedDownloadChunk(instanceId, operationId)),
    checkpoint: async () => checkpoint.checkpoint(),
    restore: async (state) => checkpoint.restore(state),
    describe: async () => describe.describe(),
    codec: (request) => codecAnswer(codec, request),
    resolveEffect: (requestId, value) => hostShim.__resolveEffect(requestId, value),
    rejectEffect: (requestId, message) => hostShim.__rejectEffect(requestId, message),
  };
}
`;
}
