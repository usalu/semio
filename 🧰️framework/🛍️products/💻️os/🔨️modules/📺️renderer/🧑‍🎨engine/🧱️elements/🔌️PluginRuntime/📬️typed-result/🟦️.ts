/** 📬️ Typed-operation result pages: terminal and pending output carriers, the per-instance owner ledger routing fault pages to the host call that owns them, and the effect consumer.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { type ShardEventEnvelope } from "@semio-tech/framework-actor/shard-client/wire";
import {
  TYPED_OPERATION_LANE_FAULT,
  TYPED_OPERATION_LANE_TERMINAL,
  typedOperationAcknowledgements as wireTypedOperationAcknowledgements,
  typedOperationResult as wireTypedOperationResult
} from "../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
import { SEGMENTED_DOWNLOAD_MARKER_PREFIX } from "../../📤️SegmentedDownload/🟦️.ts";
import { decodeWirePack, DIRECTORY_PROJECTION_RECEIPT_SCHEMA, type WireTurnResult, type WireVariant } from "../🪝️wire-codec/🟦️.ts";

export const TYPED_OPERATION_TERMINAL_OUTPUT = "typed-operation-terminal-output";
export const TYPED_OPERATION_PENDING_OUTPUT = "typed-operation-pending-output";
export const TYPED_OPERATION_TERMINAL_SEEN = "typed-operation-terminal-seen";

//#region 🔖️plugin-runtime
//#region 🔖️ActorAdapter
//#region 📬️TypedOperationResult
/** 📬️ One turn's result pages and the acknowledgements they owe — the page codec is
 * `🖼️wire-turn.ts`'s, shared with the wgpu bridge; only the OWNER-attributing policy below is this
 * renderer's own. */
export const typedOperationResult = wireTypedOperationResult;

export function typedOperationAcknowledgements(result: WireTurnResult): ShardEventEnvelope[] {
  return wireTypedOperationAcknowledgements(result);
}

/** 🚨️ Real fault code a typed-operation fault page is surfaced under when no host call may be
 * rejected with it — its operation never revealed a first result page to a live call, or the call that
 * owned it has already returned. Rejecting an arbitrary in-flight caller would misattribute someone
 * else's failure, which is exactly the defect this router exists to remove. */
export const TYPED_OPERATION_UNATTRIBUTED_FAULT = "interactive-job.unattributed-result-fault";

/** 🚨️ Real fault code the parked-result registry evicts its oldest entry under once its fixed
 * capacity is exhausted — an evicted fault is always surfaced, never silently dropped. */
export const TYPED_OPERATION_PARK_EVICTION_FAULT = "interactive-job.parked-result-capacity";

/** 📏️ Foreign fault pages one instance's router parks for their owning host call before the oldest is
 * evicted through {@link TYPED_OPERATION_PARK_EVICTION_FAULT}. */
export const TYPED_OPERATION_PARK_CAPACITY = 32;

/** 🚨️ Shell fault path for a typed-operation fault page no host call may be rejected with. */
function reportTypedOperationFault(code: string, message: string): void {
  console.error(`${code}: ${message}`);
}

/** 🧭️ One host call's typed-operation ownership scope — one queued command turn, one `refreshUi`, one
 * instance-open settle, one extension completion, one document-port exchange. Opened by
 * {@link TypedOperationRouter.open}, closed exactly once by {@link withTypedOperationCall}. */
export class TypedOperationCall {
  constructor(readonly router: TypedOperationRouter, readonly label: string) {}
  observe(operation: bigint, sequence: number): boolean { return this.router.observe(this, operation, sequence); }
  fault(operation: bigint, sequence: number, message: string): boolean { return this.router.fault(this, operation, sequence, message); }
  takeFault(): string | null { return this.router.takeFault(this); }
  close(): void { this.router.close(this); }
}

/** 🧭️ Per-instance owner ledger routing typed-operation result pages to the host call that owns their
 * operation. Ownership is read off the wire the Rust host already publishes: `🔌️plugin/🦀️.rs`'s
 * `MountedTypedCommandFullOperation` mounts every operation at `result_sequence: 0` and bumps it per
 * acknowledged page (`acknowledge_result_page`), so the page carrying `sequence === 0` is that
 * operation's first reveal and names the call that dispatched it. A fault page observed inside a
 * foreign call's settle loop is parked for its owner — still acknowledged, since the guest blocks on
 * the ACK — instead of rejecting the observer; a fault whose owner already returned, or whose
 * operation never revealed a first page here, goes to the shell fault path exactly once. */
export class TypedOperationRouter {
  private readonly owners = new Map<bigint, TypedOperationCall>();
  private readonly live = new Set<TypedOperationCall>();
  private readonly parked: { readonly call: TypedOperationCall; readonly operation: bigint; readonly message: string }[] = [];
  constructor(private readonly report: (code: string, message: string) => void = reportTypedOperationFault) {}
  open(label: string): TypedOperationCall {
    const call = new TypedOperationCall(this, label);
    this.live.add(call);
    return call;
  }
  observe(call: TypedOperationCall, operation: bigint, sequence: number): boolean {
    const owner = this.owners.get(operation);
    if (owner) return owner === call;
    if (sequence !== 0 || !this.live.has(call)) return false;
    this.owners.set(operation, call);
    return true;
  }
  fault(call: TypedOperationCall, operation: bigint, sequence: number, message: string): boolean {
    if (this.observe(call, operation, sequence)) return true;
    const owner = this.owners.get(operation);
    if (!owner || !this.live.has(owner)) {
      this.report(TYPED_OPERATION_UNATTRIBUTED_FAULT, `${message} (operation ${operation} sequence ${sequence} observed by ${call.label})`);
      return false;
    }
    if (this.parked.length >= TYPED_OPERATION_PARK_CAPACITY) {
      const evicted = this.parked.shift()!;
      this.report(TYPED_OPERATION_PARK_EVICTION_FAULT, `${evicted.message} (operation ${evicted.operation} parked for ${evicted.call.label} evicted at capacity ${TYPED_OPERATION_PARK_CAPACITY})`);
    }
    this.parked.push({ call: owner, operation, message });
    return false;
  }
  takeFault(call: TypedOperationCall): string | null {
    const index = this.parked.findIndex((entry) => entry.call === call);
    return index === -1 ? null : this.parked.splice(index, 1)[0]!.message;
  }
  close(call: TypedOperationCall): void {
    this.live.delete(call);
    for (const [operation, owner] of [...this.owners]) if (owner === call) this.owners.delete(operation);
    for (let index = this.parked.length - 1; index >= 0; index -= 1) {
      if (this.parked[index]!.call !== call) continue;
      const dropped = this.parked.splice(index, 1)[0]!;
      this.report(TYPED_OPERATION_UNATTRIBUTED_FAULT, `${dropped.message} (operation ${dropped.operation} outlived ${call.label})`);
    }
  }
}

/** 🧭️ One {@link TypedOperationRouter} per live plugin instance, keyed by its actor id (1:1 with the
 * instance, the same keying {@link retainedWindowByActor} uses) and dropped by
 * {@link teardownPluginActor}. */
export const typedOperationRoutersByActor = new Map<string, TypedOperationRouter>();

/** 🧭️ Runs one host call under `actorId`'s typed-operation ownership scope: the call rejects with the
 * fault page parked for this exact call and never with a foreign operation's. */
export async function withTypedOperationCall<T>(actorId: string, label: string, body: (call: TypedOperationCall) => Promise<T>): Promise<T> {
  let router = typedOperationRoutersByActor.get(actorId);
  if (!router) {
    router = new TypedOperationRouter();
    typedOperationRoutersByActor.set(actorId, router);
  }
  const call = router.open(`${actorId}:${label}`);
  try {
    const value = await body(call);
    const fault = call.takeFault();
    if (fault !== null) throw new Error(`typed-operation failed: ${fault}`);
    return value;
  } finally {
    call.close();
  }
}

export function consumeTypedOperationEffects(effects: readonly WireVariant[], call?: TypedOperationCall): WireVariant[] {
  const consumed: WireVariant[] = [];
  let terminal = false;
  let terminalOutput: unknown = undefined;
  for (const effect of effects) {
    if (effect.tag === TYPED_OPERATION_PENDING_OUTPUT) {
      if (terminalOutput !== undefined) throw new Error("typed-operation emitted more than one directory projection receipt");
      terminalOutput = effect.val;
      continue;
    }
    if (effect.tag === TYPED_OPERATION_TERMINAL_SEEN) {
      terminal = true;
      continue;
    }
    // 🧾️ The receipt's REAL carrier. A retained typed command's `AppEvent` crosses as an ordinary
    // `publish-event` host effect — topic plus a pack-encoded payload — not as the lane-7
    // typed-operation result page the rule below expects: measured on the signed-in `s` Home, where the
    // settled operation's turn carried `["publish-event", "typed-operation-terminal-seen"]` and the
    // completion's terminal output was therefore `undefined` (ticket 26/09/18 S8). The effect itself is
    // KEPT, because the event is also a real backbone publication; only its value is lifted.
    if (effect.tag === "publish-event" && (effect.val as { readonly topic?: unknown } | null)?.topic === DIRECTORY_PROJECTION_RECEIPT_SCHEMA) {
      if (terminalOutput !== undefined) throw new Error("typed-operation emitted more than one directory projection receipt");
      terminalOutput = decodeWirePack((effect.val as { readonly payload?: unknown }).payload, "typed-operation.terminal-output");
      consumed.push(effect);
      continue;
    }
    const page = typedOperationResult(effect);
    if (!page) {
      consumed.push(effect);
      continue;
    }
    if (page.lane === TYPED_OPERATION_LANE_FAULT) {
      const message = new TextDecoder().decode(page.payload);
      if (!call || call.fault(page.token.operation, page.token.sequence, message)) throw new Error(`typed-operation failed: ${message}`);
      continue;
    }
    call?.observe(page.token.operation, page.token.sequence);
    if (page.lane === TYPED_OPERATION_LANE_TERMINAL) {
      terminal = true;
      continue;
    }
    if (page.lane === 7) {
      const event: unknown = JSON.parse(new TextDecoder().decode(page.payload));
      if (event && typeof event === "object" && !Array.isArray(event) && (event as { readonly kind?: unknown }).kind === DIRECTORY_PROJECTION_RECEIPT_SCHEMA) {
        if (terminalOutput !== undefined) throw new Error("typed-operation emitted more than one directory projection receipt");
        terminalOutput = (event as { readonly payload?: unknown }).payload;
      }
      continue;
    }
    if (page.lane !== 9) continue;
    const metadata: unknown = JSON.parse(new TextDecoder().decode(page.payload));
    if (!Array.isArray(metadata) || typeof metadata[0] !== "string" || typeof metadata[1] !== "string" || (metadata[2] !== null && metadata[2] !== "base64" && metadata[2] !== "identity")) throw new Error("typed-operation download metadata is invalid");
    consumed.push({ tag: "download-media-export", val: { filename: metadata[0], mimeType: metadata[1], data: String(page.token.operation), encoding: `${SEGMENTED_DOWNLOAD_MARKER_PREFIX}${metadata[2] ?? "identity"}` } });
  }
  if (terminalOutput !== undefined) {
    consumed.push({ tag: terminal ? TYPED_OPERATION_TERMINAL_OUTPUT : TYPED_OPERATION_PENDING_OUTPUT, val: terminalOutput });
  } else if (terminal) {
    consumed.push({ tag: TYPED_OPERATION_TERMINAL_SEEN, val: null });
  }
  return consumed;
}
//#endregion 📬️TypedOperationResult
//#endregion 🔖️ActorAdapter
//#endregion 🔖️plugin-runtime
