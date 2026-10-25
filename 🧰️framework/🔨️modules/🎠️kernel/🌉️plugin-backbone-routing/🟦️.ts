/** 🌉️ Backbone routing: the main-thread relay a direct-import plugin reaches through a well-known global, the inbound push and the per-document routes.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H2): `SerializedPluginWasm` (`pluginErrorText`/
// `isPluginInstanceBusyError`/`withSerializedPluginWasmHandle`) and `PluginWorkerClient` — one Worker
// per plugin, capping the browser at ~20 plugins on V8's 4 GiB wasm-module guard region per worker —
// are DELETED (grepped clean: neither had a live caller left outside this file once
// `loadPluginModuleViaWorker`/`loadPluginModuleUncached` went with them; see
// `📓️terra-H2-web-shard-report.md`). Replaced by `ShardClient`
// (`🎭️actor/📮️shard-client/🟦️.ts` — a bounded pool multiplexed by actorId) and
// `ActivationRegistry` below. `runSerialized`'s busy-retry/reload loop has no equivalent: the new
// ABI's traps are `ActivationRegistry`'s `FailurePolicy` job (design-runtime.md §1) — drop + restore
// from checkpoint — not a local blind-retry loop with no visibility into checkpoint state.

// #region 🎠️Kernel
export function relayPluginBackboneOutbound(uri: string, message: Uint8Array): void {
  pluginBackboneRoutes.get(pluginBackboneDocumentIdFromUri(uri))?.(uri, message);
}

/** 🐝️ A direct-import (main-thread, no-worker) plugin's generated `🟨️.js` runs in this
 * same realm but can't import from this module, so it reaches the outbound relay through this
 * well-known global instead — the same relay a worker-backed program reaches via `postMessage`. */
(globalThis as unknown as { __semioMainThreadPluginBackboneOutbound?: (uri: string, message: Uint8Array) => void }).__semioMainThreadPluginBackboneOutbound = relayPluginBackboneOutbound;

/** 📥️ Inbound counterpart: pushes straight into the same global queue a direct-import plugin's
 * `🟨️.js` `backbonePoll` drains, keyed by `uri` (globally unique per document, so no pluginId
 * scoping is needed even though several plugins may share this realm). */
function pushMainThreadPluginBackboneInbound(uri: string, messages: readonly Uint8Array[]): void {
  const bridge = globalThis as unknown as { __semioBackboneInbound?: Map<string, Uint8Array[]> };
  const queue = bridge.__semioBackboneInbound ?? new Map<string, Uint8Array[]>();
  queue.set(uri, [...(queue.get(uri) ?? []), ...messages]);
  bridge.__semioBackboneInbound = queue;
}

/**
 * 🚧️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H2) GAP — read before relying on this. The
 * per-worker fast path this function used to take (`activeWorkerByPluginId.get(pluginId)` →
 * `PluginWorkerClient.postBackboneInbound`, a raw `"backboneInbound"` postMessage type) is gone along
 * with `PluginWorkerClient` itself, and its counterpart on the guest side is ALSO gone:
 * `🟨️.js` now implements only the `pure` WIT interface (`log`/`now-ms`/`trace-span`) —
 * `backboneSend`/`backbonePoll`/`backboneStatus` were deleted (design-runtime.md §3), because
 * `world actor` has no synchronous host import for them anymore. Every read/write/network/backbone
 * -shaped call now flows through the effect/event turn loop instead (`events::message-event`
 * replaces "the `backbone-poll` push", per `component.wit`'s own doc comment on that variant).
 *
 * This function is kept — `pluginId` stays a real parameter, `ShellHost/🟦️.tsx` (registrar-
 * only, not this packet's to edit) still imports it — but its ONLY remaining path is the main-thread
 * global queue below, which nothing on the guest side drains anymore either post-flip. Wiring
 * `message-event`-addressed delivery through `ActivationRegistry`/`ShardClient` is real, non-mechanical
 * work belonging to whichever packet finishes rewiring `ShellHost` off the pre-flip `PluginWasmHandle`
 * ABI entirely (see this packet's report for the full list of what that touches) — flagged here rather
 * than silently left to look functional.
 */
export function postPluginBackboneInbound(pluginId: string, uri: string, messages: readonly Uint8Array[]): void {
  void pluginId;
  pushMainThreadPluginBackboneInbound(uri, messages);
}

//#region 🐚️PluginBackboneRouting
/** 🐚️ Extracts the `<documentId>` a plugin's `actor://<documentId>` backbone uri names — the
 * `framework/sync` `ChannelBackbone::pair` convention (see the react renderer's `openArtifact`). Falls
 * back to the whole uri for any other scheme so an unrecognized realm still gets a routing key instead
 * of being silently dropped. */
function pluginBackboneDocumentIdFromUri(uri: string): string {
  return uri.startsWith("actor://") ? uri.slice("actor://".length) : uri;
}

const pluginBackboneRoutes = new Map<string, (uri: string, message: Uint8Array) => void>();

/**
 * 🧭️ Routes a plugin's outbound backbone bytes for one document to whichever shell instance owns
 * it — replaces the old page-global relay slot (`setPluginBackboneOutboundRelay`), which a second
 * mounted shell silently overwrote: misrouting the first shell's document sync into the second shell's
 * backbone worker, then severing it entirely the moment that second shell unmounted (it cleared the
 * slot to `null`). Register at the same point a shell learns it owns `documentId` (the react renderer's
 * `openArtifact`) and call the returned unregister function at the matching `closeArtifact`/unmount.
 */
export function registerPluginBackboneRoute(documentId: string, relay: (uri: string, message: Uint8Array) => void): () => void {
  pluginBackboneRoutes.set(documentId, relay);
  return () => {
    if (pluginBackboneRoutes.get(documentId) === relay) pluginBackboneRoutes.delete(documentId);
  };
}
//#endregion 🐚️PluginBackboneRouting
// #endregion 🎠️Kernel
