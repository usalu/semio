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
 * the activation's own host object, shared by the component instance and its returned API.
 *
 * 🏭️ The module exports one factory, `createActorHost(actorId, activationGeneration)`, and the bridge creates one host per activation, so the
 * host state (clock binding, checked-out turn, pending effects) lives and dies with that activation's instance instead of in a module
 * that the browser's module map keeps for the page's lifetime. `__resolveEffect`/`__rejectEffect` settle the Promise an async import
 * handed back when `🟨️shard-worker.js` delivers the matching `effect-complete`/`effect-error`; `emit`/`emitPatch` post fire-and-forget
 * envelopes of the same shape without a correlation id; a `stream<u8>` result is adapted into the per-byte async generator jco consumes.
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

export function createActorHost(actorId, activationGeneration) {
  let originalClock;
  function __bindOriginalClock(clock) { if (originalClock || typeof clock !== "function") throw new Error("original-turn.clock-binding-refused"); originalClock = clock; }
  function operationClockUs() { if (!originalClock) throw new Error("original-turn.clock-unbound"); return originalClock(); }
  let originalTurn;
  let originalProducer;
  function __checkoutOriginal(turn) { if (originalTurn || !turn || typeof turn.deadlineUs !== "bigint") throw new Error("original-turn.checkout-refused"); originalTurn = turn; }
  function __returnOriginal(turn) { if (originalTurn !== turn) throw new Error("original-turn.foreign-return"); originalTurn = undefined; const raw = originalProducer; originalProducer = undefined; return raw; }
  function originalCheck() { if (!originalTurn) return 6; if (originalTurn.cancelled) return 1; if (operationClockUs() >= originalTurn.deadlineUs) return 3; return 0; }
  function operationBegin() { const refused = originalCheck(); if (refused) throw refused; return BigInt(originalTurn.input.grant.maximumCapacityBytes); }
  function operationProgress(completed, total, owned) { const refused = originalCheck(); if (refused) return refused; return owned > BigInt(originalTurn.input.grant.maximumCapacityBytes) ? 2 : 0; }
  function operationAllocation(bytes, owned, next, maximum) { const refused = originalCheck(); if (refused) return refused; const capacity = BigInt(originalTurn.input.grant.maximumCapacityBytes); if (maximum !== capacity) return 4; return owned > capacity || next > capacity - owned ? 2 : 0; }
  function operationReserveReturn(kind, count) { const refused = originalCheck(); if (refused) return refused; return count > BigInt(originalTurn.input.grant.maximumItems) ? 2 : 0; }
  function operationFinish(owned) { const refused = originalCheck(); if (refused) return refused; return owned > BigInt(originalTurn.input.grant.maximumCapacityBytes) ? 2 : 0; }
  let actorFault;
  function log(level, message) {
    if (level === "error") { if (message.startsWith("original-turn.semantic ")) actorFault = message.slice(23); console.error(\`[actor] \${message}\`); }
    else console.log(\`[actor] \${message}\`);
  }
  function __takeActorFault() { const fault = actorFault; actorFault = undefined; return fault; }

  function nowMs() {
    return BigInt(Date.now());
  }

  function traceSpan(name) {
    if (typeof performance !== "undefined" && typeof performance.mark === "function") performance.mark(name);
  }

  const boundActorId = typeof actorId === "string" && actorId.length > 0 ? actorId : null;
  const boundActivationGeneration = typeof activationGeneration === "bigint" && activationGeneration > 0n ? activationGeneration : null;
  let effectSeq = 0;
  const pendingEffects = new Map();

  function assertHostActivation() {
    if (!boundActorId || boundActivationGeneration === null || boundActivationGeneration > 0xffffffffffffffffn) throw new Error("actor-activation.host-unbound");
  }

  function __resolveEffect(requestId, value) {
    const entry = pendingEffects.get(requestId);
    if (!entry) return;
    pendingEffects.delete(requestId);
    entry.resolve(value);
  }

  function __rejectEffect(requestId, message) {
    const entry = pendingEffects.get(requestId);
    if (!entry) return;
    pendingEffects.delete(requestId);
    entry.reject(new Error(message));
  }

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
  const storageRead = call("storage-read");
  const storageWrite = call("storage-write");
  const storageDelete = call("storage-delete");
  const blobLoad = call("blob-load");
  const blobWrite = call("blob-write");
  const blobRead = (hash) => effectRequest("blob-read", { hash }).then(streamToByteGenerator);
  const httpFetch = (params) => effectRequest("http-fetch", params).then((response) => ({ ...response, body: streamToByteGenerator(response.body) }));
  const artifactRead = call("artifact-read");
  const documentWrite = call("artifact-write");
  const linkResolve = (link) => effectRequest("link-resolve", { link });
  const registryQuery = call("registry-query");
  const ioCompose = call("io-compose");
  const ioRun = call("io-run");
  const cacheDerive = call("cache-derive");
  const cacheRead = call("cache-read");
  const invokeExtension = call("invoke-extension");
  const openWindow = call("open-window");
  const openDialog = call("open-dialog");
  const dispatchAction = call("dispatch-action");
  const spawnPluginInstance = call("spawn-plugin-instance");
  const requestFileOpen = call("request-file-open");
  const requestMediaFrames = call("request-media-frames");
  const requestCapability = call("request-capability");
  const spawnJob = (job, kind, input, placement) => effectRequest("spawn-job", { job, kind, input, placement });

  function emit(value) {
    postFireAndForget("effect-emit", value);
  }

  function emitPatch(patch) {
    postFireAndForget("ui-patch-emit", patch);
  }
  return Object.freeze({ __bindOriginalClock, operationClockUs, __checkoutOriginal, __returnOriginal, operationBegin, operationProgress, operationAllocation, operationReserveReturn, operationFinish, log, __takeActorFault, nowMs, traceSpan, __resolveEffect, __rejectEffect, storageRead, storageWrite, storageDelete, blobLoad, blobWrite, blobRead, httpFetch, artifactRead, documentWrite, linkResolve, registryQuery, ioCompose, ioRun, cacheDerive, cacheRead, invokeExtension, openWindow, openDialog, dispatchAction, spawnPluginInstance, requestFileOpen, requestMediaFrames, requestCapability, spawnJob, emit, emitPatch });
}
`;
}
