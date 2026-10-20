/** 🔸️ The actor arena handback fixture phase of registerTests1. */
import type { ActorInstanceLifecycleReceipt, ActorInstanceLifecycleWire, ActorInstanceOpenRequest } from "../../../🟦️.ts";
import type { registerActorInstanceCloseFaultTests } from "../🔶️actor-instance-close-fault/🟦️.ts";
import type { TestSource } from "../🟢️shared/🟦️.ts";

/** 🔹 Registers the actor arena handback fixture tests of registerTests1. */
export function registerActorArenaHandbackFixtureTests({ it, source, expect, encodeActorInstanceLifecycle, ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES, decodeActorInstanceLifecycle }: Pick<ReturnType<typeof registerActorInstanceCloseFaultTests>, "it" | "expect" | "encodeActorInstanceLifecycle" | "ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES" | "decodeActorInstanceLifecycle"> & { readonly source: TestSource }) {
  it("actor arena handback fixture preserves exact fair obligations across word boundaries", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/📮️handback/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const counter = fixture.aliasCounter;
    const independent = Buffer.alloc(8);
    independent.writeBigUInt64LE(BigInt(counter.before));
    let carry = 1;
    for (let index = 0; index < independent.length; index++) {
      const next = independent[index]! + carry;
      independent[index] = next & 255;
      carry = next >>> 8;
    }
    expect(independent.readBigUInt64LE().toString()).toBe(counter.afterReturn);
    expect((BigInt(counter.afterReturn) - 1n).toString()).toBe(counter.afterConsume);
    expect((2n ** 64n - 1n).toString()).toBe(counter.maximum);
    const counts = new Map<number, number>();
    for (const owner of fixture.obligations) counts.set(owner.slot, (counts.get(owner.slot) ?? 0) + 1);
    const moduleName = "lodash-es/groupBy.js";
    const module: unknown = await import(moduleName);
    const groupBy: unknown = module && typeof module === "object" ? Reflect.get(module, "default") : null;
    if (typeof groupBy !== "function") throw new Error("invalid independent handback grouping oracle");
    const groups: unknown = groupBy(fixture.obligations, "slot");
    if (!groups || typeof groups !== "object") throw new Error("invalid independent handback groups");
    for (const [slot, count] of counts) {
      const group: unknown = Reflect.get(groups, String(slot));
      if (!Array.isArray(group)) throw new Error("invalid independent handback group");
      expect(group.length).toBe(count);
    }
    const order: number[] = [];
    let cursor = fixture.start;
    while (counts.size) {
      const slots = [...counts.keys()].sort((a, b) => a - b);
      const slot = slots.find((slot) => slot >= cursor) ?? slots[0];
      if (slot === undefined) throw new Error("nonempty handback obligations require an occupied slot");
      const count = counts.get(slot)!;
      if (count === 1) counts.delete(slot);
      else counts.set(slot, count - 1);
      order.push(slot);
      cursor = (slot + 1) % fixture.slots;
    }
    expect(order).toEqual(fixture.expectedOrder);
  });

  it("actor patch storage separates physical placement from semantic retirement grants", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("../../🖱️ui/🧬️contract/♻️retirement/🩹️patch/🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const chunkModule = "lodash-es/chunk.js";
    const module: unknown = await import(chunkModule);
    const chunk: unknown = module && typeof module === "object" ? Reflect.get(module, "default") : null;
    if (typeof chunk !== "function") throw new Error("invalid independent patch paging oracle");
    const pages: unknown = chunk(fixture.operations, 1);
    if (!Array.isArray(pages) || !pages.every(Array.isArray)) throw new Error("invalid independent patch pages");
    expect(pages.map((page) => page.length)).toEqual(fixture.pageLengths);
    expect(
      pages
        .slice()
        .reverse()
        .map((page) => page[0].id),
    ).toEqual(fixture.retirementOrder);
    const native = fixture.native64;
    expect(fixture.logicalCapacity * native.descriptorBytes).toBe(native.directoryBytes);
    expect(native.directoryBytes + native.operationBytes).toBe(native.firstBackingBytes);
    expect(fixture.placementGrants.map((grant: number) => (grant >= native.operationBytes ? native.operationBytes : 0))).toEqual(fixture.placedBytes);
    expect(native.directoryBytes).toBeLessThanOrEqual(fixture.physicalGrant);
    expect(native.firstPayloadBytes).toBeLessThanOrEqual(fixture.physicalGrant);
    expect(native.operationBytes).toBeGreaterThan(Math.max(...fixture.semanticGrants));
    expect(Buffer.byteLength("é".repeat(256))).toBe(fixture.unplaced.semanticBytes);
    expect(new TextEncoder().encode("é".repeat(256)).length).toBe(fixture.unplaced.semanticBytes);
  });

  it("actor instance close wire matches strict shared fixtures and an independent LEB128 encoder", async () => {
    const { readFileSync } = await import("node:fs");
    const { default: Ajv } = await import("ajv");
    const fixture = JSON.parse(readFileSync(new URL("./🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", source.url), "utf8"));
    const ajv = new Ajv({ strict: true });
    ajv.addSchema(JSON.parse(readFileSync(new URL("../../🌱️value/🧬️schema/🔣️.json", source.url), "utf8")));
    ajv.addSchema(schema);
    const validate = ajv.getSchema(`${schema.$id}#/$defs/Lifetime`)!;
    for (const invalid of ["0", "-1", "01", "18446744073709551616"]) {
      expect(validate({ ...fixture.vectors[0].value, activationGeneration: invalid })).toBe(false);
      expect(validate({ ...fixture.vectors[1].value, lifetime: { ...fixture.vectors[1].value.lifetime, guestLifetime: invalid } })).toBe(false);
    }
    expect(validate({ ...fixture.vectors[0].value, unexpected: true })).toBe(false);
    const dropModule = "lodash-es/drop.js";
    const importedDrop: unknown = await import(dropModule);
    if (!importedDrop || typeof importedDrop !== "object") throw new Error("invalid ordered owner oracle module");
    const drop: unknown = Reflect.get(importedDrop, "default");
    if (typeof drop !== "function") throw new Error("invalid ordered owner oracle interface");
    expect(fixture.publishedClose.complete.map((_value: boolean, index: number) => drop(fixture.publishedClose.owners, index + 1))).toEqual(fixture.publishedClose.remaining);
    expect(fixture.publishedClose.remaining.map((_value: unknown, index: number) => index >= fixture.publishedClose.owners.length)).toEqual(fixture.publishedClose.complete);
    const oracleModule = "@webassemblyjs/leb128/lib/leb.js";
    const imported: unknown = await import(oracleModule);
    if (!imported || typeof imported !== "object") throw new Error("invalid LEB128 oracle module");
    const oracle: unknown = Reflect.get(imported, "default");
    if (!oracle || typeof oracle !== "object") throw new Error("invalid LEB128 oracle interface");
    const encoder: unknown = Reflect.get(oracle, "encodeUIntBuffer");
    if (typeof encoder !== "function") throw new Error("missing LEB128 oracle encoder");
    const u64 = (value: bigint): number[] => {
      const input = Buffer.alloc(8);
      input.writeBigUInt64LE(value);
      const output: unknown = encoder(input);
      if (!(output instanceof Uint8Array)) throw new Error("invalid LEB128 oracle bytes");
      return Array.from(output);
    };
    for (const row of fixture.vectors) {
      const value: ActorInstanceLifecycleWire = JSON.parse(JSON.stringify(row.value), (key, field) => (["activationGeneration", "guestLifetime", "closeGeneration"].includes(key) ? BigInt(field) : field));
      const bytes = encodeActorInstanceLifecycle(value);
      const body = value.kind === "ack" ? value.receipt : value;
      const tag = value.kind === "ack" ? { captured: 5, accepted: 6, retired: 7 }[value.receipt.kind] : { open: 0, captured: 1, close: 2, accepted: 3, retired: 4 }[value.kind];
      const expected =
        body.kind === "open"
          ? [tag, ...u64(body.activationGeneration), ...u64(BigInt(body.instanceId)), ...u64(BigInt(body.requestSequence))]
          : [tag, ...u64(body.lifetime.activationGeneration), ...u64(BigInt(body.lifetime.instanceId)), ...u64(body.lifetime.guestLifetime), ...u64(BigInt(body.requestSequence)), ...("closeGeneration" in body ? u64(body.closeGeneration) : [])];
      expect(Array.from(bytes)).toEqual(expected);
      expect(Buffer.from(bytes).toString("hex")).toBe(row.hex);
      expect(bytes.length).toBeLessThanOrEqual(ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES);
      expect(decodeActorInstanceLifecycle(bytes)).toEqual(value);
      expect(() => decodeActorInstanceLifecycle(Uint8Array.from([...bytes, 0]))).toThrow();
      for (let length = 0; length < bytes.length; length += 1) expect(() => decodeActorInstanceLifecycle(bytes.subarray(0, length))).toThrow();
    }
    for (const bytes of [
      [8, 1, 7, 9],
      [0, 0, 7, 9],
      [0, 0x81, 0, 7, 9],
      [0, ...Array(10).fill(255), 7, 9],
      [0, 1, 7, 0],
      [1, 1, 7, 0, 9],
      [3, 1, 7, 13, 9, 0],
      [0, 1, 7, ...u64(9007199254740992n)],
    ])
      expect(() => decodeActorInstanceLifecycle(Uint8Array.from(bytes))).toThrow();
    for (const activationGeneration of [0n, -1n, 18446744073709551616n, 1, "1"]) expect(() => encodeActorInstanceLifecycle({ kind: "open", activationGeneration, instanceId: 7, requestSequence: 8 } as ActorInstanceOpenRequest)).toThrow();
    for (const guestLifetime of [0n, -1n, 18446744073709551616n, 13, "13"])
      expect(() => encodeActorInstanceLifecycle({ kind: "captured", lifetime: { activationGeneration: 1n, instanceId: 7, guestLifetime }, requestSequence: 8 } as ActorInstanceLifecycleReceipt)).toThrow();
    expect(() => encodeActorInstanceLifecycle({ kind: "ack", receipt: { kind: "open", activationGeneration: 1n, instanceId: 7, requestSequence: 8 } } as unknown as ActorInstanceLifecycleWire)).toThrow();
  });
}
