/** 🪝️ The wasm wire boundary decoders: WIT variant and turn-result shapes, `list<u8>` coercion, pack decoding, surface identity and the shell-frame peel the rest of the runtime reads turns through.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { nodeIdList } from "@semio-tech/framework/browser-bundle/patch-handoff";
import {
  decodePackValue
} from "@semio-tech/framework/packs";
import {
  type AppFrameValue
} from "@semio-tech/framework/wire-contracts";
import {
  decodeAppFrame,
  faultDisplayMessage
} from "@semio-tech/framework/app-channel";
import {
  decodePackWire,
  packWireNatural
} from "@semio-tech/framework/packs";
import { type UiNodeRecord, type UiPatchOp } from "@semio-tech/framework";
import { leftoverShellInvocationFrames as wireLeftoverShellInvocationFrames, shellFrameBytes as wireShellFrameBytes } from "@semio-tech/framework/actor/wire-turn";
// #region 🔌️Adapters
import { BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES } from "@semio-tech/framework/wire-contracts";
import { decodeBackboneMessage } from "@semio-tech/framework/backbone";
// #endregion 🔌️Adapters

//#region 🔖️plugin-runtime
//#region 🔖️ActorAdapter
/** 🚧️ Best-effort JS representation of one raw WIT `effect`/`patch-op` variant crossing the wasm
 * boundary — UNVERIFIED against a real compiled artifact (no plugin has migrated onto `world actor`
 * yet; W3 hasn't started — same gap `🧵️shard-client.ts`'s and `🟦️.ts`'s own
 * header docs flag). Assumed shape: jco's standard variant binding, `tag` the WIT case name
 * (kebab-case) and `val` its payload record (fields camelCased from kebab), matching the SAME
 * convention this ticket's other packets already documented for this exact boundary. */
export type WireVariant<T = unknown> = { readonly tag?: string; readonly val?: T };

export type WireUiPatch = {
  readonly surface?: { readonly instance?: number; readonly surface?: string };
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
   * selection or hover moved. Selection deliberately never rides a revisioned `UiPatch`, so this array
   * is the ONLY channel by which a guest-owned selection reaches a rendered row. */
  readonly presence?: readonly { readonly update?: unknown }[];
  readonly nextWake: number | null;
  readonly status?: unknown;
  readonly commandIngress?: WireVariant;
};

/** 📥️ Defensive parse of `ShardClient.turn()`'s opaque `unknown` return (typed opaque at that
 * module's own public boundary — see its header doc) into the fields this file needs, tolerating a
 * missing/differently-shaped field rather than throwing mid-turn. */
export function coerceTurnResult(raw: unknown): WireTurnResult {
  const record = (raw && typeof raw === "object" ? raw : {}) as Record<string, unknown>;
  const uiPatches = Array.isArray(record.uiPatches) ? (record.uiPatches as WireUiPatch[]) : [];
  const effects = Array.isArray(record.effects) ? (record.effects as WireVariant[]) : [];
  const presence = Array.isArray(record.presence) ? (record.presence as { readonly update?: unknown }[]) : [];
  const nextWake = typeof record.nextWake === "number" ? record.nextWake : null;
  const status = record.status;
  const commandIngress = record.commandIngress && typeof record.commandIngress === "object" ? (record.commandIngress as WireVariant) : undefined;
  const lifecycleReceipt = record.lifecycleReceipt;
  if (lifecycleReceipt !== undefined && lifecycleReceipt !== null && !(lifecycleReceipt instanceof Uint8Array)) throw new Error("actor-lifecycle.receipt-bytes");
  const uiPatchReceipt = record.uiPatchReceipt;
  if (uiPatchReceipt !== undefined && uiPatchReceipt !== null && !(uiPatchReceipt instanceof Uint8Array)) throw new Error("actor-ui-patch.receipt-bytes");
  return { original: raw !== null && typeof raw === "object" ? raw : undefined, lifecycleReceipt: lifecycleReceipt ?? undefined, uiPatchReceipt: uiPatchReceipt ?? undefined, uiPatches, effects, presence, nextWake, status, commandIngress };
}

/** 🧯️ Decodes the scalar WIT command-ingress fault envelope while retaining readable kernel
 * rejection codes used by pre-decode validation paths. */
export function commandIngressFaultDisplay(status: WireVariant | undefined): string {
  if (status?.tag !== "fault" || !status.val || typeof status.val !== "object") return "unknown fault";
  const fault = (status.val as { readonly fault?: unknown }).fault;
  const option = fault && typeof fault === "object" && "tag" in fault ? (fault as { readonly tag?: unknown; readonly val?: unknown }) : null;
  if (option && option.tag === "none") return "unknown fault";
  const raw = option && option.tag === "some" ? option.val : fault && typeof fault === "object" && "val" in fault ? (fault as { readonly val?: unknown }).val : fault;
  if (raw === undefined || raw === null) return "unknown fault";
  return faultDisplayMessage(Array.from(coerceWireBytes(raw)), decodePackValue);
}

/** 🔀️ `Effect::SendMessage{target: Shell{instance}}` → the raw `AppFrame` bytes it wraps. The
 * demux itself lives in `🖼️wire-turn.ts` so BOTH renderer targets read one rule (it is what tells an
 * `AppFrame` apart from a typed-operation result page riding the same endpoint); this alias keeps
 * every call site and the contract-test export surface below unchanged. */
export const shellFrameBytes = wireShellFrameBytes;

export const DIRECTORY_PROJECTION_RECEIPT_SCHEMA = "semio.space.home.directory-projection-receipt.v1";

//#region 🔖️RetainedUiPatch
/** 🩹️ `kernel::PatchOp`, TS twin restricted to what `⚛️reactor/🩹️patches/🦀️.rs`'s
 * `PatchTracker` actually emits this wave (its own doc: "full-body only — every dirty surface emits
 * one `PatchOp::Replace` at the root path"). `path` is `list<u32>` at the WIT boundary (an empty
 * array for the root).
 *
 * MIGRATION (semantic UI contract, ticket 26/08/20): the old recursive `UiNode` this file's `node`
 * payload used to carry no longer exists — `🛂️manifest/🟦️.ts`'s hand-written mirror was
 * deleted in favor of `semio-framework-ui-contract`'s flat, id-keyed `UiSnapshot`/`UiNodeRecord`
 * (`📃️UiDocumentStore`'s header doc). A "whole-body replace" is now a whole `UiSnapshot` (root pointer +
 * flat node table), not a single recursive node; an "insert one child" is a single `UiNodeRecord` row.
 * The kernel/actor WIT boundary this file bridges has not flipped onto the new wire tags yet (owned by
 * the MICROKERNEL program's `sdk-flip`/`wit-flip` packets, forbidden here) — this is therefore a
 * type-level migration only, matching this whole boundary's own doc ("UNVERIFIED against a real
 * compiled artifact... no plugin has migrated onto `world actor` yet"). */
export function decodeWirePatchOps(ops: readonly WireVariant[]): readonly UiPatchOp[] {
  const decoded: UiPatchOp[] = [];
  for (const op of ops) {
    const val = (op.val ?? {}) as Record<string, unknown>;
    switch (op.tag) {
      case "upsert":
        decoded.push({ type: "upsert", ...normalizeWireUiNodeRecord(decodeWirePack(val.node, "upsert.node")) });
        break;
      case "set-component":
        decoded.push({ type: "setComponent", id: wireNatural(val.node, "op.node"), component: decodeWirePack(val.component, "set-component.component") as Extract<UiPatchOp, { type: "setComponent" }>["component"] });
        break;
      case "set-layout":
        decoded.push({ type: "setLayout", id: wireNatural(val.node, "op.node"), layout: decodeWirePack(val.layout, "set-layout.layout") as Extract<UiPatchOp, { type: "setLayout" }>["layout"] });
        break;
      case "set-activity": {
        const activity = decodeWirePack(val.activity, "set-activity.activity") as Pick<Extract<UiPatchOp, { type: "setActivity" }>, "activity" | "disabled">;
        decoded.push({ type: "setActivity", id: wireNatural(val.node, "op.node"), activity: activity.activity, disabled: activity.disabled });
        break;
      }
      case "set-children":
        decoded.push({ type: "setChildren", id: wireNatural(val.node, "op.node"), children: nodeIdList(val.children, "set-children.children").map((child) => wireNatural(child, "set-children.children[]")) });
        break;
      case "set-style":
        decoded.push({ type: "setStyle", id: wireNatural(val.node, "op.node"), style: decodeWirePack(val.style, "set-style.style") as Extract<UiPatchOp, { type: "setStyle" }>["style"] });
        break;
      case "set-accessibility":
        decoded.push({ type: "setAccessibility", id: wireNatural(val.node, "op.node"), accessibility: decodeWirePack(val.accessibility, "set-accessibility.accessibility") as Extract<UiPatchOp, { type: "setAccessibility" }>["accessibility"] });
        break;
      case "set-bindings":
        decoded.push({ type: "setBindings", id: wireNatural(val.node, "op.node"), bindings: decodeWirePack(val.bindings, "set-bindings.bindings") as Extract<UiPatchOp, { type: "setBindings" }>["bindings"] });
        break;
      case "set-menu":
        decoded.push({ type: "setMenu", id: wireNatural(val.node, "op.node"), menu: decodeWirePack(val.menu, "set-menu.menu") as Extract<UiPatchOp, { type: "setMenu" }>["menu"] });
        break;
      case "remove":
        decoded.push({ type: "remove", id: wireNatural(op.val, "remove.val") });
        break;
      case "set-root":
        decoded.push({ type: "setRoot", id: wireNatural(op.val, "set-root.val") });
        break;
      default:
        break;
    }
  }
  return decoded;
}

/** 🔢️ One natural number off the actor WIT/pack boundary — {@link packWireNatural}, the single
 * implementation the wgpu bridge shares, applied to whatever jco handed this shell. */
export const wireNatural = packWireNatural;

/** 📦️ Decodes one pack-encoded WIT byte payload and projects its lossless integer carriers onto exact
 * JSON numbers — the only shape the UI contract twins (`UiNodeRecord`, components, layouts, styles)
 * declare. Wraps {@link decodePackWire} with this boundary's own `list<u8>` coercion. */
export function decodeWirePack(raw: unknown, path: string): unknown {
  return decodePackWire(coerceWireBytes(raw), path);
}

export function normalizeWireUiNodeRecord(raw: unknown): UiNodeRecord {
  const record = raw as Partial<UiNodeRecord>;
  return {
    ...(record as UiNodeRecord),
    id: wireNatural(record.id, "record.id"),
    disabled: record.disabled ?? false,
    transition: record.transition ?? null,
    bindings: Array.isArray(record.bindings) ? record.bindings : [],
    menu: record.menu ?? null,
    children: Array.isArray(record.children) ? record.children.map((child) => wireNatural(child, "record.children[]")) : [],
  };
}
//#endregion 🔖️RetainedUiPatch

//#region 🔖️PluginTurnScheduler
/** 🪟️ A patch's surface IS its mounted identity: the guest addresses one concrete window
 * instance, app panel or reserved section, and there is no fallback name for a patch that names none.
 *
 * 🐛️ This used to default a missing body to `window` (ticket 26/09/02/PUZZLE-3D-END-TO-END wave
 * B56), which minted the synthetic `<instance>:window` surface the host then had to bind a host context
 * for — a surface no pane reads, competing for the process-wide reconcile credit with every real one
 * and holding the reservation that starved the whole reactor (wave B54 §6.4). A patch with no body key
 * is now refused by its caller (`plugin-ui.projection-surface-required`) instead of routed to an alias. */
export function wirePatchSurfaceId(patch: WireUiPatch): string | null {
  const body = patch.surface?.surface;
  return patch.surface && body ? retainedSurfaceId(wireNatural(patch.surface.instance), body) : null;
}
//#endregion 🔖️PluginTurnScheduler
//#endregion 🔖️ActorAdapter

export function retainedSurfaceId(instance: number, surface: string): string {
  return `${instance}:${surface}`;
}

/** 🔌️ One document-port payload, admitted only when it is a bounded non-genesis hot backbone message. */
export function documentPortHotBytes(raw: unknown): Uint8Array {
  const length = raw instanceof Uint8Array || Array.isArray(raw) ? raw.length : 0;
  if (!length || length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || (Array.isArray(raw) && raw.some(value => !Number.isInteger(value) || value < 0 || value > 255))) throw new Error("actor-document-port.message-size");
  const bytes = raw instanceof Uint8Array ? raw : Uint8Array.from(raw as number[]);
  if (decodeBackboneMessage(bytes).kind === "genesis") throw new Error("actor-document-port.genesis-requires-cold-pair");
  return bytes;
}

//#region 🔖️ChannelAdapter
/** 🎯️ DslValue may ship `Vec<u8>` as a number array, a Uint8Array, or a `{ kind:"bytes", value }`
 * object — used both for the old DSL-pack byte fields AND (H1-react) for `pack`-typed fields inside a
 * raw WIT `effect`/`patch-op` variant, which jco represents as a plain byte array or `Uint8Array`. */

/** 🕹️ ONE law for both renderer targets (`🖼️wire-turn.ts`) — a reserved tool job publishes the whole
 * answer of its verb on a later turn than the dispatch that admitted it, and both hosts fold it
 * through this same peel. */
export function leftoverShellInvocationFrames(leftover: readonly WireVariant[]): Extract<AppFrameValue, { readonly Invocation: unknown }>[] {
  return wireLeftoverShellInvocationFrames<AppFrameValue>(leftover, (bytes) => decodeAppFrame(bytes)).filter((frame): frame is Extract<AppFrameValue, { readonly Invocation: unknown }> => "Invocation" in frame);
}

export function coerceWireBytes(raw: unknown): Uint8Array {
  if (raw instanceof Uint8Array) return raw;
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
//#endregion 🔖️ChannelAdapter

//#region 🔖️Transaction
/** 🔢️ FNV-1a — used only for the transaction cycle-detection key and this coordinator's own
 * best-effort `MutationOrigin.contributed.payloadHash` (see {@link encodeMutationOrigin}'s doc); never
 * asserted byte-identical against Rust's `PayloadHash`. */
export function fnv1aHex(bytes: Uint8Array): string {
  let hash = 0x811c9dc5;
  for (let index = 0; index < bytes.length; index += 1) {
    hash ^= bytes[index]!;
    hash = Math.imul(hash, 0x01000193);
  }
  return (hash >>> 0).toString(16);
}
//#endregion 🔖️Transaction
//#endregion 🔖️plugin-runtime
