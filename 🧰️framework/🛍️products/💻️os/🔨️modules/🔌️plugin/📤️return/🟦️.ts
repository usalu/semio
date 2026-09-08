//#region 🧬️WitReturnContract
import { encodeActorReturnDrive, encodeActorReturnResult, type ActorReturnControl, type ActorReturnDrive, type ActorReturnFault, type ActorReturnIdentity, type ActorReturnOrigin, type ActorReturnPageReceipt, type ActorReturnResult } from "../../../../../🔨️modules/🎭️actor/📤️return/🟦️.ts";
import type { ActorBytePage } from "../../../../../🔨️modules/🎭️actor/📃️page/🟦️.ts";
type WitOrigin = { readonly activationGeneration: bigint; readonly requestSequence: bigint };
type WitIdentity = { readonly origin: WitOrigin; readonly returnSequence: bigint };
type WitPageReceipt = { readonly identity: WitIdentity; readonly pageSequence: bigint; readonly length: number; readonly final: boolean };
type WitControl = { readonly tag: "poll" | "cancel" | "retired-ack"; readonly val: WitIdentity } | { readonly tag: "input-ack"; readonly val: WitPageReceipt };
export type PluginReturnWitDrive = { readonly tag: "execute"; readonly val: WitOrigin } | { readonly tag: "control"; readonly val: WitControl };
function mappingFault(reason: string): never { throw new Error(`plugin-return.${reason}`); }
function generation(value: bigint): void { if (typeof value !== "bigint" || value <= 0n || value > 0xffffffffffffffffn) mappingFault("activation"); }
function field(value: unknown, name: string): unknown {
  if (value === null || typeof value !== "object") return mappingFault("field");
  const descriptor = Object.getOwnPropertyDescriptor(value, name);
  if (!descriptor || !("value" in descriptor)) return mappingFault("field");
  return descriptor.value;
}
function controlOrigin(value: ActorReturnControl): ActorReturnOrigin { return (value.kind === "inputAck" ? value.receipt.identity : value.identity).origin; }
function toOrigin(value: ActorReturnOrigin): WitOrigin { return Object.freeze({ activationGeneration: value.activationGeneration, requestSequence: BigInt(value.requestSequence) }); }
function toIdentity(value: ActorReturnIdentity): WitIdentity { return Object.freeze({ origin: toOrigin(value.origin), returnSequence: value.returnSequence }); }
function toReceipt(value: ActorReturnPageReceipt): WitPageReceipt { return Object.freeze({ identity: toIdentity(value.identity), pageSequence: value.pageSequence, length: value.length, final: value.final }); }
function toControl(value: ActorReturnControl): WitControl {
  if (value.kind === "inputAck") return Object.freeze({ tag: "input-ack", val: toReceipt(value.receipt) });
  return Object.freeze({ tag: value.kind === "retiredAck" ? "retired-ack" : value.kind, val: toIdentity(value.identity) });
}

/** 🧵️ Maps one validated canonical drive to the type-only WIT contract; the captured caller retains dispatch authority. */
export function pluginReturnDriveToWit(drive: ActorReturnDrive, activationGeneration: bigint): PluginReturnWitDrive {
  generation(activationGeneration);
  encodeActorReturnDrive(drive);
  const origin = drive.kind === "execute" ? drive.origin : controlOrigin(drive.control);
  if (origin.activationGeneration !== activationGeneration) mappingFault("activation-mismatch");
  return drive.kind === "execute" ? Object.freeze({ tag: "execute", val: toOrigin(drive.origin) }) : Object.freeze({ tag: "control", val: toControl(drive.control) });
}
//#endregion 🧬️WitReturnContract

//#region 📤️WitReturnEncoding
function fromOrigin(value: unknown, activation: bigint): ActorReturnOrigin {
  const requestSequence = field(value, "requestSequence");
  if (typeof requestSequence !== "bigint" || requestSequence <= 0n || requestSequence > BigInt(Number.MAX_SAFE_INTEGER)) return mappingFault("request-sequence");
  const activationGeneration = field(value, "activationGeneration");
  if (activationGeneration !== activation) return mappingFault("activation-mismatch");
  return { activationGeneration, requestSequence: Number(requestSequence) };
}
function fromIdentity(value: unknown, activation: bigint): ActorReturnIdentity { return { origin: fromOrigin(field(value, "origin"), activation), returnSequence: field(value, "returnSequence") as bigint }; }
function fromReceipt(value: unknown, activation: bigint): ActorReturnPageReceipt {
  return { identity: fromIdentity(field(value, "identity"), activation), pageSequence: field(value, "pageSequence") as bigint, length: field(value, "length") as number, final: field(value, "final") as boolean };
}
function fromControl(value: unknown, activation: bigint): ActorReturnControl {
  const tag = field(value, "tag"); const body = field(value, "val");
  switch (tag) {
    case "poll": return { kind: "poll", identity: fromIdentity(body, activation) };
    case "cancel": return { kind: "cancel", identity: fromIdentity(body, activation) };
    case "retired-ack": return { kind: "retiredAck", identity: fromIdentity(body, activation) };
    case "input-ack": return { kind: "inputAck", receipt: fromReceipt(body, activation) };
    default: return mappingFault("control-tag");
  }
}
function fromFault(value: unknown): ActorReturnFault {
  switch (value) {
    case "none": case "capacity": case "deadline": return value;
    case "sequence-exhausted": return "sequenceExhausted";
    case "stale-origin": return "staleOrigin";
    case "stale-identity": return "staleIdentity";
    case "wrong-page": return "wrongPage";
    case "input-not-retired": return "inputNotRetired";
    case "not-retired": return "notRetired";
    case "clock-unavailable": return "clockUnavailable";
    case "clock-backward": return "clockBackward";
    case "owner-fault": return "ownerFault";
    case "malformed-control": return "malformedControl";
    case "mixed-control": return "mixedControl";
    default: return mappingFault("fault-enum");
  }
}
function fromReason(value: unknown): Extract<ActorReturnResult, { kind: "pending" }>["reason"] {
  switch (value) {
    case "working": case "blocked": case "closing": return value;
    case "awaiting-input": return "awaitingInput";
    default: return mappingFault("pending-reason");
  }
}

/** 📮️ Encodes selected fixed WIT fields; the caller must retain the original response and pre-admit all format allocations. */
export function encodePluginReturnResult(original: unknown, activationGeneration: bigint): Uint8Array {
  generation(activationGeneration);
  const tag = field(original, "tag"); const body = field(original, "val");
  let result: ActorReturnResult;
  switch (tag) {
    case "protocol-fault": {
      const fault = fromFault(body);
      if (fault !== "malformedControl" && fault !== "mixedControl") return mappingFault("protocol-fault");
      result = { kind: "protocolFault", fault }; break;
    }
    case "refused": result = { kind: "refused", origin: fromOrigin(field(body, "origin"), activationGeneration), fault: fromFault(field(body, "fault")) }; break;
    case "pending": result = { kind: "pending", identity: fromIdentity(field(body, "identity"), activationGeneration), reason: fromReason(field(body, "reason")) }; break;
    case "page": result = { kind: "page", receipt: fromReceipt(field(body, "receipt"), activationGeneration), page: field(body, "page") as ActorBytePage }; break;
    case "retired": result = { kind: "retired", identity: fromIdentity(field(body, "identity"), activationGeneration), completion: field(body, "completion") as Extract<ActorReturnResult, { kind: "retired" }>["completion"] }; break;
    case "control": result = { kind: "control", control: fromControl(field(body, "control"), activationGeneration), outcome: field(body, "outcome") as Extract<ActorReturnResult, { kind: "control" }>["outcome"], fault: fromFault(field(body, "fault")) }; break;
    default: return mappingFault("result-tag");
  }
  return encodeActorReturnResult(result);
}
//#endregion 📤️WitReturnEncoding

//#region 🧪️WitReturnLaws
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️pluginreturnwit-maps-every-canonical-drive-to-the-exact-wit-nesting-and/🟦️.ts");
  await registerTests1(import.meta.vitest, { encodePluginReturnResult, generation, pluginReturnDriveToWit }, { url: import.meta.url });
}
//#endregion 🧪️WitReturnLaws
