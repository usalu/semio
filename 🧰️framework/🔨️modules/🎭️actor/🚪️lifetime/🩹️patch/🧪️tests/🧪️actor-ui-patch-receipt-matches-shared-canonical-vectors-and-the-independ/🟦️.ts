type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES, actorUiPatchReceiptEquals, decodeActorUiPatchReceipt, encodeActorUiPatchReceipt, validateActorUiPatchPairing } = dependencies;

  const { it, expect } = vitest;
  it("actor UI patch receipt matches shared canonical vectors and the independent LEB128 encoder", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixture/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
    const lifetimeSchema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", source.url), "utf8"));
    const valueSchema = JSON.parse(readFileSync(new URL("../../../🌱️value/🧬️schema/🔣️.json", source.url), "utf8"));
    const validate = new Ajv({ strict: true }).addSchema(valueSchema).addSchema(lifetimeSchema).addSchema(schema).getSchema(`${schema.$id}#/$defs/PatchFixture`)!;
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
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixture/🔣️.json", source.url), "utf8"));
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
