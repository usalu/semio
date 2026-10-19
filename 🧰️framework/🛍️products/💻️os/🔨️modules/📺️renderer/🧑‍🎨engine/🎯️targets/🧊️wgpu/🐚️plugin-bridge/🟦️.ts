import { packValueFromBase64 } from "@semio-tech/framework-os";
import type { InvocationResponse, PluginDispatchHintV1 } from "@semio-tech/framework";
import type { WgpuEphemeralSnapshot, WgpuPluginHandle } from "./🧾️typed-operations-and-handle/🟦️.ts";

/** 🌉️ The raw string-in/string-out JS surface `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s `wasm32` branch reads
 * via `Reflect::get` — `manifest`/`createApp`/`render` are HARD requirements at
 * `ProgramBridgeEntry::from_js` construction time; the rest are looked up lazily per call
 * (`destroyApp`/`handleAction` degrade to a harmless no-op/empty result if absent, `handleCommand`
 * errors loudly if actually invoked while absent — this adapter always provides all of them, so none
 * of those fallbacks trigger). */
export interface WgpuJsBridge {
  readonly manifest: () => string;
  /** 🪪️ `{"packageId","componentSha256"}` JSON — `ProgramBridgeEntry::from_js` reads it as a hard requirement. */
  readonly packageIdentity: () => string;
  readonly createApp: (appId: string) => Promise<number>;
  readonly destroyApp: (instanceId: number) => Promise<void>;
  readonly readAppDocumentIdentity: (instanceId: number) => Promise<string>;
  /** 🎯️ `order` is the flat (wasm-bindgen-friendly) spelling of `PluginDispatchHintV1.order` — the
   * input ledger's causal key for this one call; omitted ⇒ arrival order. */
  readonly handleAction: (instanceId: number, invocationPack: string, contextJson: string, order?: number) => Promise<string>;
  readonly handleCommand: (instanceId: number, invocationPack: string, contextJson: string, order?: number) => Promise<string>;
  readonly render: (instanceId: number, surfaceId: string, bodyKey: string, viewStatePack: string) => Promise<string>;
  readonly renderDocument: (instanceId: number, surfaceId: string, bodyKey: string, viewStatePack: string) => Promise<string>;
  readonly contextMenu: (instanceId: number, requestJson: string) => Promise<string>;
  readonly dispatchInvokeExtension: (instanceId: number, extensionId: string, capability: string, requestJson: string, req: number) => Promise<string>;
  readonly pushScopedContributions: (instanceId: number, appId: string, viewStateJson: string) => Promise<string>;
  /** 📡️ `bindingGeneration` crosses as a decimal string: it is a `u64`, which a JS `number` cannot
   * carry exactly. Resolves the `InvocationResponse` JSON whose `requestedEffects` the shell routes. */
  readonly documentBackbone: (instanceId: number, operation: string, bindingGeneration: string, uri: string) => Promise<string>;
  readonly receiveDocumentBackbone: (instanceId: number, uri: string, payload: Uint8Array) => Promise<string>;
  readonly applyMutations: (instanceId: number, operations: Uint8Array) => Promise<void>;
  readonly loadAppDocumentArchive: (instanceId: number, archive: Uint8Array) => Promise<void>;
  readonly readAppDocumentArchive: (instanceId: number) => Promise<Uint8Array>;
  /** 🧾️ The history projection as JSON (`u64` carriers as numbers, `invocationResponseJson`'s rule). */
  readonly readHistory: (instanceId: number) => Promise<string>;
  /** 🧬️ `codec.pack-schema-hash(artifactKind)`: the kind's 32-byte structural fingerprint. */
  readonly codecPackSchemaHash: (artifactKind: string) => Promise<Uint8Array>;
  /** 📥️ `codec.print-mirror(artifactKind, pair)`: the pair's `[dsl, ops]` text mirror, its validation fence. */
  readonly codecPrintMirror: (artifactKind: string, pack: Uint8Array, spr: Uint8Array) => Promise<readonly [string, string]>;
  /** 👥️ `WgpuPluginHandle.ephemeralSnapshot`, synchronous — read after every action and command. */
  readonly ephemeralSnapshot: (instanceId: number) => WgpuEphemeralSnapshot | null;
  /** 🏁️ JSON array of `WgpuOperationPublication` — synchronous, taken by the shell every frame; `u64` carriers cross as numbers. */
  readonly takeOperationPublications: (instanceId: number) => string;
}

/** 📦️ `handle_action_js`/`handle_command_js` pass the INVOCATION as `pk:`-prefixed pack too, for the
 * same reason the view state crosses that way: `JSON.parse` collapses `0` and `0.0` onto one JS
 * `number` and {@link encodePackValue} then writes every one of them as `TAG_F64`, so every INTEGER
 * argument an action carried reached the guest as a float and failed its `FromValue` decode — an
 * import chunk envelope (`{payload, name, chunk: u32, chunkCount: u32}`) is refused outright by the
 * unsigned arm (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A decoded {@link PackValue} keeps its
 * integer carriers and re-encodes through `encodePackValue` unchanged.
 *
 * 📥️ `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`'s `handle_action_js`/`handle_command_js` pass a THIRD argument
 * that is `{"viewStatePack": "pk:…", "actor": "local"}` JSON (its own `context_json`, not the bare view
 * state) — this unwraps and DECODES `.viewStatePack` from it.
 *
 * ⚖️ The payload is pack, not JSON, because JSON is lossy about integrality here and pack is not: the
 * Rust producer writes a `u64` as `1` and an `f64` as `1.0`, `JSON.parse` collapses both onto one JS
 * `number`, and {@link encodePackValue} then writes every one of them as `TAG_F64` — so the guest saw
 * `toolRunTraceCursorByWindowId.<window>.run` as `Float(1.0)`, failed its `u64` decode, and every
 * dispatch on this target died with it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). A decoded
 * {@link PackValue} keeps its integer carriers and re-encodes through `encodePackValue` unchanged. */
function viewStateFromContextJson(contextJson: string): unknown {
  try {
    const parsed = JSON.parse(contextJson) as { readonly viewStatePack?: unknown } | null;
    const packed = parsed && typeof parsed === "object" ? parsed.viewStatePack : undefined;
    return typeof packed === "string" ? packValueFromBase64(packed) : undefined;
  } catch {
    return undefined;
  }
}

/** 🔗️ Lifts the JS bridge's flat `order` argument into a `PluginDispatchHintV1`; a missing or
 * non-finite value (wasm-bindgen hands `undefined` for an omitted optional) means "no hint". */
function bridgeDispatchHint(order: number | undefined): PluginDispatchHintV1 | undefined {
  return typeof order === "number" && Number.isFinite(order) ? { order } : undefined;
}

export function pluginHandleForBridge(handle: WgpuPluginHandle): WgpuJsBridge {
  return {
    manifest: () => JSON.stringify(handle.manifest),
    packageIdentity: () => JSON.stringify({ packageId: handle.packageId, componentSha256: handle.componentSha256 }),
    createApp: (appId) => handle.createApp(appId),
    destroyApp: (instanceId) => handle.destroyApp(instanceId),
    handleAction: (instanceId, invocationPack, contextJson, order) => handle.handleAction(instanceId, packValueFromBase64(invocationPack), viewStateFromContextJson(contextJson), bridgeDispatchHint(order)).then(invocationResponseJson),
    handleCommand: (instanceId, invocationPack, contextJson, order) => handle.handleCommand(instanceId, packValueFromBase64(invocationPack), viewStateFromContextJson(contextJson), bridgeDispatchHint(order)).then(invocationResponseJson),
    render: (instanceId, surfaceId, bodyKey, viewStatePack) => handle.render(instanceId, surfaceId, bodyKey, packValueFromBase64(viewStatePack)).then((node) => JSON.stringify(node)),
    renderDocument: (instanceId, surfaceId, bodyKey, viewStatePack) => handle.renderDocument(instanceId, surfaceId, bodyKey, packValueFromBase64(viewStatePack)),
    contextMenu: (instanceId, requestJson) => handle.contextMenu(instanceId, JSON.parse(requestJson)).then((items) => JSON.stringify(items)),
    dispatchInvokeExtension: (instanceId, extensionId, capability, requestJson, req) => handle.dispatchInvokeExtension(instanceId, extensionId, capability, requestJson, BigInt(req)).then(invocationResponseJson),
    pushScopedContributions: (instanceId, appId, viewStateJson) => handle.pushScopedContributions(instanceId, appId, viewStateJson).then(invocationResponseJson),
    documentBackbone: (instanceId, operation, bindingGeneration, uri) => {
      if (operation !== "bind" && operation !== "retire") return Promise.reject(new Error(`actor-document-control.operation:${operation}`));
      return handle.documentBackbone(instanceId, operation, BigInt(bindingGeneration), uri).then(invocationResponseJson);
    },
    receiveDocumentBackbone: (instanceId, uri, payload) => handle.receiveDocumentBackbone(instanceId, uri, payload).then(invocationResponseJson),
    applyMutations: (instanceId, operations) => handle.applyMutations(instanceId, operations),
    loadAppDocumentArchive: (instanceId, archive) => handle.loadAppDocumentArchive(instanceId, archive),
    readAppDocumentArchive: (instanceId) => handle.readAppDocumentArchive(instanceId),
    readHistory: (instanceId) => handle.readHistory(instanceId).then((patch) => JSON.stringify(patch, (_key, value: unknown) => (typeof value === "bigint" ? Number(value) : value))),
    readAppDocumentIdentity: (instanceId) => handle.readAppDocumentIdentity(instanceId).then((identity) => JSON.stringify(identity)),
    codecPackSchemaHash: (artifactKind) => handle.codec({ operation: "pack-schema-hash", artifactKind }),
    codecPrintMirror: (artifactKind, pack, spr) => handle.codec({ operation: "print-mirror", artifactKind, pair: { pack, spr } }),
    ephemeralSnapshot: (instanceId) => handle.ephemeralSnapshot(instanceId),
    takeOperationPublications: (instanceId) => JSON.stringify(handle.takeOperationPublications(instanceId), (_key, value: unknown) => (typeof value === "bigint" ? Number(value) : value)),
  };
}

/** 📤️ The ONE projection of an `InvocationResponse` onto the JSON string
 * `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` parses back into `kernel::InvocationResult`.
 *
 * ⚖️ `req` is a `u64` and crosses this seam as a JS `bigint`, which `JSON.stringify` refuses outright.
 * Two of the four verbs carried a replacer and two did not, so the FIRST command whose reply
 * requested an `invokeExtension` died with `Do not know how to serialize a BigInt` — the same
 * `InvocationResponse` shape, projected two different ways (ticket
 * 26/09/09/PROCEDURAL-3D-END-TO-END). One projection, all four verbs. */
export function invocationResponseJson(result: InvocationResponse): string {
  return JSON.stringify(result, (_key, value: unknown) => (typeof value === "bigint" ? Number(value) : value));
}
