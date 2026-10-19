/** 🧾️ Tool run JSON wire: the lossless JSON forms (u64 as decimal text, bytes as hex) of identities, steps, progress, trace and ticks. */

import { ToolRunCodecError, type ToolRunIdentity } from "../🟦️.ts";
import type { ToolRunEvent, ToolRunSlot } from "../🚦️lifecycle/🟦️.ts";
import type { ToolRunProgress, ToolRunStep } from "../📊️progress/🟦️.ts";
import type { ToolRunTraceDelta, ToolRunTraceOp, ToolRunTracePage, ToolRunTraceSubject } from "../🗄️trace-store/🟦️.ts";
import type { ToolRunTick } from "../🎞️tick/🟦️.ts";

//#region 🔖️Json
/** 🧾️ Schema JSON form (`🧬️schema/🔣️.json`): u64 as integers, bytes as lowercase hex. */
export type ToolRunJson = null | boolean | number | string | readonly ToolRunJson[] | { readonly [key: string]: ToolRunJson };
type JsonObject = { readonly [key: string]: any };

export function toolRunHexToBytes(hex: string): Uint8Array {
  if (!/^([0-9a-f]{2})*$/.test(hex)) throw new ToolRunCodecError("malformed", "hex");
  const bytes = new Uint8Array(hex.length / 2);
  for (let index = 0; index < bytes.length; index += 1) bytes[index] = parseInt(hex.slice(2 * index, 2 * index + 2), 16);
  return bytes;
}

export function toolRunBytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

const u64Json = (value: bigint): number => {
  if (value > BigInt(Number.MAX_SAFE_INTEGER)) throw new ToolRunCodecError("limit", "u64 beyond JSON safe integers");
  return Number(value);
};

export function toolRunIdentityFromJson(json: JsonObject): ToolRunIdentity {
  return { id: { appInstanceId: json.id.appInstanceId, run: BigInt(json.id.run) }, generation: json.generation, baseRevision: toolRunHexToBytes(json.baseRevision) };
}

export function toolRunIdentityToJson(identity: ToolRunIdentity): ToolRunJson {
  return { id: { appInstanceId: identity.id.appInstanceId, run: u64Json(identity.id.run) }, generation: identity.generation, baseRevision: toolRunBytesToHex(identity.baseRevision) };
}

export function toolRunSlotFromJson(json: JsonObject | null): ToolRunSlot | null {
  return json === null ? null : { run: BigInt(json.run), generation: json.generation, state: json.state };
}

export function toolRunSlotToJson(slot: ToolRunSlot | null): ToolRunJson {
  return slot === null ? null : { run: u64Json(slot.run), generation: slot.generation, state: slot.state };
}

export function toolRunEventFromJson(json: JsonObject): ToolRunEvent {
  return (json.type === "closed" ? { type: "closed" } : { ...json, run: BigInt(json.run) }) as ToolRunEvent;
}

export function toolRunStepFromJson(json: JsonObject): ToolRunStep {
  return { sequence: BigInt(json.sequence), kind: json.kind, stage: json.stage, reason: json.reason, ...(json.subject === undefined ? {} : { subject: BigInt(json.subject) }), repeat: json.repeat, args: json.args.map((arg: JsonObject) => ("unsigned" in arg ? { unsigned: BigInt(arg.unsigned) } : { float: arg.float })) };
}

export function toolRunStepToJson(step: ToolRunStep): ToolRunJson {
  return { sequence: u64Json(step.sequence), kind: step.kind, stage: step.stage, reason: step.reason, ...(step.subject === undefined ? {} : { subject: u64Json(step.subject) }), repeat: step.repeat, args: step.args.map((arg): ToolRunJson => ("unsigned" in arg ? { unsigned: u64Json(arg.unsigned) } : { float: arg.float })) };
}

export function toolRunTraceOpFromJson(json: JsonObject): ToolRunTraceOp {
  if (json.op === "clear") return { op: "clear" };
  if (json.op === "retire") return { op: "retire", key: BigInt(json.key) };
  const subject: ToolRunTraceSubject = json.subject.kind === "entity" ? { kind: "entity", entity: BigInt(json.subject.entity) } : json.subject;
  return { op: "upsert", key: BigInt(json.key), verdict: json.verdict, reason: json.reason, subject };
}

export function toolRunTraceOpToJson(op: ToolRunTraceOp): ToolRunJson {
  if (op.op === "clear") return { op: "clear" };
  if (op.op === "retire") return { op: "retire", key: u64Json(op.key) };
  const subject: ToolRunJson = op.subject.kind === "entity" ? { kind: "entity", entity: u64Json(op.subject.entity) } : { ...op.subject, position: [...op.subject.position], rotation: op.subject.kind === "instance3d" ? [...op.subject.rotation] : op.subject.rotation };
  return { op: "upsert", key: u64Json(op.key), verdict: op.verdict, reason: op.reason, subject };
}

export function toolRunTracePageFromJson(json: JsonObject): ToolRunTracePage {
  return { identity: toolRunIdentityFromJson(json.identity), page: json.page, ops: json.ops.map(toolRunTraceOpFromJson) };
}

export function toolRunTracePageToJson(page: ToolRunTracePage): ToolRunJson {
  return { identity: toolRunIdentityToJson(page.identity), page: page.page, ops: page.ops.map(toolRunTraceOpToJson) };
}

export function toolRunTraceDeltaFromJson(json: JsonObject): ToolRunTraceDelta {
  return { identity: toolRunIdentityFromJson(json.identity), clear: json.clear, next: json.next, pages: json.pages.map(toolRunTracePageFromJson) };
}

export function toolRunTraceDeltaToJson(delta: ToolRunTraceDelta): ToolRunJson {
  return { identity: toolRunIdentityToJson(delta.identity), clear: delta.clear, next: delta.next, pages: delta.pages.map(toolRunTracePageToJson) };
}

export function toolRunProgressFromJson(json: JsonObject): ToolRunProgress {
  return {
    identity: toolRunIdentityFromJson(json.identity),
    sequence: BigInt(json.sequence),
    state: json.state,
    stage: json.stage,
    completed: BigInt(json.completed),
    ...(json.total === undefined ? {} : { total: BigInt(json.total) }),
    counters: json.counters.map((counter: JsonObject) => ({ counter: counter.counter, value: BigInt(counter.value) })),
    unitsPerSecond: json.unitsPerSecond,
    conflicts: json.conflicts,
    steps: json.steps.map(toolRunStepFromJson),
  };
}

export function toolRunProgressToJson(progress: ToolRunProgress): ToolRunJson {
  return {
    identity: toolRunIdentityToJson(progress.identity),
    sequence: u64Json(progress.sequence),
    state: progress.state,
    stage: progress.stage,
    completed: u64Json(progress.completed),
    ...(progress.total === undefined ? {} : { total: u64Json(progress.total) }),
    counters: progress.counters.map((counter) => ({ counter: counter.counter, value: u64Json(counter.value) })),
    unitsPerSecond: progress.unitsPerSecond,
    conflicts: progress.conflicts,
    steps: progress.steps.map(toolRunStepToJson),
  };
}

export function toolRunTickFromJson(json: JsonObject): ToolRunTick {
    const payload=json.payload===undefined?undefined:toolRunHexToBytes(json.payload);
  const payloadUpdate=json.payloadUpdate===undefined?undefined:json.payloadUpdate.kind==="replace"?{kind:"replace"as const}:json.payloadUpdate.kind==="range"?{kind:"range"as const,publication:BigInt(json.payloadUpdate.publication),totalBytes:BigInt(json.payloadUpdate.totalBytes),offset:BigInt(json.payloadUpdate.offset),chunkBytes:json.payloadUpdate.chunkBytes,final:json.payloadUpdate.final}:undefined;
  if(json.payloadUpdate!==undefined&&payloadUpdate===undefined)throw new ToolRunCodecError("malformed","payloadUpdate");
  validateTickPayload(payload,payloadUpdate);
return {
    identity: toolRunIdentityFromJson(json.identity),
    sequence: BigInt(json.sequence),
    ...(json.progress === undefined ? {} : { progress: toolRunProgressFromJson(json.progress) }),
    steps: json.steps.map(toolRunStepFromJson),
    trace: json.trace.map(toolRunTracePageFromJson),
    appendOps: json.appendOps.map(toolRunHexToBytes),
    appendEntities: json.appendEntities.map((entity: number) => BigInt(entity)),
    ...(json.retractTo === undefined ? {} : { retractTo: json.retractTo }),
    ...(payload===undefined?{}:{payload}),
    ...(payloadUpdate===undefined?{}:{payloadUpdate}),
  };
}

export function toolRunTickToJson(tick: ToolRunTick): ToolRunJson {
  return {
    identity: toolRunIdentityToJson(tick.identity),
    sequence: u64Json(tick.sequence),
    ...(tick.progress === undefined ? {} : { progress: toolRunProgressToJson(tick.progress) }),
    steps: tick.steps.map(toolRunStepToJson),
    trace: tick.trace.map(toolRunTracePageToJson),
    appendOps: tick.appendOps.map(toolRunBytesToHex),
    appendEntities: tick.appendEntities.map(u64Json),
    ...(tick.retractTo === undefined ? {} : { retractTo: tick.retractTo }),
    ...(tick.payload === undefined ? {} : { payload: toolRunBytesToHex(tick.payload) }),
      ...(tick.payloadUpdate===undefined?{}:{payloadUpdate:tick.payloadUpdate.kind==="replace"?{kind:"replace"}:{kind:"range",publication:u64Json(tick.payloadUpdate.publication),totalBytes:u64Json(tick.payloadUpdate.totalBytes),offset:u64Json(tick.payloadUpdate.offset),chunkBytes:tick.payloadUpdate.chunkBytes,final:tick.payloadUpdate.final}}),
};
}
//#endregion 🔖️Json
