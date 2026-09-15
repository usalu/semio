// #region 🧲️Header
/** @emoji 🖼️ Renderer-agnostic interpretation of one `ShardClient.turn()` result — coercing its
 * opaque `unknown` shape, decoding retained-mode `UiPatch` ops onto a kept tree, and translating a
 * raw wire `effect` variant into the shared `kernel::Effect` TS union. Lifted out of
 * `PluginRuntime/🟦️.tsx`'s `🔖️ActorAdapter`/`🔖️RetainedUiPatch` regions (MICROKERNEL-POOLED-
 * ACTOR-PLUGIN-RUNTIME, `wgpu-web-shard`) so a second renderer target does not reimplement this wire
 * parsing independently — the exact "third divergent copy" that packet's own brief warns against.
 * `decodePackValue` is injected rather than imported (`@semio-tech/framework-os`'s own codec) so this
 * package stays free of a hard dependency on that product package, matching the `🎭️actor` crate's own
 * "stay pure, transports/codecs are injected" discipline (📌️important.md naming hazards). `PluginRuntime`
 * predates this module and still carries its own inline copy (outside `wgpu-web-shard`'s lease to
 * edit) — a future packet should point it here too. */
// #endregion 🧲️Header

// #region 🔌️Imports
import type { Effect, SpawnedJobCompletion, SpawnedJobStep } from "../../🎠️kernel/🟦️.ts";
// 🧵️ The spawned-job drive RULE is the kernel's (Rust `kernel::spawned_job_completion` and this
// twin), not this module's: the wire layer recovers a `spawn-job` off the boundary and pumps it, the
// kernel alone decides what a transcript of `step-job` observations means. A value import, unlike
// `decodePackValue`'s injected codec, because nothing in `🎠️kernel/🟦️.ts`'s own import graph reaches
// back here — verified, not assumed.
import { jobPlacementFromWireName, spawnedJobCompletion, SPAWNED_JOB_DEADLINE_MS, SPAWNED_JOB_FUEL, SPAWNED_JOB_STEP_CEILING } from "../../🎠️kernel/🟦️.ts";
import { parseWitColdPairIngressStatus, type ColdPairIngressStatus } from "../📥️cold-pair/🟦️.ts";
// #endregion 🔌️Imports

//#region 🔖️WireBytes
/** 🎯️ A `pack`-typed field inside a raw WIT `effect`/`patch-op` variant may ship as a plain number
 * array, a `Uint8Array`, a `{kind:"bytes", value}` object, or (defensively) a base64 string — jco's
 * possible encodings for `list<u8>` at this boundary. */
export function coerceWireBytes(raw: unknown): Uint8Array {
  if (raw instanceof Uint8Array) return raw;
  if (ArrayBuffer.isView(raw) && Object.prototype.toString.call(raw) === "[object Uint8Array]") return new Uint8Array(raw.buffer, raw.byteOffset, raw.byteLength);
  if (Array.isArray(raw)) return Uint8Array.from(raw as number[]);
  if (raw && typeof raw === "object") {
    const record = raw as Record<string, unknown>;
    if (record.kind === "bytes" && Array.isArray(record.value)) return Uint8Array.from(record.value as number[]);
    if (Array.isArray(record.data)) return Uint8Array.from(record.data as number[]);
  }
  if (typeof raw === "string") {
    const binary = atob(raw);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
    return bytes;
  }
  throw new Error(`coerceWireBytes: unsupported payload ${JSON.stringify(raw)?.slice(0, 120)}`);
}
//#endregion 🔖️WireBytes

//#region 🔖️TurnResult
/** 🚧️ Best-effort JS representation of one raw WIT `effect`/`patch-op` variant crossing the wasm
 * boundary — UNVERIFIED against a real compiled artifact (no plugin has migrated onto `world actor`
 * yet). Assumed shape: jco's standard variant binding, `tag` the WIT case name (kebab-case) and `val`
 * its payload record (fields camelCased from kebab). */
export type WireVariant<T = unknown> = { readonly tag?: string; readonly val?: T };

export type WireUiPatch = {
  readonly surface?: { readonly instance?: number | bigint; readonly surface?: string };
  readonly kind?: string;
  readonly revision?: number | bigint;
  readonly baseRevision?: number | bigint;
  readonly ops?: readonly WireVariant[];
};

export type WireTurnResult = {
  readonly original?: object;
  readonly lifecycleReceipt?: Uint8Array;
  readonly uiPatchReceipt?: Uint8Array;
  readonly uiPatches: readonly WireUiPatch[];
  readonly effects: readonly WireVariant[];
  /** 👥️ WIT `turn-result.presence` — one `{ update: pack }` per `(surface, node key)` whose own/peer
   * selection or hover moved this turn. Carried through the coercion because selection deliberately
   * never rides a revisioned `UiPatch`: this array is the ONLY channel by which a guest-owned
   * selection reaches a rendered row, on either renderer. */
  readonly presence?: readonly { readonly update?: unknown }[];
  readonly nextWake: number | null;
  readonly status?: unknown;
  readonly commandIngress?: WireVariant;
  readonly coldPairIngress: ColdPairIngressStatus;
};

/** 📥️ Defensive parse of `ShardClient.turn()`'s opaque `unknown` return into the fields a caller
 * needs. Private lifecycle and UI-patch receipts retain their exact byte owner and the original turn
 * identity; malformed receipt carriers fail before any renderer can project a patch without authority. */
export function coerceTurnResult(raw: unknown): WireTurnResult {
  const record = (raw && typeof raw === "object" ? raw : {}) as Record<string, unknown>;
  const uiPatches = Array.isArray(record.uiPatches) ? (record.uiPatches as WireUiPatch[]) : [];
  const effects = Array.isArray(record.effects) ? (record.effects as WireVariant[]) : [];
  const presence = Array.isArray(record.presence) ? (record.presence as { readonly update?: unknown }[]) : [];
  const nextWake = typeof record.nextWake === "number" ? record.nextWake : null;
  const lifecycleReceipt = record.lifecycleReceipt;
  if (lifecycleReceipt !== undefined && lifecycleReceipt !== null && !(lifecycleReceipt instanceof Uint8Array)) throw new Error("actor-lifecycle.receipt-bytes");
  const uiPatchReceipt = record.uiPatchReceipt;
  if (uiPatchReceipt !== undefined && uiPatchReceipt !== null && !(uiPatchReceipt instanceof Uint8Array)) throw new Error("actor-ui-patch.receipt-bytes");
  const commandIngress = record.commandIngress && typeof record.commandIngress === "object" ? (record.commandIngress as WireVariant) : undefined;
  const coldPairIngress = parseWitColdPairIngressStatus(record.coldPairIngress);
  return {
    original: raw !== null && typeof raw === "object" ? raw : undefined,
    lifecycleReceipt: lifecycleReceipt ?? undefined,
    uiPatchReceipt: uiPatchReceipt ?? undefined,
    uiPatches,
    effects,
    presence,
    nextWake,
    status: record.status,
    commandIngress,
    coldPairIngress,
  };
}

/** 🔀️ The raw payload a `Effect::SendMessage{target: Shell{instance}}` addressed AT `instanceId`
 * carries, whatever its kind — the one place the shell transport is unwrapped. {@link
 * shellMessageKind} then says which of the two wire languages those bytes speak. */
function shellMessagePayload(effect: WireVariant, instanceId: number): Uint8Array | null {
  if (effect.tag !== "send-message") return null;
  const val = (effect.val ?? {}) as { readonly target?: WireVariant<number>; readonly payload?: unknown };
  if (!val.target || val.target.tag !== "shell") return null;
  if (Number(val.target.val) !== instanceId) return null;
  if (val.payload === undefined) return null;
  return coerceWireBytes(val.payload);
}

/** 🔀️ `Effect::SendMessage{target: Shell{instance}}` → the raw `AppFrame` bytes it wraps —
 * `⚛️reactor/🦀️.rs`'s `route_app_frame` puts EVERY non-`UiPatch` `AppFrame` reply here.
 *
 * ⚖️ `route_exchange_output` (`⚛️reactor/🔄️turn/🦀️.rs`) multiplexes a SECOND wire language onto that
 * same endpoint: a mounted typed operation's result pages ride it as
 * `TypedOperationResultPage::renderer_exchange_bytes`, and the host acknowledges them back over the
 * same `Shell{instance}` channel. Those bytes are NOT an `AppFrame`, and handing them to an
 * `AppFrame` decoder reads their magic's first byte as a frame tag (`decodeAppFrame: unknown tag 115`
 * — `'s'` of `semio.typed-operation-page.v1`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The
 * discrimination therefore belongs HERE, in the one demux both renderer targets read, exactly as the
 * native host's `apply_turn_result` already discriminates (`🧊️renderer/🦀️.rs`) — not in an ordering
 * rule that says some other pass must strip them first. */
export function shellFrameBytes(effect: WireVariant, instanceId: number): Uint8Array | null {
  const payload = shellMessagePayload(effect, instanceId);
  if (payload === null || bytesStartWith(payload, TYPED_OPERATION_PAGE_MAGIC) || bytesStartWith(payload, TYPED_OPERATION_ACK_MAGIC)) return null;
  return payload;
}

/** 🔀️ Which wire language one `Shell{instance}` payload speaks, or `null` when the effect is not a
 * shell message addressed at `instanceId` at all. Total by construction, so a third language added to
 * this endpoint has to name itself here rather than silently decoding as an `AppFrame`. */
export function shellMessageKind(effect: WireVariant, instanceId: number): "app-frame" | "typed-operation-page" | "typed-operation-ack" | null {
  const payload = shellMessagePayload(effect, instanceId);
  if (payload === null) return null;
  if (bytesStartWith(payload, TYPED_OPERATION_PAGE_MAGIC)) return "typed-operation-page";
  if (bytesStartWith(payload, TYPED_OPERATION_ACK_MAGIC)) return "typed-operation-ack";
  return "app-frame";
}

/** 🕹️ Every `Invocation` frame a turn's LEFTOVER effects carry — the answer of a call whose real work
 * ran on a later turn than the one that admitted it.
 *
 * ⚖️ This is the whole answer of a framework-reserved tool verb. `interactionSelect`/
 * `interactionHover`/`clearSelection` are admitted by the dispatch turn with an EMPTY
 * `InvocationResult` (`output: null`, `ui_scope: None`) and a `SpawnJob`; the real
 * `InvocationResult` — `output.interactionView` plus the app-declared refresh scope
 * (`dispatch_interaction_action`, `💻️os/🔨️modules/🔌️plugin/🦀️.rs`) — is published by the job's
 * completion turn, which lands in the leftover lane, never in the frames the dispatch itself
 * resolved with. A host that folds only the dispatch's own frames therefore reads `scope=none` and
 * no `interactionView`, so the guest is never asked to re-render and its `selectionJson` stays
 * empty however the pick went — measured on the wgpu target of the generation3d playground, 7 of 8
 * examples in both roles (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-selection-roundtrip-2026-09-15.md`).
 *
 * `decodeAppFrame` is injected because the codec lives in `@semio-tech/framework-os`, one layer
 * above this module — the same shape {@link decodeWirePatchOps} already uses for `decodePackValue`.
 * A leftover that is not an `AppFrame` at all (a typed-operation page, a foreign host effect) is
 * skipped rather than decoded. */
export function leftoverShellInvocationFrames<Frame>(leftover: readonly WireVariant[], decodeAppFrame: (bytes: Uint8Array) => Frame): Frame[] {
  const frames: Frame[] = [];
  for (const effect of leftover) {
    const value = effect.val as { readonly target?: WireVariant; readonly payload?: unknown } | undefined;
    if (effect.tag !== "send-message" || value?.target?.tag !== "shell" || value.payload === undefined) continue;
    const payload = coerceWireBytes(value.payload);
    if (bytesStartWith(payload, TYPED_OPERATION_PAGE_MAGIC) || bytesStartWith(payload, TYPED_OPERATION_ACK_MAGIC)) continue;
    try {
      const frame = decodeAppFrame(payload);
      if (frame !== null && typeof frame === "object" && "Invocation" in frame) frames.push(frame);
    } catch {
      continue;
    }
  }
  return frames;
}

/** 📨️ The `MessageEndpoint` tag a `send-message` effect addresses (`""` when the effect carries no
 * target at all), `null` for any other effect kind. */
export function wireSendMessageTargetTag(effect: WireVariant): string | null {
  if (effect.tag !== "send-message") return null;
  const val = (effect.val ?? {}) as { readonly target?: WireVariant };
  return val.target?.tag ?? "";
}

/** 📨️ Every `MessageEndpoint` a host route already owns. `Shell{instance}` is the reply-frame
 * transport ({@link shellFrameBytes}, and `leftoverShellInvocationFrames` for a frame that lands on a
 * later turn than the call it answers); `Backbone{uri}` is the document port. Both are consumed
 * BEFORE {@link wireEffectToFriendly} ever sees them — they are transport, not a friendly `Effect`,
 * and the friendly union deliberately declares no `sendMessage` member for them. */
export const WIRE_SEND_MESSAGE_ROUTED_TARGETS: readonly string[] = ["shell", "backbone"];

/** 📨️ Whether a `send-message` effect is already owned by one of {@link WIRE_SEND_MESSAGE_ROUTED_TARGETS}. */
export function isRoutedWireSendMessage(effect: WireVariant): boolean {
  const target = wireSendMessageTargetTag(effect);
  return target !== null && WIRE_SEND_MESSAGE_ROUTED_TARGETS.includes(target);
}
//#endregion 🔖️TurnResult

//#region 📬️TypedOperationPage
/** 🔎️ Whether `bytes` opens with `prefix` — the magic probe both shell-message languages are told
 * apart by, allocation-free and never throwing on a short or foreign payload. */
function bytesStartWith(bytes: Uint8Array, prefix: Uint8Array): boolean {
  if (bytes.length < prefix.length) return false;
  for (let index = 0; index < prefix.length; index += 1) if (bytes[index] !== prefix[index]) return false;
  return true;
}

/** 📬️ `TypedOperationResultPage::RENDERER_PAGE_MAGIC` (`🔌️plugin/🦀️.rs`), byte for byte. */
export const TYPED_OPERATION_PAGE_MAGIC = new TextEncoder().encode("semio.typed-operation-page.v1\0");
/** 📬️ `TypedOperationResultPage::RENDERER_ACK_MAGIC` (`🔌️plugin/🦀️.rs`), byte for byte. */
export const TYPED_OPERATION_ACK_MAGIC = new TextEncoder().encode("semio.typed-operation-ack.v1\0");
/** 📏️ `{receiver u32 | operation u64 | generation u64 | sequence u32 | attempt u8 | lane u8 | len u32}`,
 * little-endian — `renderer_exchange_bytes`' fixed header after the magic. */
export const TYPED_OPERATION_PAGE_HEADER_BYTES = 30;
/** 📏️ The acknowledgement body: the page header's first 25 bytes, i.e. its token alone. */
export const TYPED_OPERATION_TOKEN_BYTES = 25;
/** 📏️ `TYPED_OPERATION_RESULT_PAGE_BYTES` (`🔌️plugin/🦀️.rs`) — one page's fixed payload authority. */
export const TYPED_OPERATION_RESULT_PAGE_BYTES = 4_096;
/** 🛤️ Highest `TypedOperationResultLane` discriminant the guest emits (`🔌️plugin/🦀️.rs`: Artifact 0 …
 * Fault 11, Interaction 12, WindowTransient 13, WindowConfig 14). */
export const TYPED_OPERATION_RESULT_LANE_MAX = 14;
/** 🛤️ The lane a terminal result page rides. */
export const TYPED_OPERATION_LANE_TERMINAL = 10;
/** 🛤️ The lane a fault result page rides. */
export const TYPED_OPERATION_LANE_FAULT = 11;

/** 🪪️ One result page's exact identity — the tuple `renderer_ack_token` reads back out of an ACK. */
export type TypedOperationToken = {
  readonly receiver: number;
  readonly operation: bigint;
  readonly generation: bigint;
  readonly sequence: number;
  readonly attempt: number;
};

/** 📨️ The `Event::Message{source: Shell{instance}}` envelope that acknowledges one page. Structurally
 * a `ShardEventEnvelope`; declared here rather than imported so this module keeps its "transports are
 * injected" discipline. */
export type TypedOperationAckEvent = {
  readonly kind: "message";
  readonly payload: { readonly source: { readonly tag: "shell"; readonly val: string }; readonly payload: readonly number[] };
};

/** 📬️ One decoded typed-operation result page and the acknowledgement the guest blocks on. */
export type TypedOperationPage = {
  readonly token: TypedOperationToken;
  readonly lane: number;
  readonly payload: Uint8Array;
  readonly acknowledgement: TypedOperationAckEvent;
};

/** 📬️ Decodes one `Shell{instance}`-addressed typed-operation result page, or `null` when the effect
 * carries an `AppFrame` (or is not a shell message at all). The TS twin of the native host's
 * `TypedOperationResultPage::decode_guest_message` (`🧊️renderer/🦀️.rs`) and of the emitter
 * `renderer_exchange_bytes` (`🔌️plugin/🦀️.rs`) — a page whose receiver, lane or length contradicts
 * its own header is a wire fault, never an `AppFrame` to fall through to. */
export function typedOperationResult(effect: WireVariant): TypedOperationPage | null {
  if (effect.tag !== "send-message") return null;
  const value = effect.val as { readonly target?: WireVariant; readonly payload?: unknown } | undefined;
  if (value?.target?.tag !== "shell" || value.payload === undefined) return null;
  const bytes = coerceWireBytes(value.payload);
  if (!bytesStartWith(bytes, TYPED_OPERATION_PAGE_MAGIC)) return null;
  const body = bytes.subarray(TYPED_OPERATION_PAGE_MAGIC.length);
  if (body.length < TYPED_OPERATION_PAGE_HEADER_BYTES) throw new Error("typed-operation result header is truncated");
  const view = new DataView(body.buffer, body.byteOffset, body.byteLength);
  const receiver = view.getUint32(0, true);
  const lane = body[25]!;
  const length = view.getUint32(26, true);
  if (Number(value.target.val) !== receiver || lane > TYPED_OPERATION_RESULT_LANE_MAX || length > TYPED_OPERATION_RESULT_PAGE_BYTES || body.length !== TYPED_OPERATION_PAGE_HEADER_BYTES + length) {
    throw new Error("typed-operation result violates its receiver, lane, or page authority");
  }
  const ack = new Uint8Array(TYPED_OPERATION_ACK_MAGIC.length + TYPED_OPERATION_TOKEN_BYTES);
  ack.set(TYPED_OPERATION_ACK_MAGIC);
  ack.set(body.subarray(0, TYPED_OPERATION_TOKEN_BYTES), TYPED_OPERATION_ACK_MAGIC.length);
  return {
    token: { receiver, operation: view.getBigUint64(4, true), generation: view.getBigUint64(12, true), sequence: view.getUint32(20, true), attempt: body[24]! },
    lane,
    payload: body.subarray(TYPED_OPERATION_PAGE_HEADER_BYTES),
    acknowledgement: { kind: "message", payload: { source: { tag: "shell", val: String(receiver) }, payload: Array.from(ack) } },
  };
}

/** 📬️ Every acknowledgement one turn's result pages owe. The guest PARKS its operation until each
 * lands, so a host that collects pages without answering them freezes the operation mid-flight. */
export function typedOperationAcknowledgements(turn: { readonly effects: readonly WireVariant[] }): TypedOperationAckEvent[] {
  return turn.effects.flatMap((effect) => {
    const page = typedOperationResult(effect);
    return page ? [page.acknowledgement] : [];
  });
}

/** 📬️ What one turn's effects hold once its typed-operation pages are taken out of them. Pure: it
 * chooses no policy for a fault or a terminal, it only names them, so React's owner-attributing
 * router and the wgpu bridge's single-caller ladder read ONE scan rather than two decoders. */
export type TypedOperationPageScan = {
  readonly kept: readonly WireVariant[];
  readonly pages: readonly TypedOperationPage[];
  readonly acknowledgements: readonly TypedOperationAckEvent[];
  readonly terminal: boolean;
  readonly faults: readonly string[];
};

export function scanTypedOperationPages(effects: readonly WireVariant[]): TypedOperationPageScan {
  const kept: WireVariant[] = [];
  const pages: TypedOperationPage[] = [];
  const faults: string[] = [];
  let terminal = false;
  for (const effect of effects) {
    const page = typedOperationResult(effect);
    if (!page) {
      kept.push(effect);
      continue;
    }
    pages.push(page);
    if (page.lane === TYPED_OPERATION_LANE_FAULT) faults.push(new TextDecoder().decode(page.payload));
    if (page.lane === TYPED_OPERATION_LANE_TERMINAL) terminal = true;
  }
  return { kept, pages, acknowledgements: pages.map((page) => page.acknowledgement), terminal, faults };
}

/** 🔁️ The bounded poll that keeps a mounted typed operation advancing once no host call is left to
 * drive it: submit one continuation settle, stop the moment the actor reports anything other than
 * `more-work`, and yield between polls so a long drain never starves rendering. `live`/`settle`/
 * `yieldTurn` are injected so the stop conditions this whole feature rests on are assertable without
 * a shard worker, and so both renderer targets drive the identical loop. */
export async function drainTypedOperationTurns(
  budget: number,
  live: () => boolean,
  settle: () => Promise<{ readonly status: unknown; readonly nextWake: number | null }>,
  yieldTurn: () => Promise<void>,
): Promise<{ readonly polls: number; readonly stopped: "idle" | "closed" | "budget"; readonly nextWake: number | null }> {
  for (let poll = 0; poll < budget; poll += 1) {
    if (!live()) return { polls: poll, stopped: "closed", nextWake: null };
    const settled = await settle();
    if (wireTurnStatusTag(settled.status) !== "more-work") return { polls: poll + 1, stopped: "idle", nextWake: settled.nextWake };
    await yieldTurn();
  }
  return { polls: budget, stopped: "budget", nextWake: null };
}
//#endregion 📬️TypedOperationPage

//#region 🔖️RetainedUiPatch
/** 🩹️ `kernel::PatchOp`, TS twin restricted to what `⚛️reactor/🩹️patches/🦀️.rs`'s
 * `PatchTracker` actually emits this wave — full-body only, every dirty surface emits one
 * `PatchOp::Replace` at the root path. `path` is `list<u32>` at the WIT boundary (empty for root). */
export type PatchOp =
  | { readonly kind: "Replace"; readonly path: readonly number[]; readonly node: unknown }
  | { readonly kind: "InsertChild"; readonly path: readonly number[]; readonly index: number; readonly node: unknown }
  | { readonly kind: "RemoveChild"; readonly path: readonly number[]; readonly index: number }
  | { readonly kind: "SetProps"; readonly path: readonly number[]; readonly props: unknown };

export function decodeWirePatchOps(ops: readonly WireVariant[], decodePackValue: (bytes: Uint8Array) => unknown): readonly PatchOp[] {
  const decoded: PatchOp[] = [];
  for (const op of ops) {
    const val = (op.val ?? {}) as Record<string, unknown>;
    const path = Array.isArray(val.path) ? (val.path as number[]) : [];
    switch (op.tag) {
      case "replace":
        decoded.push({ kind: "Replace", path, node: decodePackValue(coerceWireBytes(val.node)) });
        break;
      case "insert-child":
        decoded.push({ kind: "InsertChild", path, index: Number(val.index ?? 0), node: decodePackValue(coerceWireBytes(val.node)) });
        break;
      case "remove-child":
        decoded.push({ kind: "RemoveChild", path, index: Number(val.index ?? 0) });
        break;
      case "set-props":
        decoded.push({ kind: "SetProps", path, props: val.props !== undefined ? decodePackValue(coerceWireBytes(val.props)) : undefined });
        break;
      default:
        break;
    }
  }
  return decoded;
}

export type RetainedSurface = { readonly revision: number; readonly node: unknown };

/**
 * @emoji 🖼️ Reconciles one `UiPatch`'s ops onto `previous` (the last body a caller retained for the
 * surface), so the UI thread reads an already-reconciled tree instead of awaiting a plugin turn. Only
 * a root `PatchOp::Replace` (path `[]`) is applied — the only shape any guest emits this wave; anything
 * else, or a `baseRevision` that doesn't match `previous.revision` on a non-full-replace patch, is an
 * honest desync — `previous` is kept rather than an unverified partial walk applied.
 */
export function applyUiPatchToRetained(
  previous: RetainedSurface | null,
  patch: { readonly revision: number; readonly baseRevision: number; readonly ops: readonly PatchOp[] },
): { readonly surface: RetainedSurface | null; readonly desynced: boolean } {
  let node: unknown = previous?.node ?? null;
  let sawFullReplace = false;
  for (const op of patch.ops) {
    if (op.kind === "Replace" && op.path.length === 0) {
      node = op.node;
      sawFullReplace = true;
    } else {
      return { surface: previous, desynced: true };
    }
  }
  if (!sawFullReplace && previous && patch.baseRevision !== previous.revision) return { surface: previous, desynced: true };
  return { surface: node !== null ? { revision: patch.revision, node } : previous, desynced: false };
}
//#endregion 🔖️RetainedUiPatch

//#region 🔖️EffectWire
/** 🪪️ Decodes the schema-owned nested request without narrowing its u64 identity or interpreting JSON as a pack. */
export function wireExtensionInvocation(effect: WireVariant): Extract<Effect, { readonly invokeExtension: unknown }> {
  const value = effect.val as { readonly req?: unknown; readonly params?: { readonly extensionId?: unknown; readonly capability?: unknown; readonly payload?: unknown } } | undefined;
  const req = value?.req;
  if (typeof req !== "bigint" || req <= 0n || req > 0xffffffffffffffffn) throw new Error("extension.request-id-invalid");
  const params = value?.params;
  if (typeof params?.extensionId !== "string" || !params.extensionId || typeof params.capability !== "string" || !params.capability) throw new Error("extension.request-address-invalid");
  const requestJson = new TextDecoder("utf-8", { fatal: true }).decode(coerceWireBytes(params.payload));
  return { invokeExtension: { req, extensionId: params.extensionId, capability: params.capability, requestJson } };
}

/** ↩️ Decodes `respond-effect` (`🔌️plugin/🧬️schema/📜️.wit` — `{ req, outcome: respond-result }`)
 * without narrowing its u64 identity. `respond` is the ANSWER half of the one inbound-call seam the
 * ABI has (`request-event`); it is addressed by `req` alone, so a host that submitted the request
 * correlates on that id and never on an actor-local reply channel. The outcome keeps the WIT arm
 * names (`ok`/`fault`) rather than Rust's `RequestOutcome::{Ok,Err}` because this is the wire the
 * host reads, and because it is byte-for-byte the shape `captureExtensionCompletion.complete`
 * already takes — one vocabulary for both directions of the same door. */
export function wireRespondAnswer(effect: WireVariant): Extract<Effect, { readonly respond: unknown }> {
  const value = effect.val as { readonly req?: unknown; readonly outcome?: WireVariant } | undefined;
  const req = value?.req;
  if (typeof req !== "bigint" || req <= 0n || req > 0xffffffffffffffffn) throw new Error("respond.request-id-invalid");
  const outcome = value?.outcome;
  if (outcome?.tag !== "ok" && outcome?.tag !== "fault") throw new Error("respond.outcome-invalid");
  const bytes = coerceWireBytes(outcome.val);
  return { respond: { req, result: outcome.tag === "ok" ? { ok: bytes } : { fault: bytes } } };
}

/** 🎁️ Unwraps ONE WIT `option<T>` as it really reaches a renderer door. jco's direct binding hands
 * over the bare value (`T | undefined`); the actor boundary hands over the variant record
 * (`{tag: "some", val}` / `{tag: "none"}`). Both shapes are real on the same field, so a decoder that
 * reads only one of them silently loses it — which is exactly what happened to
 * `download-media-export.encoding` and turned every binary export in the repo into base64 TEXT under
 * a binary file name (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️io-surface-2026-09-13.md` §7.3). */
export function wireOptionValue(raw: unknown): unknown {
  if (raw === null || typeof raw !== "object" || !("tag" in (raw as Record<string, unknown>))) return raw;
  const option = raw as { readonly tag?: unknown; readonly val?: unknown };
  if (option.tag === "some") return option.val;
  if (option.tag === "none") return undefined;
  return raw;
}

/** 🎁️ {@link wireOptionValue} narrowed to `option<string>` — anything that is not a string is absent. */
export function wireOptionText(raw: unknown): string | undefined {
  const value = wireOptionValue(raw);
  return typeof value === "string" ? value : undefined;
}

/** ⬇️ The one reader of `download-media-export.encoding`, shared by the React `PluginRuntime` door and
 * the wgpu `plugin-bridge` one. What the string MEANS is the kernel's contract
 * (`kernel::mediaExportBytes` / `kernel::media_export_bytes`); this only recovers it from the wire. */
export const wireMediaExportEncoding = wireOptionText;

/** ⬇️ Decodes `download-media-export-effect` (`🔌️plugin/🧬️schema/📜️.wit`). One decoder for both
 * renderers, for the same reason {@link wireRespondAnswer} is one: the React door used to carry its
 * own copy that read `encoding` flat, and the wgpu door carried no case at all — so the same guest
 * export produced a corrupt file on one target and nothing at all on the other. */
export function wireDownloadMediaExport(effect: WireVariant): Extract<Effect, { readonly downloadMediaExport: unknown }> {
  const value = (effect.val ?? {}) as Record<string, unknown>;
  return {
    downloadMediaExport: {
      filename: String(value.filename ?? ""),
      mimeType: String(value.mimeType ?? value["mime-type"] ?? ""),
      data: String(value.data ?? ""),
      encoding: wireMediaExportEncoding(value.encoding),
    },
  };
}

/** 🧵️ Decodes `spawn-job-effect` (`🔌️plugin/🧬️schema/📜️.wit`) without narrowing its u64 identity.
 * `job` IS the guest's parked request id, so a `number` here would mis-resolve a long-lived actor's
 * futures and the `start-job`/`step-job` door rejects one outright. `placement` crosses as a BARE
 * string (jco lowers a WIT `enum` that way, measured on 6118: `placement: "isolated"`), and an
 * unknown spelling is refused rather than defaulted.
 *
 * This is the effect EVERY framework reserved tool verb is delivered as — `interactionSelect`,
 * `interactionHover`, `clearSelection` — so a renderer door without this case publishes every
 * interaction perfectly and applies none (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-world3d-interaction-2026-09-13.md` §7). */
export function wireSpawnJob(effect: WireVariant): Extract<Effect, { readonly spawnJob: unknown }> {
  const value = (effect.val ?? {}) as Record<string, unknown>;
  const job = wireJobIdentity(value.job, "spawn-job");
  const kind = wireOptionValue(value.kind);
  if (typeof kind !== "string" || !kind) throw new Error("spawn-job.kind-invalid");
  const placement = jobPlacementFromWireName(wireOptionValue(value.placement));
  if (placement === undefined) throw new Error(`spawn-job.placement-invalid ${JSON.stringify(value.placement)?.slice(0, 60)}`);
  return { spawnJob: { job, kind, input: coerceWireBytes(wireOptionValue(value.input) ?? []), placement } };
}

/** 🧵️ Decodes `cancel-job-effect` — the same u64 identity, nothing else. */
export function wireCancelJob(effect: WireVariant): Extract<Effect, { readonly cancelJob: unknown }> {
  const value = (effect.val ?? {}) as Record<string, unknown>;
  return { cancelJob: { job: wireJobIdentity(value.job, "cancel-job") } };
}

/** 🪪️ One `u64` job identity off the wire. jco hands a `bigint`; a decimal string is the shape a
 * fixture (and `structuredClone`-free transport) can carry, and both must mean the same id. */
export function wireJobIdentity(raw: unknown, effectTag: string): bigint {
  const value = wireOptionValue(raw);
  const job = typeof value === "bigint" ? value : typeof value === "string" && /^\d+$/.test(value) ? BigInt(value) : typeof value === "number" && Number.isSafeInteger(value) && value >= 0 ? BigInt(value) : null;
  // 🪪️ `job` IS a request id minted by the guest's own `RequestRegistry`, which never issues 0 —
  // the same bound `wireExtensionInvocation`/`wireRespondAnswer` put on `req`, for the same reason.
  if (job === null || job <= 0n || job > 0xffffffffffffffffn) throw new Error(`${effectTag}.job-identity-invalid`);
  return job;
}

/** 🚥️ Normalizes `TurnResult.status` to its kebab-case tag — jco hands a variant `{tag}`, a plain
 * string, or a camelCase spelling depending on the boundary it crossed. `"more-work"` is the ONE tag
 * a drain loop branches on: the guest is telling the host it has work left this activation. */
export function wireTurnStatusTag(status: unknown): string {
  const raw =
    typeof status === "string"
      ? status
      : status && typeof status === "object" && "tag" in status
        ? String((status as { readonly tag?: unknown }).tag ?? "")
        : "";
  return raw.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase();
}

//#region 🚚️MoreWorkDrive
/** 🤫️ Every field of WIT `turn-result` (`🔌️plugin/🧬️schema/📜️.wit`) a host reads, as one shape. It is
 * spelled out rather than reused from {@link WireTurnResult} because {@link shardTurnCarriesNothingV1}
 * decides whether a result may be DROPPED, and a carrier it forgot would be dropped silently. */
export type ShardTurnCarriers = {
  readonly uiPatches?: unknown;
  readonly effects?: unknown;
  readonly presence?: unknown;
  readonly nextWake?: unknown;
  readonly lifecycleReceipt?: unknown;
  readonly uiPatchReceipt?: unknown;
  readonly commandIngress?: unknown;
  readonly coldPairIngress?: unknown;
  readonly status?: unknown;
};

/** 🤫️ Whether one reactor turn result carried NOTHING the host could act on — no ui patch, no
 * effect, no presence, no wake, neither receipt, and both ingress lanes idle.
 *
 * This is the whole safety argument of the shard worker's silent-turn hold: a result this predicate
 * admits may be discarded without telling anyone, because the ONLY thing it said was `more-work`, and
 * `more-work` on the far side of a Worker boundary means "pump me again" — which the worker can do
 * itself. `fuelUsed` is the single `turn-result` field with no host reader and is therefore the one
 * omission, stated rather than implied. */
export function shardTurnCarriesNothingV1(result: ShardTurnCarriers | null | undefined): boolean {
  if (!result || typeof result !== "object") return false;
  if (!Array.isArray(result.uiPatches) || result.uiPatches.length !== 0) return false;
  if (!Array.isArray(result.effects) || result.effects.length !== 0) return false;
  if (result.presence !== undefined && (!Array.isArray(result.presence) || result.presence.length !== 0)) return false;
  if (result.nextWake !== null && result.nextWake !== undefined) return false;
  if (result.lifecycleReceipt !== undefined && result.lifecycleReceipt !== null) return false;
  if (result.uiPatchReceipt !== undefined && result.uiPatchReceipt !== null) return false;
  if (!isIdleIngress(result.commandIngress)) return false;
  if (!isIdleIngress(result.coldPairIngress)) return false;
  return true;
}

function isIdleIngress(status: unknown): boolean {
  return Boolean(status) && typeof status === "object" && (status as { readonly tag?: unknown }).tag === "idle";
}

/** 🚚️ How many consecutive guest turns ONE worker-owned drive may run before it crosses back anyway —
 * the wall the host GRANTED for this crossing, divided by what one guest turn measures.
 *
 * Twin of `⚛️reactor/🔄️turn/🦀️.rs`'s `more_work_drive_steps`, and held equal to it by the shared
 * contract `⚛️reactor/🧫️fixtures/🚚️more-work-drive.json`.
 *
 * 🐛️ It replaces a WALL bound of `SHARD_TURN_SILENT_HOLD_MS = 8`, which was the reactor's own
 * executor slice (`run_until_deadline(64, 256 KiB, now + 8 ms)`) borrowed as a drive budget. That was
 * incoherent from the day it landed: a whole guest turn measures ~13 ms on the procedural 3d React
 * door — the executor slice plus everything else a turn does — so an 8 ms wall admitted
 * `floor(8 / 13) = 0` further turns and the hold degenerated into "one extra poll"
 * (`📓️reactor-reconcile-spin-2026-09-14.md` §7 item 2 named it and did not take it). The bound is now
 * a step COUNT derived from the measurement, and its wall is
 * {@link shardTurnDriveBudgetMsV1} — the steps priced at that same measurement, never more than the
 * grant that produced them. */
export function shardTurnDriveStepsV1(grantWallMs: number, guestTurnCostMs: number): number {
  if (!Number.isFinite(grantWallMs) || !Number.isFinite(guestTurnCostMs) || guestTurnCostMs <= 0) return 1;
  return Math.max(1, Math.floor(grantWallMs / guestTurnCostMs));
}

/** 🚚️ The WALL one drive may spend — the whole grant, never the grant rounded down to whole measured
 * turns.
 *
 * The step count above is the grant's DERIVED EXPECTATION, and its job is the coherence law: it is what
 * says a grant of 100 ms fits seven 13 ms turns while the reactor's own 8 ms slice fits none. It is not
 * the runtime cap, because rounding the wall down to `steps × cost` makes the drive cross back EMPTY
 * with part of its own grant unspent whenever turns come in cheaper than they measured — and a drive
 * that crosses carrying nothing is the exact cost this whole lane exists to remove.
 *
 * 🐛️ What that rounding cost, measured: a live tool run on the puzzle 3d shell requests window
 * refreshes every 100–400 ms, its world-window patches cost ~30 ms of host intake each, and they
 * arrived **250–800 ms** after the request rather than once per request (ticket
 * `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS` `📓️status.md` phase 8, `🔍️w5-fill-timeline-probe.ts
 * --profile`). A reconcile that needs more turns than the rounded wall admits is one the drive hands
 * back unfinished, and the patch then leaves on some later unrelated drain turn.
 *
 * A grant smaller than one measured turn still buys one whole turn, for the same reason
 * {@link shardTurnDriveStepsV1} always admits one. */
export function shardTurnDriveBudgetMsV1(grantWallMs: number, guestTurnCostMs: number): number {
  if (!Number.isFinite(grantWallMs) || grantWallMs <= 0) return Math.max(0, guestTurnCostMs);
  return Math.max(grantWallMs, guestTurnCostMs);
}

/** 🚚️ The WORKER-owned `MoreWork` drive, as a law rather than as inlined worker text.
 *
 * In the browser the host round trip IS the guest's pump: there is no self-driving loop on the far
 * side of the worker boundary, so a reactor that answers `more-work` while publishing nothing costs a
 * POST, a poll, a structured clone and a main-thread pickup to be asked again. Measured on the
 * procedural 3d React door, 2026-09-15: **32.9 of 60.7 worker crossings per `flowEvalTick` hop posted
 * no events and returned no patch** (`📓️ui-turn-patch-batching-2026-09-15.md` §7 item 2).
 *
 * The drive owns that pump. It re-enters the guest until the turn produces something the host can act
 * on, and it crosses back for exactly five other reasons, each named by its stop tag:
 *
 * | stop | means |
 * |---|---|
 * | `carried` | the turn produced something for the host — the ONLY reason the host is told anything |
 * | `idle` | the guest stopped answering `more-work` |
 * | `input` | a host-owned input is pending behind the drive — an ingress message, a cancel, a view-state change — and the drive may not hold the wire while the host has something to say |
 * | `steps` | the derived step ceiling is spent |
 * | `closed` | the actor was disposed or re-activated under a new generation |
 *
 * A result is only ever discarded when {@link shardTurnCarriesNothingV1} admits it, so the drive is
 * lossless by construction. `poll`/`now`/`live`/`inputPending` are injected so every stop is
 * assertable without a Worker, and so the generated worker
 * (`🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`, whose inline twin this owns) and any future
 * target drive the identical loop. */
export async function driveShardTurnMoreWorkV1(drive: {
  readonly first: ShardTurnCarriers;
  readonly poll: () => Promise<ShardTurnCarriers>;
  readonly now: () => number;
  readonly live: () => boolean;
  readonly inputPending: () => boolean;
  readonly steps: number;
  readonly budgetMs: number;
}): Promise<{ readonly result: ShardTurnCarriers; readonly polls: number; readonly stopped: "carried" | "idle" | "input" | "steps" | "budget" | "closed" }> {
  const deadline = drive.now() + drive.budgetMs;
  let result = drive.first;
  let polls = 0;
  for (;;) {
    if (wireTurnStatusTag(result.status) !== "more-work") return { result, polls, stopped: "idle" };
    if (!shardTurnCarriesNothingV1(result)) return { result, polls, stopped: "carried" };
    if (drive.inputPending()) return { result, polls, stopped: "input" };
    if (!drive.live()) return { result, polls, stopped: "closed" };
    // 🚚️ The WALL is the bound the grant states, and it is checked before the step count. `steps` is a
    // hard backstop against an unbounded loop — a guest whose turns cost microseconds — not a latency
    // control: the latency control is `input`, which releases the wire the instant the host has
    // something to say. `first` is already the first turn of this crossing, so a backstop of k admits
    // k − 1 further polls.
    if (drive.now() >= deadline) return { result, polls, stopped: "budget" };
    if (polls + 1 >= drive.steps) return { result, polls, stopped: "steps" };
    result = await drive.poll();
    polls += 1;
  }
}
//#endregion 🚚️MoreWorkDrive

//#region 📥️InboundRequest
/** ⏱️ How many guest turns ONE inbound `request` may take to produce its `respond` — the ABI's own
 * "answered … within a bounded number of turns, or by spawning a job" (`🔌️plugin/🧬️schema/📜️.wit`'s
 * `request-event`). A capability handler is a pure synchronous function, so the answer lands on turn
 * 1; the budget exists so a guest that parks a request instead of answering it fails loudly at the
 * caller rather than leaving an outstanding completion forever. */
export const INBOUND_REQUEST_TURN_BUDGET = 64;

/** 📥️ One inbound call, as this side of the ABI states it. `originInstanceId` becomes the request's
 * `message-endpoint::shell` origin — the instance the answer is being fetched FOR; `0` means the
 * shell itself asked, owning no instance of the callee. */
export type InboundRequestDrive = {
  readonly req: bigint;
  readonly capability: string;
  readonly payload: Uint8Array;
  readonly originInstanceId: number;
  /** 🔁️ Submits one turn on the CALLEE's actor and hands back its result — each target keeps its own
   * scheduling, activation and serialization discipline; this module owns only the protocol. Typed on
   * the two fields the protocol reads, not on {@link WireTurnResult}, so a target carrying a richer
   * turn record of its own satisfies it structurally without converting. */
  readonly submit: (events: readonly { readonly kind: string; readonly payload: unknown }[]) => Promise<InboundRequestTurn>;
  readonly turnBudget?: number;
  readonly signal?: AbortSignal;
  readonly onProgress?: (progress: { readonly capability: string; readonly turns: number; readonly budget: number }) => void;
};

/** 🔁️ The slice of one turn result {@link driveInboundRequest} reads: the effects it scans for a
 * matching `respond`, and the status that says whether the guest has work left to drain. */
export type InboundRequestTurn = { readonly effects: readonly WireVariant[]; readonly status?: unknown };

export type InboundRequestAnswer =
  | { readonly status: "answered"; readonly turns: number; readonly result: { readonly ok: Uint8Array } | { readonly fault: Uint8Array } }
  | { readonly status: "cancelled"; readonly turns: number }
  | { readonly status: "unanswered"; readonly turns: number };

/**
 * @emoji 📥️ Drives ONE `Event::Request` to its `respond` on the callee's actor — the host half of the
 * ABI's single inbound-call seam, shared verbatim by the React `PluginRuntime` door and the wgpu
 * `plugin-bridge` one so the two targets can never drift into two protocols (this module's own
 * header: the "third divergent copy" hazard).
 *
 * The request is submitted once; afterwards the actor is drained with EMPTY turns while it still
 * reports `more-work`, because a guest that parked the request answers on a later turn nobody else
 * would drive. Cancellation and progress are read at turn boundaries only — a turn already handed to
 * the worker is never half-abandoned. Fault vocabulary is deliberately NOT chosen here: each target
 * maps `cancelled`/`unanswered` onto its own typed refusal.
 */
export async function driveInboundRequest(drive: InboundRequestDrive): Promise<InboundRequestAnswer> {
  const budget = drive.turnBudget ?? INBOUND_REQUEST_TURN_BUDGET;
  const answerFor = (turn: InboundRequestTurn): { readonly ok: Uint8Array } | { readonly fault: Uint8Array } | null => {
    for (const effect of turn.effects) {
      if (effect.tag !== "respond") continue;
      const { respond } = wireRespondAnswer(effect);
      if (respond.req === drive.req) return respond.result;
    }
    return null;
  };
  let turn = await drive.submit([
    { kind: "request", payload: { req: drive.req, params: { origin: { tag: "shell", val: drive.originInstanceId }, capability: drive.capability, payload: Array.from(drive.payload) } } },
  ]);
  for (let turns = 1; turns <= budget; turns += 1) {
    const result = answerFor(turn);
    if (result) return { status: "answered", turns, result };
    if (drive.signal?.aborted === true) return { status: "cancelled", turns };
    drive.onProgress?.({ capability: drive.capability, turns, budget });
    if (wireTurnStatusTag(turn.status) !== "more-work") return { status: "unanswered", turns };
    turn = await drive.submit([]);
  }
  return { status: "unanswered", turns: budget };
}
//#endregion 📥️InboundRequest

//#region 🧵️SpawnedJob
/** 🧵️ The two guest-job doors a host must own to honour a `spawn-job` effect — `jobs::start-job` and
 * `jobs::step-job` on the instance the job was placed on (`🔌️plugin/🧬️schema/📜️.wit`'s `interface
 * jobs`). Injected rather than imported so this module keeps no dependency on a particular transport:
 * the wgpu bridge hands over its `ShardClient` bound to one actor, and any other host its own. */
export type SpawnedJobPort = {
  readonly startJob: (job: bigint, kind: string, input: Uint8Array) => Promise<void>;
  readonly stepJob: (job: bigint, budget: { readonly fuel: bigint; readonly deadlineMs: number }) => Promise<unknown>;
};

/** 🧵️ One spawned job, as this side of the ABI states it. `stepCeiling` exists for a law that wants
 * to drive the stall arm without 32 real round trips; production leaves it alone. */
export type SpawnedJobDrive = {
  readonly job: bigint;
  readonly kind: string;
  readonly input: Uint8Array;
  readonly port: SpawnedJobPort;
  readonly stepCeiling?: number;
  readonly signal?: AbortSignal;
  readonly onStep?: (step: { readonly job: bigint; readonly index: number; readonly status: SpawnedJobStep["status"] }) => void;
};

/** 🧵️ Normalizes one `jobs::job-step` observation. jco hands the WIT variant as `{tag, val}`; the
 * shard worker already normalizes it to `{status, value}` for `ShardClient.stepJob`, and a
 * `running` carries an OPTIONAL progress pack. Both shapes are real on the same door, so reading only
 * one of them turns a finished job into a stall — the same class of defect that lost
 * `download-media-export.encoding` (this module's `wireOptionValue`). */
export function wireJobStep(raw: unknown): SpawnedJobStep {
  const record = (raw ?? {}) as { readonly tag?: unknown; readonly val?: unknown; readonly status?: unknown; readonly value?: unknown; readonly progress?: unknown };
  const status = typeof record.status === "string" ? record.status : typeof record.tag === "string" ? record.tag : "";
  const payload = record.value !== undefined ? record.value : record.val;
  if (status === "running") return { status: "running" };
  if (status === "done" || status === "failed") return { status, value: coerceWireBytes(wireOptionValue(payload) ?? []) };
  throw new Error(`job-step.status-invalid ${JSON.stringify(status).slice(0, 60)}`);
}

/**
 * @emoji 🧵️ Drives ONE `Effect::SpawnJob` to a terminal `job-step` — the host half of the ABI's job
 * seam, shared verbatim by every renderer door so the two targets can never drift into two
 * protocols (this module's own header: the "third divergent copy" hazard).
 *
 * The job is started once, then stepped with the kernel's STATIC admission budget until it reports a
 * terminal step or the ceiling is reached. The transcript — never this loop — decides what happened:
 * `kernel::spawnedJobCompletion` owns that rule and its Rust twin answers identically.
 *
 * A host that skips this pump is not merely slow, it is silently broken: every framework reserved
 * tool verb (`interactionSelect`/`interactionHover`/`clearSelection`) is delivered as this effect and
 * nothing else, so the actor publishes a perfect interaction message and never applies it — measured
 * on 6118, 18 dropped `spawn-job` effects per run
 * (`📓️wgpu-world3d-interaction-2026-09-13.md` §7).
 */
export async function driveSpawnedJob(drive: SpawnedJobDrive): Promise<SpawnedJobCompletion> {
  const ceiling = Math.max(1, Math.min(drive.stepCeiling ?? SPAWNED_JOB_STEP_CEILING, SPAWNED_JOB_STEP_CEILING));
  const budget = { fuel: SPAWNED_JOB_FUEL, deadlineMs: SPAWNED_JOB_DEADLINE_MS } as const;
  await drive.port.startJob(drive.job, drive.kind, drive.input);
  const steps: SpawnedJobStep[] = [];
  for (let index = 0; index < ceiling; index += 1) {
    if (drive.signal?.aborted === true) break;
    const step = wireJobStep(await drive.port.stepJob(drive.job, budget));
    steps.push(step);
    drive.onStep?.({ job: drive.job, index, status: step.status });
    if (step.status !== "running") break;
  }
  return spawnedJobCompletion(steps);
}

/** 🧵️ The actor-boundary event a completed spawned job owes its guest — the WIT `job-completed-event`
 * (`{job: u64, outcome: completion-result}`) in the same `{kind, payload}` envelope every other
 * host-submitted event uses. The guest correlates on `job` ALONE: the job id IS the parked request id
 * (`⚛️reactor/🔄️turn/🦀️.rs`), so no host-side request table exists or is needed. */
export function spawnedJobCompletedEvent(job: bigint, completion: SpawnedJobCompletion): { readonly kind: "job-completed"; readonly payload: { readonly job: bigint; readonly outcome: { readonly tag: "ok" | "fault"; readonly val: readonly number[] } } } {
  const bytes = "ok" in completion.outcome ? completion.outcome.ok : completion.outcome.fault;
  return { kind: "job-completed", payload: { job, outcome: { tag: "ok" in completion.outcome ? "ok" : "fault", val: Array.from(bytes) } } };
}

/** 🧵️ Every `spawn-job` a turn handed back, decoded once. Kept here rather than at each door so a
 * renderer never re-implements "which of these effects is a job". */
export function wireSpawnedJobs(effects: readonly WireVariant[]): readonly Extract<Effect, { readonly spawnJob: unknown }>["spawnJob"][] {
  return effects.filter((effect) => effect.tag === "spawn-job").map((effect) => wireSpawnJob(effect).spawnJob);
}
//#endregion 🧵️SpawnedJob

/** 🚧️ Best-effort conversion of a raw WIT `effect` variant into the friendly `Effect` union
 * `🎠️kernel/🟦️.ts` already declares — Rust `kernel::Effect`'s externally-tagged serde shape,
 * which every downstream consumer already expects. Covers the effect kinds a renderer commonly
 * branches on; an effect kind with no case here degrades to an honest console-logged drop rather
 * than guessing an unverified shape. */
export function wireEffectToFriendly(effect: WireVariant, decodePackValue: (bytes: Uint8Array) => unknown): Effect | null {
  const val = (effect.val ?? {}) as Record<string, unknown>;
  const str = (key: string): string => String(val[key] ?? "");
  const num = (key: string): number => Number(val[key] ?? 0);
  const packField = (key: string): unknown => (val[key] !== undefined ? decodePackValue(coerceWireBytes(val[key])) : undefined);
  // 🧬️ W0 nested every `req`-bearing effect's payload into its own `…-params` record
  // (`🔌️plugin/🧬️schema/📜️.wit`), so `dispatch-action`, `open-window` and `spawn-plugin-instance`
  // carry NOTHING at the top level but `req`. Reading them flat handed the host an effect whose
  // `action` was the empty string — a re-armed `flowEvalTick` that faulted at the action door and
  // took the wgpu boot with it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
  const params = (val.params ?? {}) as Record<string, unknown>;
  // 🎁️ A WIT `option<T>` crosses as `{tag: "some"|"none", val?}`; unwrap it before anything reads the
  // payload, or a `some` wrapper reaches `coerceWireBytes` as the pack itself.
  const some = wireOptionValue;
  const pstr = (key: string): string => String(some(params[key]) ?? "");
  const pnum = (key: string): number => Number(some(params[key]) ?? 0);
  const poptstr = (key: string): string | undefined => {
    const value = some(params[key]);
    return typeof value === "string" ? value : undefined;
  };
  const ppack = (key: string): unknown => {
    const value = some(params[key]);
    return value === undefined ? undefined : decodePackValue(coerceWireBytes(value));
  };
  switch (effect.tag) {
    case "invoke-extension":
      return wireExtensionInvocation(effect);
    case "respond":
      return wireRespondAnswer(effect);
    case "request-sync":
      return "requestSync";
    case "notify":
      return { notify: { message: str("message") } };
    case "navigate":
      return { navigate: { uri: str("uri") } };
    case "open-external-url":
      return { openExternalUrl: { url: str("url") } };
    case "download-media-export":
      return wireDownloadMediaExport(effect);
    case "spawn-job":
      return wireSpawnJob(effect);
    case "cancel-job":
      return wireCancelJob(effect);
    case "set-panel":
      return { setPanel: { panelJson: str("panelJson") } };
    case "set-active-utility":
      return { setActiveUtility: { windowId: str("windowId"), utilityId: str("utilityId") } };
    case "open-window":
      return { openWindow: { req: num("req"), kind: pstr("kind"), params: ppack("params") } };
    case "close-window":
      return { closeWindow: { window: num("window") } };
    case "spawn-plugin-instance":
      return {
        spawnPluginInstance: { req: num("req"), pluginId: pstr("pluginId"), appId: pstr("appId"), osInstanceId: poptstr("osInstanceId"), label: poptstr("label"), artifactJson: poptstr("artifactJson") },
      };
    case "open-plugin-instance":
      return { openPluginInstance: { pluginId: str("pluginId"), appId: str("appId"), osInstanceId: val.osInstanceId as string | undefined } };
    // 📤️ Every plugin's IMPORT door. This case did not exist, so `Import Document…` reached the
    // guest, the guest emitted its effect, and the wgpu door answered
    // `unmapped effect "request-file-open" dropped` — no picker, ever (ticket
    // 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §C).
    // `readAs` is a WIT `option<string>` and is OMITTED rather than carried as `undefined`: the
    // friendly `Effect` union declares it optional, and a shell reads "absent" as "hand me the
    // file's own text".
    case "request-file-open": {
      const readAs = poptstr("readAs") ?? poptstr("read-as");
      return {
        requestFileOpen: {
          req: num("req"),
          accept: pstr("accept"),
          ...(readAs === undefined ? {} : { readAs }),
          importAction: pstr("importAction") || pstr("import-action"),
          multiple: Boolean(some(params.multiple) ?? false),
        },
      };
    }
    case "dispatch-action":
      return { dispatchAction: { req: num("req"), action: pstr("action"), args: ppack("args"), delayMs: pnum("delayMs") } };
    // 📨️ Transport, never a friendly `Effect`: a `Shell{instance}` send-message IS the `AppFrame`
    // reply (`shellFrameBytes`/`leftoverShellInvocationFrames` already consumed it — an
    // `interactionSelect` completion leaves two of them, its `Invocation` reply and the reserved
    // tool job's, both applied before this mapping runs) and a `Backbone{uri}` one already went out
    // the document port. Returning `null` here is the declared shape, not the unverified-conversion
    // fallback below. Any OTHER endpoint has no host route at all, which is a real hole and stays loud.
    case "send-message": {
      if (isRoutedWireSendMessage(effect)) return null;
      console.warn(`wireEffectToFriendly: send-message to "${wireSendMessageTargetTag(effect) || "no endpoint"}" has no host route — only ${WIRE_SEND_MESSAGE_ROUTED_TARGETS.join("/")} are consumed`);
      return null;
    }
    default:
      console.warn(`wireEffectToFriendly: unmapped effect "${effect.tag}" dropped — unverified wasm-boundary conversion`);
      return null;
  }
}
//#endregion 🔖️EffectWire

//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("../🧪️tests/📨️effect-wire-routes/🟦️.ts");
  await registerTests1(import.meta.vitest, { shellFrameBytes, wireEffectToFriendly, isRoutedWireSendMessage, wireSendMessageTargetTag, WIRE_SEND_MESSAGE_ROUTED_TARGETS }, { url: import.meta.url });
  const { registerTests1: registerEffectParamsNestingTests } = await import("../🧪️tests/🎁️effect-params-nesting/🟦️.ts");
  await registerEffectParamsNestingTests(import.meta.vitest, { wireEffectToFriendly }, { url: import.meta.url });
  const { registerTests1: registerTypedOperationPageTests } = await import("../🧪️tests/🗞️typed-operation-page/🟦️.ts");
  await registerTypedOperationPageTests(
    import.meta.vitest,
    {
      scanTypedOperationPages,
      shellFrameBytes,
      shellMessageKind,
      typedOperationAcknowledgements,
      typedOperationResult,
      TYPED_OPERATION_ACK_MAGIC,
      TYPED_OPERATION_LANE_FAULT,
      TYPED_OPERATION_LANE_TERMINAL,
      TYPED_OPERATION_PAGE_HEADER_BYTES,
      TYPED_OPERATION_PAGE_MAGIC,
      TYPED_OPERATION_RESULT_LANE_MAX,
      TYPED_OPERATION_RESULT_PAGE_BYTES,
      TYPED_OPERATION_TOKEN_BYTES,
    },
    { url: import.meta.url },
  );
}
//#endregion 🧪️Tests
