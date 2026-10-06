import assert from "node:assert/strict";
import Ajv from "ajv/dist/2020.js";
import { Database } from "bun:sqlite";
import { test } from "bun:test";
import { ValueError, type ValueRefusalKind } from "../../🟦️.ts";
import { NativeDecodeControl } from "../../../🛬️decode/🟦️.ts";
import type { IntrinsicValue } from "../../../🧬️schema/🌳️intrinsic/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

interface Control { checkpoint(): Promise<void>; charge(bytes: number): Promise<void>; step(): Promise<void>; advance(units: number): Promise<void>; beginStage(total: number): Promise<void>; scopedStage<T>(operation: () => Promise<T>): Promise<T> }
interface Codec {
  encodeValueErrorControlled(error: ValueError, control: Control): Promise<IntrinsicValue>;
  decodeValueErrorControlled(value: IntrinsicValue, control: Control): Promise<ValueError>;
  encodeValueRefusalKindControlled(kind: ValueRefusalKind, control: Control): Promise<IntrinsicValue>;
  decodeValueRefusalKindControlled(value: IntrinsicValue, control: Control): Promise<ValueRefusalKind>;
}
const leaf = (value: string | boolean | null): IntrinsicValue => typeof value === "string" ? { kind: "text", value } : typeof value === "boolean" ? { kind: "boolean", value } : { kind: "null" };
const wire = (kind: string, message: string): IntrinsicValue => ({ kind: "object", members: [{ name: "kind", value: leaf(kind) }, { name: "message", value: leaf(message) }] });
const control = (): NativeDecodeControl => new NativeDecodeControl(1_000_000, () => true);
const load = async (): Promise<Codec> => {
  const path = new URL("../🟦️.ts", import.meta.url);
  const loaded: Record<string, unknown> = await import(path.href);
  for (const name of ["encodeValueErrorControlled", "decodeValueErrorControlled", "encodeValueRefusalKindControlled", "decodeValueRefusalKindControlled"]) assert.equal(typeof loaded[name], "function", name);
  return loaded as unknown as Codec;
};
test("closed refusal wire schema and language-neutral cases match independent Ajv and SQLite JSON", () => {
  const ajv = new Ajv({ strict: true }).addSchema(schema);
  const valid = ajv.getSchema(`${schema.$id}#/$defs/wire`); assert(valid);
  const db = new Database(":memory:");
  try {
    for (const row of fixture.valid) {
      assert(valid(row.wire));
      const output = db.query("SELECT json_object('kind',?, 'message',?) AS wire").get(row.wire.kind, row.wire.message) as { wire: string };
      assert.deepEqual(JSON.parse(output.wire), row.wire);
    }
    for (const row of fixture.invalid) {
      const names = row.members.map(member => member.key);
      const duplicate = new Set(names).size !== names.length;
      const candidate = Object.fromEntries(row.members.map(member => [member.key, member.value]));
      assert(duplicate || !valid(candidate), row.id);
    }
  } finally { db.close(); }
});
test("actual controlled refusal codecs preserve all eight causes and reject closed-record violations", async () => {
  const codec = await load();
  for (const row of fixture.valid) {
    const source = wire(row.wire.kind, row.wire.message);
    const error = await codec.decodeValueErrorControlled(source, control());
    assert(error instanceof ValueError); assert.equal(error.kind, row.wire.kind); assert.equal(error.message, row.wire.message);
    assert.deepEqual(await codec.encodeValueErrorControlled(error, control()), source);
    const kind = await codec.encodeValueRefusalKindControlled(error.kind, control());
    assert.equal(await codec.decodeValueRefusalKindControlled(kind, control()), error.kind);
  }
  for (const row of fixture.invalid) {
    const value: IntrinsicValue = { kind: "object", members: row.members.map(member => ({ name: member.key, value: leaf(member.value) })) };
    const original = structuredClone(value);
    await assert.rejects(codec.decodeValueErrorControlled(value, control()), (error: unknown) => error instanceof ValueError && error.kind === row.expectedKind);
    assert.deepEqual(value, original);
  }
  for (const value of [{ kind: "null" }, { kind: "boolean", value: true }, { kind: "array", items: [] }] as const) await assert.rejects(codec.decodeValueErrorControlled(value as IntrinsicValue, control()), (error: unknown) => error instanceof ValueError && error.kind === "invalidValue");
  console.log("[DEBUG] portable controlled refusal transport retains all eight kinds and rejects every closed-record violation");
});
test("explicit portable controls retain the original refusal object and own long-copy cancellation", async () => {
  const codec = await load();
  const source = new ValueError(fixture.longMessage.kind as ValueRefusalKind, fixture.longMessage.unit.repeat(fixture.longMessage.repeat));
  for (const row of fixture.valid) {
    const cause = new ValueError(row.wire.kind as ValueRefusalKind, row.wire.message);
    const failed: Control = { checkpoint: async () => { throw cause; }, beginStage: async () => { throw cause; }, charge: async () => {}, step: async () => {}, advance: async () => {}, scopedStage: async operation => operation() };
    await assert.rejects(codec.encodeValueErrorControlled(source, failed), error => error === cause);
    await assert.rejects(codec.decodeValueErrorControlled(wire(source.kind, source.message), failed), error => error === cause);
  }
  for (const row of fixture.controls) {
    let callbacks = 0;
    const admission = new NativeDecodeControl(row.maximumBytes, () => ++callbacks < row.cancelAt || row.cancelAt === 0);
    const operation = row.operation === "encode" ? codec.encodeValueErrorControlled(source, admission) : codec.decodeValueErrorControlled(wire(source.kind, source.message), admission);
    await assert.rejects(operation, (error: unknown) => error instanceof ValueError && error.kind === row.expectedKind);
    if (row.cancelAt !== 0) assert.equal(callbacks, row.cancelAt);
  }
  assert.equal(source.kind, "invariantViolated");
  console.log("[DEBUG] explicit portable refusal control retains original object identity and long-copy cancellation");
});
