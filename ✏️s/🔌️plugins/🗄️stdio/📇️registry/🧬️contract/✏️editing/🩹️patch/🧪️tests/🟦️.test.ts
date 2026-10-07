import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { applyPatch, getValueByPointer, type Operation } from "fast-json-patch";
import { type SnapshotEditEvent, type SnapshotValue } from "../../🟦️";
import { prepareSnapshotPatch, applySnapshotPatch, inverseSnapshotPatch, inverseSnapshotPatches, parseSnapshotPatch, snapshotSchemaLocation, type SnapshotPatch } from "../🟦️";
import { semioSchemaAjvV1 } from "../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";

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
  shiftedParent: { before: SnapshotValue; event: SnapshotEditEvent; oracle: Operation[]; expected: SnapshotValue };
  patches: Record<string, { patch: SnapshotPatch; inverse: SnapshotPatch }>;
  chunkedInverses: { budget: number; cases: { id: string; base: SnapshotValue; patch: SnapshotPatch; parts: SnapshotPatch[] }[] };
  locations: { snapshot: string; documents: { $id: string }[]; instance: SnapshotValue; cases: { id: string; path: string; insert?: boolean; expected: string | null; valid?: SnapshotValue; invalid?: SnapshotValue }[] };
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

const escape = (segment: string): string => segment.replaceAll("~", "~0").replaceAll("/", "~1");

/** 🔁️ The RFC 6902 operations one part performs on `document` (a splice as removals plus additions, a text splice as the replaced
 * text), so fast-json-patch replays the parts independently of the twin. */
function rfc6902(document: SnapshotValue, original: SnapshotPatch): Operation[] {
  const part = structuredClone(original);
  if (part.operation === "set") return [{ op: "replace", path: part.path, value: part.value }];
  if (part.operation === "insert") return [{ op: "add", path: part.path, value: part.value }];
  if (part.operation !== "splice") throw new Error(`pointer-only part ${part.operation}`);
  const container = part.path === "" ? document : getValueByPointer(document, part.path) as SnapshotValue;
  if (Array.isArray(container)) return [...Array.from({ length: part.remove }, (): Operation => ({ op: "remove", path: `${part.path}/${part.offset}` })), ...(part.value as SnapshotValue[]).map((item, position): Operation => ({ op: "add", path: `${part.path}/${part.offset + position}`, value: item }))];
  if (typeof container === "string") {
    const bytes = new TextEncoder().encode(container);
    const text = new TextDecoder().decode(new Uint8Array([...bytes.slice(0, part.offset), ...new TextEncoder().encode(part.value as string), ...bytes.slice(part.offset + part.remove)]));
    return [{ op: "replace", path: part.path, value: text }];
  }
  const keys = Object.keys(container as object).slice(part.offset, part.offset + part.remove);
  return [...keys.map((key): Operation => ({ op: "remove", path: `${part.path}/${escape(key)}` })), ...Object.entries(part.value as object).map(([key, member]): Operation => ({ op: "add", path: `${part.path}/${escape(key)}`, value: member }))];
}

describe("compact snapshot patches", () => {
  for (const row of fixture.chunkedInverses.cases) test(`chunked exact inverse ${row.id} matches the Rust planner and replays through fast-json-patch`, () => {
    const { budget } = fixture.chunkedInverses;
    const parts = inverseSnapshotPatches(row.base, row.patch, budget);
    expect(parts).toEqual(row.parts);
    const after = applySnapshotPatch(row.base, row.patch);
    expect(JSON.stringify(parts.reduce(applySnapshotPatch, after))).toBe(JSON.stringify(row.base));
    let oracle = structuredClone(after);
    for (const part of parts) oracle = applyPatch(oracle, rfc6902(oracle, part), true, true, false).newDocument;
    expect(oracle).toEqual(row.base);
    for (const [position, part] of parts.entries()) {
      expect(validate(part), JSON.stringify(validate.errors)).toBe(true);
      expect(parseSnapshotPatch(JSON.parse(JSON.stringify(part)))).toEqual(part);
      expect(new TextEncoder().encode(JSON.stringify(part)).length).toBeLessThanOrEqual(budget);
      expect(part.operation !== "splice" || (part.continued === true) === (position < parts.length - 1)).toBe(true);
    }
  });
  test("an inverse needing more parts than its bound is refused, never truncated", () => {
    const base = { words: Array.from({ length: 4000 }, (_, position) => `word-${String(position).padStart(5, "0")}`) };
    expect(() => inverseSnapshotPatches(base, { operation: "set", path: "/words", value: [] }, 160)).toThrow(/bounded number of parts/u);
  });
  test("intrinsic object ordering preserves numeric keys and exact inverse values", () => {
    for (const row of fixture.intrinsicObjectOrder) {
      const patch: SnapshotPatch = { operation: "insert", path: `/${row.key}`, value: "Inserted", index: row.index };
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
    const protocol = await Bun.file(new URL("../../🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio", root)).text();
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
      const patch: SnapshotPatch = { operation: "insert", path: `/metadata/${row.key}`, value: "Inserted", index: row.index };
      expect(validate(patch)).toBe(true);
      const after = applySnapshotPatch(fixture.base, patch) as Record<string, SnapshotValue>;
      expect(Object.keys(after.metadata as object)).toEqual(row.keys);
      expect(after).toEqual(applyPatch(structuredClone(fixture.base), [{ op: "add", path: `/metadata/${row.key}`, value: "Inserted" }]).newDocument);
      expect(JSON.stringify(applySnapshotPatch(after, inverseSnapshotPatch(fixture.base, patch)))).toBe(JSON.stringify(fixture.base));
    }
    for (const index of fixture.invalidPositions) expect(() => applySnapshotPatch(fixture.base, { operation: "insert", path: "/metadata/bad", value: "Rejected", index })).toThrow();
    expect(() => applySnapshotPatch(fixture.base, { operation: "insert", path: "/list/0", value: "Rejected", index: 0 })).toThrow();
  });
  test("moves resolve the destination parent after removal", async () => {
    const row = fixture.shiftedParent;
    const patch = prepareSnapshotPatch(row.before, row.event);
    const result = applySnapshotPatch(row.before, patch);
    expect(result).toEqual(row.expected);
    expect(result).toEqual(applyPatch(structuredClone(row.before), row.oracle, true, true, false).newDocument);
    expect(new Ajv({ strict: true }).compile(await Bun.file(new URL("../🧪️testing/🌲️mixed-parent/🧬️schema/🔣️.json", import.meta.url)).json())(result)).toBe(true);
    expect(applySnapshotPatch(result, inverseSnapshotPatch(row.before, patch))).toEqual(row.before);
  });
  for (const row of fixture.payload.cases) test(row.id, () => {
    const patch = prepareSnapshotPatch(fixture.payload.before, row.event);
    const result = applySnapshotPatch(fixture.payload.before, patch);
    const operations: Operation[] = patch.operation === "remove" ? [{ op: "remove", path: patch.path }]
      : patch.operation === "move" ? [{ op: "move", from: patch.from, path: patch.path }]
      : patch.operation === "rename" ? [{ op: "move", from: patch.path, path: `${patch.path.slice(0, patch.path.lastIndexOf("/"))}/${patch.key}` }]
      : [{ op: patch.operation === "set" ? "replace" : "add", path: patch.path, value: patch.value }];
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
    expect(patch).toEqual(fixture.patches[row.id]!.patch);
    expect(inverse).toEqual(fixture.patches[row.id]!.inverse);
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

  test("failed path operations leave the original untouched", () => {
    const before = structuredClone(fixture.base);
    for (const patch of [{ operation: "remove", path: "/missing" }, { operation: "move", from: "/list/0", path: "/missing/0" }, { operation: "rename", path: "/list/0", key: "first" }] as SnapshotPatch[]) {
      expect(() => applySnapshotPatch(before, patch)).toThrow();
      expect(before).toEqual(fixture.base);
    }
  });

  test("every canonical patch and inverse meets the registry SnapshotPatch schema", () => {
    for (const [id, row] of Object.entries(fixture.patches)) {
      expect(validate(row.patch), id).toBe(true);
      expect(validate(row.inverse), id).toBe(true);
    }
  });

  test("the wire reader admits exactly what the registry SnapshotPatch schema admits (ajv oracle)", () => {
    for (const [id, row] of Object.entries(fixture.patches)) {
      expect(parseSnapshotPatch(JSON.parse(JSON.stringify(row.patch))), id).toEqual(row.patch);
      expect(parseSnapshotPatch(JSON.parse(JSON.stringify(row.inverse))), id).toEqual(row.inverse);
    }
    const refused: unknown[] = [null, [], { operation: "replace", path: "/a", value: 1 }, { operation: "set", path: "/a" }, { operation: "remove", path: "/a", value: 1 }, { operation: "rename", path: "/a", key: 7 }, { operation: "insert", path: "/a", value: 1, index: -1 }, { operation: "move", from: "/a", path: "/b", index: 1.5 },
      { operation: "splice", path: "/a", offset: 0, value: [] }, { operation: "splice", path: "/a", offset: -1, remove: 0, value: [] }, { operation: "splice", path: "/a", offset: 0, remove: 0, value: [], continued: "yes" }, { operation: "splice", path: "/a", offset: 0, remove: 0, value: 1 }, { operation: "set", path: "/a", value: 1, continued: true }];
    for (const value of refused) {
      expect(validate(value), JSON.stringify(value)).toBe(false);
      expect(() => parseSnapshotPatch(value), JSON.stringify(value)).toThrow();
    }
  });

  test("snapshot schema locations type exactly the values the whole-document schema admits there (ajv oracle)", () => {
    const { snapshot, documents, instance, cases } = fixture.locations;
    const resolve = (id: string) => documents.find((document) => document.$id === id);
    const ajv = semioSchemaAjvV1({ strict: true });
    for (const document of documents) ajv.addSchema(document);
    const whole = ajv.getSchema(snapshot)!;
    expect(whole(instance), JSON.stringify(whole.errors)).toBe(true);
    for (const row of cases) {
      const location = snapshotSchemaLocation(snapshot, row.path === "" ? [] : row.path.slice(1).split("/"), resolve);
      const reference = location === undefined ? null : location.pointer === "" ? location.document : `${location.document}#${location.pointer}`;
      expect(reference, row.id).toBe(row.expected);
      if (row.expected === null) continue;
      const part = ajv.compile({ $ref: row.expected });
      expect(part(row.valid), row.id).toBe(true);
      expect(part(row.invalid), row.id).toBe(false);
      const place = (value: SnapshotValue) => row.path === "" ? value : applyPatch(structuredClone(instance), [{ op: row.insert ? "add" : "replace", path: row.path, value }], true, false, false).newDocument;
      expect(whole(place(row.valid!)), row.id).toBe(true);
      expect(whole(place(row.invalid!)), row.id).toBe(false);
    }
  });
});
