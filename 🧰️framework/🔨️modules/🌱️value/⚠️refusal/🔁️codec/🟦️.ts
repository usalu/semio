import { ValueError, type ValueRefusalKind } from "../🟦️.ts";
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
  switch (text) {
    case "invalidValue": case "canceled": case "ownershipLimit": case "allocationFailed": case "workLimit": case "depthLimit": case "unsupportedOwner": case "invariantViolated": return text;
    default: throw new ValueError("invalidValue", "unknown refusal kind");
  }
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
/** 🛫️ Constructs only the two declared members under cumulative owned admission. */
export async function encodeValueErrorControlled(error: ValueError, control: ValueRefusalCodecControl): Promise<IntrinsicValue> {
  return control.scopedStage(async () => {
    await control.beginStage(2);
    await control.charge(64 + 2 * 64);
    const kindValue = await encodeValueRefusalKindControlled(error.kind, control);
    const kindName = await copyText("kind", control); await control.step();
    const message = await copyText(error.message, control); await control.charge(64);
    const messageName = await copyText("message", control); await control.step();
    return { kind: "object", members: [{ name: kindName, value: kindValue }, { name: messageName, value: { kind: "text", value: message } }] };
  });
}
/** 🛬️ Rejects missing, duplicate and unknown members before constructing the owned cause. */
export async function decodeValueErrorControlled(value: IntrinsicValue, control: ValueRefusalCodecControl): Promise<ValueError> {
  return control.scopedStage(async () => {
    await control.beginStage(2);
    if (value.kind !== "object") throw new ValueError("invalidValue", "expected refusal record");
    if (value.members.length !== 2) throw new ValueError("invalidValue", "expected exactly kind and message");
    let kindValue: IntrinsicValue | undefined, messageValue: IntrinsicValue | undefined;
    for (const member of value.members) {
      if (member.name === "kind" && kindValue === undefined) kindValue = member.value;
      else if (member.name === "message" && messageValue === undefined) messageValue = member.value;
      else throw new ValueError("invalidValue", "unknown or duplicate refusal member");
      await control.step();
    }
    if (kindValue === undefined) throw new ValueError("invalidValue", "missing refusal kind");
    const valueKind = await decodeValueRefusalKindControlled(kindValue, control);
    if (messageValue?.kind !== "text") throw new ValueError("invalidValue", "expected refusal message text");
    await control.charge(64);
    return new ValueError(valueKind, await copyText(messageValue.value, control));
  });
}
