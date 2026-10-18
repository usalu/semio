/** 🪢️ Host shim source of the plugin web materialization: the generated host shim that implements the pure and host async import interfaces of the actor world. */
/**
 * 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (A2/H2, design-abi.md §1) + 🧪️ terra-web-bridges
 * (async-worlds). `pure` (`interface pure { log; now-ms; trace-span; }`, component.wit ~:823) stays
 * plain synchronous `func` and is unchanged from H2 — the old `host` world's larger surface
 * (`read-document`/`write-document`/`open-window`/`invoke-action`/`read-asset`/`network-fetch`/
 * `write-blob`/`read-blob`, plus the ad hoc `backboneSend`/`backbonePoll`/`backboneStatus`
 * worker-postMessage relay) is still gone.
 *
 * NEW: `host-async` (component.wit ~:887 — 24 `async func` imports plus the two fire-and-forget
 * `emit`/`emit-patch` doors) is now ALSO implemented in this one file, mapped alongside `pure` by
 * `transpilePluginComponent`'s `--map` pair. Every async import posts an `effect-request` and returns
 * a Promise settled by a later `effect-complete`/`effect-error` — see `effectRequest`'s own doc below
 * for the exact `ShardFrame`/`ShardEnvelope` shape it rides (reused verbatim from
 * `🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`, never a second wire). Host state belongs to
 * an immutable actor/activation/version module URL shared by the component and its returned API.
 *
 * 🚧 UNPROVEN beyond the jcoprobe fixture (📓️terra-jco-spike-report.md): (a) whether jco expects a
 * `result<T, pack>`-returning host-async import to signal `Err` by throwing — jcoprobe's own
 * `probe-host` never used a `result<>` return, so `effectRequest` rejecting on `effect-error` follows
 * jco's documented host-import convention, not a spike-confirmed one; (b) generated JavaScript and
 * ShardClient ownership tests do not establish a fresh guest round trip through every host import.
 */
export function hostShimSource(): string {
  return `/** @generated semio actor host shim — implements the pure AND host-async import interfaces
 * (component.wit ~:823 / ~:887). See plugin-web-materialize.ts's hostShimSource doc for the design
 * this file is generated from. */

//#region 🧬️pure
let originalClock;
export function __bindOriginalClock(clock) { if (originalClock || typeof clock !== "function") throw new Error("original-turn.clock-binding-refused"); originalClock = clock; }
export function operationClockUs() { if (!originalClock) throw new Error("original-turn.clock-unbound"); return originalClock(); }
let originalTurn;
let originalProducer;
export function __checkoutOriginal(turn) { if (originalTurn || !turn || typeof turn.deadlineUs !== "bigint") throw new Error("original-turn.checkout-refused"); originalTurn = turn; }
export function __returnOriginal(turn) { if (originalTurn !== turn) throw new Error("original-turn.foreign-return"); originalTurn = undefined; const raw = originalProducer; originalProducer = undefined; return raw; }
function originalCheck() { if (!originalTurn) return 6; if (originalTurn.cancelled) return 1; if (operationClockUs() >= originalTurn.deadlineUs) return 3; return 0; }
function unsupportedProducer(phase, fields) { if (originalProducer === undefined) originalProducer = { phase, fields }; return 6; }
export function operationBegin() { const refused = originalCheck(); if (refused) throw refused; return BigInt(originalTurn.input.grant.maximumCapacityBytes); }
export function operationProgress(completed, total, owned) { const refused = originalCheck(); if (refused) return refused; if (completed !== 0n || total !== 0n || owned !== 0n) return unsupportedProducer("progress", { completed, total, owned }); return 0; }
export function operationAllocation(bytes, owned, next, maximum) { const refused = originalCheck(); if (refused) return refused; if (bytes !== 0n || owned !== 0n || next !== 0n) return unsupportedProducer("allocation", { bytes, owned, next, maximum }); return maximum === BigInt(originalTurn.input.grant.maximumCapacityBytes) ? 0 : 4; }
export function operationReserveReturn(kind, count) { const refused = originalCheck(); if (refused) return refused; return count === 0n ? 0 : unsupportedProducer("canonical-return", { kind, count }); }
export function operationFinish(owned) { const refused = originalCheck(); if (refused) return refused; return owned === 0n ? 0 : unsupportedProducer("finish", { owned }); }
export function log(level, message) {
  if (level === "error") console.error(\`[actor] \${message}\`);
  else console.log(\`[actor] \${message}\`);
}

export function nowMs() {
  return BigInt(Date.now());
}

export function traceSpan(name) {
  if (typeof performance !== "undefined" && typeof performance.mark === "function") performance.mark(name);
}
//#endregion 🧬️pure

//#region 🌉️host-async
const bindingUrl = new URL(import.meta.url);
const boundActorId = bindingUrl.searchParams.get("actor");
const generationText = bindingUrl.searchParams.get("activation");
const boundActivationGeneration = generationText && /^[1-9][0-9]*$/.test(generationText) && generationText.length <= 20 ? BigInt(generationText) : null;
let effectSeq = 0;
const pendingEffects = new Map();

function assertHostActivation() {
  if (!boundActorId || boundActivationGeneration === null || boundActivationGeneration > 0xffffffffffffffffn) throw new Error("actor-activation.host-unbound");
}

// 🌉️ Settles the Promise \`effectRequest\` handed back for \`requestId\` — called by \`🟨️shard-worker.js\`
// when an \`effect-complete\`/\`effect-error\` envelope for this actor arrives.
export function __resolveEffect(requestId, value) {
  const entry = pendingEffects.get(requestId);
  if (!entry) return;
  pendingEffects.delete(requestId);
  entry.resolve(value);
}

export function __rejectEffect(requestId, message) {
  const entry = pendingEffects.get(requestId);
  if (!entry) return;
  pendingEffects.delete(requestId);
  entry.reject(new Error(message));
}

// 🌊️ jco's proven shape for a guest-consumable \`stream<u8>\` is a plain async generator yielding ONE
// byte at a time — confirmed against a real component (jcoprobe's \`fetchBody\`/\`read-body\`, S4 in
// 📓️terra-jco-spike-report.md), NOT a \`ReadableStream\` directly. An \`effect-complete\` for
// \`http-fetch\`/\`blob-read\` is expected to carry a \`ReadableStream\` (structured-clone-transferable
// across \`postMessage\`); this adapts it into the proven per-byte generator shape. Also accepts an
// already-async-iterable value so a kernel handing back a plain byte array still works.
async function* streamToByteGenerator(body) {
  if (body == null) return;
  if (typeof body[Symbol.asyncIterator] === "function") {
    for await (const chunk of body) {
      if (chunk instanceof Uint8Array) { for (const byte of chunk) yield byte; } else yield chunk;
    }
    return;
  }
  const reader = body.getReader();
  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) return;
      if (value instanceof Uint8Array) { for (const byte of value) yield byte; } else yield value;
    }
  } finally {
    reader.releaseLock();
  }
}

// 🚪️ Every \`host-async\` ASYNC import funnels through here — posts one \`ShardFrame::Envelope\` up to
// the kernel over the SAME shape \`🧵️shard-client.ts\` declares (\`to\`/\`from\`/\`lane\`/\`seq\`/
// \`deadlineMs\`/\`coalesce\`/\`cancelOf\`/\`payload\`), with \`payload: {kind: "effect-request", payload:
// {effect, requestId, params}}\` — the SAME \`{kind, payload}\` envelope-payload shape
// \`ShardEventEnvelope\` already uses for turn events, reused rather than inventing a new one. Resolves
// or rejects once \`__resolveEffect\`/\`__rejectEffect\` fires for the matching \`requestId\`.
function effectRequest(effect, params) {
  assertHostActivation();
  const requestId = \`\${boundActorId}:\${boundActivationGeneration}:\${effect}:\${++effectSeq}\`;
  return new Promise((resolve, reject) => {
    pendingEffects.set(requestId, { resolve, reject });
    self.postMessage({
      kind: "frame",
      actorId: boundActorId,
      activationGeneration: boundActivationGeneration,
      frame: {
        kind: "Envelope",
        envelope: {
          to: "kernel",
          from: { kind: "actor", id: boundActorId },
          lane: "Background",
          seq: effectSeq,
          deadlineMs: null,
          coalesce: null,
          cancelOf: null,
          payload: { kind: "effect-request", payload: { effect, requestId, params } },
        },
      },
    });
  });
}

// 🚪️ \`emit\`/\`emit-patch\` are plain (non-async) WIT \`func\`s — the ONE fire-and-forget door for the
// ~24 one-way \`effect\` variants (plus \`respond\`) and for UI patches. No \`requestId\`/Promise: posts
// and returns immediately, same envelope shape as \`effectRequest\` above minus the correlation.
function postFireAndForget(kind, payload) {
  assertHostActivation();
  self.postMessage({
    kind: "frame",
    actorId: boundActorId,
    activationGeneration: boundActivationGeneration,
    frame: {
      kind: "Envelope",
      envelope: { to: "kernel", from: { kind: "actor", id: boundActorId }, lane: "Background", seq: ++effectSeq, deadlineMs: null, coalesce: null, cancelOf: null, payload: { kind, payload } },
    },
  });
}

const call = (effect) => (params) => effectRequest(effect, params);
export const storageRead = call("storage-read");
export const storageWrite = call("storage-write");
export const storageDelete = call("storage-delete");
export const blobLoad = call("blob-load");
export const blobWrite = call("blob-write");
export const blobRead = (hash) => effectRequest("blob-read", { hash }).then(streamToByteGenerator);
export const httpFetch = (params) => effectRequest("http-fetch", params).then((response) => ({ ...response, body: streamToByteGenerator(response.body) }));
export const artifactRead = call("artifact-read");
export const documentWrite = call("artifact-write");
export const linkResolve = (link) => effectRequest("link-resolve", { link });
export const registryQuery = call("registry-query");
export const ioCompose = call("io-compose");
export const ioRun = call("io-run");
export const cacheDerive = call("cache-derive");
export const cacheRead = call("cache-read");
export const invokeExtension = call("invoke-extension");
export const openWindow = call("open-window");
export const openDialog = call("open-dialog");
export const dispatchAction = call("dispatch-action");
export const spawnPluginInstance = call("spawn-plugin-instance");
export const requestFileOpen = call("request-file-open");
export const requestMediaFrames = call("request-media-frames");
export const requestCapability = call("request-capability");
export const spawnJob = (job, kind, input, placement) => effectRequest("spawn-job", { job, kind, input, placement });

export function emit(value) {
  postFireAndForget("effect-emit", value);
}

export function emitPatch(patch) {
  postFireAndForget("ui-patch-emit", patch);
}
//#endregion 🌉️host-async
`;
}
