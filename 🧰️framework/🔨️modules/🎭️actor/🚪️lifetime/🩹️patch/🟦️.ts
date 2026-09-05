//#region 🧬️IssuedPatchReceipt
import type { ActorInstanceLifetime } from "../🟦️.ts";

export type ActorUiPatchReceipt = { readonly lifetime: ActorInstanceLifetime; readonly patchSequence: bigint };
export const ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES = 35;

/** 📤️ Encodes the exact guest-issued patch identity without deriving authority from surface revisions. */
export function encodeActorUiPatchReceipt(receipt: ActorUiPatchReceipt): Uint8Array {
  const lifetime = receipt.lifetime;
  const valid = (value: bigint): boolean => typeof value === "bigint" && value > 0n && value <= 0xffffffffffffffffn;
  if (!valid(lifetime.activationGeneration) || !valid(lifetime.guestLifetime) || !valid(receipt.patchSequence) || !Number.isInteger(lifetime.instanceId) || lifetime.instanceId < 0 || lifetime.instanceId > 0xffffffff) throw new Error("actor-ui-patch.invalid-authority");
  const output = new Uint8Array(ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES);
  let length = 0;
  const put = (initial: bigint) => {
    let rest = initial;
    do {
      const byte = Number(rest & 127n);
      rest >>= 7n;
      output[length++] = byte | (rest === 0n ? 0 : 128);
    } while (rest !== 0n);
  };
  put(lifetime.activationGeneration); put(BigInt(lifetime.instanceId)); put(lifetime.guestLifetime); put(receipt.patchSequence);
  return output.slice(0, length);
}

/** 📥️ Rejects noncanonical, truncated, overflowed, zero and trailing receipt authority. */
export function decodeActorUiPatchReceipt(bytes: Uint8Array): ActorUiPatchReceipt {
  if (!(bytes instanceof Uint8Array) || bytes.length < 4 || bytes.length > ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES) throw new Error("actor-ui-patch.envelope");
  let offset = 0;
  const get = (maximum: bigint, nonzero: boolean) => {
    let value = 0n;
    for (let index = 0; index < 10; index += 1) {
      const byte = bytes[offset++];
      if (byte === undefined) throw new Error("actor-ui-patch.truncated");
      value |= BigInt(byte & 127) << BigInt(index * 7);
      if ((byte & 128) === 0) {
        if ((index !== 0 && byte === 0) || value > maximum || nonzero && value === 0n) throw new Error("actor-ui-patch.noncanonical-authority");
        return value;
      }
    }
    throw new Error("actor-ui-patch.overlong");
  };
  const activationGeneration = get(0xffffffffffffffffn, true);
  const instanceId = Number(get(0xffffffffn, false));
  const guestLifetime = get(0xffffffffffffffffn, true);
  const patchSequence = get(0xffffffffffffffffn, true);
  if (offset !== bytes.length) throw new Error("actor-ui-patch.trailing");
  return { lifetime: { activationGeneration, instanceId, guestLifetime }, patchSequence };
}

/** 🪪️ Compares wire identity only; equality does not mint a native retirement capability. */
export function actorUiPatchReceiptEquals(left: ActorUiPatchReceipt, right: ActorUiPatchReceipt): boolean {
  return left.lifetime.activationGeneration === right.lifetime.activationGeneration && left.lifetime.instanceId === right.lifetime.instanceId && left.lifetime.guestLifetime === right.lifetime.guestLifetime && left.patchSequence === right.patchSequence;
}

/** 🩹️ Enforces the canonical single-patch turn cardinality before receipt consumption. */
export function validateActorUiPatchPairing(patchCount: number, receipt: ActorUiPatchReceipt | null | undefined): void {
  if (patchCount !== 0 && patchCount !== 1 || (patchCount === 1) !== (receipt != null)) throw new Error("actor-ui-patch.pairing");
  if (receipt != null) encodeActorUiPatchReceipt(receipt);
}
//#endregion 🧬️IssuedPatchReceipt

//#region 🧪️IssuedPatchReceiptLaws
if (import.meta.vitest) {
  const { it, expect } = import.meta.vitest;
  it("actor UI patch receipt matches shared canonical vectors and the independent LEB128 encoder", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixture/🔣️.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema.json", import.meta.url), "utf8"));
    const fixtureSchema = JSON.parse(readFileSync(new URL("./📐️schema/🔣️.json", import.meta.url), "utf8"));
    const lifetimeSchema = JSON.parse(readFileSync(new URL("../🧬️schema.json", import.meta.url), "utf8"));
    const validate = new Ajv({ strict: true }).addSchema(lifetimeSchema).addSchema(schema).compile(fixtureSchema);
    expect(validate(fixture)).toBe(true);
    expect(validate({ ...fixture, maximumBytes: 36 })).toBe(false);
    const moduleName = "@webassemblyjs/leb128/lib/leb.js";
    const module: unknown = await import(moduleName);
    const oracle: unknown = module && typeof module === "object" ? Reflect.get(module, "default") ?? module : null;
    const encode: unknown = oracle && typeof oracle === "object" ? Reflect.get(oracle, "encodeUIntBuffer") : null;
    if (typeof encode !== "function") throw new Error("invalid independent LEB128 encoder");
    expect(ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES).toBe(fixture.maximumBytes);
    for (const row of fixture.vectors) {
      const receipt = {
        lifetime: { activationGeneration: BigInt(row.value.lifetime.activationGeneration), instanceId: row.value.lifetime.instanceId, guestLifetime: BigInt(row.value.lifetime.guestLifetime) },
        patchSequence: BigInt(row.value.patchSequence),
      };
      const bytes = encodeActorUiPatchReceipt(receipt);
      expect(Buffer.from(bytes).toString("hex")).toBe(row.hex);
      const independent = [receipt.lifetime.activationGeneration, BigInt(receipt.lifetime.instanceId), receipt.lifetime.guestLifetime, receipt.patchSequence].map((value) => {
        const input = Buffer.alloc(8);
        input.writeBigUInt64LE(value);
        return Buffer.from(encode(input));
      });
      expect(Buffer.concat(independent).toString("hex")).toBe(row.hex);
      expect(decodeActorUiPatchReceipt(bytes)).toEqual(receipt);
      for (let prefix = 0; prefix < bytes.length; prefix += 1) expect(() => decodeActorUiPatchReceipt(bytes.subarray(0, prefix))).toThrow();
    }
    for (const hex of fixture.invalidHex) expect(() => decodeActorUiPatchReceipt(Buffer.from(hex, "hex"))).toThrow();
    expect(() => decodeActorUiPatchReceipt(new Uint8Array(36))).toThrow();
  });

  it("actor UI patch receipt rejects invalid authority and enforces exact zero or one patch pairing", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixture/🔣️.json", import.meta.url), "utf8"));
    const receipt = { lifetime: { activationGeneration: 41n, instanceId: 7, guestLifetime: 13n }, patchSequence: 51n };
    const invalidCounters: unknown[] = [0n, -1n, 0x10000000000000000n, 1, "1", null, undefined];
    for (const value of invalidCounters) {
      expect(() => encodeActorUiPatchReceipt({ ...receipt, patchSequence: value as bigint })).toThrow();
      expect(() => encodeActorUiPatchReceipt({ ...receipt, lifetime: { ...receipt.lifetime, activationGeneration: value as bigint } })).toThrow();
      expect(() => encodeActorUiPatchReceipt({ ...receipt, lifetime: { ...receipt.lifetime, guestLifetime: value as bigint } })).toThrow();
    }
    for (const instanceId of [-1, 0x100000000, 1.5, NaN]) expect(() => encodeActorUiPatchReceipt({ ...receipt, lifetime: { ...receipt.lifetime, instanceId } })).toThrow();
    expect(actorUiPatchReceiptEquals(receipt, decodeActorUiPatchReceipt(encodeActorUiPatchReceipt(receipt)))).toBe(true);
    expect(actorUiPatchReceiptEquals(receipt, { ...receipt, lifetime: { ...receipt.lifetime, guestLifetime: 14n } })).toBe(fixture.feedback.oldGuestAccepted);
    expect(actorUiPatchReceiptEquals(receipt, { ...receipt, patchSequence: 52n })).toBe(fixture.feedback.oldSequenceAccepted);
    for (const row of fixture.pairing) {
      const validate = () => validateActorUiPatchPairing(row.patchCount, row.hasReceipt ? receipt : null);
      if (row.accepted) expect(validate).not.toThrow();
      else expect(validate).toThrow();
    }
    for (const count of [-1, 1.5, NaN, Infinity]) expect(() => validateActorUiPatchPairing(count, receipt)).toThrow();
    expect(() => validateActorUiPatchPairing(1, { ...receipt, patchSequence: 0n })).toThrow();
  });
}
//#endregion 🧪️IssuedPatchReceiptLaws
