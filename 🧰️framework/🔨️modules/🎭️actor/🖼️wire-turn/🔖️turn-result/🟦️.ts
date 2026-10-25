// #region 🧲️Header
/** 🔖️ The raw bytes, the `turn-result` shape and the typed-operation pages of one turn: coercing the opaque `unknown` result, the shell frames and messages it carries, its routed `send-message` effects, its status tag, and the page and acknowledgement codec — the magics, the lanes, the diagnostic fragments and the drain of typed-operation turns. */
// #endregion 🧲️Header

// #region 🔌️Imports
import { parseWitColdPairIngressStatus, type ColdPairIngressStatus } from "../../📥️cold-pair/🟦️.ts";
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

/** 🎞️ `Effect::SendMessage{target: Shell{instance}}` → the raw `AppFrame` bytes it wraps —
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

/** 💬️ Which wire language one `Shell{instance}` payload speaks, or `null` when the effect is not a
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

/** 🗺️ Every `MessageEndpoint` a host route already owns. `Shell{instance}` is the reply-frame
 * transport ({@link shellFrameBytes}, and `leftoverShellInvocationFrames` for a frame that lands on a
 * later turn than the call it answers); `Backbone{uri}` is the document port. Both are consumed
 * BEFORE {@link wireEffectToFriendly} ever sees them — they are transport, not a friendly `Effect`,
 * and the friendly union deliberately declares no `sendMessage` member for them. */
export const WIRE_SEND_MESSAGE_ROUTED_TARGETS: readonly string[] = ["shell", "backbone"];

/** 📮️ Whether a `send-message` effect is already owned by one of {@link WIRE_SEND_MESSAGE_ROUTED_TARGETS}. */
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
/** 🪄️ `TypedOperationResultPage::RENDERER_ACK_MAGIC` (`🔌️plugin/🦀️.rs`), byte for byte. */
export const TYPED_OPERATION_ACK_MAGIC = new TextEncoder().encode("semio.typed-operation-ack.v1\0");
/** 📏️ `{receiver u32 | operation u64 | generation u64 | sequence u32 | attempt u8 | lane u8 | len u32}`,
 * little-endian — `renderer_exchange_bytes`' fixed header after the magic. */
export const TYPED_OPERATION_PAGE_HEADER_BYTES = 30;
/** 🔑️ The acknowledgement body: the page header's first 25 bytes, i.e. its token alone. */
export const TYPED_OPERATION_TOKEN_BYTES = 25;
/** 🧱️ `TYPED_OPERATION_RESULT_PAGE_BYTES` (`🔌️plugin/🦀️.rs`) — one page's fixed payload authority. */
export const TYPED_OPERATION_RESULT_PAGE_BYTES = 4_096;
/** 🛤️ Highest `TypedOperationResultLane` discriminant the guest emits (`🔌️plugin/🦀️.rs`: Artifact 0 …
 * Fault 11, Interaction 12, WindowTransient 13, WindowConfig 14). */
export const TYPED_OPERATION_RESULT_LANE_MAX = 15;
/** ✏️ The lane a typed operation's document edit page rides (`{generationBefore, generationAfter}`). */
export const TYPED_OPERATION_LANE_ARTIFACT = 0;
/** 💻️ The lane a terminal result page rides. */
export const TYPED_OPERATION_LANE_TERMINAL = 10;
/** 🚦️ The lane a fault result page rides. */
export const TYPED_OPERATION_LANE_FAULT = 11;
/** ⚠️ Original operation diagnostic JSON fragments remain nonterminal and require exact ACK. */
export const TYPED_OPERATION_LANE_DIAGNOSTIC = 15;

/** 🪪️ One result page's exact identity — the tuple `renderer_ack_token` reads back out of an ACK. */
export type TypedOperationToken = {
  readonly receiver: number;
  readonly operation: bigint;
  readonly generation: bigint;
  readonly sequence: number;
  readonly attempt: number;
};

/** ✉️ The `Event::Message{source: Shell{instance}}` envelope that acknowledges one page. Structurally
 * a `ShardEventEnvelope`; declared here rather than imported so this module keeps its "transports are
 * injected" discipline. */
export type TypedOperationAckEvent = {
  readonly kind: "message";
  readonly payload: { readonly source: { readonly tag: "shell"; readonly val: string }; readonly payload: readonly number[] };
};

/** 📃️ One decoded typed-operation result page and the acknowledgement the guest blocks on. */
export type TypedOperationPage = {
  readonly token: TypedOperationToken;
  readonly lane: number;
  readonly payload: Uint8Array;
  readonly acknowledgement: TypedOperationAckEvent;
};

/** 🩺️ A borrowed original diagnostic JSON window keeps exact operation and row provenance. */
export type TypedOperationDiagnosticFragment = {
  readonly token: TypedOperationToken;
  readonly row: number;
  readonly complete: boolean;
  readonly bytes: Uint8Array;
};

/** 📤️ Diagnostic windows preserve arbitrary UTF8 boundaries for the shared asynchronous JSON reader. */
export function typedOperationDiagnosticFragment(page: TypedOperationPage): TypedOperationDiagnosticFragment | null {
  if (page.lane !== TYPED_OPERATION_LANE_DIAGNOSTIC) return null;
  if (page.payload.byteLength < 5 || page.payload[4]! > 1) throw new Error("typed-operation diagnostic fragment violates its row or completion authority");
  const header = new DataView(page.payload.buffer, page.payload.byteOffset, 5);
  return { token: page.token, row: header.getUint32(0, true), complete: page.payload[4] === 1, bytes: page.payload.subarray(5) };
}

/** 📄️ Decodes one `Shell{instance}`-addressed typed-operation result page, or `null` when the effect
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

/** ⌨️ Every acknowledgement one turn's result pages owe. The guest PARKS its operation until each
 * lands, so a host that collects pages without answering them freezes the operation mid-flight. */
export function typedOperationAcknowledgements(turn: { readonly effects: readonly WireVariant[] }): TypedOperationAckEvent[] {
  return turn.effects.flatMap((effect) => {
    const page = typedOperationResult(effect);
    return page ? [page.acknowledgement] : [];
  });
}

/** 🔂️ What one turn's effects hold once its typed-operation pages are taken out of them. Pure: it
 * chooses no policy for a fault or a terminal, it only names them, so React's owner-attributing
 * router and the wgpu bridge's single-caller ladder read ONE scan rather than two decoders. */
export type TypedOperationPageScan = {
  readonly kept: readonly WireVariant[];
  readonly pages: readonly TypedOperationPage[];
  readonly acknowledgements: readonly TypedOperationAckEvent[];
  readonly terminal: boolean;
  readonly faults: readonly string[];
  readonly diagnosticFragments: readonly TypedOperationDiagnosticFragment[];
};

export function scanTypedOperationPages(effects: readonly WireVariant[]): TypedOperationPageScan {
  const kept: WireVariant[] = [];
  const pages: TypedOperationPage[] = [];
  const faults: string[] = [];
  const diagnosticFragments: TypedOperationDiagnosticFragment[] = [];
  let terminal = false;
  for (const effect of effects) {
    const page = typedOperationResult(effect);
    if (!page) {
      kept.push(effect);
      continue;
    }
    pages.push(page);
    const diagnostic = typedOperationDiagnosticFragment(page);
    if (diagnostic !== null) diagnosticFragments.push(diagnostic);
    if (page.lane === TYPED_OPERATION_LANE_FAULT) faults.push(new TextDecoder().decode(page.payload));
    if (page.lane === TYPED_OPERATION_LANE_TERMINAL) terminal = true;
  }
  return { kept, pages, acknowledgements: pages.map((page) => page.acknowledgement), terminal, faults, diagnosticFragments };
}

/** 🖥️ What ONE typed-operation result page asks of the host driving an action turn: its acknowledgement — a fault
 * page's too, since the guest parks the operation until every page lands and an unanswered fault page left the
 * document's actor unable to take its next action — whether it is the guest's refusal of the action, and whether it
 * rides the artifact lane. Policy stays with the caller: it refuses the action only after the whole turn is answered. */
export function typedOperationPageAnswerV1(page: TypedOperationPage): Readonly<{ acknowledgement: TypedOperationAckEvent; refused: boolean; artifact: boolean }> {
  return { acknowledgement: page.acknowledgement, refused: page.lane === TYPED_OPERATION_LANE_FAULT, artifact: page.lane === TYPED_OPERATION_LANE_ARTIFACT };
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
