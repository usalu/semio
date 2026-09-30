import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import { type SnapshotEditEvent, type SnapshotValue } from "../../🟦️";
import { prepareSnapshotPatch, applySnapshotPatch, inverseSnapshotPatch } from "../🟦️";
import { semioSchemaAjvV1 } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";

const fixture = await Bun.file(new URL("../🧫️fixtures/🔣️.json", import.meta.url)).json() as {
  base: SnapshotValue;
  positionedInsertions: { key: string; index: number; keys: string[] }[];
  invalidPositions: number[];
  intrinsicObjectOrder: { before: SnapshotValue; key: string; index: number; expectedKeys: string[] }[];
  intrinsicRenames: { before: SnapshotValue; from: string; key: string }[];
  cases: { id: string; event: SnapshotEditEvent; oracle: Operation[] }[];
  rejected: { id: string; event: SnapshotEditEvent }[];
  large: { bytes: number; fill: number; metadataPath: string; value: string; maximumPatchBytes: number };
  payload: { before: number[]; cases: { id: string; event: SnapshotEditEvent; expected: number[] }[] };
  shiftedParent: { schema: object; before: SnapshotValue; event: SnapshotEditEvent; oracle: Operation[]; expected: SnapshotValue };
  nativePilots: Record<string, {
    directory: string;
    tagging: "adjacent" | "internal";
    discriminator: string;
    textOpcode: string;
    binaryTag: number;
    before: SnapshotValue;
    largeValuePath: string;
    largeValueBytes: number;
    event: Extract<SnapshotEditEvent, { operation: "setValue" }>;
    maximumPatchBytes: number;
  }>;
};
const registry = await Bun.file(new URL("../../../../🧬️schema/🔣️.json", import.meta.url)).json();
const patchRef = `${registry.$id}#/$defs/SnapshotPatch`;
const validate = semioSchemaAjvV1({ strict: true }).addSchema(registry).getSchema(patchRef)!;

describe("compact snapshot patches", () => {
  test("intrinsic object ordering preserves numeric keys and exact inverse values", () => {
    for (const row of fixture.intrinsicObjectOrder) {
      const patch = { edits: [{ path: [row.key], edit: { operation: "insertAt" as const, value: "Inserted", index: row.index } }] };
      const after = applySnapshotPatch(row.before, patch);
      expect(Object.keys(after as object)).toEqual(row.expectedKeys);
      expect(after).toEqual(applyPatch(structuredClone(row.before), [{ op: "add", path: `/${row.key}`, value: "Inserted" }]).newDocument);
      expect(JSON.stringify(applySnapshotPatch(after, inverseSnapshotPatch(row.before, patch)))).toBe(JSON.stringify(row.before));
    }
    for (const row of fixture.intrinsicRenames) {
      const patch = prepareSnapshotPatch(row.before, { operation: "renameKey", path: `/${row.from}`, key: row.key });
      const after = applySnapshotPatch(row.before, patch);
      expect(after).toEqual(applyPatch(structuredClone(row.before), [{ op: "move", from: `/${row.from}`, path: `/${row.key}` }]).newDocument);
      expect(JSON.stringify(applySnapshotPatch(after, inverseSnapshotPatch(row.before, patch)))).toBe(JSON.stringify(row.before));
    }
  });
  for (const [family, row] of Object.entries(fixture.nativePilots)) test(`${family} native patch schema and large field match independent oracles`, async () => {
    const before = applyPatch(structuredClone(row.before), [{ op: "replace", path: row.largeValuePath, value: "x".repeat(row.largeValueBytes) }], true, true, false).newDocument;
    const patch = prepareSnapshotPatch(before, row.event);
    const after = applySnapshotPatch(before, patch);
    expect(after).toEqual(applyPatch(structuredClone(before), [{ op: "replace", path: row.event.path, value: row.event.value }], true, true, false).newDocument);
    expect(applySnapshotPatch(after, inverseSnapshotPatch(before, patch))).toEqual(before);
    expect(new TextEncoder().encode(JSON.stringify(patch)).length).toBeLessThan(row.maximumPatchBytes);
    const root = new URL(`../../../../../🗿️artifacts/${row.directory}/🧬️schema/🧬️mutations/`, import.meta.url);
    const aggregate = await Bun.file(new URL("🔣️.json", root)).json();
    const leaf = await Bun.file(new URL("🩹️patch-snapshot/🧬️schema/🔣️.json", root)).json();
    const descriptor = await Bun.file(new URL("🩹️patch-snapshot/🔣️.json", root)).json();
    const protocol = await Bun.file(new URL("💾️binary/📡️.protocol.semio", root)).text();
    const branch = aggregate.oneOf.find((entry: { $ref?: string }) => entry.$ref === leaf.$id || JSON.stringify(entry).includes(JSON.stringify(row.discriminator)));
    expect(branch).toBeDefined();
    expect(leaf.properties.patch.$ref).toBe(patchRef);
    expect(descriptor.textOpcode).toBe(row.textOpcode);
    expect(descriptor.binaryTag).toBe(row.binaryTag);
    expect(protocol).toContain(`record ${row.textOpcode} tag=${row.binaryTag}`);
    const check = semioSchemaAjvV1({ strict: true }).addSchema(registry).addSchema(leaf).compile({ $id: aggregate.$id, ...branch });
    const mutation = row.tagging === "internal" ? { mutation: row.discriminator, patch } : { mutation: row.discriminator, payload: { patch } };
    expect(check(mutation), JSON.stringify(check.errors)).toBe(true);
  });

  test("positioned object insertion preserves the authored key order and rejects invalid positions", () => {
    for (const row of fixture.positionedInsertions) {
      const patch = { edits: [{ path: ["metadata", row.key], edit: { operation: "insertAt" as const, value: "Inserted", index: row.index } }] };
      expect(validate(patch)).toBe(true);
      const after = applySnapshotPatch(fixture.base, patch) as Record<string, SnapshotValue>;
      expect(Object.keys(after.metadata as object)).toEqual(row.keys);
      expect(after).toEqual(applyPatch(structuredClone(fixture.base), [{ op: "add", path: `/metadata/${row.key}`, value: "Inserted" }]).newDocument);
      expect(JSON.stringify(applySnapshotPatch(after, inverseSnapshotPatch(fixture.base, patch)))).toBe(JSON.stringify(fixture.base));
    }
    for (const index of fixture.invalidPositions) expect(() => applySnapshotPatch(fixture.base, { edits: [{ path: ["metadata", "bad"], edit: { operation: "insertAt", value: "Rejected", index } }] })).toThrow();
    expect(() => applySnapshotPatch(fixture.base, { edits: [{ path: ["list", "0"], edit: { operation: "insertAt", value: "Rejected", index: 0 } }] })).toThrow();
  });
  test("moves resolve the destination parent after removal", () => {
    const row = fixture.shiftedParent;
    const patch = prepareSnapshotPatch(row.before, row.event);
    const result = applySnapshotPatch(row.before, patch);
    expect(result).toEqual(row.expected);
    expect(result).toEqual(applyPatch(structuredClone(row.before), row.oracle, true, true, false).newDocument);
    expect(new Ajv({ strict: true }).compile(row.schema)(result)).toBe(true);
    expect(applySnapshotPatch(result, inverseSnapshotPatch(row.before, patch))).toEqual(row.before);
  });
  for (const row of fixture.payload.cases) test(row.id, () => {
    const patch = prepareSnapshotPatch(fixture.payload.before, row.event);
    const result = applySnapshotPatch(fixture.payload.before, patch);
    const operations = patch.edits.map(({ path, edit }): Operation => {
      const pointer = path.map((key) => `/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`).join("");
      return edit.operation === "remove" ? { op: "remove", path: pointer } : { op: edit.operation === "set" ? "replace" : "add", path: pointer, value: edit.value };
    });
    const oracle = applyPatch(structuredClone(fixture.payload.before), operations, true, true, false).newDocument;
    expect(validate(patch)).toBe(true);
    expect(result).toEqual(row.expected);
    expect(result).toEqual(oracle);
  });
  for (const row of fixture.cases) test(row.id, () => {
    const before = structuredClone(fixture.base);
    const patch = prepareSnapshotPatch(before, row.event);
    expect(validate(patch)).toBe(true);
    const inverse = inverseSnapshotPatch(before, patch);
    const actual = applySnapshotPatch(before, patch);
    const oracle = applyPatch(structuredClone(before), row.oracle, true, true, false).newDocument;
    expect(actual).toEqual(oracle);
    expect(applySnapshotPatch(actual, inverse)).toEqual(before);
    expect(JSON.stringify(applySnapshotPatch(actual, inverse))).toBe(JSON.stringify(before));
    expect(before).toEqual(fixture.base);
  });

  for (const row of fixture.rejected) test(row.id, () => {
    const before = structuredClone(fixture.base);
    expect(() => applySnapshotPatch(before, prepareSnapshotPatch(before, row.event))).toThrow();
    expect(before).toEqual(fixture.base);
  });

  test("metadata patches preserve a large payload by reference", () => {
    const bytes = Array(fixture.large.bytes).fill(fixture.large.fill);
    const before = { title: "Before", bytes };
    const patch = prepareSnapshotPatch(before, { operation: "setValue", path: fixture.large.metadataPath, value: fixture.large.value });
    const inverse = inverseSnapshotPatch(before, patch);
    const after = applySnapshotPatch(before, patch) as typeof before;
    expect(after.bytes).toBe(bytes);
    expect(after.title).toBe(fixture.large.value);
    expect(JSON.stringify(patch).length).toBeLessThan(fixture.large.maximumPatchBytes);
    expect(JSON.stringify(inverse).length).toBeLessThan(fixture.large.maximumPatchBytes);
    const reopened = applySnapshotPatch(after, inverse) as typeof before;
    expect(reopened.bytes).toBe(bytes);
    expect(reopened.title).toBe(before.title);
  });

  test("failed compound edits leave the original untouched", () => {
    const before = structuredClone(fixture.base);
    expect(() => applySnapshotPatch(before, { edits: [
      { path: ["title"], edit: { operation: "set", value: "Changed" } },
      { path: ["missing"], edit: { operation: "remove" } },
    ] })).toThrow();
    expect(before).toEqual(fixture.base);
  });
});
