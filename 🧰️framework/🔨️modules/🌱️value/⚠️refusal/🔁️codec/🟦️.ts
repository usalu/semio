import { ValueError, valueRefusalKindFromWire, type ValueRefusalKind, type ValueErrorRetainedProgress } from "../🟦️.ts";
import type { IntrinsicValue } from "../../🧬️schema/🌳️intrinsic/🟦️.ts";

/** 🚦️ A caller owns cumulative storage, nested workload restoration and every cancellation frontier. */
export interface ValueRefusalCodecControl {
  checkpoint(): Promise<void>;
  charge(bytes: number): Promise<void>;
  step(): Promise<void>;
  advance(units: number): Promise<void>;
  beginStage(total: number): Promise<void>;
  scopedStage<T>(operation: () => Promise<T>): Promise<T>;
}
function kind(text: string): ValueRefusalKind {
  const value=valueRefusalKindFromWire(text);if(value===undefined)throw new ValueError("invalidValue","unknown refusal kind");return value;
}
async function copyText(text: string, control: ValueRefusalCodecControl): Promise<string> {
  return control.scopedStage(async () => {
    await control.beginStage(text.length);
    let bytes = 0, advanced = 0;
    for (let index = 0; index < text.length;) {
      const word = text.charCodeAt(index); let units = 1;
      if (word < 0x80) bytes++; else if (word < 0x800) bytes += 2;
      else if (word >= 0xd800 && word <= 0xdbff) {
        const next = text.charCodeAt(index + 1);
        if (!(next >= 0xdc00 && next <= 0xdfff)) throw new ValueError("invalidValue", "refusal text has an unpaired surrogate");
        bytes += 4; units = 2;
      } else if (word >= 0xdc00 && word <= 0xdfff) throw new ValueError("invalidValue", "refusal text has an unpaired surrogate");
      else bytes += 3;
      index += units;
      if (index - advanced >= 256 || index === text.length) { await control.advance(index - advanced); advanced = index; }
    }
    await control.charge(bytes);
    return text;
  });
}
/** 🛫️ Admits the owned wire spelling with an explicit caller control. */
export async function encodeValueRefusalKindControlled(value: ValueRefusalKind, control: ValueRefusalCodecControl): Promise<IntrinsicValue> {
  await control.checkpoint();
  await control.charge(64);
  return { kind: "text", value: await copyText(kind(value), control) };
}
/** 🛬️ Validates one borrowed kind without a prose classifier or implicit control. */
export async function decodeValueRefusalKindControlled(value: IntrinsicValue, control: ValueRefusalCodecControl): Promise<ValueRefusalKind> {
  return control.scopedStage(async () => {
    await control.beginStage(1);
    if (value.kind !== "text") throw new ValueError("invalidValue", "expected refusal kind text");
    const result = kind(value.value); await control.step(); return result;
  });
}
/** 🛫️ Constructs only the three declared members under cumulative owned admission. */
export async function encodeValueErrorControlled(error: ValueError, control: ValueRefusalCodecControl): Promise<IntrinsicValue> {
  return control.scopedStage(async () => {
    await control.beginStage(3);
    await control.charge(64 + 3 * 64);
    const kindValue = await encodeValueRefusalKindControlled(error.kind, control);
    const kindName = await copyText("kind", control); await control.step();
    const message = await copyText(error.message, control); await control.charge(64);
    const messageName = await copyText("message", control); await control.step();
    const receipt = await encodeRetainedProgress(error.retainedProgress, control);
    const receiptName = await copyText("retainedProgress", control); await control.step();
    return { kind: "object", members: [{ name: kindName, value: kindValue }, { name: messageName, value: { kind: "text", value: message } }, { name: receiptName, value: receipt }] };
  });
}
/** 🛬️ Rejects missing, duplicate and unknown members before constructing the owned cause. */
export async function decodeValueErrorControlled(value: IntrinsicValue, control: ValueRefusalCodecControl): Promise<ValueError> {
  return control.scopedStage(async () => {
    await control.beginStage(3);
    if (value.kind !== "object") throw new ValueError("invalidValue", "expected refusal record");
    if (value.members.length !== 3) throw new ValueError("invalidValue", "expected exactly kind, message and retainedProgress");
    let kindValue: IntrinsicValue | undefined, messageValue: IntrinsicValue | undefined, retainedValue: IntrinsicValue | undefined;
    for (const member of value.members) {
      if (member.name === "kind" && kindValue === undefined) kindValue = member.value;
      else if (member.name === "message" && messageValue === undefined) messageValue = member.value;
      else if (member.name === "retainedProgress" && retainedValue === undefined) retainedValue = member.value;
      else throw new ValueError("invalidValue", "unknown or duplicate refusal member");
      await control.step();
    }
    if (kindValue === undefined) throw new ValueError("invalidValue", "missing refusal kind");
    const valueKind = await decodeValueRefusalKindControlled(kindValue, control);
    if (messageValue?.kind !== "text") throw new ValueError("invalidValue", "expected refusal message text");
    await control.charge(64);
    if (retainedValue === undefined) throw new ValueError("invalidValue", "missing refusal retained receipt");
    const receipt = await decodeRetainedProgress(retainedValue, control);
    return new ValueError(valueKind, await copyText(messageValue.value, control)).withRetainedProgress(receipt);
  });
}

const retainedAxes = ["copiedItems", "copiedBytes", "retainedCapacityBytes", "releasedBytes"] as const;
async function encodeRetainedProgress(receipt: ValueErrorRetainedProgress, control: ValueRefusalCodecControl): Promise<IntrinsicValue> {
  return control.scopedStage(async () => {
    await control.beginStage(4); await control.charge(64 + 4 * 64);
    const members: { name: string; value: IntrinsicValue }[] = [];
    for (const axis of retainedAxes) {
      const value = receipt[axis];
      if (!Number.isSafeInteger(value) || value < 0) throw new ValueError("invalidValue", "refusal retained receipt requires exact unsigned axes");
      const name = await copyText(axis, control); await control.charge(64);
      members.push({ name, value: { kind: "unsigned", value: BigInt(value) } }); await control.step();
    }
    return { kind: "object", members };
  });
}
async function decodeRetainedProgress(value: IntrinsicValue, control: ValueRefusalCodecControl): Promise<ValueErrorRetainedProgress> {
  return control.scopedStage(async () => {
    await control.beginStage(4);
    if (value.kind !== "object" || value.members.length !== 4) throw new ValueError("invalidValue", "refusal retained receipt requires four axes");
    const receipt: Partial<Record<typeof retainedAxes[number], number>> = {};
    for (const member of value.members) {
      const axis = retainedAxes.find(axis => axis === member.name);
      if (axis === undefined || receipt[axis] !== undefined || member.value.kind !== "unsigned" || member.value.value < 0n || member.value.value > BigInt(Number.MAX_SAFE_INTEGER)) throw new ValueError("invalidValue", "refusal retained receipt has an unknown, repeated or inexact axis");
      receipt[axis] = Number(member.value.value); await control.step();
    }
    if (retainedAxes.some(axis => receipt[axis] === undefined)) throw new ValueError("invalidValue", "refusal retained receipt is missing an axis");
    return receipt as ValueErrorRetainedProgress;
  });
}
