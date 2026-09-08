import Ajv from "ajv";
import deepEqual from "fast-deep-equal";
import { describe, expect, it } from "vitest";
import schema from "../../../🧬️schema/🔣️.json";
import fixture from "../../🧱️elements/🔌️PluginRuntime/📡️backbone/🧫️fixtures/🔣️.json";
import { ActorDocumentBindingV1, ActorDocumentMessagePortV1, decodeDocumentBackboneControlV1, encodeDocumentBackboneControlV1, requireDocumentBackboneReceiptV1 } from "../../🧱️elements/🔌️PluginRuntime/📡️backbone/🟦️.ts";
import { decodeBackboneMessage, encodeBackboneMessage, encodePackValue, packUInt, type BinaryBackboneMessage } from "@semio-tech/framework-os";
import bindingSchema from "../../../../🔌️plugin/📡️backbone/🔗️binding/🧬️schema/🔣️.json";
import bindingFixture from "../../../../🔌️plugin/📡️backbone/🔗️binding/🧪️fixture/🔣️.json";

const owner = { ...fixture.owner, activationGeneration: BigInt(fixture.owner.activationGeneration) };
const source = { runtimeKey: owner.runtimeKey, clientInstanceId: owner.clientInstanceId, scope: owner.scope };
function comparable(value: unknown): unknown {
  if (value instanceof Uint8Array) return Array.from(value);
  if (Array.isArray(value)) return value.map(comparable);
  if (value !== null && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, comparable(item)]));
  return value;
}
function deferred() {
  let resolve!: () => void;
  const promise = new Promise<void>(done => { resolve = done; });
  return { promise, resolve };
}

describe("actor-owned document backbone", () => {
  it("retains uncertain bind control authority but locally retires a verified refusal", async () => {
    for (const row of fixture.bindingRetirement) {
      const events: string[] = [];
      let current = true;
      const binding = new ActorDocumentBindingV1(owner, 3n, {
        current: () => current, assertActive: () => { if (!current) throw new Error("presentation retired"); },
        limits: fixture.limits, send: () => {}, deliver: async () => {},
        exchange: async command => {
          events.push(command.operation);
          const refused = command.operation === "bind" && row.result === "refused";
          const receipt = encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: refused ? "refused" : command.operation === "bind" ? "bound" : "retired", ...(refused ? { code: "binding-refused" } : {}) });
          if (command.operation === "bind") {
            if (row.result === "transport") throw new Error("transport lost");
            if (row.result === "duplicate") return [receipt, receipt];
            if (row.result === "replaced") current = false;
          }
          return [receipt];
        },
      });
      await expect(binding.bind()).rejects.toThrow();
      await binding.port.retire();
      expect(deepEqual(events, row.sequence), row.result).toBe(true);
      expect(binding.port.send(binding.port.uri, Uint8Array.of(1))).toBe(false);
    }
    console.log("[DEBUG] Binding retirement: refused-local=1 uncertain-retired=2 replaced-retired=1");
  });

  it("publishes only after Bound and waits for exact Retired before releasing the binding", async () => {
    const release = deferred(), events: string[] = [];
    const binding = new ActorDocumentBindingV1(owner, 1n, {
      current: () => true, assertActive: () => {}, limits: fixture.limits,
      send: () => { events.push("send"); }, deliver: async () => { events.push("deliver"); },
      exchange: async command => {
        events.push(command.operation);
        if (command.operation === "bind") await release.promise;
        return [encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: command.operation === "bind" ? "bound" : "retired" })];
      },
    });
    const ready = binding.bind();
    expect(binding.bind()).toBe(ready);
    expect(binding.port.send(binding.port.uri, Uint8Array.of(1))).toBe(false);
    const inbound = binding.port.receive(source, Uint8Array.of(2));
    expect(binding.port.pending).toEqual({ bytes: 1, messages: 1 });
    release.resolve();
    await ready;
    expect(await inbound).toBe(true);
    expect(binding.port.send(binding.port.uri, Uint8Array.of(1))).toBe(true);
    await binding.port.retire();
    expect(binding.port.send(binding.port.uri, Uint8Array.of(1))).toBe(false);
    expect(deepEqual(events, ["bind", "deliver", "send", "retire"])).toBe(true);
  });

  it("retires an in-flight bind without opening admission or accepting duplicate receipts", async () => {
    for (const duplicates of [false, true]) {
      const release = deferred(), events: string[] = [];
      const binding = new ActorDocumentBindingV1(owner, 2n, {
        current: () => true, assertActive: () => {}, limits: fixture.limits,
        send: () => { events.push("send"); }, deliver: async () => {},
        exchange: async command => {
          events.push(command.operation);
          if (command.operation === "bind") await release.promise;
          const receipt = encodeDocumentBackboneControlV1({ ...command, schema: "semio.plugin.document-backbone-binding-receipt.v1", operation: command.operation === "bind" ? "bound" : "retired" });
          return duplicates && command.operation === "bind" ? [receipt, receipt] : [receipt];
        },
      });
      const ready = binding.bind();
      const observed = duplicates ? expect(ready).rejects.toThrow("actor-document-control.receipt-count") : ready;
      const closed = binding.port.retire();
      release.resolve();
      await observed;
      await closed;
      expect(binding.port.send(binding.port.uri, Uint8Array.of(1))).toBe(false);
      expect(deepEqual(events, ["bind", "retire"])).toBe(true);
    }
  });

  it("accepts only exact bounded binding receipts with lossless UInt ownership", () => {
    const wireValue = (value: Record<string, unknown>) => ({
      ...value,
      ...(typeof value.instanceId === "number" && Number.isInteger(value.instanceId) && value.instanceId >= 0 ? { instanceId: packUInt(BigInt(value.instanceId)) } : {}),
      ...(typeof value.bindingGeneration === "string" ? { bindingGeneration: packUInt(BigInt(value.bindingGeneration)) } : {}),
    });
    for (const row of bindingFixture.cases) {
      const command = { ...row.command, bindingGeneration: BigInt(row.command.bindingGeneration) };
      const receipt = { ...row.receipt, bindingGeneration: BigInt(row.receipt.bindingGeneration) };
      const commandBytes = encodePackValue(wireValue(row.command));
      const receiptBytes = encodePackValue(wireValue(row.receipt));
      expect(deepEqual(comparable(encodeDocumentBackboneControlV1(command)), comparable(commandBytes)), row.id).toBe(true);
      expect(deepEqual(comparable(decodeDocumentBackboneControlV1(commandBytes)), comparable(command)), row.id).toBe(true);
      expect(deepEqual(comparable(decodeDocumentBackboneControlV1(receiptBytes)), comparable(receipt)), row.id).toBe(true);
      if (receipt.operation === "refused") expect(() => requireDocumentBackboneReceiptV1(receiptBytes, command), row.id).toThrow(receipt.code);
      else expect(() => requireDocumentBackboneReceiptV1(receiptBytes, command), row.id).not.toThrow();
      expect(() => requireDocumentBackboneReceiptV1(receiptBytes, { ...command, bindingGeneration: command.bindingGeneration + 1n })).toThrow();
      expect(() => decodeDocumentBackboneControlV1(Uint8Array.from([...receiptBytes, 0]))).toThrow();
    }
    for (const row of bindingFixture.hostile) expect(() => decodeDocumentBackboneControlV1(encodePackValue(wireValue(row.value))), row.id).toThrow();
    const maximum = { ...bindingFixture.cases[0]!.command, bindingGeneration: 0xffffffffffffffffn };
    expect(decodeDocumentBackboneControlV1(encodeDocumentBackboneControlV1(maximum)).bindingGeneration).toBe(maximum.bindingGeneration);
    expect(() => encodeDocumentBackboneControlV1({ ...maximum, bindingGeneration: maximum.bindingGeneration + 1n })).toThrow();
    expect(() => encodeDocumentBackboneControlV1({ ...maximum, uri: "界".repeat(427) })).toThrow();
    expect(() => decodeDocumentBackboneControlV1(new Uint8Array(bindingFixture.limits.controlBytes + 1))).toThrow();
    console.log("[DEBUG] Actor document control: exact bounded bind/retire receipts and UInt authority PASS");
  });

  it("matches the canonical native OpBinary vectors and rejects hostile records", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(bindingSchema);
    expect(validate(bindingFixture), JSON.stringify(validate.errors)).toBe(true);
    const fromHex = (value: string) => Uint8Array.from(value.match(/../g) ?? [], byte => Number.parseInt(byte, 16));
    for (const row of bindingFixture.codec.golden) {
      const value = row.value;
      const message: BinaryBackboneMessage = value.Snapshot
        ? { kind: "snapshot", pack: Uint8Array.from(value.Snapshot.pack), spr: Uint8Array.from(value.Snapshot.spr) }
        : value.Mutations
          ? { kind: "mutations", envelopes: Uint8Array.from(value.Mutations.envelopes) }
          : { kind: "ack", opIds: value.Ack!.opIds };
      const encoded = Array.from(encodeBackboneMessage(message));
      const expected = Array.from(fromHex(row.hex));
      expect(encoded, row.id).toEqual(expected);
      expect(deepEqual(encoded, expected), row.id).toBe(true);
      expect(deepEqual(comparable(decodeBackboneMessage(fromHex(row.hex))), comparable(message)), row.id).toBe(true);
    }
    for (const row of bindingFixture.codec.hostile) expect(() => decodeBackboneMessage(fromHex(row.hex)), row.id).toThrow();
    expect(() => encodeBackboneMessage({ kind: "mutations", envelopes: new Uint8Array(bindingFixture.dataLimits.hotMessageBytes) })).toThrow();
    expect(() => decodeBackboneMessage(new Uint8Array(4 * 1024 * 1024 + 1))).toThrow();
    for (const hex of ["01020001000cffffffffffffffffff01", "0102ffffffffffffffffff01", "01020001000c010701ff", "010201016101000c010601"]) {
      expect(() => decodeBackboneMessage(fromHex(hex))).toThrow();
    }
    console.log("[DEBUG] Backbone OpBinary: native golden bytes and hostile bounded record vectors PASS");
  });

  it("validates neutral ownership rows and admits only exact current sources", async () => {
    const validate = new Ajv({ strict: true, allErrors: true }).addSchema(schema).compile({ $ref: `${schema.$id}#/$defs/ActorDocumentPortFixtureV1` });
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    for (const row of fixture.cases) {
      const sent: number[][] = [], received: number[][] = [];
      const port = new ActorDocumentMessagePortV1(owner, {
        current: () => row.current,
        assertActive: () => { if (!row.activationActive) throw new Error("activation revoked"); },
        send: bytes => { sent.push([...bytes]); },
        deliver: async bytes => { received.push([...bytes]); },
        retire: async () => {},
        limits: fixture.limits,
      });
      if (row.retired) await port.retire();
      expect(port.send(row.uri, Uint8Array.of(1)), row.id).toBe(row.outbound);
      expect(await port.receive({ runtimeKey: row.runtimeKey, clientInstanceId: row.clientInstanceId, scope: { ...owner.scope, spaceId: row.spaceId } }, Uint8Array.of(2)), row.id).toBe(row.inbound);
      expect(deepEqual(sent, row.outbound ? [[1]] : []), row.id).toBe(true);
      expect(deepEqual(received, row.inbound ? [[2]] : []), row.id).toBe(true);
      await port.retire();
    }
    console.log("[DEBUG] Actor document port: eight exact owner/source/activation retirement rows PASS");
  });

  it("retires admission immediately and drains the admitted turn before exact cleanup", async () => {
    const started = deferred(), release = deferred(), events: string[] = [];
    const port = new ActorDocumentMessagePortV1(owner, {
      current: () => true, assertActive: () => {}, send: () => { events.push("send"); },
      deliver: async bytes => { events.push(`deliver:${bytes[0]}`); started.resolve(); await release.promise; events.push(`finished:${bytes[0]}`); },
      retire: async () => { events.push("retire"); }, limits: fixture.limits,
    });
    const first = port.receive(source, Uint8Array.of(1));
    await started.promise;
    const second = port.receive(source, Uint8Array.of(2));
    const retired = port.retire();
    expect(port.retire()).toBe(retired);
    expect(port.send(`actor://${owner.runtimeKey}`, Uint8Array.of(3))).toBe(false);
    expect(events).toEqual(["deliver:1"]);
    release.resolve();
    expect(await first).toBe(false);
    expect(await second).toBe(false);
    await retired;
    expect(deepEqual(events, fixture.retirement)).toBe(true);
    expect(port.pending).toEqual({ bytes: 0, messages: 0 });
  });

  it("bounds retained bytes and messages, owns admitted bytes, and releases reservations after failure", async () => {
    const started = deferred(), release = deferred(), received: number[][] = [];
    let fail = true;
    const port = new ActorDocumentMessagePortV1(owner, {
      current: () => true, assertActive: () => {}, send: () => {},
      deliver: async bytes => { received.push([...bytes]); started.resolve(); await release.promise; if (fail) { fail = false; throw new Error("guest failed"); } },
      retire: async () => {}, limits: fixture.limits,
    });
    const bytes = Uint8Array.of(1, 2, 3);
    const first = port.receive(source, bytes);
    const failed = expect(first).rejects.toThrow("guest failed");
    bytes.fill(9);
    await started.promise;
    const second = port.receive(source, Uint8Array.of(4, 5, 6));
    await expect(port.receive(source, Uint8Array.of(7))).rejects.toThrow("actor-document-port.capacity");
    await expect(port.receive(source, new Uint8Array(5))).rejects.toThrow("actor-document-port.message-size");
    expect(() => port.send(`actor://${owner.runtimeKey}`, new Uint8Array(5))).toThrow("actor-document-port.message-size");
    expect(port.pending).toEqual({ bytes: 6, messages: 2 });
    release.resolve();
    await failed;
    expect(await second).toBe(true);
    expect(deepEqual(received, [[1, 2, 3], [4, 5, 6]])).toBe(true);
    await port.retire();
    expect(port.pending).toEqual({ bytes: 0, messages: 0 });
  });
});
