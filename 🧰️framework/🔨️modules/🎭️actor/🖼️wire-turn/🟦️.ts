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
import type { Effect } from "../../🎠️kernel/🟦️.ts";
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
  const some = (value: unknown): unknown => {
    if (value === null || typeof value !== "object" || !("tag" in (value as Record<string, unknown>))) return value;
    const option = value as { readonly tag?: unknown; readonly val?: unknown };
    if (option.tag === "some") return option.val;
    if (option.tag === "none") return undefined;
    return value;
  };
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
        spawnPluginInstance: { req: num("req"), pluginId: pstr("pluginId"), appId: pstr("appId"), osInstanceId: poptstr("osInstanceId"), label: poptstr("label"), documentJson: poptstr("documentJson") },
      };
    case "open-plugin-instance":
      return { openPluginInstance: { pluginId: str("pluginId"), appId: str("appId"), osInstanceId: val.osInstanceId as string | undefined } };
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
